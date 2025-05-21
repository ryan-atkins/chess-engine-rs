pub const PAWN: u32 = 0;
pub const KNIGHT: u32 = 1;
pub const BISHOP: u32 = 2;
pub const ROOK: u32 = 3;
pub const QUEEN: u32 = 4;
pub const KING: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

impl core::ops::Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
