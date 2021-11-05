#[derive(Debug, Copy, Clone)]
pub enum Player {
    PlayerOne,
    PlayerTwo, 
}

pub struct Message {
    pub msg: Vec<u8>
}

pub const OPPONENT_SYMBOL: &str = "O";
pub const PLAYER_SYMBOL: &str = "X";
