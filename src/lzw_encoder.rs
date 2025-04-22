pub mod LZWEncoder {
    use core::num;
    use std::collections::HashMap;

    use crate::varsize::{decode_varsize, encode_varsize, get_first_decoded};

    fn insert(
        // &self,
        codeword: Vec<u8>,
        codewords: &mut HashMap<Vec<u8>, usize>,
        encoding: &mut Vec<Vec<u8>>,
    ) {
        let index = encoding.len();
        encoding.push(codeword.clone());
        codewords.insert(codeword, index);
    }

    fn usize_to_bytes(xs: &[usize]) -> Vec<u8> {
        let mut bytes = Vec::new();

        for x in xs {
            let mut new_bytes = encode_varsize(*x);
            bytes.append(&mut new_bytes);
        }

        bytes
    }

    /// Encode given bytes using the lzw encoding
    ///
    /// @**returns** (Vec\<u8\>, Vec\<u8\>) => (single chars in order, the encoded indexes)
    pub fn encode(input: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut indexes: Vec<usize> = Vec::new();
        let mut unique_char: Vec<u8> = Vec::new();
        let mut codewords: HashMap<Vec<u8>, usize> = HashMap::new();

        let mut encoding: Vec<Vec<u8>> = Vec::new();
        // add unique char to dict
        for &c in input {
            let codeword = vec![c];

            if let None = codewords.get(&codeword) {
                unique_char.push(c);
                insert(codeword, &mut codewords, &mut encoding);
            };
        }

        // Encoding the rest while creating new codewords
        let mut i: usize = 0;
        while i < input.len() {
            let mut current = vec![input[i]];
            let mut next = i + 1;
            let mut index = 0;

            let mut done = false;

            while let Some(codeword_index) = codewords.get(&current) {
                index = *codeword_index;

                if next == input.len() {
                    done = true;
                    break;
                }
                current.push(input[next]);
                next += 1;
            }

            // NOTE validation can be better
            if !done {
                let new_codeword = current.clone();
                insert(new_codeword, &mut codewords, &mut encoding);
                current.pop();
            }

            indexes.push(index);

            i = next - 1;
            if done {
                break;
            }
        }

        // DEBUG
        // println!("dict: {:?}", self.dict);
        // println!("encoding: {:?}", self.encoding);
        // println!("data: {:?}", self.data);
        // LZWEncoder::usize_to_bytes(&self.data)
        (unique_char, usize_to_bytes(&indexes))
    }

    /// Decode previously encoded data
    /// -
    pub fn decode(single_chars: &[u8], input: &[u8]) -> Vec<u8> {
        let mut codewords: HashMap<Vec<u8>, usize> = HashMap::new();

        let mut encoding: Vec<Vec<u8>> = Vec::new();
        let input = decode_varsize(input);

        // add unique char to dict
        for &c in single_chars {
            let codeword = vec![c];
            insert(codeword, &mut codewords, &mut encoding);
        }

        let mut decoded_chunks = Vec::new();

        let mut previous_string = Vec::new();
        let mut first = true;
        for index in input {
            let word = if (index as usize) < encoding.len() {
                encoding[index as usize].clone()
            } else if (index as usize) == encoding.len() {
                let mut new_word = previous_string.clone();
                new_word.push(previous_string[0]);
                new_word
            } else {
                panic!("Should no be possible");
            };

            // DEBUG
            // println!("word: {}, index: {}", word.clone(), index);

            decoded_chunks.push(word.clone());

            if !first {
                let mut new_encoding = previous_string.clone();
                new_encoding.push(word[0]);
                encoding.push(new_encoding);
            }

            first = false;
            previous_string = word;
        }

        // NOTE useless? Will let it there in case we need to return the encoding at some point.
        encoding.pop();

        let mut decoded = Vec::new();

        for decoded_chunk in decoded_chunks {
            decoded.extend_from_slice(&decoded_chunk);
        }

        decoded
    }

    /// return the encoding preceded by the unique chars and the number of unique chars.
    ///
    /// ## Example:
    ///
    /// It would be represented as follow:
    /// [num_unique_chars][chars][encoded data]
    /// [3][A, B, C][0, 0, 1, 4, 2, 2, 6]
    pub fn encode_with_metadatas(input: &[u8]) -> Vec<u8> {
        let (unique_chars, encoded) = encode(input);
        let num_chars = encode_varsize(unique_chars.len());
        let mut new_encoded =
            Vec::with_capacity(num_chars.len() + unique_chars.len() + encoded.len());
        new_encoded.extend_from_slice(&num_chars);
        new_encoded.extend_from_slice(&unique_chars);
        new_encoded.extend_from_slice(&encoded);

        new_encoded
    }

    /// from an encoded input with metadatas return decoded bytes
    pub fn decode_with_metadatas(input: &[u8]) -> Vec<u8> {
        let (num_chars, new_first_index) = get_first_decoded(&input);

        let single_chars = &input[new_first_index..num_chars + 1];

        let encoded = &input[num_chars + 1..];

        decode(single_chars, encoded)
    }
}

