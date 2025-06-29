#[allow(non_snake_case)]
pub mod ArithmeticEncoder {
    use num_bigint::BigUint;
    use std::collections::HashMap;

    use crate::{
        bit_queue::BitQueue,
        compressed_buffer::{Bit, CompressedBuffer},
        fenwick_tree::FenwickTree,
        varsize::{encode_varsize, get_first_decoded},
    };

    struct Bounderies {
        pub TOP_VALUE: BigUint,
        pub FIRST_QUARTER: BigUint,
        pub HALF: BigUint,
        pub THIRD_QUARTER: BigUint,
    }

    impl Bounderies {
        pub fn new(num_bits: usize) -> Self {
            let mut TOP_VALUE = BigUint::from(1u8);
            TOP_VALUE <<= num_bits;
            TOP_VALUE -= 1u8;
            let FIRST_QUARTER = (&TOP_VALUE >> 2) + 1u8;
            let HALF = &FIRST_QUARTER * 2u8;
            let THIRD_QUARTER = &FIRST_QUARTER * 3u8;
            Bounderies {
                TOP_VALUE,
                FIRST_QUARTER,
                HALF,
                THIRD_QUARTER,
            }
        }
    }

    // NOTE: good ressources for implementation
    // https://github.com/tommyod/arithmetic-coding/blob/main/arithmetic_coding.py
    // https://dl.acm.org/doi/10.1145/214762.214771
    /// encode an array of bytes using the arithmetic encoding
    ///
    /// @**returns** (u8, Vec\<u8\>, Vec\<u8\>) => (
    ///   bit index of the last byte (to offset the zeros that aren't a part of the last byte),
    ///   single chars in order,
    ///   the encoded indexes
    /// )
    pub fn encode(input: &[u8]) -> (u8, Vec<(u8, isize)>, Vec<u8>) {
        let mut frequency: HashMap<u8, usize> = HashMap::new();

        // get frequency
        for c in input {
            if let Some(freq) = frequency.get_mut(c) {
                *freq += 1;
            } else {
                frequency.insert(*c, 1);
            }
        }

        let mut sorted_freq: Vec<(u8, isize)> =
            frequency.iter().map(|(&k, &v)| (k, v as isize)).collect();
        // FIXME use proper sort (not alphabetical)
        // sorted_freq.sort_by(|a, b| a.1.cmp(&b.1));
        sorted_freq.sort_by(|a, b| a.0.cmp(&b.0));

        // NOTE possibly make mut if adaptative
        let cum_freq = FenwickTree::new(sorted_freq.clone());

        let size = cum_freq.total_sum() as usize;
        let num_bits: usize = size * cum_freq.len();
        let bounderies = Bounderies::new(num_bits);

        let mut low = BigUint::ZERO;
        let mut high = bounderies.TOP_VALUE.clone();

        let mut encoded = Vec::new();
        let mut bit_queue = BitQueue::new();

        for symbol in input {
            let range = &high - &low + BigUint::from(1u8);
            let (symbol_low, symbol_high) = cum_freq.get_bounds(*symbol);

            if range < BigUint::from(cum_freq.get_total_count() as usize) {
                println!("Insufficient precision to encode low-probability symbols.");
            }

            assert!(&low <= &high && high <= bounderies.TOP_VALUE);
            assert!(low < bounderies.HALF && bounderies.HALF <= high);
            assert!(&high - &low > bounderies.FIRST_QUARTER);

            let total_cum = cum_freq.get_total_count() as usize;
            high = &low + (&range * symbol_high as usize / total_cum) - BigUint::from(1u8);
            low = &low + (&range * symbol_low as usize / total_cum);

            loop {
                if high < bounderies.HALF {
                    // both msb are 0

                    let new_bits = bit_queue.bit_followed_by_inverted(0);
                    encoded.extend_from_slice(&new_bits);

                    // println!("Range in lower half - both start with 0");
                } else if low >= bounderies.HALF {
                    // both msb are 1
                    let new_bits = bit_queue.bit_followed_by_inverted(1);
                    encoded.extend_from_slice(&new_bits);

                    low -= &bounderies.HALF;
                    high -= &bounderies.HALF;

                    // println!("Range in upper half - both start with 1");
                } else if low >= bounderies.FIRST_QUARTER && high < bounderies.THIRD_QUARTER {
                    // both msb are opposite

                    // println!("Range in middle half - first 2 bits are opposite");
                    low -= &bounderies.FIRST_QUARTER;
                    high -= &bounderies.FIRST_QUARTER;
                    bit_queue += 1;
                } else {
                    break;
                }

                // rescaling
                low *= 2u8;
                high = 2u8 * high + 1u8;
                // println!("LOW: {low}, HIGH {high}");
            }

            // adapting probabilities
            // cum_freq.add_count(*symbol);
            // cum_freq.reduce_count(*symbol);
        }

        bit_queue += 1;

        // end of encoding
        let bit = if low > bounderies.FIRST_QUARTER { 1 } else { 0 };
        let new_bits = bit_queue.bit_followed_by_inverted(bit);
        encoded.extend_from_slice(&new_bits);

        let mut compressed_buffer = CompressedBuffer::new();
        for bit in encoded {
            let new_bit = if bit == 0 { Bit::ZERO } else { Bit::ONE };
            compressed_buffer.push_bit(new_bit);
        }

        let encoded_buffer = compressed_buffer.get_buffer();

        (
            compressed_buffer.get_current_bit_index(),
            sorted_freq,
            encoded_buffer,
        )
    }

