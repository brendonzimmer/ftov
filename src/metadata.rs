use core::fmt;

pub struct Metadata {
    pub w: u16,
    pub h: u16,
    /// width of the frame in squares
    pub sw: u16,

    /// height of the frame in squares
    pub sh: u16,

    /// square size in pixels
    pub ss: u16,

    /// frames per second
    pub fps: u8,

    /// number of bytes read per file read
    pub buffer_size: usize,
}

impl Metadata {
    pub fn new(w: u16, h: u16, fps: u8, ss: u16, buffer_size: usize) -> Result<Self, MetadataError> {        
        if ss < 1 || w % ss != 0 || h % ss != 0 {
            return Err(MetadataError::InvalidSquare)
        }

        if h < ss {
            return Err(MetadataError::InvalidHeight);
        }

        if w < 1 || w % 8 != 0 {
            return Err(MetadataError::InvalidWidth);
        }

        Ok(Self {
            w,
            h,
            sw: w / ss,
            sh: h / ss,
            ss,
            fps,
            buffer_size,
        })
    }
}

#[derive(Debug)]
pub enum MetadataError {
    InvalidSquare,
    InvalidHeight,
    InvalidWidth,
}


impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSquare => write!(f, "Square size must be a factor of the video width and height and >= 1"),
            Self::InvalidHeight => write!(f, "The video height must be >= square size"),
            Self::InvalidWidth => write!(f, "The video width must be a multiple of 8 and >= 1"),
        }
    }
}

impl std::error::Error for MetadataError {}