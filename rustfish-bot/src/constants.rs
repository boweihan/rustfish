pub struct EngineInput;
impl EngineInput {
    pub const UCI: &str = "uci";
    pub const DEBUG: &str = "debug";
    pub const IS_READY: &str = "isready";
    pub const SET_OPTION: &str = "setoption";
    pub const REGISTER: &str = "register";
    pub const UCI_NEW_GAME: &str = "ucinewgame";
    pub const POSITION: &str = "position";
    pub const GO: &str = "go";
    pub const PERFT: &str = "perft";
    pub const STOP: &str = "stop";
    pub const PONDER_HIT: &str = "ponderhit";
    pub const QUIT: &str = "quit";
}

pub struct EngineOutput;
impl EngineOutput {
    pub const ID: &str = "id";
    pub const UCI_OK: &str = "uciok";
    pub const READY_OK: &str = "readyok";
    pub const BEST_MOVE: &str = "bestmove";
    pub const COPY_PROTECTION: &str = "copyprotection";
    pub const REGISTRATION: &str = "registration";
    pub const INFO: &str = "info";
    pub const OPTION: &str = "option";
}

pub struct Sides;
impl Sides {
    pub const WHITE: usize = 1;
    pub const BLACK: usize = 1;
}

pub struct Pieces;
impl Pieces {
    pub const PAWN: usize = 0;
    pub const BISHOP: usize = 1;
    pub const KNIGHT: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}
