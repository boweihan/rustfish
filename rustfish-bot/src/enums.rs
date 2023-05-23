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
