use std::collections::{HashMap, HashSet};

use crate::varsize::{decode_varsize, encode_varsize};

// NOTE possibly remove struct and just use the methods

pub struct LZWEncoder {
    dict: HashMap<String, usize>,
    pub encoding: Vec<String>,
    pub data: Vec<usize>,
}

impl LZWEncoder {
    pub fn new() -> Self {
        LZWEncoder {
            dict: HashMap::new(),
            encoding: Vec::new(),
            data: Vec::new(),
        }
    }

    fn insert(&mut self, codeword: String) {
        let index = self.encoding.len();
        self.encoding.push(codeword.clone());

        self.dict.insert(codeword, index);
    }

    fn usize_to_bytes(xs: &[usize]) -> Vec<u8> {
        let mut bytes = Vec::new();

        for x in xs {
            let mut new_bytes = encode_varsize(*x);
            bytes.append(&mut new_bytes);
        }

        bytes
    }

    pub fn encode(&mut self, stream: Vec<char>) -> Vec<u8> {
        // add unique char to dict
        for c in &stream {
            let codeword = c.to_string();
            if let None = self.dict.get(&codeword) {
                self.insert(codeword);
            };
        }

        // Encoding the rest while creating new codewords
        let mut i: usize = 0;
        while i < stream.len() {
            let mut current = stream[i].to_string();
            let mut next = i + 1;
            let mut index = 0;

            while let Some(codeword_index) = self.dict.get(&current) {
                index = *codeword_index;
                if next == stream.len() {
                    // adding char that will be removed afterward
                    current.push('\0');
                    break;
                }
                current.push(stream[next]);
                next += 1;
            }

            let done = current.chars().last().unwrap() == '\0';
            if !done {
                let new_codeword = current.clone();
                self.insert(new_codeword);
                current.pop();
            }

            self.data.push(index);

            i = next - 1;
            if done {
                break;
            }
        }

        // DEBUG
        // println!("dict: {:?}", self.dict);
        // println!("encoding: {:?}", self.encoding);
        // println!("data: {:?}", self.data);
        LZWEncoder::usize_to_bytes(&self.data)
    }

    /// Decode previously encoded data
    /// -
    pub fn decode(&mut self, single_chars: Vec<char>, stream: &[u8]) -> String {
        // pub fn decode(&mut self, single_chars: Vec<char>, stream: &[usize]) -> String {
        let stream = decode_varsize(stream);

        // add unique char to dict
        for c in &single_chars {
            let codeword = c.to_string();
            self.insert(codeword);
        }

        let mut decoded = Vec::new();

        let mut previous_string = String::from("");
        let mut first = true;
        for index in stream {
            let word = if (index as usize) < self.encoding.len() {
                self.encoding[index as usize].clone()
            } else if (index as usize) == self.encoding.len() {
                format!(
                    "{}{}",
                    previous_string,
                    previous_string.chars().next().unwrap()
                )
            } else {
                panic!("Should no be possible");
            };

            // DEBUG
            // println!("word: {}, index: {}", word.clone(), index);
            decoded.push(word.clone());

            if !first {
                self.encoding.push(format!(
                    "{}{}",
                    previous_string,
                    word.chars().next().unwrap()
                ));
            }

            first = false;
            previous_string = word;

            // DEBUG
            // println!("encoding: {:?}", self.encoding.join(","));
            // println!("decoded: {:?}", decoded.join(""));
            // println!();
        }

        self.encoding.pop();
        let decoded = decoded.join("");

        // DEBUG
        // println!("encoding: {:?}", self.encoding.join(","));
        // println!("decoded: {:?}", decoded.join(""));
        // println!();

        decoded
    }

    pub fn get_unique_sequent_char(input: &str) -> Vec<char> {
        let mut discovered_chars: HashSet<char> = HashSet::new();
        let mut chars = Vec::new();
        for c in input.chars() {
            if let None = discovered_chars.get(&c) {
                chars.push(c);
                discovered_chars.insert(c);
            }
        }

        chars
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        compressed_buffer::{self, Bit, CompressedBuffer},
        huffman_tree::HuffmanTree,
    };

    use super::*;

    #[test]
    fn test_encode_variable_width_code() {}

    #[test]
    fn test_encode() {
        let to_encode = vec!['A', 'A', 'B', 'A', 'B', 'C', 'C', 'A', 'B', 'C'];

        let mut encoder = LZWEncoder::new();

        let encoded = encoder.encode(to_encode);
        println!("encoded: {:?}", encoded);

        assert_eq!(vec![0, 0, 1, 4, 2, 2, 6], encoded);
    }

