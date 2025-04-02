use ::bincode::{Decode, Encode};
use core::panic;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufReader, BufWriter},
    ops::Index,
};

const INTERNAL_NODE_VALUE: char = '\0';

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct FrequencyChar(pub char, pub usize);

#[derive(Debug, Eq, PartialEq, Decode, Encode)]
struct Node {
    pub frequency: usize,
    pub c: char,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.frequency.partial_cmp(&self.frequency)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl Node {
    pub fn new(c: char, frequency: usize) -> Self {
        Node {
            c,
            frequency,
            left: None,
            right: None,
        }
    }

    pub fn get_encoding(&self, encoding: Vec<u8>) -> Vec<(char, Vec<u8>)> {
        if self.c != INTERNAL_NODE_VALUE {
            return vec![(self.c, encoding)];
        } else {
            let mut sub_encodings: Vec<(char, Vec<u8>)> = Vec::new();

            // get left sub encodings
            if let Some(l) = &self.left {
                let mut new_encoding = encoding.clone();
                new_encoding.push(0);
                let mut left_sub_encodings = l.get_encoding(new_encoding);
                sub_encodings.append(&mut left_sub_encodings);
            }

            // get right sub encodings
            if let Some(r) = &self.right {
                let mut new_encoding = encoding.clone();
                new_encoding.push(1);
                let mut right_sub_encodings = r.get_encoding(new_encoding);
                sub_encodings.append(&mut right_sub_encodings);
            }

            return sub_encodings;
        }
    }

    pub fn print_encoding(&self, encoding: Vec<u8>) {
        if self.c != INTERNAL_NODE_VALUE {
            print!("{}: ", self.c);

            for bit in encoding {
                print!("{}", bit);
            }

            println!();
        } else {
            if let Some(l) = &self.left {
                let mut new_encoding = encoding.clone();
                new_encoding.push(0);
                l.print_encoding(new_encoding);
            }

            if let Some(r) = &self.right {
                let mut new_encoding = encoding.clone();
                new_encoding.push(1);
                r.print_encoding(new_encoding);
            }
        }
    }
}

pub struct HuffmanTree {
    root: Node,
    encoding: HashMap<char, Vec<u8>>,
}

impl HuffmanTree {
    pub fn new(frequencies: &mut Vec<FrequencyChar>) -> Self {
        let mut min_heap = BinaryHeap::new();

        for f in frequencies {
            min_heap.push(Node::new(f.0, f.1));
        }

        // DEBUG
        // while (min_heap.peek() != None) {
        //     let node = min_heap.pop().unwrap();
        //     println!("CHAR: {}, FREQ: {}", node.c, node.frequency);
        // }

        while min_heap.len() > 1 {
            let left = min_heap.pop();
            let right = min_heap.pop();

            // Sum the new frequency based on the children of the node
            let new_frequency = match (&left, &right) {
                (Some(l), Some(r)) => l.frequency + r.frequency,
                (None, Some(r)) => r.frequency,
                (Some(l), None) => l.frequency,
                (None, None) => panic!("This is not supposed to be possible"),
            };

            // INTERVAL_NODE_VALUE is a special value that distinguied internal node from leaf
            let mut top = Node::new(INTERNAL_NODE_VALUE, new_frequency);

            // update left node
            top.left = if let Some(l) = left {
                Some(Box::new(l))
            } else {
                None
            };

            // update right node
            top.right = if let Some(r) = right {
                Some(Box::new(r))
            } else {
                None
            };

            // push a new internal node into the heap
            min_heap.push(top);
        }

        let root = min_heap.pop().unwrap();

        // DEBUG
        // println!("value: {}, frequency: {}", root.c, root.frequency);

        let mut tree = HuffmanTree {
            root,
            encoding: HashMap::new(),
        };

        tree.set_encoding();

        tree
    }

    pub fn get_encoding(&self) -> Vec<(char, Vec<u8>)> {
        self.root.get_encoding(Vec::new())
    }

