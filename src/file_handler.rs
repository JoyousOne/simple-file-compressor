use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    mem,
    path::{Path, PathBuf},
};

use crate::{
    compressed_buffer::{Bit, CompressedBuffer},
    huffman_tree::{FrequencyChar, HuffmanTree},
};

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

// TODO probably remove
// pub fn load_tree_from_file(file_path: &str) -> HuffmanTree {
//     let mut map: HashMap<char, usize> = HashMap::new();

//     // DEBUG
//     // println!("FILEPATH: {}", &file_path);

//     let bytes = fs::read(file_path)
//         .expect("Failed to read file in src/filereader.rs => fn load_tree_from_file");

//     for c in bytes {
//         let entry = map.get_mut(&(c as char));

//         match entry {
//             Some(value) => {
//                 *value += 1;
//             }
//             None => {
//                 map.insert(c as char, 1);
//             }
//         }
//     }

//     let mut frequencies: Vec<FrequencyChar> = map
//         .iter()
//         .map(|(c, freq)| FrequencyChar(*c, *freq))
//         .collect();

//     HuffmanTree::new(&mut frequencies)
// }

// fn get_huffman_tree_filepath(output_file: &str) -> String {
//     String::from(format!(".{}.hfmt", output_file))
// }

fn inputname_to_outputname(input_file: &str) -> String {
    let tree_path = PathBuf::from(input_file);

    let current_dir = Path::new("");
    let base_filename = tree_path.file_name().unwrap();

    let filepath = current_dir.join(format!("{}.compressed", base_filename.to_string_lossy()));

    String::from(filepath.to_str().unwrap())
}

fn get_original_filename(filename: &str) -> String {
    let filename = filename.strip_suffix(".compressed").unwrap_or(filename);

    String::from(filename)
}

// NOTE output_file not implemented for now
pub fn compress_file(input_file: &str, output_file: Option<&str>) -> String {
    let bytes =
        fs::read(input_file).expect("Failed to read file in src/filereader.rs => fn compress_file");

    let tree = load_tree_from_bytes(&bytes);

    let mut compressed_buffer = CompressedBuffer::new();

    // add tree size at the beginning of the buffer
    let tree_size = tree.len() as u8;
    compressed_buffer.push_byte(tree_size);

    // add tree in the buffer
    let tree_to_byte = tree.as_bytes();
    for byte in tree_to_byte {
        compressed_buffer.push_byte(byte);
    }

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

    // inserting size at the beginning
    // println!("bit_size: {}", bit_size); // DEBUG
    let mut size_in_bytes: [u8; mem::size_of::<usize>()] = bit_size.to_le_bytes();
    size_in_bytes.reverse();
    for byte in size_in_bytes {
        compressed_buffer.insert_byte((tree_size + 1) as usize, byte);
    }

    // OUTPUT FILES //
    // let output_file = inputname_to_outputname(input_file);
    let output_file = match output_file {
        Some(filename) => filename,
        None => &inputname_to_outputname(input_file),
    };

    let mut output_f = File::create(&output_file)
        .expect("Failed to create file in src/filereader.rs => fn compress_file");

    output_f
        .write_all(&compressed_buffer.buffer)
        .expect("Failed to write to file in src/filereader.rs => fn compress_file");

    output_f
        .flush()
        .expect("Failed to flush in src/filereader.rs => fn compress_file");

    // write huffman_tree
    // let tree_path = get_huffman_tree_filepath(&output_file);
    // tree.save_as_file(&tree_path);

    String::from(output_file)
}

pub fn uncompress(compressed_filepath: &str, output_file: Option<&str>) -> String {
    let mut compressed_content = fs::read(compressed_filepath)
        .expect("Failed to read file in src/filereader.rs => fn uncompress");

    let tree_size = compressed_content[0];
    compressed_content.remove(0);

    let (encoded_tree, compressed_data) = compressed_content.split_at(tree_size as usize);

    // converting bytes to tree
    let encoded_tree: Vec<char> = encoded_tree.iter().map(|&byte| byte as char).collect();
    let tree = HuffmanTree::from(encoded_tree);

    // converting compressed_data to data
    let (size, compressed_data) = compressed_data.split_at(mem::size_of::<usize>());

    // Converting back to usize
    let mut size_bytes = [0u8; mem::size_of::<usize>()];
    size_bytes.copy_from_slice(size);
    let size = usize::from_le_bytes(size_bytes);

    let uncompressed_content = tree.decode(&compressed_data, size);

    // extracting to file
    let output_file = match output_file {
        Some(output) => output,
        None => &get_original_filename(compressed_filepath),
    };

    let mut output_f = File::create(output_file)
        .expect("Failed to create file in src/filereader.rs => fn uncompress_file");

    let uncompressed_bytes = uncompressed_content.into_bytes();
    // let uncompressed_bytes = uncompressed_content.as_mut_vec();
    output_f
        .write_all(&uncompressed_bytes)
        .expect("Failed to write to file in src/filereader.rs => fn uncompress_file");

    output_f
        .flush()
        .expect("Failed to flush in src/filereader.rs => fn uncompress_file");

    String::from(output_file)
}

#[cfg(test)]
mod tests {

    use super::*;

    /* NOTE the file test test_uncommpressed_file.txt was generated with:
     ```py
     python3 -c "print('a'*5+'b'*9+'c'*12+'d'*13+'e'*16+'f'*45)" > tests/test_uncommpressed_file.txt
    ```
    */
    // #[test]
    // fn test_load_tree() {
    //     let tree = load_tree_from_file("tests/test_uncompressed_file.txt");

    //     let encoding = tree.get_encoding();

    //     // Should be:
    //     // f: 0
    //     // c: 100
    //     // d: 101
    //     // a: 1100
    //     // b: 1101
    //     // e: 111
    //     assert_eq!(encoding[0], ('f', vec![0]));
    //     assert_eq!(encoding[1], ('c', vec![1, 0, 0]));
    //     assert_eq!(encoding[2], ('d', vec![1, 0, 1]));
    //     assert_eq!(encoding[3], ('\n', vec![1, 1, 0, 0, 0]));
    //     assert_eq!(encoding[4], ('a', vec![1, 1, 0, 0, 1]));
    //     assert_eq!(encoding[5], ('b', vec![1, 1, 0, 1]));
    //     assert_eq!(encoding[6], ('e', vec![1, 1, 1]));
    // }

    #[test]
    fn test_compress_n_uncompress() {
        let input_file = "tests/test_uncompressed_file.txt";
        let output_file = "tests/test_compressed_file";

        compress_file(input_file, Some(output_file));

        let input_content =
            fs::read(input_file).expect("Failed to read file in src/filereader.rs => in test");
        let input_content = String::from_utf8(input_content).unwrap();

        let output_file = inputname_to_outputname(&input_file);
        let restored_file = "tests/restored.txt";
        uncompress(&output_file, Some(&restored_file));

        let output_content =
            fs::read(&restored_file).expect("Failed to read file in src/filereader.rs => in test");
        let output_content = String::from_utf8(output_content).unwrap();

        assert_eq!(input_content, output_content);
    }
}
