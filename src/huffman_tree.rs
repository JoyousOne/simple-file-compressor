use std::{cmp::Ordering, collections::BinaryHeap};

const INTERNAL_NODE_VALUE: char = '\0';

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct FrequencyChar(pub char, pub usize);

#[derive(Debug, Eq, PartialEq)]
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

        while (min_heap.len() > 1) {
            let mut left = min_heap.pop();
            let mut right = min_heap.pop();

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

        HuffmanTree { root }
    }

    pub fn get_encoding(&self) -> Vec<(char, Vec<u8>)> {
        self.root.get_encoding(Vec::new())
    }

    pub fn print_encoding(&self) {
        self.root.print_encoding(Vec::new());
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
}
