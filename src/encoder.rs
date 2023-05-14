use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
};
use crate::metadata::Metadata;

pub fn encode(mut input: BufReader<File>, output: PathBuf, meta: Metadata) -> io::Result<()> {
    let Metadata {w, h, sw, sh, ss, fps, buffer_size } = meta;
    
    let mut ffmpeg = Command::new("ffmpeg");
    ffmpeg
        .args(&[
            "-f", "rawvideo",
            "-pix_fmt", "monob",
            "-s", &format!("{}x{}", w, h),
            "-r", &format!("{}", fps),
            "-i", "-",
            "-c:v", "libx264",
            "-crf", "0",
            &format!("{}", output.to_str().unwrap()),
        ])
        .stdin(Stdio::piped());
    // .stdout(Stdio::null())
    // .stderr(Stdio::inherit());

    let mut x = ffmpeg.spawn().expect("Failed to spawn ffmpeg");
    let mut stdin = x.stdin.as_ref().expect("Failed to get stdin");

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

            let mut byte: u8 = 0;
            let mut bit_count = 0;
            for h in 0..sh {
                for dy in 0..ss {
                    for w in 0..sw {
                        for dx in 0..ss {
                            byte = (byte << 1) | frames[h][w][dx][dy];  
                            bit_count += 1;

                            #[cfg(debug_assertions)] print!("{}", frames[h][w][dx][dy]);

                            if bit_count == 8 {
                                stdin.write(&[byte]);
                                byte = 0;
                                bit_count = 0;
                            }
                        }
                        #[cfg(debug_assertions)] print!("|");
                    }

                    #[cfg(debug_assertions)] println!();
                }

                #[cfg(debug_assertions)]
                if h < sh - 1 {
                    println!(); // newline after printing each frame row
                }
            }
            // Check if there are remaining bits after all squares are processed.
            if bit_count != 0 {
                stdin.write(&[byte << (8 - bit_count)]);
            }
        }
    }

    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);
    x.wait().expect("Failed to wait for ffmpeg");
    Ok(())
}
