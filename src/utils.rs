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
