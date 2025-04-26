use core::panic;

use docopt::Docopt;
use file_handler::{compress, uncompress};

mod algorithms;
mod big_num;
mod compressed_buffer;
mod file_handler;
mod utils;
mod varsize;

// http://docopt.org/
const USAGE: &'static str = "
Usage:
    simple-file-compressor (--compress | -c) [--algo=<algorithm>...] <file> [<output_file>]
    simple-file-compressor (--uncompress | -u) [--algo=<algorithm>...] <file> [<output_file>]
    simple-file-compressor (--help | -h)

Options:
    -h, --help               Show this message.
    -c, --compress           compress a given file.
    -u, --uncompress         uncompress a given file.
    --algo=<algorithm>       Compression algorithm(s) to use (in order).
                                [default: huff lzw]
                                Options:
                                    - huff, huffman
                                    - lzw, lempel-ziv-welch
                                    - bwt, burrows-wheeler, burrows-wheeler-transform
                                    - mtf, move-to-front
                                    - others to come soon
";

fn main() {
    let argv = std::env::args();

    let args = Docopt::new(USAGE)
        .and_then(|d| d.argv(argv.into_iter()).parse())
        .unwrap_or_else(|e| e.exit());

    // DEBUG
    // println!("{:?}", args);

    let file = args.get_str("<file>");

    let compressing = args.get_bool("--compress");
    let uncompressing = args.get_bool("--uncompress");

    let output_file_arg = args.get_str("<output_file>");
    let output_file = match output_file_arg {
        "" => None,
        filename => Some(filename),
    };

    let algos: Vec<&str> = args.get_vec("--algo");
    let algos = if algos.len() == 0 { None } else { Some(algos) };

    if compressing {
        // compress file
        let compressed_filename = compress(file, output_file, algos);

        println!("Succesfully compressed as {}", compressed_filename);
    } else if uncompressing {
        // validate file format
        if let None = file.find(".compressed") {
            panic!(
                "Invalid file given. Compressed file should end with the extension '.compressed'."
            );
        }

        // uncompress file
        let uncompressed_filename = uncompress(file, output_file, algos);
        println!("Succesfully uncompressed as {}", uncompressed_filename);
    }
}
