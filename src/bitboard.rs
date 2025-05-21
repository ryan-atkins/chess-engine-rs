use std::fmt;
use std::collections::HashMap;

use crate::piece::Color;

pub const EMPTY: usize = 0;
pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

pub const INITIAL_BOARD: [u64; 12] = [
    // white pieces
    0x000000000000FF00, // Pawns
    0x0000000000000042, // Knights
    0x0000000000000024, // Bishops
    0x0000000000000081, // Rooks
    0x0000000000000008, // Queen
    0x0000000000000010, // King

    // black pieces
    0x00FF000000000000, // Pawns
    0x4200000000000000, // Knights
    0x2400000000000000, // Bishops
    0x8100000000000000, // Rooks
    0x0800000000000000, // Queen
    0x1000000000000000, // King
];

pub const PIECE_SYMBOLS: [[char; 6]; 2] = [
    ['P', 'N', 'B', 'R', 'Q', 'K'], // White pieces
    ['p', 'n', 'b', 'r', 'q', 'k'], // Black pieces
];

lazy_static::lazy_static! {
    pub static ref UNICODE_PIECES: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('P', '♙');
        m.insert('N', '♘');
        m.insert('B', '♗');
        m.insert('R', '♖');
        m.insert('Q', '♕');
        m.insert('K', '♔');
        m.insert('p', '♟');
        m.insert('n', '♞');
        m.insert('b', '♝');
        m.insert('r', '♜');
        m.insert('q', '♛');
        m.insert('k', '♚');
        m.insert('.', '·');
        m
    };
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard {
    // 12 bitboard constants. 6 pieces per color
    bitboards: [u64; 12],
    side_to_move: Color,
}

impl Bitboard {
    pub fn new() -> Self {
        Bitboard {
            bitboards: INITIAL_BOARD,
            side_to_move: Color::White,
        }
    }

    pub fn get_side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn make_move(&mut self, from: usize, to: usize) -> bool {
        // Implement making moves on the board

        // Update the side to move
        self.side_to_move = !self.side_to_move;

        true
    }

    fn get_piece_at(&self, square: usize) -> Option<[usize; 2]> {
        let bit = 1 << square;
        // check white pieces
        for piece_type in 0..6 {
            if self.bitboards[piece_type] & bit != 0 {
                return Some([WHITE, piece_type]);
            }
        }
        // check black pieces
        for piece_type in 0..6 {
            if self.bitboards[piece_type + 6] & bit != 0 {
                return Some([BLACK, piece_type]);
            }
        }
        None
    }

    fn is_check(&self) -> bool {
        // Check if the current player's king is in check
        false
    }

    fn is_checkmate(&self) -> bool {
        // Check if the current player is in checkmate
        false
    }

    fn is_stalemate(&self) -> bool {
        // Check if the game is in stalemate
        false
    }


}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_str = String::new();
        board_str.push_str("  a b c d e f g h\n");
        board_str.push_str("  ---------------\n");

        for row in (0..8).rev() {
            // iterate down from 7->0
            board_str.push_str(&format!("{}|", row + 1));

            for col in 0..8 {
                let mut square = row * 8 + col;
                let mut piece = self.get_piece_at(square);
                let mut symbol;
                match piece {
                    Some([color, piece_type]) => {
                        symbol = PIECE_SYMBOLS[color][piece_type];
                    }
                    None => {
                        symbol = '.';
                    }
                }
                let mut buf = [0u8; 4];
                let unicode_char = UNICODE_PIECES[&symbol];
                let unicode_str_ref = unicode_char.encode_utf8(&mut buf);
                let row_str = format!("{} ", unicode_str_ref);
                board_str.push_str(&row_str);
            }

            board_str.push_str(&format!("|{}\n", row + 1));
        }

        board_str.push_str("  ---------------\n");
        board_str.push_str("  a b c d e f g h\n");

        board_str.push_str(&format!("{:?} to move\n", self.side_to_move));

        if self.is_check() {
            board_str.push_str("Check!\n");
        }

        if self.is_checkmate() {
            board_str.push_str("Checkmate!\n");
        }

        if self.is_stalemate() {
            board_str.push_str("Stalemate!\n");
        }

        write!(f, "{}", board_str);
        Ok(())
    }
}