    pub fn decode(last_byte_offset: u8, frequency: Vec<(u8, isize)>, encoded: &[u8]) -> Vec<u8> {
        // let mut cummul = 0;

        let cum_freq = FenwickTree::new(frequency);

        let size = cum_freq.total_sum() as usize;
        let num_bits: usize = size * cum_freq.len();
        let bounderies = Bounderies::new(num_bits);

        let mut low = BigUint::ZERO;
        let mut high = bounderies.TOP_VALUE.clone();

        // converting encoded value to Big num
        let mut value = BigUint::ZERO;
        let mut offset = 0;

        // shifting the bits of every encoded bytes execpt the last one
        let first_bytes = &encoded[0..encoded.len() - 1];
        for &byte in first_bytes {
            for i in (0..8).rev() {
                let bit = (byte >> i) & 1;
                value = (value << 1) + bit as usize;
                offset += 1;
            }
        }

        // shifting n bits of the last encoded bytes, where n is 8 - (num of unused bits + 1)
        let last_byte = encoded.last().unwrap();
        for i in (8 - (last_byte_offset + 1)..8).rev() {
            let bit = (last_byte >> i) & 1;
            value = (value << 1) + bit as usize;
            offset += 1;
        }

        // shifting the value by the number of bits in order to scale the encoded value
        value <<= num_bits - offset;

        let mut decoded = Vec::new();
        let mut i = 0;
        loop {
            let range = &high - &low + 1u8;
            let total_count = cum_freq.get_total_count() as usize;

            // scalling value
            let scaled_value: BigUint =
                ((&value - &low + 1 as usize) * total_count - 1 as usize) / &range;

            let scaled_value = match scaled_value.trailing_zeros() {
                Some(_) => {
                    let to_digit = scaled_value.to_u64_digits();
                    to_digit[0]
                }
                None => 0,
            };

            let symbol = cum_freq.search_range(scaled_value as isize).unwrap();
            decoded.push(symbol);

            // updating high and low
            let (symbol_low, symbol_high) = cum_freq.get_bounds(symbol);
            high = &low + (&range * symbol_high as usize / total_count) - 1u8;
            low = &low + (&range * symbol_low as usize / total_count);

            loop {
                if high < bounderies.HALF {
                    // low, high and encoded first bit is 0
                    // NOTE: does nothing but kept in case I want to add verbose
                    // println!("high: {high}, HALF: {}", bounderies.HALF);
                    // continue;
                } else if low >= bounderies.HALF {
                    // low, high and encoded first bit is 1
                    value -= &bounderies.HALF;
                    low -= &bounderies.HALF;
                    high -= &bounderies.HALF;
                } else if low >= bounderies.FIRST_QUARTER && high < bounderies.THIRD_QUARTER {
                    // low ->  in second quarter
                    // high -> in third quarter
                    value -= &bounderies.FIRST_QUARTER;
                    low -= &bounderies.FIRST_QUARTER;
                    high -= &bounderies.FIRST_QUARTER;
                } else {
                    break;
                }

                // shift all bits one to the left
                low *= 2u8;
                high = 2u8 * high + 1u8;
                i += 1;
                let bit = if i < encoded.len() - 1 { encoded[i] } else { 0 };
                value = 2 as usize * &value + bit as usize;
            }

            // All the symbols are decoded
            if decoded.len() == size {
                break;
            }
        }

        decoded
    }

