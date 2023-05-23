use crate::bitboard::BitBoard;
use crate::state::State;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    // example: 8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50
    fen: Box<String>,
    bb_sides: [BitBoard; 2],
    bb_pieces: [[BitBoard; 6]; 2],
    state: State,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            fen: Box::new("startpos".to_string()),
            bb_sides: [BitBoard::empty(); 2],
            bb_pieces: [[BitBoard::empty(); 6]; 2],
            state: State::empty(),
        }
    }
}

impl FromStr for Position {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // return a Position
        Ok(Position {
            fen: Box::new(s.to_string()),
            bb_sides: [BitBoard::empty(); 2],
            bb_pieces: [[BitBoard::empty(); 6]; 2],
            state: State::empty(),
        })
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("{}", self.fen);
        Ok(())
    }
}
