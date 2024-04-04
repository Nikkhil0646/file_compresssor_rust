extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    // Check if the correct number of command-line arguments is provided
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    // Get the source and target file paths from command-line arguments
    let source_path = &args[1];
    let target_path = &args[2];

    // Attempt to open the source file
    let source_file = match File::open(source_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening source file '{}': {}", source_path, err);
            return;
        }
    };

    // Create the target file
    let target_file = match File::create(target_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating target file '{}': {}", target_path, err);
            return;
        }
    };

    // Create a GzEncoder to compress data into the target file with default compression settings
    let mut encoder = GzEncoder::new(target_file, Compression::default());

    // Create a buffered reader for the source file
    let mut input = BufReader::new(source_file);

    // Start measuring time using Instant
    let start = Instant::now();

    // Copy data from the source to the encoder, which handles compression
    if let Err(err) = copy(&mut input, &mut encoder) {
        eprintln!("Error compressing data: {}", err);
        return;
    }

    // Finish the compression process
    if let Err(err) = encoder.finish() {
        eprintln!("Error finishing compression: {}", err);
        return;
    }

    // Print information about the source and target files, and the elapsed time
    if let (Ok(source_metadata), Ok(target_metadata)) = (File::open(source_path).and_then(|f| f.metadata()), File::open(target_path).and_then(|f| f.metadata())) {
        println!("Source len: {:?}", source_metadata.len());
        println!("Target len: {:?}", target_metadata.len());
    } else {
        eprintln!("Error retrieving file metadata");
    }
    println!("Elapsed time: {:?}", start.elapsed());
}
