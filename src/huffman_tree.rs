use core::panic;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs::{self, File},
    io::Write,
    ops::Index,
};

const INTERNAL_NODE_VALUE: char = '\0';
// const INTERNAL_NODE_VALUE: char = '\u{1}';

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct FrequencyChar(pub char, pub usize);

#[derive(Debug, Eq, PartialEq)]
struct HeapNode {
    pub frequency: usize,
    pub c: char,
    pub left: Option<Box<HeapNode>>,
    pub right: Option<Box<HeapNode>>,
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    // pub frequency: usize,
    pub c: char,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.frequency.partial_cmp(&self.frequency)
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl HeapNode {
    pub fn new(c: char, frequency: usize) -> Self {
        HeapNode {
            c,
            frequency,
            left: None,
            right: None,
        }
    }

    pub fn convert_to_node(&self) -> Node {
        let mut node = Node::new(self.c);

        if let Some(l) = &self.left {
            if l.c == INTERNAL_NODE_VALUE {
                node.left = Some(Box::new(l.convert_to_node()));
            } else {
                node.left = Some(Box::new(Node::new(l.c)));
            }
        }

        if let Some(r) = &self.right {
            if r.c == INTERNAL_NODE_VALUE {
                node.right = Some(Box::new(r.convert_to_node()));
            } else {
                node.right = Some(Box::new(Node::new(r.c)));
            }
        }

        node
    }
}

impl Node {
    pub fn new(c: char) -> Self {
        Node {
            c,
            left: None,
            right: None,
        }
    }

