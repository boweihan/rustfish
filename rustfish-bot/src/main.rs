mod bitboard;
mod constants;
mod controller;
mod engine;
mod enums;
mod position;

use crate::controller::UCIController;

fn main() {
    UCIController::default().start();
}