    pub fn decode(&self, bytes: &[u8], bit_length: usize) -> String {
        let mut node = &self.root;
        let mut decoded = Vec::new();
        let mut visited_bits = 0;

        for byte in bytes {
            for i in (0..8).rev() {
                if visited_bits >= bit_length {
                    break;
                }

                let bit = (byte >> i) & 1;
                // DEBUG
                // println!("byte: {:#010b}", byte);
                // println!("byte >> i: {:#010b}", byte >> i);
                // println!("(byte >> i) & 1: {:#010b}", (byte >> i) & 1);
                // println!("(byte >> i) & 1: {}", (byte >> i) & 1);
                // println!("bit: {}", bit);
                // println!("bit: {:#010b}\n", bit);

                match bit {
                    0 => node = node.left.as_ref().unwrap(),
                    1 => node = node.right.as_ref().unwrap(),
                    _ => {
                        panic!("Cannot index encoding with a non binary value")
                    }
                }

                if node.c != INTERNAL_NODE_VALUE {
                    // DEBUG
                    // println!("char: {}", node.c);
                    decoded.push(node.c as u8);
                    node = &self.root;
                }

                visited_bits += 1;
            }
        }

        String::from_utf8(decoded).unwrap()
    }

    fn set_encoding(&mut self) {
        let encoding = self.get_encoding();

        for (c, bits) in encoding {
            self.encoding.insert(c, bits);
        }
    }

    pub fn print_encoding(&self) {
        self.root.print_encoding(Vec::new());
    }

    pub fn save_as_file(&self, filepath: &str) {
        let file = File::create(filepath).expect("Failed to create file");
        let mut writer = BufWriter::new(file);

        // bincode::serialize_into(&mut writer, &self.root).expect("Failed to read JSON");
        bincode::encode_into_std_write(&self.root, &mut writer, bincode::config::standard())
            .expect("Failed to write the Huffman Tree to a file");
    }

    pub fn load_from_file(filepath: &str) -> Self {
        let file = File::open(filepath).expect("Failed to open file");
        let mut reader = BufReader::new(file);

        // decoding from the given file
        let new_root: Node =
            bincode::decode_from_std_read(&mut reader, bincode::config::standard())
                .expect("Failed to read JSON");
        // self.root = new_root;

        let mut tree = HuffmanTree {
            root: new_root,
            encoding: HashMap::new(),
        };

        tree.set_encoding();

        tree
    }
}

impl Index<char> for HuffmanTree {
    type Output = Vec<u8>;

    fn index(&self, index: char) -> &Self::Output {
        self.encoding
            .get(&index)
            .expect("No encoding exist for this character")
    }
}

impl Index<Vec<u8>> for HuffmanTree {
    type Output = char;

