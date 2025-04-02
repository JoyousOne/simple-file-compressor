use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use crate::{
    compressed_buffer::{Bit, CompressedBuffer},
    huffman_tree::{FrequencyChar, HuffmanTree},
};

pub fn load_tree_from_file(file_path: &str) -> HuffmanTree {
    let mut map: HashMap<char, usize> = HashMap::new();

    let bytes = fs::read(file_path)
        .expect("Failed to read file in src/filereader.rs => fn load_tree_from_file");

    for c in bytes {
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

    println!("frequencies: {:?}", frequencies);
    HuffmanTree::new(&mut frequencies)
}

pub fn compress_file(tree: &HuffmanTree, input_file: &str, output_file: &str) -> usize {
    let mut output_f = File::create(output_file)
        .expect("Failed to create file in src/filereader.rs => fn compress_file");

    let bytes =
        fs::read(input_file).expect("Failed to read file in src/filereader.rs => fn compress_file");

    let mut compressed_buffer = CompressedBuffer::new();

    let mut bit_size: usize = 0;

    for c in bytes {
        let bits = &tree[c as char];

        for bit in bits {
            match bit {
                0 => compressed_buffer.push_bit(Bit::ZERO),
                1 => compressed_buffer.push_bit(Bit::ONE),
                _ => panic!("should not be possible"),
            }
            bit_size += 1;
        }
    }

    output_f
        .write_all(&compressed_buffer.buffer)
        .expect("Failed to write to file in src/filereader.rs => fn compress_file");

    output_f
        .flush()
        .expect("Failed to flush in src/filereader.rs => fn compress_file");

    return bit_size;
}

#[cfg(test)]
mod tests {
    use super::*;

    /* NOTE the file test test_uncommpressed_file.txt was generated with:
     ```py
     python3 -c "print('a'*5+'b'*9+'c'*12+'d'*13+'e'*16+'f'*45)" > tests/test_uncommpressed_file.txt
    ```
    */
    #[test]
    fn test_load_tree() {
        let tree = load_tree_from_file("tests/test_uncompressed_file.txt");

        let encoding = tree.get_encoding();

        println!("encoding length {}", encoding.len());
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
        assert_eq!(encoding[3], ('\n', vec![1, 1, 0, 0, 0]));
        assert_eq!(encoding[4], ('a', vec![1, 1, 0, 0, 1]));
        assert_eq!(encoding[5], ('b', vec![1, 1, 0, 1]));
        assert_eq!(encoding[6], ('e', vec![1, 1, 1]));
    }

    #[test]
    fn test_compress() {
        let input_file = "tests/test_uncompressed_file.txt";
        let output_file = "tests/test_compressed_file.txt";

        // NOTE possible put load_tree_from_file inside compress_file()
        let tree = load_tree_from_file(input_file.clone());

        let bit_size = compress_file(&tree, input_file, output_file.clone());

        let input_content = fs::read(input_file)
            .expect("Failed to read file in src/filereader.rs => fn load_tree_from_file");
        let input_content = String::from_utf8(input_content).unwrap();

        let compressed_content = fs::read(output_file)
            .expect("Failed to read file in src/filereader.rs => fn load_tree_from_file");

        let uncompressed_content = tree.decode(&compressed_content, bit_size);

        assert_eq!(input_content, uncompressed_content);
    }
}
