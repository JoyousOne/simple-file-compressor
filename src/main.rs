use core::panic;

use docopt::Docopt;
use file_handler::{compress_file, uncompress};

mod compressed_buffer;
mod file_handler;
mod huffman_tree;

// http://docopt.org/
const USAGE: &'static str = "
Usage:
    simple-file-compressor (--compress | -c) <file>
    simple-file-compressor (--uncompress | -u) <file> [<output_file>]
    simple-file-compressor (--help | -h)

Options:
    -h, --help               Show this message.
    -c, --compress           compress a given file.
    -u, --uncompress           uncompress a given file.
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

    if compressing {
        // compress file
        let compressed_filename = compress_file(file, "");

        println!("Succesfully compressed as {}", compressed_filename);
    }

    if uncompressing {
        // validate file format
        if let None = file.find(".compressed") {
            panic!(
                "Invalid file given. Compressed file should end with the extension '.compressed'."
            );
        }

        // uncompress file
        let output_file = args.get_str("<output_file>");
        let uncompressed_filename = match output_file {
            "" => uncompress(file, None),
            _ => uncompress(file, Some(output_file)),
        };

        println!("Succesfully uncompressed as {}", uncompressed_filename);
    }
}
