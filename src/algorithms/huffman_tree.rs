use core::panic;
use std::{
    cmp::{Eq, Ordering},
    collections::{BinaryHeap, HashMap},
    fs::{self, File},
    io::Write,
    ops::Index,
};

use crate::{
    compressed_buffer::{Bit, CompressedBuffer},
    varsize::{encode_varsize, get_first_decoded},
};

// const INTERNAL_NODE_VALUE: char = 255 as char;

const LEAF_NULL_CHAR: char = '\0';
// FIXME changing value to 1 from 255 is a temporary fix
const ENCODED_NULL_CHAR: char = 255 as char;
// const ENCODED_NULL_CHAR: char = 1 as char;
const INTERNAL_NODE_VALUE: char = '\0';

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct FrequencyChar(pub char, pub usize);

#[derive(Debug, Eq, PartialEq)]
struct HeapNode {
    pub frequency: usize,
    pub c: Option<char>,
    pub left: Option<Box<HeapNode>>,
    pub right: Option<Box<HeapNode>>,
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    pub c: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl PartialOrd for HeapNode {
    // fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    //     other.frequency.partial_cmp(&self.frequency)
    // }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.frequency.partial_cmp(&self.frequency) {
            Some(Ordering::Equal) => {
                // We prioritize the node with nothing inside
                match (self.c.is_none(), other.c.is_none()) {
                    (true, false) => Some(Ordering::Greater),
                    (false, true) => Some(Ordering::Less),
                    _ => Some(Ordering::Equal),
                }
            }
            other => other,
        }
    }
}

impl Ord for HeapNode {
    // fn cmp(&self, other: &Self) -> Ordering {
    //     other.frequency.cmp(&self.frequency)
    // }

    fn cmp(&self, other: &Self) -> Ordering {
        match other.frequency.cmp(&self.frequency) {
            Ordering::Equal => {
                // We prioritize the node with nothing inside
                match (self.c.is_none(), other.c.is_none()) {
                    (true, false) => Ordering::Greater,
                    (false, true) => Ordering::Less,
                    _ => Ordering::Equal,
                }
            }
            other => other,
        }
    }
}

impl HeapNode {
    pub fn new(c: Option<char>, frequency: usize) -> Self {
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
            if let Some(c) = l.c {
                node.left = Some(Box::new(Node::new(Some(c))));
            } else {
                node.left = Some(Box::new(l.convert_to_node()));
            }
        }

        if let Some(r) = &self.right {
            if let Some(c) = r.c {
                node.right = Some(Box::new(Node::new(Some(c))));
            } else {
                node.right = Some(Box::new(r.convert_to_node()));
            }
        }

        node
    }
}

