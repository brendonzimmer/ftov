mod encoder;

use crate::encoder::{encode, Metadata};
use std::{path::{PathBuf, Path}, io::{BufReader, self}, fs::File};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_name = "FILEPATH")]
    input: PathBuf,

    #[arg(short, long, value_name = "PATH", default_value = Option::None)]
    output: Option<PathBuf>
}

fn main() -> io::Result<()> {
    let Args {input, output} = Args::parse();
    let mut output = output.unwrap_or_else(|| Path::new("./").with_file_name(input.file_name().unwrap()));
    output.set_extension("mp4");

    if input.is_dir() { 
        panic!("Input must be a file, not a directory")
    }

    encode(
        BufReader::new(File::open(input)?),
        output,
        Metadata::new(3840, 2160, 30, 3, 50_000_000)
    );

    Ok(())
}