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
