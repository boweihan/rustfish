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
            Some(constants::UCI) => Ok(GUICommand::Uci),
            Some(constants::DEBUG) => Ok(GUICommand::Debug),
            Some(constants::IS_READY) => Ok(GUICommand::IsReady),
            Some(constants::SET_OPTION) => Ok(GUICommand::SetOption),
            Some(constants::REGISTER) => Ok(GUICommand::Register),
            Some(constants::UCI_NEW_GAME) => Ok(GUICommand::UciNewGame),
            Some(constants::POSITION) => Ok(GUICommand::Position(Box::new(
                tokens.collect::<Vec<&str>>().join(" ").parse()?,
            ))),
            Some(constants::GO) => Ok(GUICommand::Go),
            Some(constants::PERFT) => Ok(GUICommand::Perft),
            Some(constants::STOP) => Ok(GUICommand::Stop),
            Some(constants::PONDER_HIT) => Ok(GUICommand::PonderHit),
            Some(constants::QUIT) => Ok(GUICommand::Quit),
            _ => Err("Unable to parse input"),
        }
    }
}