    #[test]
    fn test_decode() {
        let to_decode = [0, 0, 1, 4, 2, 2, 6];

        let chars = vec!['A', 'B', 'C'];
        let mut encoder = LZWEncoder::new();

        let decoded = encoder.decode(chars, &to_decode);

        assert_eq!("AABABCCABC", decoded);
        // encoder.encode(&to_decode);
    }

    #[test]
    fn encode_n_decode() {
        let text = "aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffff";
        // let text = "aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffffsdashjdgasjhdgasjhdbvasjhvdjhasgdajhsgdkhasgdjgvbgwsyfghewirfuywgyubefkhicruygwesyurfhgb uyeg rbwnhs jbgvfzfgujwa jge He";
        let to_encode: Vec<char> = text.chars().collect();

        let mut encoder = LZWEncoder::new();
        let to_decode = encoder.encode(to_encode);

        // DEBUG
        // println!("Initial length: {}", text.len());
        // println!("encoded length: {}", to_decode.len());
        // println!("encoded length: {}", to_decode.len() * 8);
        // println!("encoded: {:?}", to_decode);

        let chars = LZWEncoder::get_unique_sequent_char(text);
        let mut encoder = LZWEncoder::new();
        let decoded = encoder.decode(chars, &to_decode);

        assert_eq!(text, decoded);
    }

    #[test]
    fn encode_n_decode_default_text_255() {
        let to_encode: Vec<char> = (1..=255).map(|c: u8| c as char).collect();

        // let mut to_encode2: Vec<char> = (1..=255).map(|c: u8| c as char).collect();
        // to_encode.append(&mut to_encode2);

        let mut encoder = LZWEncoder::new();
        let to_decode = encoder.encode(to_encode.clone());

        let text: String = to_encode.iter().collect();

        let chars = LZWEncoder::get_unique_sequent_char(&text);
        let mut encoder = LZWEncoder::new();
        let decoded = encoder.decode(chars, &to_decode);

        // DEBUG
        // println!("decoded.len(): {}", decoded.len());
        // println!("text: {}", text.len());

        assert_eq!(text, decoded);
    }

    // NOTE to run the following to get the compression rates:
    // cargo test parsing_encoding -- --nocapture
    #[test]
    fn parsing_encoding_through_huffman_encoding() {
        let text = "aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffffsdashjdgasjhdgasjhdbvasjhvdjhasgdajhsgdkhasgdjgvbgwsyfghewirfuywgyubefkhicruygwesyurfhgb uyeg rbwnhs jbgvfzfgujwa jge He";

        // LZW ENCODING
        let to_encode: Vec<char> = text.chars().collect();
        let mut encoder = LZWEncoder::new();
        let lzw_encoded = encoder.encode(to_encode);

        // HUFFMAN_ENCODING
        let to_encode_with_huffman: Vec<u8> = text.bytes().collect();
        let tree = HuffmanTree::load_tree_from_bytes(&to_encode_with_huffman);
        let mut compressed_buffer_huffman = CompressedBuffer::new();
        let bits = tree.encode(&to_encode_with_huffman);
        bits.iter()
            .for_each(|&bit| compressed_buffer_huffman.push_bit(bit));

        // LZW + HUFFMAN encoding
        let to_encode: Vec<char> = text.chars().collect();
        let mut lzw_encoder = LZWEncoder::new();
        let encoded_with_lzw = lzw_encoder.encode(to_encode);

        let tree = HuffmanTree::load_tree_from_bytes(&encoded_with_lzw);
        let mut compressed_buffer = CompressedBuffer::new();
        let bits = tree.encode(&encoded_with_lzw);
        bits.iter().for_each(|&bit| compressed_buffer.push_bit(bit));

        // DEBUG print compression rates
        //
        println!("Initial length: {}", text.len());
        println!(
            "encoded length (with lzw only): {}. Total compression rate: {:.2} %",
            lzw_encoded.len(),
            (lzw_encoded.len() as f64 / text.len() as f64) * 100.0
        );
        println!(
            "encoded length (huffman only): {}. Total compression rate: {:.2} %",
            compressed_buffer_huffman.buffer.len(),
            (compressed_buffer_huffman.buffer.len() as f64 / text.len() as f64) * 100.0
        );
        println!(
            "encoded length (with lzm followed by huffman): {}. Total compression rate: {:.2} %",
            compressed_buffer.buffer.len(),
            (compressed_buffer.buffer.len() as f64 / text.len() as f64) * 100.0
        );

        let decoded_huffman = tree.decode(&compressed_buffer.buffer, bits.len());

        assert_eq!(encoded_with_lzw, decoded_huffman);

        let mut lzw_decoder = LZWEncoder::new();
        let single_chars = LZWEncoder::get_unique_sequent_char(text);
        let decoded = lzw_decoder.decode(single_chars, &decoded_huffman);

        assert_eq!(text, decoded);
    }
}
