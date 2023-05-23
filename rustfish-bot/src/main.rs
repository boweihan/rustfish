mod bitboard;
mod constants;
mod controller;
mod engine;
mod enums;
mod position;
mod state;

use crate::controller::UCIController;

fn main() {
    UCIController::default().start();
}
