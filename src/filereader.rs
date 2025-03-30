use std::{collections::HashMap, fs};

use crate::huffman_tree::{FrequencyChar, HuffmanTree};

pub fn load_tree_from_file(file_path: &str) -> HuffmanTree {
    let mut map: HashMap<char, usize> = HashMap::new();

    let binding = fs::read_to_string(file_path)
        .expect("Failed to read file in src/filereader.rs => fn load_tree_from_file");

    let lines = binding.lines();

    for line in lines {
        for c in line.chars() {
            let entry = map.get_mut(&c);
            match entry {
                Some(value) => {
                    *value += 1;
                }
                None => {
                    map.insert(c, 1);
                }
            }
        }
    }

    let mut frequencies: Vec<FrequencyChar> = map
        .iter()
        .map(|(c, freq)| FrequencyChar(*c, *freq))
        .collect();

    HuffmanTree::new(&mut frequencies)
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
    fn load_tree() {
        let tree = load_tree_from_file("tests/test_uncommpressed_file.txt");

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
