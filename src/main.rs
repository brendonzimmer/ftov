mod args;
mod encoder;
mod metadata;

use args::Args;
use clap::Parser;
use metadata::Metadata;
use encoder::encode;
use std::{fs::File, io::{self, BufReader}, path::Path};

fn main() -> io::Result<()> {
    let Args { input, output, width, height, square, fps, buffer, .. } = Args::parse();

    let mut output = output.unwrap_or_else(|| Path::new("./").with_file_name(input.file_name().unwrap()));
    output.set_extension("mp4");

    if input.is_dir() {
        panic!("Input must be a file, not a directory")
    }

    encode(
        BufReader::new(File::open(input)?),
        output,
        Metadata::new(width, height, fps, square, buffer)
    )
}
