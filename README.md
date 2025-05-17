# Rust Chess Engine

This project is a simple chess engine implemented in Rust. It provides the basic functionality to play chess, including move generation, position evaluation, and a command-line interface for user interaction.

## Project Structure

- `src/main.rs`: Entry point of the application. Initializes the chess engine and handles user input and game loop.
- `src/engine.rs`: Contains the `Engine` struct with methods for making moves, evaluating positions, and implementing the chess engine's logic.
- `src/board.rs`: Contains the `Board` struct which represents the chessboard and includes methods for setting up the board, making moves, and checking for valid moves.
- `src/types.rs`: Defines types and enums such as `Piece`, `Color`, and `Move` used throughout the chess engine.

## Setup Instructions

1. Ensure you have Rust installed on your machine. You can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-chess-engine.git
   ```
3. Navigate to the project directory:
   ```
   cd rust-chess-engine
   ```
4. Build the project:
   ```
   cargo build
   ```

## Usage

To run the chess engine, use the following command:
```
cargo run
```

Follow the on-screen instructions to play chess against the engine.

## Features

- Basic move generation and validation
- Simple position evaluation
- Command-line interface for user interaction

Feel free to contribute to the project by submitting issues or pull requests!