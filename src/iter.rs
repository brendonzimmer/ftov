use core::fmt;
use bitvec::prelude::*;

#[derive(Debug)]
pub struct SquarePattern<T = u8, O = Msb0> where T: BitStore, O: BitOrder {
    /// the vector of bits to repeat
    bitvec: BitVec<T, O>,
    /// the side length of the square
    ss: usize,
    /// the number of squares per row
    sw: usize,
    /// index of the current bit
    idx: usize,
    /// the current row in a square
    row: usize,
    /// the current column in a square
    col: usize,
}

impl<T, O> SquarePattern<T, O> where T: BitStore, O: BitOrder {
    pub fn new(bitvec: BitVec<T, O>, sq_length: usize, fr_width: usize) -> Result<Self, SquarePatternError> {
        if sq_length < 1 {
            return Err(SquarePatternError::InvalidSquare);
        }
        
        if fr_width % sq_length != 0 || fr_width < sq_length {
            return Err(SquarePatternError::InvalidWidth(fr_width + (fr_width % sq_length)));
        }

        if (bitvec.len() * sq_length) % fr_width != 0 {
            return Err(SquarePatternError::InvalidBitVecLength((fr_width - ((bitvec.len()*sq_length) % fr_width))/sq_length));
        }

        Ok(Self {
            bitvec,
            ss: sq_length,
            sw: fr_width / sq_length,
            idx: 0,
            row: 0,
            col: 0,
        })
    }
}

impl Iterator for SquarePattern {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.bitvec.len() {
            return None;
        }
        
        let bit = self.bitvec[self.idx];
        
        self.col += 1;
        if self.col == self.ss { // if we printed one ss worth of a row
            self.col = 0;
            self.idx += 1;
            if self.idx % self.sw == 0 { // if the row is full
                self.row += 1;
                if self.row != self.ss { // reset idx to the idx at the beginning of the row ONLY if row != ss
                    self.idx -= self.sw;
                } else { // start new row with next idx
                    self.row = 0;
                }
            }
        }
        
        Some(bit)
    }
}

trait SquarePatternIter<T = u8, O = Msb0> where T: BitStore, O: BitOrder {
    fn square_pattern(self, sq_length: usize, fr_width: usize) -> Result<SquarePattern<T, O>, SquarePatternError>;
}

impl<T, O> SquarePatternIter<T, O> for BitVec<T, O> where T: BitStore, O: BitOrder {
    fn square_pattern(self, sq_length: usize, fr_width: usize) -> Result<SquarePattern<T, O>, SquarePatternError> {
        SquarePattern::new(self, sq_length, fr_width)
    }
}

#[derive(Debug)]
pub enum SquarePatternError {
    InvalidSquare,
    InvalidWidth(usize),
    InvalidBitVecLength(usize),
}

impl fmt::Display for SquarePatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSquare => write!(f, "Square length must be >= 1"),
            Self::InvalidWidth(valid) => write!(f, "Frame width must be a multiple of square length\nThe next valid frame width is: {}", valid),
            Self::InvalidBitVecLength(len) => write!(f, "In order to have valid squares on the last row, the length of the bitvec times the square length must be divisible by the frame width\nExtend the bitvec by {} bits for the next valid length", len),
        }
    }
}

impl std::error::Error for SquarePatternError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_square() {
        let bits = bitvec![u8, Msb0; 1, 0, 1, 0];
        let result = bits.square_pattern(0, 4);
        assert!(matches!(result, Err(SquarePatternError::InvalidSquare)));
    }

    #[test]
    fn test_invalid_width() {
        let bits = bitvec![u8, Msb0; 1, 0, 1, 0];
        let result = bits.square_pattern(2, 5);
        assert!(matches!(result, Err(SquarePatternError::InvalidWidth(6))));
    }

    #[test]
    fn test_invalid_bitvec_length() {
        let bits = bitvec![u8, Msb0; 1, 0, 1];
        let result = bits.square_pattern(2, 4);
        assert!(matches!(result, Err(SquarePatternError::InvalidBitVecLength(1))));
    }

    #[test]
    fn test_valid_square_pattern() {
        let bits = bitvec![u8, Msb0; 1, 0, 1, 0];
        let mut sp = bits.square_pattern(2, 4).unwrap();
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), None);
    }

    #[test]
    fn test_large_square_pattern_2sqrows() {
        let bits = bitvec![u8, Msb0; 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
        let mut sp = bits.square_pattern(2, 8).unwrap();
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(true));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), Some(false));
        assert_eq!(sp.next(), None);
    }
}