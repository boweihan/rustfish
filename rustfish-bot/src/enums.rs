// UCI Spec: https://www.wbec-ridderkerk.nl/html/UCIProtocol.html
pub enum GUICommand {
    Uci,
    Debug,
    IsReady,
    SetOption,
    Register,
    UciNewGame,
    Position,
    Go,
    Stop,
    PonderHit,
    Quit,
}

impl GUICommand {
    fn value(&self) -> &str {
        match *self {
            GUICommand::Uci => "uci",
            GUICommand::Debug => "debug",
            GUICommand::IsReady => "isready",
            GUICommand::SetOption => "setoption",
            GUICommand::Register => "register",
            GUICommand::UciNewGame => "ucinewgame",
            GUICommand::Position => "position",
            GUICommand::Go => "go",
            GUICommand::Stop => "stop",
            GUICommand::PonderHit => "ponderhit",
            GUICommand::Quit => "quit",
        }
    }
}

pub enum EngineCommand {
    Id,
    UciOk,
    ReadyOk,
    BestMove,
    CopyProtection,
    Registration,
    Info,
    Option,
}

impl EngineCommand {
    fn value(&self) -> &str {
        match *self {
            EngineCommand::Id => "id",
            EngineCommand::UciOk => "uciok",
            EngineCommand::ReadyOk => "readyok",
            EngineCommand::BestMove => "bestmove",
            EngineCommand::CopyProtection => "copyprotection",
            EngineCommand::Registration => "registration",
            EngineCommand::Info => "info",
            EngineCommand::Option => "option",
        }
    }
}
