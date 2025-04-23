use std::collections::HashMap;

#[allow(dead_code)]
/// display the data compression ratio
///
/// based on the following: <https://en.wikipedia.org/wiki/Data_compression_ratio>
#[rustfmt::skip]
pub fn display_data_compression_ratio(
    uncompressed_size: usize,
    compressed_size: usize
) {
    let compression_ratio = uncompressed_size as f64 / compressed_size as f64;
    let space_saving = 1. - (compressed_size as f64 / uncompressed_size as f64);

    println!("\t Uncompressed size: {uncompressed_size} bytes");
    println!("\t Compressed size: {compressed_size} bytes");
    println!("\t compression ratio: {compression_ratio:.1}:1");
    println!("\t space saving: {} %", space_saving * 100.);
}

#[allow(dead_code)]
/// get the entropy for given values
pub fn entropy(values: &[u8]) -> f64 {
    let mut freq: HashMap<u8, usize> = HashMap::new();
    let mut total: usize = 0;

    for key in values {
        total += 1;
        if let Some(val) = freq.get_mut(key) {
            *val += 1;
        } else {
            freq.insert(*key, 1);
        }
    }

    let ps: Vec<f64> = freq
        .iter()
        .map(|(_, &num)| num as f64 / total as f64)
        .collect();

    let mut entropy: f64 = 0.;
    for p in ps {
        entropy += p * p.log2();
    }

    -entropy
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_entropy() {
        let bytes: Vec<u8> = "AABBCCDD".bytes().collect();
        let entropy = entropy(&bytes);

        assert_eq!(2., entropy);
    }
}
