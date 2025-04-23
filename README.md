# simple-file-compressor

A simple file compressor implemented written in rust.

A personal Rust project exploring various file compression algorithms — for learning, experimentation, and fun.

## Table of content:

- [About](#about)
- [Features](#features)
- [Supported algorithm](#-supported-algorithms)
- [Installation](#installation)
- [Usage](#usage)
- [How does it word](#how-does-it-work)
- [TODO](#todo)

## About

This project is a learning exercise aimed at better understanding **how different compression algorithms work**, how they compare, and how they can be implemented efficiently in Rust.

You can use it to compress and decompress files using multiple algorithms in sequence (e.g. Huffman followed by LZW), allowing for a flexible and modular approach to compression.

## Features

- 📦 Compress and decompress files via CLI
- 🔗 Chain multiple compression algorithms
- 🦀 Written in Rust for safety and performance

## 🔧 Supported Algorithms

- **Huffman Coding** (`huff`, `huffman`)
- **LZW (Lempel-Ziv-Welch)** (`lzw`, `lempel-ziv-welch`)
- _(More to come soon!)_

> NOTE: words following the algorithm name are use to reorder the algorithm usage.

## Installation

1. **Install [Rust](https://www.rust-lang.org/fr/tools/install)**

2. **Clone the repo:**

```sh
git clone https://github.com/JoyousOne/simple-file-compressor.git
cd simple-file-compressor
```

3. **Dependencies**: Will be automatically installed when running cargo. To consult the dependencies, see [Cargo.toml](/Cargo.toml).

## Usage

- [Manual](#manual)
- [Default usage](#default-usage)
- [Algorithm Reordering](#algorithm-reordering)

### Manual:

```sh
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
                                    - others to come soon
```

### Default Usage:

> Uses the Lempel-Ziv-Welch algorithm followed by huffman encoding.

```sh
# creating a file which is very conveniently full of the same characters to compress.
➜ python3 -c "print('A' * 1000 + 'B' * 2000 + 'A' * 540 + 'C' * 3000)" > regular_file.txt

➜ ll -B regular_file.txt
.rw-r--r-- 6,541 user regular_file.txt
# original size 6541 bytes

➜ simple-file-compressor --compress regular_file.txt
Succesfully compressed as regular_file.txt.compressed

➜ ll -B regular_file.txt.compressed
.rw-r--r-- 155 user regular_file.txt.compressed
# compressed size 38 bytes

➜ simple-file-compressor --uncompress regular_file.txt.compressed restored.txt
Succesfully uncompressed as restored.txt

➜ ll -B restored.txt
.rw-r--r-- 6,541 user restored.txt

➜ diff regular_file.txt restored.txt
# No output indicating that the files are the same
```

### Algorithm Reordering:

Using only one algorithm:

```sh
# Huffman encoding
simple-file-compressor --compress --algo=huff file.txt
simple-file-compressor --uncompress --algo=huff file.txt.compressed

# Lempel-Ziv-Welch
simple-file-compressor --compress --algo=lzw file.txt
simple-file-compressor --uncompress --algo=lzw file.txt
```

Using multiple algorithm:

The algorithms are applied in the order they are given. For Lempel-Ziv-Welch followed by Huffman encoding you would do:

> Default when no algorithms are specified.

```sh
# Lempel-Ziv-Welch followed by Huffman encoding

# Encoding
simple-file-compressor --compress --algo=lzw --algo=huff file.txt

# Decoding
# Must put the algorithms in the same order as the encoding even thought they are applied in the reversed order to decode (Didn't want to make it more complicated then it is).
simple-file-compressor --uncompress --algo=lzw --algo=huff file.txt
```

## How does it work

### Lempel–Ziv–Welch

> NOTE: Will convert pdf to gif soon

> ![lzw-slides](assets/demo_lzw.pdf)

## TODO

- Add option to combine algorithm like one would like
- Burrows-Wheeler Transform
- arithmetic compression
- asymmetric numeral systems (ANS)
    excellent YouTube video about it: https://youtu.be/RFWJM8JMXBs?si=PXemuPzI_-kTOMfj
    worth looking at
    - range ANS (rANS)
    - Uniform Birary Variant (uABS)
    
- Possibly add option for Dynamic Markov Compression
