#[derive(Debug, Copy, Clone)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
}

pub const PLAYER_TWO_SYMBOL: &str = "  \x1B[31mO\x1B[0m  ";
pub const PLAYER_ONE_SYMBOL: &str = "  \x1B[32mX\x1B[0m  ";
