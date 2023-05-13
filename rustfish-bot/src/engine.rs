use std::sync::mpsc;

use crate::enums::GUICommand;

pub struct UCIEngine {
    receiver: mpsc::Receiver<GUICommand>,
}

impl UCIEngine {
    pub fn new(rx: mpsc::Receiver<GUICommand>) -> UCIEngine {
        UCIEngine { receiver: rx }
    }

    pub fn start(&self) {
        println!("Engine started!");

        for command in &self.receiver {
            match command {
                GUICommand::SetOption => println!("SetOption command received!"),
                GUICommand::UciNewGame => println!("UciNewGame command received!"),
                GUICommand::Position => println!("Position command received!"),
                GUICommand::Go => println!("Go command received!"),
                GUICommand::Perft => println!("Perft command received!"),
                _ => panic!("Invalid command received!"),
            }
        }
    }
}
