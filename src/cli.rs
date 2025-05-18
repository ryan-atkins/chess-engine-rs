use crate::board::Move;
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
    computer_depth: i32,
    current_turn: Color,
    game_mode: Color,
}

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

impl Cli {
    pub fn new(engine: Engine, computer_depth: i32, current_turn: Color) -> Cli {
        Cli {
            engine,
            computer_depth,
            current_turn,
            game_mode: Color::White,
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
            "white" => self.game_mode = Color::White,
            "black" => self.game_mode = Color::Black,
            _ => println!("invalid mode selected. defaulting to white."),
        }
    }

    fn display_board(&self) {
        println!("{:?} to move", self.current_turn);
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

        // Main game loop
        loop {
            let game_over = self.engine.is_game_over();
            if game_over {
                println!("Game over.");
                break;
            }

            // Determine if the current player is the computer or human
            if self.current_turn != self.game_mode {
                println!("Computer is thinking...");
                self.engine.make_computer_move(self.computer_depth);
                self.display_board();
            } else {
                println!("Player's turn...");
                let input = get_user_input("{self.current_turn} to move: ");
                match input.as_str() {
                    "resign" => {
                        println!("Thanks for playing!");
                        break;
                    }
                    _ => self.process_command(input),
                }
            }
        }
    }

    fn process_command(&mut self, command: String) {
        match command.as_str() {
            "help" => self.display_welcome(),
            "board" => self.display_board(),
            "moves" => self.display_legal_moves(),
            _ => {
                // Parse the move and make it
                let mv = Move::from_string(command);
                if mv.is_valid() {
                    self.engine.make_move(mv);
                } else {
                    println!("Invalid move.");
                }
            }
        }
    }
}
