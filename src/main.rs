// This is the entry point of the chess engine application.
mod engine;
mod board;
mod types;

use std::io::{self, Write};
use engine::Engine;

fn main() {
    let mut engine = Engine::new();
    engine.setup_board();

    loop {
        println!("Enter your move (or 'quit' to exit): ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        match engine.make_move(input) {
            Ok(_) => {
                println!("Move made: {}", input);
                // Here you could add additional logic to evaluate the board state or switch turns
            }
            Err(e) => {
                println!("Error making move: {}", e);
            }
        }
    }
}