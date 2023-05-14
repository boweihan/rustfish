use std::{fmt, str::FromStr};

pub struct Position {
    fen: Box<String>,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            fen: Box::new("startpos".to_string()),
        }
    }
}

impl FromStr for Position {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // return a Position
        Ok(Position {
            fen: Box::new(s.to_string()),
        })
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("{}", self.fen);
        Ok(())
    }
}
