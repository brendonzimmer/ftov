# **Welcome to ftov, a Rust-based file-to-video encoding and decoding library!**

This is a Rust-based library that provides functionality for encoding and decoding videos using a square pattern iterator. The primary usage of this library is to manipulate videos and their metadata, specifically the width, height, frames per second, square size, and buffer size. The library uses a command line utility, ffmpeg, for processing the video files.

## **Features**

- Reading and writing videos: The library allows for encoding videos into an mp4 format and decoding them for use.
- Square pattern iterator: This feature is used for iterating over the video data in a unique square pattern format.
- Ffmpeg integration: This library uses the ffmpeg command-line tool for video processing.
- Error handling: Custom errors are implemented to handle potential issues with the video metadata.

## **Example Usage**

```rust
use std::{
    fs::File,
    io::BufReader,
    path::PathBuf
};

use crate::Metadata;

let input = BufReader::new(File::open("input_file_path").unwrap());
let output = PathBuf::from("output_file_path");
let meta = Metadata::new(1920, 1080, 60, 16, 4096).unwrap();

// Encoding the video
encode(input, output, meta);
```

This example demonstrates how to encode a video. The `Metadata::new` function is used to define the video's width, height, frames per second, square size, and buffer size.

## **Limitations**

The library currently doesn't have a fully implemented decode function. The encode function is not complete and doesn't yet support writing to stdin in a non-debug environment.

## **Error Types**

The library defines a `MetadataError` enum to handle potential issues with the video metadata. The errors include:

- `InvalidSquare`: Square size must be a factor of the video width and height and >= 1
- `InvalidHeight`: The video height must be >= square size
- `InvalidWidth`: The video width must be >= 1

These errors are returned when the Metadata::new function is called with invalid parameters.

## **Dependencies**

This library depends on the `bitvec`, `std`, and `ffmpeg` crates. The `bitvec` crate is used for bit-level manipulation of data, the `std` crate provides standard Rust functionality, and `ffmpeg` is a command-line tool for handling multimedia data.

## **Future Work**

This library is a work-in-progress. Future updates will aim to provide a complete implementation for the decode function, as well as support for writing to stdin in non-debug environments in the encode function.
