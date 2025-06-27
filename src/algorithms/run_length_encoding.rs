pub mod RLE {
    use crate::varsize::{encode_varsize, get_first_decoded};

    /// Adjust the number of bytes needed to represent the count of each run length encoded values
    ///
    /// we are using varsize to reduce the number of bytes necessary to store the counts
    /// ex without varsize: ('A', 1) ==> (1000001, 00000000(x63) + 00000001) 65 bytes
    /// ex with    varsize: ('A', 1) ==> (1000001, 00000001) 2 bytes
    fn reduce_sizes(encoded: Vec<(u8, usize)>) -> Vec<u8> {
        let mut encoded_smaller = Vec::new();
        for (c, count) in encoded {
            encoded_smaller.push(c);

            let count_bytes = encode_varsize(count);
            encoded_smaller.extend_from_slice(&count_bytes);
        }

        encoded_smaller
    }

    pub fn encode(input: &[u8]) -> Vec<u8> {
        assert!(input.len() > 0);

        let mut encoded = vec![(input[0], 1usize)];

        for i in 1..input.len() {
            let c = input[i];

            if c == input[i - 1] {
                // increment count for last entry
                let last = encoded.last_mut().unwrap();
                last.1 += 1;
            } else {
                // add new entry
                encoded.push((c, 1));
            }
        }

        reduce_sizes(encoded)
    }

    pub fn decode(input: &[u8]) -> Vec<u8> {
        let mut decoded = Vec::new();

        let mut i = 0usize;
        while i < input.len() {
            let c = input[i];
            let (num, ending_index) = get_first_decoded(&input[i + 1..]);

            // push c char num times in decoded
            for _ in 0..num {
                decoded.push(c);
            }

            i += ending_index + 1;
        }

        decoded
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn rle_simple_test() {
            let text: Vec<u8> = "ABBCCCDDDDFFFFF\n".bytes().collect();
            let encoded = encode(&text);
            let decoded = decode(&encoded);
            assert_eq!(text, decoded);
        }
    }
}