    pub fn count(&self) -> usize {
        let mut count = 1;

        if let Some(l) = &self.left {
            count += l.count();
        }

        // get right sub encodings
        if let Some(r) = &self.right {
            count += r.count();
        }

        count
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

    pub fn print_as_tree(&self, prefix: &str, is_left: bool) {
        print!("{}", prefix);

        if is_left {
            print!("├──");
        } else {
            print!("└──")
        }

        println!("({})", self.c);

        let new_prefix = if is_left {
            format!("{}|   ", prefix)
        } else {
            format!("{}    ", prefix)
        };

        if let Some(l) = &self.left {
            l.print_as_tree(&new_prefix, true);
        }

        if let Some(r) = &self.right {
            r.print_as_tree(&new_prefix, false);
        }
    }

    pub fn convert_to_vec(&self, values: &mut Vec<char>) {
        values.push(self.c);

        if let Some(l) = &self.left {
            l.convert_to_vec(values);
        }

        if let Some(r) = &self.right {
            r.convert_to_vec(values);
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
            min_heap.push(HeapNode::new(f.0, f.1));
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
            let mut top = HeapNode::new(INTERNAL_NODE_VALUE, new_frequency);

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

        let root = min_heap.pop().unwrap().convert_to_node();

        // DEBUG
        // println!("value: {}, frequency: {}", root.c, root.frequency);

        let mut tree = HuffmanTree {
            root,
            encoding: HashMap::new(),
        };

        tree.set_encoding();

        tree
    }

    pub fn load_tree_from_bytes(bytes: &[u8]) -> HuffmanTree {
        let mut map: HashMap<char, usize> = HashMap::new();

        for &c in bytes {
            let entry = map.get_mut(&(c as char));

            match entry {
                Some(value) => {
                    *value += 1;
                }
                None => {
                    map.insert(c as char, 1);
                }
            }
        }

        let mut frequencies: Vec<FrequencyChar> = map
            .iter()
            .map(|(c, freq)| FrequencyChar(*c, *freq))
            .collect();

        HuffmanTree::new(&mut frequencies)
    }

    pub fn len(&self) -> usize {
        self.root.count()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = self.convert_to_vec().iter().map(|&c| c as u8).collect();

        bytes
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

    pub fn print_tree(&self) {
        self.root.print_as_tree("", false);
    }

    pub fn convert_to_vec(&self) -> Vec<char> {
        let mut values: Vec<char> = Vec::new();

        self.root.convert_to_vec(&mut values);

        values
    }

    pub fn save_as_file(&self, file_path: &str) {
        let tree = self.convert_to_vec();
        let bytes: Vec<u8> = tree.iter().map(|c| *c as u8).collect();

        let mut output_f = File::create(&file_path)
            .expect("Failed to create file in src/huffman_tree.rs => fn save_as_file");

        output_f
            .write_all(&bytes)
            .expect("Failed to write to file in src/huffman_tree.rs => fn save_as_file");

        output_f
            .flush()
            .expect("Failed to flush in src/huffman_tree.rs => fn save_as_file");
    }

    pub fn load_from_file(file_path: &str) -> Self {
        let decoded_tree: Vec<char> = fs::read(file_path)
            .expect("Failed to read file in src/huffman_tree.rs => fn load_from_file")
            .iter()
            .map(|byte| *byte as char)
            .collect();

        HuffmanTree::from(decoded_tree)
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

fn node_from_vec(values: &Vec<char>, index: usize) -> Node {
    let c = values[index];

    let mut left = None;
    let mut right = None;

    // Last Value
    if index + 1 == values.len() {
        return Node::new(values[index]);
    }

    if values[index] == INTERNAL_NODE_VALUE {
        left = Some(Box::new(node_from_vec(values, index + 1)));

        // We know that the next right value is after all the values of the left
        // node so we just need to skip them
        let count = left.as_ref().unwrap().count();
        right = Some(Box::new(node_from_vec(values, index + 1 + count)));
    }

    let mut node = Node::new(c);
    node.left = left;
    node.right = right;

    node
}

impl From<Vec<char>> for HuffmanTree {
    fn from(value: Vec<char>) -> Self {
        let mut tree = HuffmanTree {
            root: node_from_vec(&value, 0),
            encoding: HashMap::new(),
        };

        tree.set_encoding();

        tree
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
        // let encoded: Vec<u8> = vec![
        //      /*f*/ 0,
        //      /*a*/ 1, 1, 0, 0,
        //      /*c*/ 1, 0, 0,
        //      /*e*/ 1, 1, 1,
        //      /*d*/ 1, 0, 1];

        // Should be: 0b0110_0100 0b1111_01--
        //                8 char +  6 char = 14 char
        let encoded: [u8; 2] = [0b0110_0100, 0b1111_0100];
        let decoded = tree.decode(&encoded, 14);

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

    #[test]
    fn tree_len() {
        let mut array = vec![
            FrequencyChar('a', 5),
            FrequencyChar('b', 9),
            FrequencyChar('c', 12),
            FrequencyChar('d', 13),
            FrequencyChar('e', 16),
            FrequencyChar('f', 45),
        ];

        let tree = HuffmanTree::new(&mut array);

        let len = tree.len();

        assert_eq!(11, len);
    }

    #[test]
    fn converting_tree_to_array() {
        // let mut array = vec![
        //     FrequencyChar('a', 5),
        //     FrequencyChar('b', 9),
        //     FrequencyChar('c', 12),
        //     FrequencyChar('d', 13),
        //     FrequencyChar('e', 16),
        //     FrequencyChar('f', 45),
        //     FrequencyChar('g', 46),
        //     FrequencyChar('h', 47),
        //     FrequencyChar('i', 48),
        //     FrequencyChar('j', 49),
        //     FrequencyChar('k', 50),
        //     FrequencyChar('l', 51),
        // ];
        let mut array = vec![
            FrequencyChar('a', 5),
            FrequencyChar('b', 9),
            FrequencyChar('c', 12),
            FrequencyChar('d', 13),
            FrequencyChar('e', 16),
            FrequencyChar('f', 45),
        ];

        let tree = HuffmanTree::new(&mut array);

        // DEBUG
        // tree.print_tree();

        let values = tree.convert_to_vec();
        println!("values: {:?}", values);

        let new_tree: HuffmanTree = HuffmanTree::from(values);

        // DEBUG
        // new_tree.print_tree();

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
