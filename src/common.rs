//! Module containing basic information shared between other modules.
//!
//! # Authors
//! - Pierre-Louis GAUTIER
//! - Damien FRISSANT

#[derive(Debug, Copy, Clone, PartialEq)]
/// Enumeration containing the different roles of the players
pub enum Player {
    /// The role player one
    PlayerOne,
    /// The role player two
    PlayerTwo,
}

/// The symbol associated with player one: a green X
pub const PLAYER_ONE_SYMBOL: &str = "  \x1B[32mX\x1B[0m  ";

/// The symbol associated with player two: a red O
pub const PLAYER_TWO_SYMBOL: &str = "  \x1B[31mO\x1B[0m  ";
