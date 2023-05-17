mod args;
mod iter;
mod encoder;
mod decoder;
mod metadata;

use args::Args;
use clap::Parser;
use encoder::encode;
use metadata::Metadata;
use std::{fs::File, io::BufReader, path::Path};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let Args {
        input,
        output,
        width,
        height,
        square,
        fps,
        buffer,
        ..
    } = Args::parse();

    if input.is_dir() {
        panic!("Input must be a file, not a directory")
    }

    Ok(encode(
        BufReader::new(File::open(&input)?),
        output.unwrap_or_else(|| Path::new("./").with_file_name(input.file_name().unwrap())),
        Metadata::new(width, height, fps, square, buffer)?,
    )?)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}