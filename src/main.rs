mod encoder;

use crate::encoder::{encode, Metadata};
use std::{path::{PathBuf, Path}, io::{BufReader, self}, fs::File};

fn main() -> io::Result<()> {
    let Args {
        input, output, width, height, square, fps, buffer_size
    } = Args::parse();

    let mut output = output.unwrap_or_else(|| Path::new("./").with_file_name(input.file_name().unwrap()));
    output.set_extension("mp4");

    if input.is_dir() { 
        panic!("Input must be a file, not a directory")
    }

    encode(
        BufReader::new(File::open(input)?),
        output,
        Metadata::new(width, height, fps, square, buffer_size)
    );

    Ok(())
}

use clap::Parser;
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_name = "FILEPATH")]
    input: PathBuf,

    #[arg(short, long, value_name = "PATH", default_value = Option::None)]
    output: Option<PathBuf>,

    #[arg(short, long, value_name = "PIXELS", default_value_t = 1920)]
    width: usize,
    
    #[arg(short, long, value_name = "PIXELS", default_value_t = 1080)]
    height: usize,
    
    #[arg(short, long, value_name = "FPS", default_value_t = 30)]
    fps: usize,
    
    #[arg(short, long, value_name = "PIXELS", default_value_t = 3)]
    square: usize,
    
    #[arg(short, long, value_name = "BYTES", default_value_t = 50_000_000)]
    buffer_size: usize,
}