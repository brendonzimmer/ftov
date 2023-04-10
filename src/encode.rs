use std::{process::{Command, Stdio}, io::Write, path::PathBuf};

// 1 square == 3x3 pixels -> 2 bytes
pub const FPS: usize = 1;
pub const SQUARE: usize = 120;
pub const HEIGHT: usize = 1080; // 2180;

pub const WIDTH: usize = 1920; // 3840;
pub const BWIDTH: usize = WIDTH*3;
pub const UWIDTH: usize = WIDTH*2/SQUARE;

pub const UFRAME: usize = UWIDTH*HEIGHT/SQUARE;

pub fn encode(data: &mut Vec<[u8; BWIDTH]>, output: PathBuf) {
    let mut ffmpeg = Command::new("ffmpeg");

    ffmpeg.args(&[
        "-f", "rawvideo",
        "-pix_fmt", "rgb24",
        "-s", &format!("{WIDTH}x{HEIGHT}"),
        "-r", &FPS.to_string(),
        "-i", "-",
        "-filter:v", "format=yuv420p",
        "-c:v", "libx264",
        // "-tag:v", "hvc1",
        // "-threads", "9",
        &format!("{}", output.to_str().unwrap()),
    ]).stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::inherit());
    
    let mut run = ffmpeg.spawn().unwrap();
    
    let stdin = run.stdin.as_mut().unwrap();
    
    // flatten the 2D array into a 1D array
    let mut h = data.len();
    
    stdin.write_all(&data.iter().flatten().copied().collect::<Vec<u8>>()).unwrap();
    while h < HEIGHT {
        stdin.write_all(&[0u8; BWIDTH]).unwrap();
        h += 1;
    }
    
    
    stdin.flush().unwrap();
    drop(stdin);  // Close the pipe to signal the end of input
    
    // Wait for ffmpeg to finish encoding the video
    let status = run.wait().unwrap();
    assert!(status.success());
}