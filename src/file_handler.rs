use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::algorithms::{
    arithmetic_encoder::ArithmeticEncoder, burrows_wheeler::BurrowsWheeler,
    huffman_tree::HuffmanTree, lzw_encoder::LZWEncoder, move_to_front::MoveToFront,
    run_length_encoding::RLE,
};

macro_rules! match_algo {
    ($algo:expr => {$huff:expr, $lzw:expr, $bwt:expr, $mtf:expr, $arith:expr, $rle:expr}) => {
        match $algo {
            "huff" | "huffman" => $huff,
            "lzw" | "lempel-ziv-welch" => $lzw,
            "bwt" | "burrows-wheeler" | "burrows-wheeler-transform" => $bwt,
            "mtf" | "move-to-front" => $mtf,
            "arith" | "arithmetic" => $arith,
            "rle" | "run-length-encoding" => $rle,
            _ => panic!("Invalid algorithm selected: {}", $algo),
            // _ => $default,
        }
    };
}

const DEFAULT_COMPRESSION: [&'static str; 2] = ["lzw", "huff"];

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

fn apply_compressing_algos(algos: &mut Vec<&str>, to_encode: &[u8]) -> Vec<u8> {
    let algo = algos.remove(0);

    let mut encoded = match_algo!(
        algo => {
            HuffmanTree::encode_with_metadatas(to_encode),
            LZWEncoder::encode_with_metadatas(to_encode),
            BurrowsWheeler::encode_with_metadata(to_encode, true),
            MoveToFront::encode(to_encode),
            ArithmeticEncoder::encode_with_metadatas(to_encode),
            RLE::encode(to_encode)
        }
    );

    if algos.len() > 0 {
        encoded = apply_compressing_algos(algos, &encoded);
    }

    encoded
}

fn apply_uncompressing_algos(algos: &mut Vec<&str>, to_decode: &[u8]) -> Vec<u8> {
    let algo = algos.pop().unwrap();

    let mut decoded = match_algo!(
        algo => {
            HuffmanTree::decode_with_metadatas(to_decode),
            LZWEncoder::decode_with_metadatas(to_decode),
            BurrowsWheeler::decode_with_metadata(to_decode),
            MoveToFront::decode(to_decode),
            ArithmeticEncoder::decode_with_metadatas(to_decode),
            RLE::decode(to_decode)
         }
    );

    if algos.len() > 0 {
        decoded = apply_uncompressing_algos(algos, &decoded);
    }

    decoded
}

pub fn compress(input_file: &str, output_file: Option<&str>, algos: Option<Vec<&str>>) -> String {
    let bytes =
        fs::read(input_file).expect("Failed to read file in src/filereader.rs => fn compress_file");

    let mut algos: Vec<&str> = match algos {
        Some(al) => al,
        None => DEFAULT_COMPRESSION.to_vec(),
    };

    let encoded = apply_compressing_algos(&mut algos, &bytes);

    // getting file name
    let output_file = match output_file {
        Some(filename) => filename,
        None => &inputname_to_outputname(input_file),
    };

    create_file(output_file, &encoded);

    String::from(output_file)
}

pub fn uncompress(
    compressed_filepath: &str,
    output_file: Option<&str>,
    algos: Option<Vec<&str>>,
) -> String {
    let compressed_content = fs::read(compressed_filepath)
        .expect("Failed to read file in src/filereader.rs => fn uncompress");

    let mut algos = match algos {
        Some(al) => al,
        None => DEFAULT_COMPRESSION.to_vec(),
    };

    let decoded = apply_uncompressing_algos(&mut algos, &compressed_content);

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

    use crate::algorithms::burrows_wheeler::BurrowsWheeler;

    use super::*;

    /* NOTE the file test test_uncommpressed_file.txt was generated with:
     ```sh
     python3 -c "print('a'*5+'b'*9+'c'*12+'d'*13+'e'*16+'f'*45)" > tests/test_uncommpressed_file.txt
    ```
    */
    #[test]
    fn test_compress_n_uncompress_with_huffman_n_lzw() {
        let input_file = "tests/test_uncompressed_file.txt";
        let output_file = "tests/test_compressed_file.compressed";

        // compress_file(input_file, Some(output_file));
        compress(input_file, Some(output_file), None);

        let input_content =
            fs::read(input_file).expect("Failed to read file in src/filereader.rs => in test");
        let input_content = String::from_utf8(input_content).unwrap();

        // let output_file = inputname_to_outputname(&input_file);
        let restored_file = "tests/restored.txt";
        uncompress(&output_file, Some(&restored_file), None);

        let output_content =
            fs::read(&restored_file).expect("Failed to read file in src/filereader.rs => in test");
        let output_content = String::from_utf8(output_content).unwrap();

        assert_eq!(input_content, output_content);
    }

    #[test]
    fn compress_with_lzw_then_huffman() {
        let text: Vec<u8> = "AAABBCCDACCAA".bytes().collect();

        let encoded_lzw = LZWEncoder::encode_with_metadatas(&text);
        let encoded_huff = HuffmanTree::encode_with_metadatas(&encoded_lzw);

        let decoded = HuffmanTree::decode_with_metadatas(&encoded_huff);
        assert_eq!(encoded_lzw, decoded);

        let decoded = LZWEncoder::decode_with_metadatas(&decoded);
        assert_eq!(text, decoded)
    }

    #[test]
    fn compress_with_huffman_then_lzw() {
        let text: Vec<u8> = "AAABBCCDACCAA".bytes().collect();

        let encoded_huff = HuffmanTree::encode_with_metadatas(&text);
        let encoded_lzw = LZWEncoder::encode_with_metadatas(&encoded_huff);

        let decoded = LZWEncoder::decode_with_metadatas(&encoded_lzw);
        assert_eq!(encoded_huff, decoded);

        let decoded = HuffmanTree::decode_with_metadatas(&decoded);
        assert_eq!(text, decoded)
    }

    #[test]
    fn compress_with_bwt_mtf_huff() {
        let mut text: Vec<u8> = "AAABBCCDACCAA".bytes().collect();
        let bloat = text.clone();
        for _ in 0..100 {
            text.extend_from_slice(&bloat);
        }

        let encoded_bwt = BurrowsWheeler::encode_with_metadata(&text, false);
        let encoded_mtf = MoveToFront::encode(&encoded_bwt);
        let encoded_huff = HuffmanTree::encode_with_metadatas(&encoded_mtf);

        // println!("text size: {}", text.len());

        // println!("encoded_bwt {encoded_bwt:?}");
        // println!("encoded_bwt size {}", encoded_bwt.len());

        // println!("encoded_mtf {encoded_mtf:?}");
        // println!("encoded_mtf size {}", encoded_mtf.len());

        // println!("encoded_huff {encoded_huff:?}");
        // println!("encoded_huff size {}", encoded_huff.len());

        let decoded_huff = HuffmanTree::decode_with_metadatas(&encoded_huff);
        assert_eq!(decoded_huff, encoded_mtf);

        let decoded_mft = MoveToFront::decode(&decoded_huff);
        assert_eq!(decoded_mft, encoded_bwt);

        let decoded_bwt = BurrowsWheeler::decode_with_metadata(&decoded_mft);
        assert_eq!(text, decoded_bwt);
    }
}
