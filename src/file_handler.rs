use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    compressed_buffer::CompressedBuffer, huffman_tree::HuffmanTree, lzw_encoder::LZWEncoder,
    varsize::encode_varsize,
};

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
// pub fn compress_file(input_file: &str, output_file: Option<&str>) -> String {
//     let bytes =
//         fs::read(input_file).expect("Failed to read file in src/filereader.rs => fn compress_file");

//     // let tree = load_tree_from_bytes(&bytes);
//     let tree = HuffmanTree::load_tree_from_bytes(&bytes);

//     let mut compressed_buffer = CompressedBuffer::new();

//     // add tree size at the beginning of the buffer
//     // FIXME tree size could (in theory) surpass 255
//     let tree_size = tree.len() as u8;
//     compressed_buffer.push_byte(tree_size);

//     // add tree in the buffer
//     let tree_to_byte = tree.as_bytes();
//     for byte in tree_to_byte {
//         compressed_buffer.push_byte(byte);
//     }

//     // NOTE may need to return size
//     let (bit_size, bits) = tree.encode(&bytes);
//     // let bit_size = bits.len();

//     // bits.iter().for_each(|&bit| compressed_buffer.push_bit(bit));

//     // inserting size at the beginning
//     // println!("bit_size: {}", bit_size); // DEBUG
//     let mut size_in_bytes = encode_varsize(bit_size);
//     size_in_bytes.reverse();

//     for byte in size_in_bytes {
//         compressed_buffer.insert_byte((tree_size + 1) as usize, byte);
//     }

//     // OUTPUT FILES //
//     // let output_file = inputname_to_outputname(input_file);
//     let output_file = match output_file {
//         Some(filename) => filename,
//         None => &inputname_to_outputname(input_file),
//     };

//     let mut output_f = File::create(&output_file)
//         .expect("Failed to create file in src/filereader.rs => fn compress_file");

//     output_f
//         .write_all(&compressed_buffer.buffer)
//         .expect("Failed to write to file in src/filereader.rs => fn compress_file");

//     output_f
//         .flush()
//         .expect("Failed to flush in src/filereader.rs => fn compress_file");

//     // write huffman_tree
//     // let tree_path = get_huffman_tree_filepath(&output_file);
//     // tree.save_as_file(&tree_path);

//     String::from(output_file)
// }

pub fn compress(input_file: &str, output_file: Option<&str>) -> String {
    let bytes =
        fs::read(input_file).expect("Failed to read file in src/filereader.rs => fn compress_file");

    let lzw_encoded = LZWEncoder::encode_with_metadata(&bytes);

    let encoded = HuffmanTree::encode_with_metadatas(&lzw_encoded);

    // TODO refactore file write
    let output_file = match output_file {
        Some(filename) => filename,
        None => &inputname_to_outputname(input_file),
    };

    println!("FILENAME: {output_file}");
    let mut output_f = File::create(&output_file)
        .expect("Failed to create file in src/filereader.rs => fn compress_file");

    output_f
        .write_all(&encoded)
        .expect("Failed to write to file in src/filereader.rs => fn compress_file");

    output_f
        .flush()
        .expect("Failed to flush in src/filereader.rs => fn compress_file");

    String::from(output_file)
}

// pub fn uncompress(compressed_filepath: &str, output_file: Option<&str>) -> String {
//     let mut compressed_content = fs::read(compressed_filepath)
//         .expect("Failed to read file in src/filereader.rs => fn uncompress");

//     let decoded = HuffmanTree::decode_with_metadatas(&compressed_content);
//     let decoded = LZWEncoder::decode_with_metadata(&decoded);

//     // extracting to file
//     let output_file = match output_file {
//         Some(output) => output,
//         None => &get_original_filename(compressed_filepath),
//     };

//     // TODO refactore file write
//     let mut output_f = File::create(output_file)
//         .expect("Failed to create file in src/filereader.rs => fn uncompress_file");

//     output_f
//         .write_all(&decoded)
//         .expect("Failed to write to file in src/filereader.rs => fn uncompress_file");

//     output_f
//         .flush()
//         .expect("Failed to flush in src/filereader.rs => fn uncompress_file");

//     String::from(output_file)
// }

pub fn uncompress(compressed_filepath: &str, output_file: Option<&str>) -> String {
    let compressed_content = fs::read(compressed_filepath)
        .expect("Failed to read file in src/filereader.rs => fn uncompress");

    let decoded = HuffmanTree::decode_with_metadatas(&compressed_content);
    let decoded = LZWEncoder::decode_with_metadata(&decoded);

    // TODO refactore file write
    // extracting to file
    let output_file = match output_file {
        Some(output) => output,
        None => &get_original_filename(compressed_filepath),
    };

    let mut output_f = File::create(output_file)
        .expect("Failed to create file in src/filereader.rs => fn uncompress_file");

    output_f
        .write_all(&decoded)
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

    // #[test]
    // fn test_compress_n_uncompress_with_huffman() {
    //     let input_file = "tests/test_uncompressed_file.txt";
    //     // let output_file = "tests/test_compressed_file.txt.compressed";

    //     // compress_file(input_file, Some(output_file));
    //     compress_file(input_file, None);

    //     let input_content =
    //         fs::read(input_file).expect("Failed to read file in src/filereader.rs => in test");
    //     let input_content = String::from_utf8(input_content).unwrap();

    //     let output_file = inputname_to_outputname(&input_file);
    //     let restored_file = "tests/restored.txt";
    //     uncompress(&output_file, Some(&restored_file));

    //     let output_content =
    //         fs::read(&restored_file).expect("Failed to read file in src/filereader.rs => in test");
    //     let output_content = String::from_utf8(output_content).unwrap();

    //     println!("input_content: {}", input_content);
    //     println!("output_content: {}", output_content);

    //     assert_eq!(input_content, output_content);
    // }

    #[test]
    fn test_compress_n_uncompress_with_huffman_n_lzw() {
        let input_file = "tests/test_uncompressed_file.txt";
        // let output_file = "tests/test_compressed_file.txt.compressed";

        // compress_file(input_file, Some(output_file));
        compress(input_file, None);

        let input_content =
            fs::read(input_file).expect("Failed to read file in src/filereader.rs => in test");
        let input_content = String::from_utf8(input_content).unwrap();

        let output_file = inputname_to_outputname(&input_file);
        let restored_file = "tests/restored2.txt";
        uncompress(&output_file, Some(&restored_file));

        let output_content =
            fs::read(&restored_file).expect("Failed to read file in src/filereader.rs => in test");
        let output_content = String::from_utf8(output_content).unwrap();

        assert_eq!(input_content, output_content);
    }

    #[test]
    fn compress_with_lzw_then_huffman() {
        let text: Vec<u8> = "AAABBCCDACCAA".bytes().collect();

        let encoded_lzw = LZWEncoder::encode_with_metadata(&text);
        let encoded_huff = HuffmanTree::encode_with_metadatas(&encoded_lzw);

        let decoded = HuffmanTree::decode_with_metadatas(&encoded_huff);
        assert_eq!(encoded_lzw, decoded);

        let decoded = LZWEncoder::decode_with_metadata(&decoded);
        assert_eq!(text, decoded)
    }
}
