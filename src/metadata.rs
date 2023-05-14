pub struct Metadata {
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
        if ss < {if cfg!(debug_assertions) {1} else {3}} || w % ss != 0 || h % ss != 0 {
            eprintln!("Square size must be a factor of the video width and height and >= 3");
            std::process::exit(1)
        }

        Self {
            sw: w / ss,
            sh: h / ss,
            ss,
            fps,
            buffer_size,
        }
    }
}