    pub fn encode_with_metadatas(input: &[u8]) -> Vec<u8> {
        let (bits_offset, frequency, encoded) = encode(input);

        // convert frequency to a frequency with variable size
        let mut compressed_frequency = Vec::new();
        for (char, num) in frequency {
            compressed_frequency.push(char);
            let var_num = encode_varsize(num as usize);
            compressed_frequency.extend_from_slice(&var_num);
        }
        let compressed_frequency_size = encode_varsize(compressed_frequency.len());

        // capacity => bits_offset + sizeof_frequency + frequency + encoded.len()
        let mut encoded_with_meta_datas = Vec::with_capacity(
            1 + compressed_frequency_size.len() + compressed_frequency.len() + encoded.len(),
        );

        encoded_with_meta_datas.push(bits_offset);
        encoded_with_meta_datas.extend_from_slice(&compressed_frequency_size);
        encoded_with_meta_datas.extend_from_slice(&compressed_frequency);
        encoded_with_meta_datas.extend_from_slice(&encoded);

        encoded_with_meta_datas
    }

    pub fn decode_with_metadatas(input: &[u8]) -> Vec<u8> {
        let last_byte_offset = input[0];

        // reformating the frequency
        let (freq_size, last_byte_found) = get_first_decoded(&input[1..]);
        let mut frequency = Vec::new();

        let mut i = last_byte_found + 1;
        while i < freq_size + 1 {
            // getting the character
            let c = input[i];

            // getting the frequency of the found character
            let (num, last_byte_found) = get_first_decoded(&input[i + 1..]);

            // adding the frequency to the list and update the index
            frequency.push((c, num as isize));
            i += last_byte_found + 1;
        }

        // remaining bytes are the encoded content
        let encoded = &input[i..];

        let decoded = decode(last_byte_offset, frequency, encoded);

        decoded
    }
}

#[cfg(test)]
mod tests {

    use crate::algorithms::arithmetic_encoder::ArithmeticEncoder::{
        decode_with_metadatas, encode_with_metadatas,
    };

    use super::*;

    #[test]
    fn arithmetic_simple_test() {
        let text: Vec<u8> = "RGGRRRGGGB\n".bytes().collect();

        let (offset, freq, encoded) = ArithmeticEncoder::encode(&text);
        let decoded = ArithmeticEncoder::decode(offset, freq, &encoded);

        assert_eq!(text, decoded);
    }

    #[test]
    fn arithmetic_medium_test() {
        let echantillon: Vec<u8> = "AAAAACCCCadkjahsdkjashdkjashdjkashdkjashdCCCBBB"
            .bytes()
            .collect();

        let mut text: Vec<u8> = Vec::new();
        for _ in 0..100 {
            text.extend_from_slice(&echantillon);
        }

        let (offset, freq, encoded) = ArithmeticEncoder::encode(&text);
        let decoded = ArithmeticEncoder::decode(offset, freq, &encoded);

        assert_eq!(text, decoded);
    }

    #[test]
    fn arithmetic_extensive_test() {
        let echantillon: Vec<u8> = (0..256).into_iter().map(|x| x as u8).collect();
        let mut text: Vec<u8> = Vec::new();

        for _ in 0..3 {
            text.extend_from_slice(&echantillon);
        }

        let (offset, freq, encoded) = ArithmeticEncoder::encode(&text);
        let decoded = ArithmeticEncoder::decode(offset, freq, &encoded);

        assert_eq!(text, decoded);
    }

    #[test]
    fn arithmetic_encode_n_decode_with_metadatas() {
        let text: Vec<u8> = "RGGRRRGGGB\n".bytes().collect();

        let encoded = encode_with_metadatas(&text);
        let decoded = decode_with_metadatas(&encoded);

        assert_eq!(text, decoded);
    }
}
