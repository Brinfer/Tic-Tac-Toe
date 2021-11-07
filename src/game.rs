//! Module allowing all the management of the tic-tac-toe game.
//!
//! All information about the current game is saved in a [`Game`] instance. It is then possible to have several games
//! at the same time.
//!
//! # Example
//!
//! The `game` module is closely related to the [screen] module, but can be used independently of it.
//! In this first example, the [screen] module is not used:
//!
//! ```rust
//! mod game;
//! mod screen;
//!
//! let size_grid: usize = 3; // An integer greater than 2, and less than 10, preferably
//! let mut game = Game::new(size_grid);
//!
//! println!(game);
//!
//! while game.is_over() != true {
//!     game.toggle_player();
//!     game.set_cell(<x>, <y>, game.current_symbol());
//! }
//! ```
//!
//! In the second example the [screen] module is used:
//!
//! ```rust
//! mod game;
//! mod screen;
//!
//! let screen = screen::Screen::new_and_start();
//! let mut l_game: game::Game = game::create_game(&screen);
//!
//! while game.is_over() != true {
//!     screen.send_game(&game);
//!
//!     player_turn(,&screen, &game);
//!     game.toggle_player();
//! }
//!
//! screen.stop_and_free();
//! ```
//!
//! # Authors
//! - Pierre-Louis GAUTIER
//! - Damien FRISSANT

use crate::{common, screen, DEBUG, TRACE};
use std::fmt;
use std::io::stdin;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// The structure saving all the elements necessary for the good functioning of a game.
#[derive(Debug, Clone)]
pub struct Game {
    /// The game of the current game
    grid: Vec<Vec<String>>,
    /// The current player playing
    current_player: common::Player,
}

impl Game {
    /// Create a new game
    pub fn new(p_size: usize) -> Self {
        let mut l_grid = vec![vec![0.to_string(); p_size]; p_size];

        for i in 0..l_grid.len() {
            for j in 0..l_grid[i].len() {
                l_grid[i][j] = ((p_size * i) + j).to_string();
            }
        }

        Game {
            grid: l_grid,
            current_player: common::Player::PlayerOne,
        }
    }

    /// Return the size of the grid.
    pub fn len(&self) -> usize {
        self.grid[0].len()
    }

