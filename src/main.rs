mod board;
mod cli;
mod engine;
mod piece;

use crate::cli::Cli;

fn main() {
    let mut cli = Cli::new(engine::Engine::new(), piece::Color::White);
    cli.start();
}
