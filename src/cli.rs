use crate::engine::Engine;
use crate::piece::Color;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameMode {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Cli {
    engine: Engine,
    current_player: Color,
    game_mode: GameMode,
}

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    input.trim().to_string()
}

impl Cli {
    pub fn new(engine: Engine, current_player: Color) -> Cli {
        Cli {
            engine,
            current_player,
            game_mode: GameMode::White,
        }
    }

    pub fn start(&mut self) {
        // Start the cli
        self.display_welcome();
        self.select_game_mode();
        self.game_loop();
    }

    fn display_welcome(&self) {
        println!("{}", "=".repeat(50));
        println!("Bitboard Chess Engine CLI");
        println!("{}", "=".repeat(50));
        println!("Supported commands:");
        println!("  help     - Show this help message");
        println!("  board    - Display the current board position");
        println!("  moves    - Show all legal moves");
        println!("  resign   - Resign the current game");
        println!("{}\n", "=".repeat(50));
    }

    fn select_game_mode(&mut self) {
        let input = get_user_input("select your color [white/black]: ");
        match input.as_str() {
            "white" => self.game_mode = GameMode::White,
            "black" => self.game_mode = GameMode::Black,
            _ => println!("invalid mode selected. defaulting to white."),
        }
    }

    fn display_board(&self) {
        println!("{:?} to move", self.current_player);
    }

    fn display_legal_moves(&self) {
        let legal_moves = self.engine.get_legal_moves();

        if legal_moves.is_empty() {
            println!("No legal moves available.");
        } else {
            println!("\nLegal moves:");
        }
    }

    fn game_loop(&mut self) {
        self.display_board();
    }
}