    /// Set the value in the grid, if this cell exist and is free.
    pub fn set_cell(&mut self, p_x: usize, p_y: usize, p_value: &String) -> bool {
        DEBUG!("Row to change {}", p_x);
        DEBUG!("Column to change {}", p_y);

        if p_x < self.grid.len() && p_y < self.grid[0].len() {
            if self.cell_is_free(p_x, p_y) {
                self.grid[p_x][p_y] = p_value.to_string();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Return the current player, see [`common::Player`]
    pub fn current_player(&self) -> common::Player {
        self.current_player
    }

    /// Change the current player to the other one.
    pub fn toggle_player(&mut self) {
        self.current_player = match self.current_player {
            common::Player::PlayerOne => common::Player::PlayerTwo,
            common::Player::PlayerTwo => common::Player::PlayerOne,
        }
    }

    /// Return the symbol to place in the grid of the current player, see [`common::PLAYER_ONE_SYMBOL`] and  [`common::PLAYER_TWO_SYMBOL`]
    pub fn current_symbol(&self) -> &str {
        match self.current_player {
            common::Player::PlayerOne => common::PLAYER_ONE_SYMBOL,
            common::Player::PlayerTwo => common::PLAYER_TWO_SYMBOL,
        }
    }

    /// Test if there is a winner.
    /// Return `true` is there is a winner, `false` otherwise.
    pub fn is_over(&self) -> bool {
        test_winner(&self.grid)
    }

    /// Test id the cell at the given coordinate is free.
    /// Return `true` if the cell is free, `false` otherwise
    fn cell_is_free(&self, p_x: usize, p_y: usize) -> bool {
        DEBUG!(
            "Is already taken by opponent ? {}",
            self.grid[p_x][p_y] == common::PLAYER_TWO_SYMBOL.to_string()
        );
        DEBUG!(
            "Is already taken by player ? {}",
            self.grid[p_x][p_y] == common::PLAYER_ONE_SYMBOL.to_string()
        );

        !(self.grid[p_x][p_y] == common::PLAYER_TWO_SYMBOL.to_string()
            || self.grid[p_x][p_y] == common::PLAYER_ONE_SYMBOL.to_string())
    }
}

/// Implementation of the [`fmt::Display`] trait for a [`Game`] instance
/// # Example
/// ```rust
/// let size_grid: usize = 3; // An integer greater than 2, and less than 10, preferably
/// let mut game = Game::new(size_grid);
///
/// println!(game);
/// ```
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut l_grid: String = format!("{}+\n", "+-----".repeat(self.grid.len()));
        for i in 0..self.grid.len() {
            l_grid.push_str(&format!("|"));
            for j in 0..self.grid[i].len() {
                l_grid.push_str(&format!("{:^5}|", self.grid[i][j]));
            }
            l_grid.push_str(&format!("\n{}+\n", "+-----".repeat(self.grid[i].len())));
        }

        write!(f, "{}", l_grid)
    }
}

/// Create a new [`Game`] associated to a [`screen::Screen`].
///
/// The player will be asked to enter the size of the grid.
pub fn create_game(p_screen: &screen::Screen) -> Game {
    p_screen.send_msg("\x1B[34mEnter the size of the grid you want (between 3 and 9):\x1B[0m ");

    let l_game_returned: Game;

    loop {
        match read_keyboard().trim().parse::<usize>() {
            Ok(l_value) => {
                if l_value > 2 && l_value < 10 {
                    l_game_returned = Game::new(l_value);
                    break;
                } else {
                    p_screen.send_msg("\x1B[41mBad entry, please enter en number greater than 2 and lower than 9. Please retry :\x1B[0m  ");
                }
            }
            Err(_) => {
                p_screen.send_msg("\x1B[41mBad entry, please retry :\x1B[0m  ");
            }
        }
    }

    l_game_returned
}

/// Plays the current player's turn of [`Game`] and displays the grid and information on the [`screen::Screen`].
///
/// Return `true` if the player has modified the grid, `false` if the player has pressed the letter `q`.
/// As long as a valid entry has not been made, the player will be asked again to enter a value
pub fn player_turn(p_screen: &screen::Screen, p_game: &mut Game) -> bool {

    if p_game.current_player() == common::Player::PlayerOne {
        p_screen.send_msg(
            "\x1B[32mPlayer one it is your turn. Enter the cell you want to fill.\x1B[0m \x1B[41mq to quit the game\x1B[0m",
        );
    } else {
        p_screen.send_msg(
            "\x1B[31mPlayer two it is your turn. Enter the cell you want to fill.\x1B[0m \x1B[41mq to quit the game\x1B[0m",
        );
    }

    loop {
        let entered_key = read_keyboard();
        if entered_key == "q" {
            return false;
        } else {
            match entered_key.parse() {
                Ok(l_cell) => {
                    if change_cell(p_game, l_cell, &String::from(p_game.current_symbol())) {
                        return true;
                    } else {
                        p_screen.send_msg(
                            "\x1B[41mBad entry, the cell is already taken or out of range\x1B[0m",
                        );
                    }
                }
                Err(_) => {
                    p_screen.send_msg("\x1B[41mBad entry, please retry\x1B[0m");
                }
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn test_winner(p_grid: &Vec<Vec<String>>) -> bool {
    let mut l_counter_diagonal_lru: i16 = 0; // Left-to-Right upward Diagonal
    let mut l_counter_diagonal_lrd: i16 = 0; // Left-to-Right downward Diagonal
    let score_to_win: i16 = p_grid.len() as i16;

    for i in 0..p_grid.len() {
        let mut l_counter_row: i16 = 0;
        let mut l_counter_column: i16 = 0;

        for j in 0..p_grid[i].len() {
            // Row
            if p_grid[i][j] == common::PLAYER_TWO_SYMBOL {
                l_counter_row -= 1;
            } else if p_grid[i][j] == common::PLAYER_ONE_SYMBOL {
                l_counter_row += 1;
            }

            // Column
            if p_grid[j][i] == common::PLAYER_TWO_SYMBOL {
                l_counter_column -= 1;
            } else if p_grid[j][i] == common::PLAYER_ONE_SYMBOL {
                l_counter_column += 1;
            }
        }

        DEBUG!("[Game] Score row {}", l_counter_row);
        DEBUG!("[Game] Score column {}", l_counter_column);

        if l_counter_row >= score_to_win
            || l_counter_row <= -score_to_win
            || l_counter_column >= score_to_win
            || l_counter_column <= -score_to_win
        {
            return true;
        }

        // Left-to-Right upward Diagonal
        if p_grid[i][i] == common::PLAYER_TWO_SYMBOL {
            l_counter_diagonal_lru -= 1;
        } else if p_grid[i][i] == common::PLAYER_ONE_SYMBOL {
            l_counter_diagonal_lru += 1;
        }

        // Check Left-to-Right upward Diagonal
        if p_grid[i][(p_grid[i].len() - 1) - i] == common::PLAYER_TWO_SYMBOL {
            l_counter_diagonal_lrd -= 1;
        } else if p_grid[i][(p_grid[i].len() - 1) - i] == common::PLAYER_ONE_SYMBOL {
            l_counter_diagonal_lrd += 1;
        }
    }

    DEBUG!(
        "[Game] Score left-to-tight upward diagonal {}",
        l_counter_diagonal_lru
    );
    DEBUG!(
        "[Game] Score left-to-right downward diagonal {}",
        l_counter_diagonal_lrd
    );

    return l_counter_diagonal_lru >= score_to_win
        || l_counter_diagonal_lru <= -score_to_win
        || l_counter_diagonal_lrd >= score_to_win
        || l_counter_diagonal_lrd <= -score_to_win;
}

fn change_cell(p_game: &mut Game, p_cell: u8, p_value: &String) -> bool {
    let p_x: usize = (p_cell as usize) / p_game.len();
    let p_y: usize = (p_cell as usize) % p_game.len();

    p_game.set_cell(p_x, p_y, p_value)
}

fn read_keyboard() -> String {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("\x1B[31mCouldn't read line\x1B[0m");
    //To don't care about the letter case, every thing is in lowercase
    buf.to_lowercase();
    buf.replace("\n", "").replace("\r", "")
}
