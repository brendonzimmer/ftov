use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
};
use crate::{metadata::Metadata, iter::SquarePatternIter};
use bitvec::prelude::*;

pub fn encode(mut input: BufReader<File>, mut output: PathBuf, meta: Metadata) -> Result<(), Box<dyn std::error::Error>> {
    let Metadata {w, h, sw, sh, ss, fps, buffer_size } = meta;
    
    output.set_extension("mp4");
    // let mut ffmpeg = Command::new("ffmpeg");
    // ffmpeg
    //     .args(&[
    //         "-f", "rawvideo",
    //         "-pixel_format", "monob", // input pixels
    //         "-s", &format!("{}x{}", w, h),
    //         "-r", &format!("{}", fps),
    //         "-i", "-", // use stdin
    //         "-c:v", "libx264", // h.264 encoding
    //         "-pix_fmt", "yuv420p", // output pixels
    //         "-crf", "0", // 1 to view in quicktime
    //         &format!("{}", output.to_str().unwrap()),
    //     ])
    //     .stdin(Stdio::piped());
    // // .stdout(Stdio::null())
    // // .stderr(Stdio::inherit());
    // let mut x = ffmpeg.spawn().expect("Failed to spawn ffmpeg");
    // let mut stdin = x.stdin.as_ref().expect("Failed to get stdin");

    let mut buffer = vec![0; buffer_size];
    let bytes_read = input.read(&mut buffer).expect("Failed to read from input file");

    #[cfg(debug_assertions)] {
        let mut count = 0;
        buffer[..bytes_read].view_bits::<Msb0>().to_bitvec().square_pattern(ss.into(), w.into())?.for_each(|pixel| {
            print!("{}", {if pixel { "1" } else { "0" }});
            count += 1;
            if count % (ss * sw) == 0 {
                println!();
                if count % (ss * sw * ss * sh) == 0 {
                    println!("-");
                }
            }
        });
    }
    
    #[cfg(not(debug_assertions))] {
        todo!("Square pattern iterator but in variable number bytes at a time to write to stdin");
        stdin.write("").expect("Failed to write to stdin");
        
        stdin.flush().expect("Failed to flush stdin");
        drop(stdin);
        x.wait().expect("Failed to wait for ffmpeg");
    }

    Ok(())
}
