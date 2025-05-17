pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum Color {
    White,
    Black,
}

pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub piece: Piece,
    pub color: Color,
}