impl Node {
    pub fn new(c: Option<char>) -> Self {
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
    pub fn get_encoding(&self, encoding: Vec<Bit>) -> Vec<(char, Vec<Bit>)> {
        if let Some(c) = self.c {
            return vec![(c, encoding)];
        } else {
            let mut sub_encodings: Vec<(char, Vec<Bit>)> = Vec::new();

            // get left sub encodings
            if let Some(l) = &self.left {
                let mut new_encoding = encoding.clone();
                new_encoding.push(Bit::ZERO);
                let mut left_sub_encodings = l.get_encoding(new_encoding);
                sub_encodings.append(&mut left_sub_encodings);
            }

            // get right sub encodings
            if let Some(r) = &self.right {
                let mut new_encoding = encoding.clone();
                new_encoding.push(Bit::ONE);
                let mut right_sub_encodings = r.get_encoding(new_encoding);
                sub_encodings.append(&mut right_sub_encodings);
            }

            return sub_encodings;
        }
    }

    pub fn print_encoding(&self, encoding: Vec<u8>) {
        if let Some(c) = self.c {
            print!("'{}':    \t", c.escape_default());
            // print!("{}: ", self.c);

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

        match self.c {
            Some(c) => println!("('{}')", c.escape_default()),
            // Some(c) => println!("({})", c as u8),
            None => println!("()"),
        }

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
        // values.push(self.c);
        match self.c {
            // TODO doit prendre en compte le scénario où le leaf est 0
            Some(LEAF_NULL_CHAR) => {
                for _ in 0..2 {
                    values.push(ENCODED_NULL_CHAR);
                    // values.push(1 as char);
                }
            }
            Some(c) => values.push(c),
            None => values.push(INTERNAL_NODE_VALUE),
        }

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
    encoding: HashMap<char, Vec<Bit>>,
}

// DEBUG use to see bad encoding
impl PartialEq for HuffmanTree {
    fn eq(&self, other: &Self) -> bool {
        let print_error = |c: &char, bits: &Vec<Bit>, other_bits: &Vec<Bit>| {
            println!("left: '{}': {bits:?}", c.escape_default());
            println!("right: '{}': {other_bits:?}", c.escape_default());
        };

        let mut valid = true;
        for (c, bits) in &self.encoding {
            // let other_bits = &other[c];
            let other_bits = match other.encoding.get(c) {
                Some(other_bits) => other_bits,
                None => {
                    println!(
                        "right does not have an encoding for the char {}",
                        c.escape_default()
                    );
                    // return false;
                    valid = false;
                    continue;
                }
            };
            if bits.len() != other_bits.len() {
                print_error(c, bits, other_bits);
                // return false;
                valid = false;
            } else {
                for i in 0..bits.len() {
                    if bits[i] != other_bits[i] {
                        print_error(c, bits, other_bits);
                        // return false;
                        valid = false;
                    }
                }
            }
        }

        if self.encoding.len() != other.encoding.len() {
            println!("different encoding length");
            return false;
        }

        valid
    }
}

impl HuffmanTree {
    pub fn new(frequencies: &mut Vec<FrequencyChar>) -> Self {
        let mut min_heap = BinaryHeap::new();

        for f in frequencies {
            min_heap.push(HeapNode::new(Some(f.0), f.1));
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

            // Internal None do not possess a char
            let mut top = HeapNode::new(None, new_frequency);

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

    pub fn get_encoding(&self) -> Vec<(char, Vec<Bit>)> {
        self.root.get_encoding(Vec::new())
    }

    /// encode given bytes
    ///
    /// @**returns** (usize, Vec\<u8\>) => (number of bits encoded, the encoded bytes)
    pub fn encode(&self, bytes: &[u8]) -> (usize, Vec<u8>) {
        let mut compressed_buffer = CompressedBuffer::new();
        let mut num_bits: usize = 0;

        for &byte in bytes {
            let new_bits = self[byte as char].clone();

            for bit in new_bits {
                compressed_buffer.push_bit(bit);
                num_bits += 1;
            }
        }

        (num_bits, compressed_buffer.get_buffer())
    }

    /// return the encoding preceded by the tree content and the size of said tree.
    ///
    /// ## Example:
    ///
    /// It would be represented as follow:
    /// [tree_size][tree_content][encoded data]
    pub fn encode_with_metadatas(input: &[u8]) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();
        let tree = HuffmanTree::load_tree_from_bytes(&input);

        let tree_to_byte = tree.as_bytes();

        // add tree size at the beginning of the buffer
        // let tree_size = tree_to_byte.len() as u8;
        let tree_size = encode_varsize(tree_to_byte.len());

        encoded.extend_from_slice(&tree_size);

        // add tree in the buffer
        for byte in tree_to_byte {
            encoded.push(byte);
        }

        let (num_bits, encoded_data) = tree.encode(input);
        let num_bits = encode_varsize(num_bits);

        // Add converted bits and their number
        encoded.extend_from_slice(&num_bits);
        encoded.extend_from_slice(&encoded_data);

        encoded
    }

    pub fn decode_with_metadatas(input: &[u8]) -> Vec<u8> {
        // extracting tree
        let (tree_size, tree_content_start) = get_first_decoded(&input);
        let tree_content = &input[tree_content_start..(tree_content_start + tree_size)];
        let tree = HuffmanTree::from(tree_content);

        let compressed_data = &input[(tree_content_start + tree_size)..];
        let (size, size_last_byte_index) = get_first_decoded(compressed_data);

        let compressed_data = &compressed_data[size_last_byte_index..];

        tree.decode(&compressed_data, size)
    }

    pub fn decode(&self, bytes: &[u8], bit_length: usize) -> Vec<u8> {
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

                if let Some(c) = node.c {
                    // DEBUG
                    // println!("char: {}", node.c);
                    decoded.push(c as u8);
                    node = &self.root;
                }

                visited_bits += 1;
            }
        }

        decoded
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
    type Output = Vec<Bit>;

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

        if let Some(c) = &node.c {
            return c;
        } else {
            panic!("Attempting to return internal node which should be impossible.");
        }
        // &node.c
    }
}

fn slice_contains_nullchar(values: &[char]) -> bool {
    for i in 1..values.len() {
        if values[i - 1] == ENCODED_NULL_CHAR && values[i] == ENCODED_NULL_CHAR {
            return true;
        }
    }

    false
}

// FIXME there is some very specific cases that would probably not work, try with 1 to see.
// error occurs when the far right is the ENCODED_NULL_CHAR is the same as the previous one
// [...0,68,90,1,1,1]
//             ^ ^ the 2 last digit are the encoded value
//             | the function mistake is as: ['\u{0}', '\u{1}']
//                               instead of: ['\u{1}', '\u{0}']
fn node_from_vec(values: &Vec<char>, index: usize) -> Node {
    let c = values[index];

    let mut left = None;
    let mut right = None;

    // Last Value
    if index + 1 == values.len() {
        return Node::new(Some(c));
    }

    // Special scenario: null char are encoded as 0x01 0x01
    if c == ENCODED_NULL_CHAR && values[index + 1] == ENCODED_NULL_CHAR {
        // if c == (1 as char) && values[index + 1] == (1 as char) {
        return Node::new(Some(LEAF_NULL_CHAR));
    }

    if values[index] == INTERNAL_NODE_VALUE {
        left = Some(Box::new(node_from_vec(values, index + 1)));
        // We know that the next right value is after all the values of the left
        // node so we just need to skip them
        let mut count = left.as_ref().unwrap().count();

        // in case the left node was a null char else if null char in right
        if count == 1
            && values[index + 1] == ENCODED_NULL_CHAR
            && values[index + 2] == ENCODED_NULL_CHAR
        {
            count += 1;
        } else if index + 1 + count < values.len()
            && slice_contains_nullchar(&values[index..index + count + 2])
        {
            count += 1;
        }

        right = Some(Box::new(node_from_vec(values, index + 1 + count)));
    }

    let mut node = if c == INTERNAL_NODE_VALUE {
        Node::new(None)
    } else {
        Node::new(Some(c))
    };

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

impl From<&[u8]> for HuffmanTree {
    fn from(value: &[u8]) -> Self {
        let value: Vec<char> = value.iter().map(|&byte| byte as char).collect();
        let mut tree = HuffmanTree {
            root: node_from_vec(&value, 0),
            encoding: HashMap::new(),
        };

        tree.set_encoding();

        tree
    }
}

impl From<&Vec<u8>> for HuffmanTree {
    fn from(value: &Vec<u8>) -> Self {
        let value: Vec<char> = value.iter().map(|&byte| byte as char).collect();
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
    use crate::bitvec;

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
        // bitvec!['a'];
        assert_eq!(encoding[0], ('f', bitvec![0]));
        assert_eq!(encoding[1], ('c', bitvec![1, 0, 0]));
        assert_eq!(encoding[2], ('d', bitvec![1, 0, 1]));
        assert_eq!(encoding[3], ('a', bitvec![1, 1, 0, 0]));
        assert_eq!(encoding[4], ('b', bitvec![1, 1, 0, 1]));
        assert_eq!(encoding[5], ('e', bitvec![1, 1, 1]));
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
        assert_eq!(tree['f'], bitvec![0]);
        assert_eq!(tree['c'], bitvec![1, 0, 0]);
        assert_eq!(tree['d'], bitvec![1, 0, 1]);
        assert_eq!(tree['a'], bitvec![1, 1, 0, 0]);
        assert_eq!(tree['b'], bitvec![1, 1, 0, 1]);
        assert_eq!(tree['e'], bitvec![1, 1, 1]);
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

        let text: Vec<u8> = "faced".bytes().collect();
        assert_eq!(text, decoded);
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
        assert_eq!(new_tree['f'], bitvec![0]);
        assert_eq!(new_tree['c'], bitvec![1, 0, 0]);
        assert_eq!(new_tree['d'], bitvec![1, 0, 1]);
        assert_eq!(new_tree['a'], bitvec![1, 1, 0, 0]);
        assert_eq!(new_tree['b'], bitvec![1, 1, 0, 1]);
        assert_eq!(new_tree['e'], bitvec![1, 1, 1]);
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
        assert_eq!(new_tree['f'], bitvec![0]);
        assert_eq!(new_tree['c'], bitvec![1, 0, 0]);
        assert_eq!(new_tree['d'], bitvec![1, 0, 1]);
        assert_eq!(new_tree['a'], bitvec![1, 1, 0, 0]);
        assert_eq!(new_tree['b'], bitvec![1, 1, 0, 1]);
        assert_eq!(new_tree['e'], bitvec![1, 1, 1]);
    }

    #[test]
    pub fn encode_n_decode_with_metadatas() {
        let text = "AAABBCCDACCAA";
        // let text = "AAABBCCDACCAAAAABBCCDACCAAAAABBCCDACCAAAAABBCCDACCAAAAABBCCDACCAAAAABBCCDACCAA";
        let text: Vec<u8> = text.bytes().collect();
        let text: &[u8] = &text;

        let encoded = HuffmanTree::encode_with_metadatas(text);
        println!("huff: {encoded:?}");

        let decoded = HuffmanTree::decode_with_metadatas(&encoded);
        for c in &decoded {
            print!("{}", *c as char);
        }
        println!();
        // decoded.reverse();

        assert_eq!(text, decoded);
    }

    #[test]
    pub fn tree_with_null_char() {
        let text: Vec<u8> = "ABBBCCCCCDDDDDD\0\0".bytes().collect();
        let tree = HuffmanTree::load_tree_from_bytes(&text);
        let (nb_bits, encoded) = tree.encode(&text);

        let formatted_tree = tree.as_bytes();
        let new_tree = HuffmanTree::from(&formatted_tree);
        let decoded = new_tree.decode(&encoded, nb_bits);
        assert_eq!(text, decoded);

        let text: Vec<u8> = "ABBBCCCCCDDDDDDD\0\0\0\0".bytes().collect();
        let tree = HuffmanTree::load_tree_from_bytes(&text);
        let (nb_bits, encoded) = tree.encode(&text);

        let formatted_tree = tree.as_bytes();
        let new_tree = HuffmanTree::from(&formatted_tree);
        let decoded = new_tree.decode(&encoded, nb_bits);
        assert_eq!(text, decoded);

        let text: Vec<u8> = "ABBBCCCCCDDDDDD\0\0\0\0\0\0".bytes().collect();
        let tree = HuffmanTree::load_tree_from_bytes(&text);
        let (nb_bits, encoded) = tree.encode(&text);

        let formatted_tree = tree.as_bytes();
        let new_tree = HuffmanTree::from(&formatted_tree);
        let decoded = new_tree.decode(&encoded, nb_bits);
        assert_eq!(text, decoded);

        let text: Vec<u8> = "ABBBCCCCCDDDDDD\0\0\0\0\0\0\0\0\0\0".bytes().collect();
        let tree = HuffmanTree::load_tree_from_bytes(&text);
        let (nb_bits, encoded) = tree.encode(&text);

        let formatted_tree = tree.as_bytes();
        let new_tree = HuffmanTree::from(&formatted_tree);
        let decoded = new_tree.decode(&encoded, nb_bits);
        assert_eq!(text, decoded);
    }
}
