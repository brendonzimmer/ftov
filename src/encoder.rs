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
    let mut rem: Option<BitVec<u8, Msb0>> = None;
    while let Ok(bytes_read) = input.read(&mut buffer) {        
        if bytes_read == 0 {
            println!("in bytes_read = 0");
            if let Some(rem) = rem {
                let mut h_count = 0;
                for row in rem.chunks(ss * sw) {
                    println!("{}", row);
                    h_count += 1;
                    
                    if h_count % ss == 0 {
                        println!();
                    }

                    if h_count == h {
                        println!("-");
                        h_count = 0;
                    }
                }
            }
            break;
        }

        // take each bit and duplicate it ss times next to itself
        let extended_bits = BitVec::<u8, Msb0>::from_slice(&buffer[..bytes_read])
            .into_iter()
            .flat_map(|bit| std::iter::repeat(bit).take(ss))
            .collect::<BitVec<u8, Msb0>>();

        // chunked into frames without rem (two was to go about it: frame => repeat row || repeat row => frame)
        let frames = extended_bits.chunks_exact(ss * sw * sh);
        rem = Some(frames.remainder().to_owned());

        // there are ss*sw bits in a row, and ss*sh bits in a column
        // split the bits into ss*sw bits in a row, and then repeat that ss*sh times
        let frames: BitVec<u8, Msb0> = frames.flatten().collect::<BitVec<u8, Msb0>>()
            .chunks_exact(ss * sw)
            .flat_map(|chunk| std::iter::repeat(chunk).take(ss))
            .flatten()
            .collect();

        // add 0s until the last frame is full  //// issue: this is ss * sw * sh where above is ss * sw
        // rem = Some(frames.chunks_exact(ss * sw * sh).remainder().to_owned());

        let mut h_count = 0;
        for row in frames.chunks(ss * sw) {
            println!("{}", row);
            h_count += 1;
            
            if h_count % ss == 0 {
                println!();
            }

            if h_count == h {
                println!("-");
                h_count = 0;
            }
        }
        


        // let mut byte_index = 0;
        // while byte_index < bytes_read {
        //     let mut frames = vec![vec![vec![vec![0; ss]; ss]; sw]; sh];

        //     for h in 0..sh {
        //         for w in 0..sw {
        //             if byte_index >= bytes_read {
        //                 break;
        //             }
        //             let byte = buffer[byte_index];
        //             byte_index += 1;

        //             for bit_pos in (0..8).rev() {
        //                 // Traverse the byte from MSB to LSB
        //                 let bit = (byte >> bit_pos) & 1; // Extract the bit
        //                 for dx in 0..ss {
        //                     for dy in 0..ss {
        //                         frames[h][w][dx][dy] = bit;
        //                     }
        //                 }
        //             }
        //         }
        //     }

        //     #[cfg(debug_assertions)] println!("-"); // print '-' to denote a new frame

        //     let mut byte: u8 = 0;
        //     let mut bit_count = 0;
        //     for h in 0..sh {
        //         for dy in 0..ss {
        //             for w in 0..sw {
        //                 for dx in 0..ss {
        //                     byte = (byte << 1) | frames[h][w][dx][dy];  
        //                     bit_count += 1;

        //                     #[cfg(debug_assertions)] print!("{}", frames[h][w][dx][dy]);

        //                     if bit_count == 8 {
        //                         // stdin.write(&[byte]);
        //                         byte = 0;
        //                         bit_count = 0;
        //                     }
        //                 }
        //                 #[cfg(debug_assertions)] print!("|");
        //             }

        //             #[cfg(debug_assertions)] println!();
        //         }

        //         #[cfg(debug_assertions)]
        //         if h < sh - 1 {
        //             println!(); // newline after printing each frame row
        //         }
        //     }
        //     // Check if there are remaining bits after all squares are processed.
        //     if bit_count != 0 {
        //         //stdin.write(&[byte << (8 - bit_count)]);
        //     }
        // }
    }

    // stdin.flush().expect("Failed to flush stdin");
    // drop(stdin);
    // x.wait().expect("Failed to wait for ffmpeg");
    Ok(())
}
