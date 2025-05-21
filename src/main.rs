mod bitboard;
mod cli;
mod engine;
mod movegen;
mod piece;

use crate::cli::Cli;

fn main() {
    let mut cli = Cli::new(engine::Engine::new(), 3, piece::Color::White);
    cli.start();
}
