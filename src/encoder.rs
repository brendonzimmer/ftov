use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
    path::PathBuf,
    process::{Command, Stdio}, u8,
};
use crate::metadata::Metadata;
use bitvec::prelude::*;

pub fn encode(mut input: BufReader<File>, output: PathBuf, meta: Metadata) -> io::Result<()> {
    let Metadata {w, h, sw, sh, ss, fps, buffer_size } = meta;
    
    // let mut ffmpeg = Command::new("ffmpeg");
    // ffmpeg
    //     .args(&[
    //         "-f", "rawvideo",
    //         "-pix_fmt", "monob",
    //         "-s", &format!("{}x{}", w, h),
    //         "-r", &format!("{}", fps),
    //         "-i", "-",
    //         "-c:v", "libx264",
    //         "-crf", "0",
    //         &format!("{}", output.to_str().unwrap()),
    //     ])
    //     .stdin(Stdio::piped());
    // // .stdout(Stdio::null())
    // // .stderr(Stdio::inherit());

    // let mut x = ffmpeg.spawn().expect("Failed to spawn ffmpeg");
    // let mut stdin = x.stdin.as_ref().expect("Failed to get stdin");

    let mut buffer = vec![0; buffer_size];
    
    let row_zero = &bitvec![u8, Msb0; 0; ss];
    let row_one = &bitvec![u8, Msb0; 1; ss];

    while let Ok(bytes_read) = input.read(&mut buffer) {        
        if bytes_read == 0 {
            break;
        }

        let mut b_row: BitVec<u8, Msb0> = BitVec::new();
        let mut b_row_idx = 0;
        let mut h_count = 0;
        for bit in buffer[..bytes_read].view_bits::<Msb0>() {
            b_row.push(*bit);
            b_row_idx += 1;

            if b_row_idx == sw {
                for _ in 0..ss { // meant to print the same row sh times
                    // prints one row of frame
                    for b in b_row.iter() {
                        print!("{}", if *b { row_one } else { row_zero });
                    }
                    println!();
                }
                h_count += ss;
                
                if h_count == h {
                    println!("-");
                    h_count = 0;
                }
                println!();

                b_row_idx = 0;
                b_row.clear();
            }
        }
    }
    
    // stdin.flush().expect("Failed to flush stdin");
    // drop(stdin);
    // x.wait().expect("Failed to wait for ffmpeg");
    Ok(())
}
