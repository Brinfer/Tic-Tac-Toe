//! Module allowing all the management of the tic-tac-toe grid.
//!
//! # Author
//! Pierre-Louis GAUTIER
//! Damien FRISSANT

use crate::{common, screen, DEBUG, TRACE};
use std::fmt;
use std::io::stdin;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<Vec<String>>,
    current_player: common::Player,
}

impl Grid {
    pub fn new(p_size: usize) -> Self {
        let mut l_grid = vec![vec![0.to_string(); p_size]; p_size];

        for i in 0..l_grid.len() {
            for j in 0..l_grid[i].len() {
                l_grid[i][j] = ((p_size * i) + j).to_string();
            }
        }

        Grid {
            grid: l_grid,
            current_player: common::Player::PlayerOne,
        }
    }

    pub fn len(&self) -> usize {
        self.grid[0].len()
    }

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

    pub fn current_player(&self) -> common::Player {
        self.current_player
    }

    pub fn toggle_player(&mut self) {
        self.current_player = match self.current_player {
            common::Player::PlayerOne => common::Player::PlayerTwo,
            common::Player::PlayerTwo => common::Player::PlayerOne,
        }
    }

    pub fn current_symbol(&self) -> &str {
        match self.current_player {
            common::Player::PlayerOne => common::PLAYER_ONE_SYMBOL,
            common::Player::PlayerTwo => common::PLAYER_TWO_SYMBOL,
        }
    }


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

impl fmt::Display for Grid {
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

pub fn create_grid(p_screen: &screen::Screen) -> Grid {
    p_screen.send_msg("Enter the size of the grid you want: ");

    let l_grid_returned: Grid;

    loop {
        match read_keyboard().trim().parse::<usize>() {
            Ok(l_value) => {
                l_grid_returned = Grid::new(l_value);
                break;
            }
            Err(_) => {
                p_screen.send_msg("Bad entry, please retry");
            }
        }
    }

    l_grid_returned
}

pub fn change_cell(p_grid: &mut Grid, p_cell: u8, p_value: &String) -> bool {
    let p_x: usize = (p_cell as usize) / p_grid.len();
    let p_y: usize = (p_cell as usize) % p_grid.len();

    p_grid.set_cell(p_x, p_y, p_value)
}

pub fn is_over(p_grid: &Grid) -> bool {
    test_winner(&p_grid.grid)
}

pub fn player_turn(p_screen: &screen::Screen, p_grid: &mut Grid) {
    loop {
        match read_keyboard().parse() {
            Ok(l_cell) => {
                if change_cell(p_grid, l_cell, &String::from(p_grid.current_symbol())) {
                    break;
                } else {
                    p_screen.send_msg("Bad entry, the cell is already taken or out of range");
                }
            }
            Err(_) => {
                p_screen.send_msg("Bad entry, please retry");
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
    //TODO optimize the "winner" variable
    let mut l_counter: i8 = 0;
    let mut winner = false;
    // Test in the row
    for i in 0..p_grid.len() {
        for j in 0..p_grid[i].len() {
            if p_grid[i][j] == common::PLAYER_TWO_SYMBOL {
                l_counter -= 1;
            } else if p_grid[i][j] == common::PLAYER_ONE_SYMBOL {
                l_counter += 1;
            }
        }
    }

    if (l_counter >= 3) || (l_counter <= -3) {
        winner = true;
    }

    // Test in the column
    for i in 0..p_grid.len() {
        l_counter = 0;
        for j in 0..p_grid[i].len() {
            if p_grid[i][j] == common::PLAYER_TWO_SYMBOL {
                l_counter -= 1;
            } else if p_grid[i][j] == common::PLAYER_ONE_SYMBOL {
                l_counter += 1;
            }
        }
        if (l_counter >= 3) || (l_counter <= -3) {
            winner = true;
        }
    }

    l_counter = 0;

    // Check Left-to-Right downward Diagonal:
    for i in 0..p_grid.len() {
        if p_grid[i][i] == common::PLAYER_TWO_SYMBOL {
            l_counter -= 1;
        } else if p_grid[i][i] == common::PLAYER_ONE_SYMBOL {
            l_counter += 1;
        }
    }

    if (l_counter >= 3) || (l_counter <= -3) {
        winner = true;
    }
    // TODO Rationalize

    // Check Left-to-Right upward Diagonal
    l_counter = 0;
    for i in 0..p_grid.len() {
        if p_grid[i][(p_grid[i].len() - 1) - i] == common::PLAYER_TWO_SYMBOL {
            l_counter -= 1;
        } else if p_grid[i][(p_grid[i].len() - 1) - i] == common::PLAYER_ONE_SYMBOL {
            l_counter += 1;
        }
    }

    if (l_counter >= 3) || (l_counter <= -3) {
        winner = true;
    }
    return winner;
}

pub fn read_keyboard() -> String {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("\x1B[31mCouldn't read line\x1B[0m");
    //To don't care about the letter case, every thing is in lowercase
    buf.to_lowercase();
    buf.replace("\n", "").replace("\r", "")
}
