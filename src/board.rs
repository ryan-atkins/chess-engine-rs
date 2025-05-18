pub struct Move {
    from: (usize, usize),
    to: (usize, usize),
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Self {
        Move { from, to }
    }

    pub fn from(&self) -> (usize, usize) {
        self.from
    }

    pub fn to(&self) -> (usize, usize) {
        self.to
    }

    pub fn from_string(input: String) -> Move {
        Move {
            from: (0, 0), // Placeholder for parsing logic
            to: (0, 0),   // Placeholder for parsing logic
        }
    }

    pub fn is_valid(&self) -> bool {
        // Implement logic to check if a move is valid
        true
    }
}
