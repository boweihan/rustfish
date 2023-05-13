use std::{io, io::BufRead, sync::mpsc, thread};

use crate::constants;
use crate::engine::UCIEngine;
use crate::enums::GUICommand;

pub struct UCIController {}

impl Default for UCIController {
    fn default() -> Self {
        // create a channel with sender and receiver to communicate between threads
        let (tx, rx) = mpsc::channel::<GUICommand>();

        // create a thread for the engine that processes the commands from the main thread
        // move closure variables into thread ownership
        thread::spawn(move || UCIEngine::new());

        UCIController {}
    }
}

impl UCIController {
    // kick off the main loop to process stdin from the GUI
    pub fn start(&self) {
        println!("Welcome to RustFish!");

        let mut lines = io::stdin().lock().lines();

        while let Some(line) = lines.next() {
            match line.unwrap().parse::<GUICommand>() {
                Ok(command) => match command {
                    GUICommand::Uci => {
                        println!(
                            "{} name RustFish\nid author Bowei Han\n{}",
                            constants::ID,
                            constants::UCI_OK
                        )
                    }
                    GUICommand::Debug => println!("Not implemented yet!"),
                    GUICommand::IsReady => println!("{}", constants::READY_OK),
                    GUICommand::SetOption => println!("Not implemented yet!"),
                    GUICommand::Register => println!("Not implemented yet!"),
                    GUICommand::UciNewGame => println!("Not implemented yet!"),
                    GUICommand::Position => println!("Not implemented yet!"),
                    GUICommand::Go => println!("Not implemented yet!"),
                    GUICommand::Stop => println!("Not implemented yet!"),
                    GUICommand::PonderHit => println!("Not implemented yet!"),
                    GUICommand::Quit => break,
                    _ => println!("Not implemented yet!"),
                },
                Err(e) => eprintln!("{e}"),
            }
        }

        println!("Thanks for using RustFish! Exiting...")

        // delegate engine operations to the engine on a separate thread
    }
}
