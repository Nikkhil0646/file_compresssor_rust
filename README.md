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




# LOGIC

In the main.rs file, the file compression is achieved using the flate2 crate, which provides bindings to the miniz_oxide library for working with DEFLATE format compression (including gzip).

Here's a high-level overview of how the file compression process is implemented in the main.rs file:

### Setting Up Compression:
A GzEncoder object is created with the target file handle and default compression settings from the flate2 crate. This encoder will handle compressing the data into the target file using gzip compression.
### Copying Data for Compression:
Data is read from the source file using a buffered reader and then copied to the GzEncoder. The copy() function from std::io is used for this purpose. This step effectively compresses the data as it is read and written.
### Finishing Compression:
Once all data is copied and compressed, the compression process is finalized. This involves calling encoder.finish() to complete the compression process and ensure that any remaining data is properly compressed and flushed to the target file.
### Error Handling:
Throughout the compression process, error handling is implemented to catch and handle any potential errors that may occur during reading, writing, or compressing the data. If any error occurs, an error message is printed, and the program exits to prevent corrupt or incomplete compressed files.
### Metadata and Time Output:
After compression is completed, the program retrieves metadata information about the source and target files (such as file sizes) and prints this information to the console. Additionally, the elapsed time for the compression process is calculated using Instant and printed as well.
Overall, the main.rs file utilizes the GzEncoder from the flate2 crate to compress data using gzip compression, ensuring efficient and reliable compression of files within the Rust File Compressor tool.
