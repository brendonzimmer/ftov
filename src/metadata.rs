pub struct Metadata {
    pub w: usize,
    pub h: usize,

    /// width of the frame in squares
    pub sw: usize,

    /// height of the frame in squares
    pub sh: usize,

    /// square size in pixels
    pub ss: usize,

    /// frames per second
    pub fps: u8,

    /// number of bytes read per file read
    pub buffer_size: usize,
}

impl Metadata {
    pub fn new(w: usize, h: usize, fps: u8, ss: usize, buffer_size: usize) -> Self {
        if ss < 1 || w % ss != 0 || h % ss != 0 {
            eprintln!("Square size must be a factor of the video width and height and >= 1");
            std::process::exit(1)
        }

        // if w % 8 != 0 || h % 8 != 0 {
        //     eprintln!("Video width and height must be a multiple of 8");
        //     std::process::exit(1)
        // }

        Self {
            w,
            h,
            sw: w / ss,
            sh: h / ss,
            ss,
            fps,
            buffer_size,
        }
    }
}