
use std::{io::{Write, BufReader, Read}, path::PathBuf, process::{Command, Stdio}, fs::File};

pub fn encode(mut input: BufReader<File>, output: PathBuf, meta: Metadata) {
    let mut ffmpeg = Command::new("ffmpeg");
    ffmpeg.args(&[
        "-f", "rawvideo",
        "-pix_fmt", "rgb24",
        "-s", &format!("{}x{}", meta.vw, meta.vh),
        "-r", &format!("{}", meta.fps),
        "-i", "-",
        "-c:v", "libx264",
        "-crf", "0",
        &format!("{}", output.to_str().unwrap())
    ])
    .stdin(Stdio::piped());
    // .stdout(Stdio::null())
    // .stderr(Stdio::inherit());

    let mut x = ffmpeg.spawn().expect("Failed to spawn ffmpeg");
    let mut stdin = x.stdin.as_ref().expect("Failed to get stdin");

    let mut bytes = 0;
    let mut first_byte = true;
    let mut remainder = vec![];
    let mut buf = vec![0u8; meta.buffer_size];
    loop {
        match input.read(&mut buf) {
            Ok(0) => {
                let frames = ((bytes as f64/2.0).ceil()/(meta.vf_sqs as f64)).ceil() as usize;
                let mut remain_squares = ((frames * meta.vf_sqs) as f64 - (bytes as f64/2.0).ceil()).ceil() as usize;
                let vw_left = remain_squares % meta.vw_sqs;

                if remainder.len() > 0 {
                    remain_squares -= vw_left;
                    remainder.extend(vec![0u8; 3*vw_left*meta.sq]); // [0u8].repeat(vw_left) or vec![0u8; vw_left]?
                    remainder = remainder.repeat(meta.sq);
                    stdin.write(&remainder).expect("Failed to write row to stdin");
                }

                stdin.write(&vec![0u8; 3*remain_squares*meta.sq*meta.sq]).expect("Failed to write row to stdin");
                
                first_byte = true;
                break;
            },
            Ok(num) => { // maybe make R spell filename // mapping could be done in parallel
                let pixels = remainder.iter().copied().chain(
                    buf[..num].chunks(2).flat_map(|b| {
                    let mut pixel = vec![0u8, b[0], 0u8];
                    if b.len() == 2 {
                        pixel[2] = b[1];
                    }

                    if first_byte {
                        first_byte = false;
                        pixel[0] = 255;
                    }

                    pixel.repeat(meta.sq)
                })).collect::<Vec<u8>>();
                
                let rows = pixels.chunks_exact(meta.vw*3);
                remainder =  if rows.remainder().len() > 0 { rows.remainder().to_vec() } else { vec![] };
                
                rows.for_each(|row| { // more expensive to [].repeat than loop with big square
                    for _ in 0..meta.sq {
                        stdin.write(&row).expect("Failed to write row to stdin");
                    }
                });

                bytes += num;
            },
            Err(e) => panic!("Error reading file: {}", e)
        };
    };

    stdin.flush().expect("Failed to flush stdin");
    drop(stdin);
    x.wait().expect("Failed to wait for ffmpeg");
}

pub struct Metadata {
    /// video width
    vw: usize,
    
    /// video height
    vh: usize,
    
    /// square size
    sq: usize,
    
    /// frames per second
    fps: usize,

    /// squares per frame
    vf_sqs: usize,
    
    /// squares per vw
    vw_sqs: usize,

    /// bytes read per file read
    buffer_size: usize,
}

impl Metadata {
    pub fn new(vw: usize, vh: usize, fps: usize, sq: usize, buffer_size: usize) -> Self {
        if sq < 3 || vw % sq != 0 || vh % sq != 0 {
            panic!("Square size must be a factor of the video width and height and >= 3")
        }

        Self { 
            vw, vh, sq, fps, buffer_size,
            vf_sqs: (vw*vh)/(sq*sq),
            vw_sqs: vw/sq
        }
    }
}