    fn index(&self, index: Vec<u8>) -> &Self::Output {
        let mut node = &self.root;

        for bit in index {
            match bit {
                0 => node = node.left.as_ref().unwrap(),
                1 => node = node.right.as_ref().unwrap(),
                _ => {
                    panic!("Cannot index encoding with a non binary value")
                }
            }
        }

        &node.c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Big shout out to geeksforgeeks
    // https://www.geeksforgeeks.org/huffman-coding-greedy-algo-3/
    #[test]
    fn get_encoding() {
        let mut array = vec![
            FrequencyChar('a', 5),
            FrequencyChar('b', 9),
            FrequencyChar('c', 12),
            FrequencyChar('d', 13),
            FrequencyChar('e', 16),
            FrequencyChar('f', 45),
        ];

        let tree = HuffmanTree::new(&mut array);

        let encoding = tree.get_encoding();

        // Should be:
        // f: 0
        // c: 100
        // d: 101
        // a: 1100
        // b: 1101
        // e: 111
        assert_eq!(encoding[0], ('f', vec![0]));
        assert_eq!(encoding[1], ('c', vec![1, 0, 0]));
        assert_eq!(encoding[2], ('d', vec![1, 0, 1]));
        assert_eq!(encoding[3], ('a', vec![1, 1, 0, 0]));
        assert_eq!(encoding[4], ('b', vec![1, 1, 0, 1]));
        assert_eq!(encoding[5], ('e', vec![1, 1, 1]));
    }

    #[test]
    fn testing_indexing_by_char() {
        let mut array = vec![
            FrequencyChar('a', 5),
            FrequencyChar('b', 9),
            FrequencyChar('c', 12),
            FrequencyChar('d', 13),
            FrequencyChar('e', 16),
            FrequencyChar('f', 45),
        ];

        let tree = HuffmanTree::new(&mut array);

        // let encoding = tree.get_encoding();

        // Should be:
        // f: 0
        // c: 100
        // d: 101
        // a: 1100
        // b: 1101
        // e: 111
        assert_eq!(tree['f'], vec![0]);
        assert_eq!(tree['c'], vec![1, 0, 0]);
        assert_eq!(tree['d'], vec![1, 0, 1]);
        assert_eq!(tree['a'], vec![1, 1, 0, 0]);
        assert_eq!(tree['b'], vec![1, 1, 0, 1]);
        assert_eq!(tree['e'], vec![1, 1, 1]);
    }

    #[test]
    fn testing_indexing_by_encoding() {
        let mut array = vec![
            FrequencyChar('a', 5),
            FrequencyChar('b', 9),
            FrequencyChar('c', 12),
            FrequencyChar('d', 13),
            FrequencyChar('e', 16),
            FrequencyChar('f', 45),
        ];

        let tree = HuffmanTree::new(&mut array);

        // let encoding = tree.get_encoding();

        // Should be:
        // f: 0
        // c: 100
        // d: 101
        // a: 1100
        // b: 1101
        // e: 111
        assert_eq!(tree[vec![0]], 'f');
        assert_eq!(tree[vec![1, 0, 0]], 'c');
        assert_eq!(tree[vec![1, 0, 1]], 'd');
        assert_eq!(tree[vec![1, 1, 0, 0]], 'a');
        assert_eq!(tree[vec![1, 1, 0, 1]], 'b');
        assert_eq!(tree[vec![1, 1, 1]], 'e');
    }

    #[test]
    fn test_decode() {
        let mut array = vec![
            FrequencyChar('a', 5),
            FrequencyChar('b', 9),
            FrequencyChar('c', 12),
            FrequencyChar('d', 13),
            FrequencyChar('e', 16),
            FrequencyChar('f', 45),
        ];

        let tree = HuffmanTree::new(&mut array);

        // We wish to encode 'faced'
        #[cfg_attr(any(), rustfmt::skip)]
        let encoded: Vec<u8> = vec![
             /*f*/ 0,
             /*a*/ 1, 1, 0, 0,
             /*c*/ 1, 0, 0,
             /*e*/ 1, 1, 1,
             /*d*/ 1, 0, 1];

        // Should be: 0b0110_0100 0b1111_01--
        //                8 char +  6 char = 14 char
        let encoded: [u8; 2] = [0b0110_0100, 0b1111_0100];
        let decoded = tree.decode(&encoded, 14);
        println!("DECODED: {}", decoded);

        assert_eq!(decoded, String::from("faced"));
    }

    #[test]
    fn test_save_n_load_tree() {
        let mut array = vec![
            FrequencyChar('a', 5),
            FrequencyChar('b', 9),
            FrequencyChar('c', 12),
            FrequencyChar('d', 13),
            FrequencyChar('e', 16),
            FrequencyChar('f', 45),
        ];

        let original_tree = HuffmanTree::new(&mut array);

        let filename = "tests/test_saved_huffman_tree";
        original_tree.save_as_file(filename);

        let new_tree = HuffmanTree::load_from_file(filename);
        // let encoding = tree.get_encoding();

        // Should be:
        // f: 0
        // c: 100
        // d: 101
        // a: 1100
        // b: 1101
        // e: 111
        assert_eq!(new_tree['f'], vec![0]);
        assert_eq!(new_tree['c'], vec![1, 0, 0]);
        assert_eq!(new_tree['d'], vec![1, 0, 1]);
        assert_eq!(new_tree['a'], vec![1, 1, 0, 0]);
        assert_eq!(new_tree['b'], vec![1, 1, 0, 1]);
        assert_eq!(new_tree['e'], vec![1, 1, 1]);
    }
}
