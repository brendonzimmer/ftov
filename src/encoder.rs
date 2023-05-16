use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
};
use crate::metadata::Metadata;
use bitvec::prelude::*;

pub fn encode(mut input: BufReader<File>, mut output: PathBuf, meta: Metadata) -> Result<(), Box<dyn std::error::Error>> {
    let Metadata {w, h, sw, sh, ss, fps, buffer_size } = meta;
    
    output.set_extension("mp4");
    let mut ffmpeg = Command::new("ffmpeg");
    ffmpeg
        .args(&[
            "-f", "rawvideo",
            "-pixel_format", "monob", // input pixels
            "-s", &format!("{}x{}", w, h),
            "-r", &format!("{}", fps),
            "-i", "-", // use stdin
            "-c:v", "libx264", // h.264 encoding
            "-pix_fmt", "yuv420p", // output pixels
            "-crf", "0", // 1 to view in quicktime
            &format!("{}", output.to_str().unwrap()),
        ])
        .stdin(Stdio::piped());
    // .stdout(Stdio::null())
    // .stderr(Stdio::inherit());

    let mut x = ffmpeg.spawn().expect("Failed to spawn ffmpeg");
    let mut stdin = x.stdin.as_ref().expect("Failed to get stdin");

    let mut buffer = vec![0; buffer_size];
    
    let row_zero = &bitvec![u8, Msb0; 0; ss.into()];
    let row_one = &bitvec![u8, Msb0; 1; ss.into()];

    while let Ok(bytes_read) = input.read(&mut buffer) {        
        if bytes_read == 0 {
            break;
        }

        let mut b_row: BitVec<u8, Msb0> = BitVec::new();
        let mut row_idx = 0;
        let mut h_count = 0;
        for bit in buffer[..bytes_read].view_bits::<Msb0>() {
            b_row.push(*bit);
            row_idx += 1;

            // will not print row until it is full BUT will print frames even if not full
            if row_idx == sw {
                let mut sender: BitVec<u8, Msb0> = BitVec::new();
                for _ in 0..ss { // meant to print the same row sh times
                    // prints one row of frame
                    for b in b_row.iter() {
                        #[cfg(debug_assertions)] print!("{}", if *b { row_one } else { row_zero });
                        for _ in 0..ss { sender.push(*b); }
                    }
                    #[cfg(debug_assertions)] println!();
                }
                stdin.write(sender.as_raw_slice()).expect("Failed to write to stdin");
                h_count += ss;
                
                if h_count == h {
                    #[cfg(debug_assertions)] println!("-");
                    h_count = 0;
                }
                #[cfg(debug_assertions)] println!();

                row_idx = 0;
                b_row.clear();
            }
        }
        
        // if there are any bits left (unfilled row or frame)
        if !b_row.is_empty() || h_count != 0 {
            #[cfg(debug_assertions)] println!("unfinished square row at idx {row_idx} (added {} sq_rows to finish)", sw-row_idx);
            b_row.extend(std::iter::repeat(false).take((sw - row_idx).into()));
            let mut sender: BitVec<u8, Msb0> = BitVec::new();
            for _ in 0..ss { // meant to print the same row sh times
                // prints one row of frame
                for b in b_row.iter() {
                    #[cfg(debug_assertions)] print!("{}", if *b { row_one } else { row_zero });
                    for _ in 0..ss { sender.push(*b); }
                }
                #[cfg(debug_assertions)] println!();
            }
            stdin.write(sender.as_raw_slice()).expect("Failed to write to stdin");
            b_row.clear();
            h_count += ss;
            
            #[cfg(debug_assertions)] println!("unfinished frame (added {} rows to finish)", h - h_count);
            let mut sender2: BitVec<u8, Msb0> = BitVec::new();
            for _ in 0..(h - h_count) {
                for _ in 0..(sw*ss) {
                    // #[cfg(debug_assertions)] print!("{}", 0);
                    sender2.push(false);
                }
                // #[cfg(debug_assertions)] println!();
            }
            stdin.write(sender2.as_raw_slice()).expect("Failed to write to stdin");
            #[cfg(debug_assertions)] println!("-");
            dbg!()
        }
    }
    
    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);
    x.wait().expect("Failed to wait for ffmpeg");
    Ok(())
}
