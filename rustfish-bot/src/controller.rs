use std::{sync::mpsc, thread, time::Duration};

use crate::engine::UCIEngine;
use crate::enums::{EngineCommand, GUICommand};

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
        println!("RustFish by Bowei Han")

        // handle synchronous operations

        // delegate engine operations to the engine on a separate thread
    }
}