#[cfg(test)]
mod tests {
    use crate::{huffman_tree::HuffmanTree, utils::display_data_compression_ratio};

    use super::*;

    #[test]
    fn test_encode() {
        let text = "AABABCCABC";
        let to_encode: Vec<u8> = text.bytes().collect();

        let (_, encoded) = LZWEncoder::encode(&to_encode);

        assert_eq!(vec![0, 0, 1, 4, 2, 2, 6], encoded);
    }

    #[test]
    fn test_decode() {
        let to_decode = [0, 0, 1, 4, 2, 2, 6];

        let single_chars = [65u8, 66u8, 67u8];
        let decoded = LZWEncoder::decode(&single_chars, &to_decode);

        let text: Vec<u8> = "AABABCCABC".bytes().collect();
        assert_eq!(text, decoded);
    }

    #[test]
    fn encode_n_decode() {
        let text = "aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffff";
        // let text = "aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffffsdashjdgasjhdgasjhdbvasjhvdjhasgdajhsgdkhasgdjgvbgwsyfghewirfuywgyubefkhicruygwesyurfhgb uyeg rbwnhs jbgvfzfgujwa jge He";
        let to_encode: Vec<u8> = text.bytes().collect();

        let (single_chars, to_decode) = LZWEncoder::encode(&to_encode);

        // DEBUG
        // println!("Initial length: {}", text.len());
        // println!("encoded length: {}", to_decode.len());
        // println!("encoded length: {}", to_decode.len() * 8);
        // println!("encoded: {:?}", to_decode);

        let decoded = LZWEncoder::decode(&single_chars, &to_decode);

        assert_eq!(to_encode, decoded);
    }

    #[test]
    fn encode_n_decode_default_text_255() {
        let to_encode: Vec<u8> = (1..=255).collect();

        // let mut to_encode2: Vec<char> = (1..=255).map(|c: u8| c as char).collect();
        // to_encode.append(&mut to_encode2);

        let (single_chars, to_decode) = LZWEncoder::encode(&to_encode);

        let decoded = LZWEncoder::decode(&single_chars, &to_decode);

        // DEBUG
        // println!("decoded.len(): {}", decoded.len());
        // println!("text: {}", text.len());

        assert_eq!(to_encode, decoded);
    }

    // NOTE to run the following to get the compression rates:
    // cargo test parsing_encoding -- --nocapture
    #[test]
    fn parsing_encoding_through_huffman_encoding() {
        let text = "aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffff";
        // let text = "aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffffsdashjdgasjhdgasjhdbvasjhvdjhasgdajhsgdkhasgdjgvbgwsyfghewirfuywgyubefkhicruygwesyurfhgb uyeg rbwnhs jbgvfzfgujwa jge He";

        // LZW ENCODING
        let to_encode: Vec<u8> = text.bytes().collect();
        let (_, lzw_encoded) = LZWEncoder::encode(&to_encode);

        // HUFFMAN_ENCODING
        let to_encode_with_huffman: Vec<u8> = text.bytes().collect();
        let tree = HuffmanTree::load_tree_from_bytes(&to_encode_with_huffman);
        let (_, compressed_buffer_huffman) = tree.encode(&to_encode_with_huffman);

        // LZW + HUFFMAN encoding
        let to_encode: Vec<u8> = text.bytes().collect();
        let (single_chars_huff_lzw, encoded_with_lzw) = LZWEncoder::encode(&to_encode);

        let tree = HuffmanTree::load_tree_from_bytes(&encoded_with_lzw);
        let (nb_bits, compressed_buffer) = tree.encode(&encoded_with_lzw);

        // DEBUG print compression rates
        println!("LZW ONLY:");
        display_data_compression_ratio(text.len(), lzw_encoded.len());

        println!("HUFFMAN ONLY:");
        display_data_compression_ratio(text.len(), compressed_buffer_huffman.len());

        println!("LZM FOLLOWED BY HUFFMAN:");
        display_data_compression_ratio(text.len(), compressed_buffer.len());
        // END DEBUG //

        let decoded_huffman = tree.decode(&compressed_buffer, nb_bits);

        assert_eq!(encoded_with_lzw, decoded_huffman);

        let decoded = LZWEncoder::decode(&single_chars_huff_lzw, &decoded_huffman);

        let text: Vec<u8> = text.bytes().collect();
        assert_eq!(text, decoded);
    }

    #[test]
    fn encode_n_decode_with_metadatas() {
        let text = "AABABCCABC";
        let to_encode: Vec<u8> = text.bytes().collect();

        let encoded = LZWEncoder::encode_with_metadatas(&to_encode);

        assert_eq!(vec![3, 65, 66, 67, 0, 0, 1, 4, 2, 2, 6], encoded);

        let decoded = LZWEncoder::decode_with_metadatas(&encoded);

        let text: Vec<u8> = text.bytes().collect();
        assert_eq!(text, decoded);
    }
}
