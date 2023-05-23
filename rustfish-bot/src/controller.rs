use std::{io, io::BufRead, sync::mpsc, thread};

use crate::constants;
use crate::engine::UCIEngine;
use crate::enums::GUICommand;

pub struct UCIController {
    sender: mpsc::Sender<GUICommand>,
}

impl Default for UCIController {
    fn default() -> Self {
        // create a channel with sender and receiver to communicate between threads
        let (tx, rx) = mpsc::channel::<GUICommand>();

        // create a thread for the engine that processes the commands from the main thread
        // move closure variables into thread ownership
        thread::spawn(move || UCIEngine::new(rx).start());

        UCIController { sender: tx }
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
                            constants::EngineOutput::ID,
                            constants::EngineOutput::UCI_OK
                        )
                    }
                    GUICommand::Debug => println!("debug is not implemented yet!"),
                    GUICommand::IsReady => println!("{}", constants::EngineOutput::READY_OK),
                    GUICommand::Register => println!("register is not implemented yet!"),
                    GUICommand::Stop => println!("stop is not implemented yet!"),
                    GUICommand::PonderHit => println!("ponderhit is not implemented yet!"),
                    GUICommand::Quit => break,
                    _ => self.sender.send(command).unwrap(),
                },
                Err(e) => eprintln!("{e}"),
            }
        }

        println!("Thanks for using RustFish! Exiting...")

        // delegate engine operations to the engine on a separate thread
    }
}
