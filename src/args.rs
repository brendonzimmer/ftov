use std::path::PathBuf;
use clap::{Parser, ArgAction};

#[derive(Parser)]
#[command(about = "Description here", disable_help_flag = true)]
pub struct Args {
    /// Input file to convert
    #[arg(short, value_name = "FILE", )]
    pub input: PathBuf,

    /// A custom output filepath
    #[arg(short, value_name = "FILE", default_value = Option::None)]
    pub output: Option<PathBuf>,

    /// Width of the video in pixels
    #[arg(short = 'w', long, value_name = "NUMBER", default_value_t = 1920)]
    pub width: usize,

    /// Height of the video in pixels
    #[arg(short = 'h', long, value_name = "NUMBER", default_value_t = 1080)]
    pub height: usize,

    /// Defines the size of one file bit in pixels
    #[arg(short, long, value_name = "NUMBER", default_value_t = 3)]
    pub square: usize,

    /// Frames per second
    #[arg(short, long, value_name = "NUMBER", default_value_t = 30)]
    #[arg(value_parser = clap::value_parser!(u8).range(1..=120))]
    pub fps: u8,

    /// Size of the buffer to use to read the input (in bytes)
    #[arg(long, value_name = "NUMBER", default_value_t = 50_000_000)]
    pub buffer: usize,

    /// Prints help information
    #[arg(long, action = ArgAction::Help)]
    pub help: Option<String>,
}