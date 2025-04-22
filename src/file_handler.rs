use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{huffman_tree::HuffmanTree, lzw_encoder::LZWEncoder};

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

fn create_file(output_file: &str, content: &Vec<u8>) {
    let mut output_f = File::create(output_file).expect("Failed to create file.");

    output_f
        .write_all(&content)
        .expect("Failed to write to file.");

    output_f.flush().expect("Failed to flush");
}

pub fn compress(input_file: &str, output_file: Option<&str>) -> String {
    let bytes =
        fs::read(input_file).expect("Failed to read file in src/filereader.rs => fn compress_file");

    let lzw_encoded = LZWEncoder::encode_with_metadata(&bytes);

    let encoded = HuffmanTree::encode_with_metadatas(&lzw_encoded);

    // getting file name
    let output_file = match output_file {
        Some(filename) => filename,
        None => &inputname_to_outputname(input_file),
    };

    create_file(output_file, &encoded);

    String::from(output_file)
}

pub fn uncompress(compressed_filepath: &str, output_file: Option<&str>) -> String {
    let compressed_content = fs::read(compressed_filepath)
        .expect("Failed to read file in src/filereader.rs => fn uncompress");

    let decoded = HuffmanTree::decode_with_metadatas(&compressed_content);
    let decoded = LZWEncoder::decode_with_metadata(&decoded);

    // getting file name
    let output_file = match output_file {
        Some(output) => output,
        None => &get_original_filename(compressed_filepath),
    };

    create_file(output_file, &decoded);

    String::from(output_file)
}

#[cfg(test)]
mod tests {

    use super::*;

    /* NOTE the file test test_uncommpressed_file.txt was generated with:
     ```sh
     python3 -c "print('a'*5+'b'*9+'c'*12+'d'*13+'e'*16+'f'*45)" > tests/test_uncommpressed_file.txt
    ```
    */
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
