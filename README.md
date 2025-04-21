# simple-file-compressor

A simple file compressor implemented written in rust, using Huffman coding.

## Table of content:

- [Installation](#installation)
- [Usage](#usage)
- [How does it word](#how-does-it-work)
- [TODO](#todo)

## Installation

1. **Install [Rust](https://www.rust-lang.org/fr/tools/install)**

2. **Clone the repo:**

```sh
git clone https://github.com/JoyousOne/simple-file-compressor.git
cd simple-file-compressor
```

3. **Dependencies**: Will be automatically installed when running cargo. To consult the dependencies, see [Cargo.toml](/Cargo.toml).

## Usage

```sh
➜ echo "aaaaabbbbbbbbbccccccccccccdddddddddddddeeeeeeeeeeeeeeeefffffffffffffffffffffffffffffffffffffffffffff" > regular_file.txt

➜ ll regular_file.txt
.rw-r--r-- 101 user  4 Apr 13:09 regular_file.txt
# original size 101 bytes

➜ simple-file-compressor --compress regular_file.txt
Succesfully compressed as regular_file.txt.compressed

➜ ll regular_file.txt.compressed
.rw-r--r-- 38 user  4 Apr 13:17 regular_file.txt.compressed
# compressed size 38 bytes

➜ simple-file-compressor --uncompress regular_file.txt.compressed restored.txt
Succesfully uncompressed as restored.txt

➜ diff regular_file.txt restored.txt
# No output indicating that the files are the same
```

## How does it work

### Lempel–Ziv–Welch

> NOTE: Will convert pdf to gif soon
![lzw-slides](assets/demo_lzw.pdf)

## TODO

- Add option to combine algorithm like one would like
- Burrows-Wheeler Transform
- Possibly add option for Dynamic Markov Compression
