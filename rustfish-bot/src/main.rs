mod constants;
mod controller;
mod engine;
mod enums;

use crate::controller::UCIController;

fn main() {
    UCIController::default().start();
}
