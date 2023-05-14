use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
    path::PathBuf,
    process::{Command, Stdio, exit},
};

use crate::metadata::Metadata;

// make last square R = 255
// pub fn encode(mut input: BufReader<File>, output: PathBuf, meta: Metadata) {
//     let mut ffmpeg = Command::new("ffmpeg");
//     ffmpeg
//         .args(&[
//             "-f",
//             "rawvideo",
//             "-pix_fmt",
//             "monob",
//             "-s",
//             &format!("{}x{}", meta.vw, meta.vh),
//             "-r",
//             &format!("{}", meta.fps),
//             "-i",
//             "-",
//             "-c:v",
//             "libx264",
//             "-crf",
//             "0",
//             &format!("{}", output.to_str().unwrap()),
//         ])
//         .stdin(Stdio::piped());
//     // .stdout(Stdio::null())
//     // .stderr(Stdio::inherit());

//     let mut x = ffmpeg.spawn().expect("Failed to spawn ffmpeg");
//     let mut stdin = x.stdin.as_ref().expect("Failed to get stdin");

//     let mut bytes = 0;
//     let mut remainder = vec![];
//     let mut buf = vec![0u8; meta.buffer_size];
//     loop {
//         match input.read(&mut buf) {
//             Ok(0) => {
//                 let frames = ((bytes as f64 / 2.0).ceil() / (meta.vf_sqs as f64)).ceil() as usize;
//                 let mut remain_squares =
//                     ((frames * meta.vf_sqs) as f64 - (bytes as f64 / 2.0).ceil()).ceil() as usize;
//                 let vw_left = remain_squares % meta.vw_sqs;

//                 if remainder.len() > 0 {
//                     remain_squares -= vw_left;
//                     remainder.extend(vec![0u8; vw_left * meta.sq]); // [0u8].repeat(vw_left) or vec![0u8; vw_left]?
//                     remainder = remainder.repeat(meta.sq);
//                     stdin
//                         .write(&remainder)
//                         .expect("Failed to write row to stdin");
//                 }

//                 stdin
//                     .write(&vec![0u8; remain_squares * meta.sq * meta.sq])
//                     .expect("Failed to write row to stdin");
//                 break;
//             }
//             Ok(num) => {
//                 // maybe make R spell filename // mapping could be done in parallel
//                 let pixels = remainder
//                     .iter()
//                     .copied()
//                     .chain(buf[..num].iter().copied())
//                     .collect::<Vec<u8>>();

//                 stdin.write(&pixels).expect("Failed to write row to stdin");

//                 bytes += num;
//             }
//             Err(e) => panic!("Error reading file: {}", e),
//         };
//     }

//     stdin.flush().expect("Failed to flush stdin");
//     drop(stdin);
//     x.wait().expect("Failed to wait for ffmpeg");
// }

pub fn write_frame(mut input: BufReader<File>, output: PathBuf, meta: Metadata) -> io::Result<()> {
    let Metadata {sw, sh, ss, fps, buffer_size } = meta;
    
    let mut buffer = vec![0; buffer_size];

    while let Ok(bytes_read) = input.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let mut byte_index = 0;
        while byte_index < bytes_read {
            let mut frames = vec![vec![vec![vec![0; ss]; ss]; sw]; sh];

            for h in 0..sh {
                for w in 0..sw {
                    if byte_index >= bytes_read {
                        break;
                    }
                    let byte = buffer[byte_index];
                    byte_index += 1;

                    for bit_pos in (0..8).rev() {
                        // Traverse the byte from MSB to LSB
                        let bit = (byte >> bit_pos) & 1; // Extract the bit
                        for dx in 0..ss {
                            for dy in 0..ss {
                                frames[h][w][dx][dy] = bit;
                            }
                        }
                    }
                }
            }

            #[cfg(debug_assertions)] println!("-"); // print '-' to denote a new frame

            // frames row by row
            for h in 0..sh {
                // each square row by row
                for dy in 0..ss {
                    for w in 0..sw {
                        // each square column by column
                        for dx in 0..ss {
                            #[cfg(debug_assertions)] print!("{}", frames[h][w][dx][dy]);
                        }
                        #[cfg(debug_assertions)] print!(" ");
                    }
                    #[cfg(debug_assertions)] println!(); // newline after printing each row of squares
                }

                #[cfg(debug_assertions)]
                if h < sh - 1 {
                    println!(); // newline after printing each frame row
                }
            }
        }
    }

    Ok(())
}
