# Rust File Compressor

Rust File Compressor is a simple command-line tool written in Rust for compressing files using gzip compression.

## Features

- Compress single files using gzip compression.
- Fast and efficient compression with Rust's `flate2` crate.
- Command-line interface for easy usage.

## Usage

To compress a file using Rust File Compressor, run the following command:

cargo init
cargo run
cargo run guide.pdf compressed [to get the compressed file]


## NB

To use the `flate2` crate, you have to install the dependencies in the toml file.
