#[allow(non_snake_case)]
pub mod MoveToFront {

    pub fn encode(input: &[u8]) -> Vec<u8> {
        let mut unique_symbols: Vec<u8> = (0..255).collect();
        let mut encoded = Vec::with_capacity(input.len());

        for c in input {
            let index = unique_symbols
                .iter()
                .position(|symbol| c == symbol)
                .unwrap();

            // NOTE: Since unique_symbol should never have a number of value > 2^8,
            //       we can safely use u8 as an index
            encoded.push(index as u8);

            if index == 0 {
                continue;
            }

            for i in (1..index + 1).rev() {
                unique_symbols[i] = unique_symbols[i - 1];
            }

            unique_symbols[0] = *c;
        }

        encoded
    }

    pub fn decode(encoded: &[u8]) -> Vec<u8> {
        let mut unique_symbols: Vec<u8> = (0..255).collect();
        let mut decoded = Vec::with_capacity(encoded.len());

        for &index in encoded {
            let symbol = unique_symbols[index as usize];
            decoded.push(symbol);

            if index == 0 {
                continue;
            }

            for i in (1..index as usize + 1).rev() {
                unique_symbols[i] = unique_symbols[i - 1];
            }

            unique_symbols[0] = symbol;
        }

        decoded
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::get_entropy;

    use super::*;

    #[test]
    fn move_to_front_encode() {
        let text: Vec<u8> = "NNBAAA".bytes().collect();
        let encoded = MoveToFront::encode(&text);

        let initial_entropy = (get_entropy(&text) * 100.).round() / 100.;
        let encoded_entropy = (get_entropy(&encoded) * 100.).round() / 100.;

        assert_eq!(vec![78, 0, 67, 67, 0, 0], encoded);
        assert_eq!(initial_entropy, encoded_entropy);
    }

    #[test]
    fn move_to_front_decode() {
        let encoded = vec![78, 0, 67, 67, 0, 0];
        let decoded = MoveToFront::decode(&encoded);

        let expected: Vec<u8> = "NNBAAA".bytes().collect();
        assert_eq!(expected, decoded);
    }
}
