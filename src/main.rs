mod encode;

use crate::{encode::{encode, UWIDTH, BWIDTH, SQUARE, WIDTH}};
use std::{path::{PathBuf, Path}, io::{BufReader, self, Read}, fs::File};
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

    let mut frame = vec![[0u8; BWIDTH]];
    for _ in 0..SQUARE-1 {
        frame.push([0u8; BWIDTH]);
    }

    let mut w: usize = 0;
    let mut h: usize = 0;

    let mut reader = BufReader::new(File::open(input)?);
    let mut buf = [0u8; UWIDTH];
    loop {
        match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(num) => {
                let iter = buf[..num].chunks_exact(2);
                
                iter.clone().for_each(|pair| {
                    // println!("Writing {:?}", pair);
                    for wd in 0..SQUARE {
                        for hd in 0..SQUARE { 
                            if frame[h+hd][w+1+(wd*3)] != 0 || frame[h+hd][w+2+(wd*3)] != 0 {
                                panic!("Overwriting non-zero value");
                            } 
                            frame[h+hd][w+1+(wd*3)] = pair[0];
                            frame[h+hd][w+2+(wd*3)] = pair[1];
                        }
                    }
                    w += (SQUARE)*3;
                });

                if let Some(byte) = iter.remainder().first() {
                    for wd in 0..SQUARE {
                        for hd in 0..SQUARE { 
                            if frame[h+hd][w+1+(wd*3)] != 0 || frame[h+hd][w+2+(wd*3)] != 0 {
                                panic!("Overwriting non-zero value");
                            } 
                            frame[h+hd][w+1+(wd*3)] = *byte;
                        }
                    }
                }

                w = 0;
                h += SQUARE;
                
                for _ in 0..SQUARE {
                    frame.push([0u8; BWIDTH]);
                }
            },
            Err(e) => panic!("Error reading file: {}", e)
        };
    }

    // let size = frame.len();
    // for wd in 0..SQUARE {
    //     for hd in 0..SQUARE { 
    //         frame[hd][(wd*3)] = 5;
    //         frame[hd+(size-SQUARE)][(WIDTH-SQUARE+(wd*3))] = 255;
    //     }
    // }
    
    encode(&mut frame, output);

    Ok(())
}