#[derive(Debug, PartialEq, Eq)]
pub enum PlayerRole {
    UNKNOWN = 0,
    HOST,
    GUEST,
}

pub struct Message {
    pub msg: Vec<u8>
}

pub const OPPONENT_SYMBOL: &str = "O";
pub const PLAYER_SYMBOL: &str = "X";
