// Variable size encoding

/// Take a number usize and return the a representation in as little bytes as possible.
/// The most significant bit of each byte is used to indicated if a following byte awaits.
///
/// ## Example:
///
/// if given: n => 512 =>  0x00 0x00 0x00 0x00 0x00 0x00 0x02 0x00
/// would return: [0x84, 0x00]
/// ```
/// would return: [0x84, 0x00]
///             : [1000_0100, 0000_0000]
///                ^
///                |
///                |
///    indicated that a byte follows
/// ```
///
/// ```rs
/// let n = 1;
/// let encoded = encode_varsize(n);
/// assert_eq!(vec![0x01], encoded);
///
/// let n = 512;
/// let encoded = encode_varsize(n);
/// assert_eq!(vec![0x84, 0x00], encoded);
///
/// let n = 1024;
/// let encoded = encode_varsize(n);
/// assert_eq!(vec![0x88, 0x00], encoded);
///
/// let n = 99999;
/// let encoded = encode_varsize(n);
/// assert_eq!(vec![0x86, 0x8D, 0x1F], encoded);
/// ```
pub fn encode_varsize(n: usize) -> Vec<u8> {
    let mut n = n;
    let mut bytes = Vec::new();

    while n > 0 {
        let byte = (n & 0x7F) as u8 | 0x80;

        bytes.push(byte);

        n >>= 7;
    }

    bytes[0] = bytes[0] << 1 >> 1;
    bytes.reverse();

    bytes
}

/// Take a one or multiple previously encoded usize in a vector and return all the contained usize element.
///
/// Example:
/// ```rs
/// let n = vec![0x01];
/// let decoded = decode_varsize(n);
/// assert_eq!(vec![1], decoded);
///
/// let n = vec![0x84, 0x00];
/// let decoded = decode_varsize(n);
/// assert_eq!(vec![512], decoded);
///
/// let n = vec![0x88, 0x00];
/// let decoded = decode_varsize(n);
/// assert_eq!(vec![1024], decoded);
///
/// let n = vec![0x86, 0x8D, 0x1F];
/// let decoded = decode_varsize(n);
/// assert_eq!(vec![99999], decoded);
///
/// ```
pub fn decode_varsize(encoded: Vec<u8>) -> Vec<usize> {
    let mut decoded = Vec::new();
    let mut i: usize = 0;

    while i < encoded.len() {
        let mut bytes = Vec::new();

        // collecting bytes to reconstruct
        while 0x80 & encoded[i] == 0x80 {
            bytes.push(encoded[i]);
            i += 1;
        }
        bytes.push(encoded[i]);

        i += 1;

        let mut usized: usize = 0;
        for j in 0..bytes.len() {
            usized |= (bytes[j] as usize & 0x7F) << (7 * (bytes.len() - 1 - j));
        }
        decoded.push(usized);
    }
    decoded
}

/// Return the first encoded usize found
///
/// @**returns** (usize, usize) => (reconstructed value, index of the last byte it was found)
pub fn get_first_decoded(encoded: &[u8]) -> (usize, usize) {
    let mut bytes = Vec::new();

    // collecting bytes to reconstruct
    let mut i: usize = 0;
    while 0x80 & encoded[i] == 0x80 {
        bytes.push(encoded[i]);
        i += 1;
    }
    bytes.push(encoded[i]);

    i += 1;

    let mut usized: usize = 0;
    for j in 0..bytes.len() {
        usized |= (bytes[j] as usize & 0x7F) << (7 * (bytes.len() - 1 - j));
    }

    (usized, i)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encode_variable_width_code() {
        let n = 1;
        let encoded = encode_varsize(n);
        assert_eq!(vec![0x01], encoded);

        let n = 512;
        let encoded = encode_varsize(n);
        assert_eq!(vec![0x84, 0x00], encoded);

        let n = 1024;
        let encoded = encode_varsize(n);
        assert_eq!(vec![0x88, 0x00], encoded);

        // 99999 =>           0x01 0x86 0x9F
        //                     1    141   159
        // 99999 (encoded) => 0x86 0x8D 0x0F
        let n = 99999;
        let encoded = encode_varsize(n);
        assert_eq!(vec![0x86, 0x8D, 0x1F], encoded);
    }

    #[test]
    fn test_decode_variable_width_code() {
        let n = vec![0x01];
        let decoded = decode_varsize(n);
        assert_eq!(vec![1], decoded);

        let n = vec![0x84, 0x00];
        let decoded = decode_varsize(n);
        assert_eq!(vec![512], decoded);

        // 1024           => 0x04 0x00
        // 1024 (encoded) => 0x82 0x83 0x00
        let n = vec![0x88, 0x00];
        let decoded = decode_varsize(n);
        assert_eq!(vec![1024], decoded);

        // 99999 =>           0x01 0x86 0x9F
        // 99999 (encoded) => 0x86 0x8D 0x0F
        let n = vec![0x86, 0x8D, 0x1F];
        let decoded = decode_varsize(n);
        assert_eq!(vec![99999], decoded);
    }
}
