use std::str::FromStr;

use crate::{constants, position::Position};

// UCI Spec: https://www.wbec-ridderkerk.nl/html/UCIProtocol.html
pub enum GUICommand {
    Uci,
    Debug,
    IsReady,
    SetOption,
    Register,
    UciNewGame,
    Position(Box<Position>),
    Go,
    Perft,
    Stop,
    PonderHit,
    Quit,
}

impl FromStr for GUICommand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        match tokens.next() {
            Some(constants::EngineInput::UCI) => Ok(GUICommand::Uci),
            Some(constants::EngineInput::DEBUG) => Ok(GUICommand::Debug),
            Some(constants::EngineInput::IS_READY) => Ok(GUICommand::IsReady),
            Some(constants::EngineInput::SET_OPTION) => Ok(GUICommand::SetOption),
            Some(constants::EngineInput::REGISTER) => Ok(GUICommand::Register),
            Some(constants::EngineInput::UCI_NEW_GAME) => Ok(GUICommand::UciNewGame),
            Some(constants::EngineInput::POSITION) => Ok(GUICommand::Position(Box::new(
                tokens.collect::<Vec<&str>>().join(" ").parse()?,
            ))),
            Some(constants::EngineInput::GO) => Ok(GUICommand::Go),
            Some(constants::EngineInput::PERFT) => Ok(GUICommand::Perft),
            Some(constants::EngineInput::STOP) => Ok(GUICommand::Stop),
            Some(constants::EngineInput::PONDER_HIT) => Ok(GUICommand::PonderHit),
            Some(constants::EngineInput::QUIT) => Ok(GUICommand::Quit),
            _ => Err("Unable to parse input"),
        }
    }
}

// #[repr(usize)]
#[rustfmt::skip]
pub enum SquareLabels {
    None,
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}
