use std::sync::mpsc;

use crate::enums::GUICommand;
use crate::position::Position;

pub struct UCIEngine {
    receiver: mpsc::Receiver<GUICommand>,
    position: Position,
}

impl UCIEngine {
    pub fn new(rx: mpsc::Receiver<GUICommand>) -> UCIEngine {
        UCIEngine {
            receiver: rx,
            position: Position::default(),
        }
    }

    pub fn start(&mut self) {
        println!("Engine started!");

        for command in &self.receiver {
            match command {
                GUICommand::SetOption => println!("setoption not currently available!"),
                GUICommand::UciNewGame => {
                    println!("ucinewgame command received!");
                    self.position = Position::default();
                }
                GUICommand::Position(position) => {
                    println!("position received");
                    self.position = *position;
                }
                GUICommand::Go => {
                    println!("go command received!");
                    println!("current position is {}", self.position);

                    // find the best move and return it
                }
                GUICommand::Perft => println!("perft is not implemented yet!"),
                _ => panic!("invalid UCI command received!"),
            }
        }
    }
}
