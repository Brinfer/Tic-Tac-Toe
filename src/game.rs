//! Module allowing all the management of the tic-tac-toe grid.
//!
//! # Author
//! Pierre-Louis GAUTIER
//! Damien FRISSANT

const OPPONENT_SYMBOL: &str = "O";
const PLAYER_SYMBOL: &str = "X";

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Grid {
    grid: Vec<Vec<String>>,
}

impl Grid {
    pub fn new() -> Self {
        let mut l_grid = vec![vec![0.to_string(); 4]; 4];

        for i in 0..l_grid.len() {
            for j in 0..l_grid[i].len() {
                l_grid[i][j] = ((4 * i) + j).to_string();
            }
        }

        Grid { grid: l_grid }
    }

    pub fn len(&self) -> usize {
        self.grid[0].len()
    }

    pub fn get_column(&self, p_x: usize) -> Vec<String> {
        self.grid[p_x].clone()
    }

    pub fn get_cell(&self, p_x: usize, p_y: usize) -> String {
        self.grid[p_x][p_y].to_string()
    }

    pub fn set_cell(&mut self, p_x: usize, p_y: usize, p_value: &String) {
        self.grid[p_x][p_y] = p_value.to_string();
    }
}

pub fn change_cell(p_grid: &mut Grid, p_cell: u8, p_value: &String) {
    let p_x: usize = (p_cell as usize) / p_grid.len();
    let p_y: usize = (p_cell as usize) % p_grid.len();

    println!("Row {}", p_x);
    println!("Column {}", p_y);

    p_grid.set_cell(p_x, p_y, p_value);

    // let flattened = p_grid.clone().into_iter().flatten().collect::<Vec<String>>();

    // println!("{:?}", flattened);

    // if flattened[p_cell] == OPPONENT_SYMBOL || flattened[p_cell] == PLAYER_SYMBOL {
    //     println!("Cell already taken");
    // } else {

    // }

    // let l_contain = &p_grid[p_grid.len() % (p_cell as usize)][p_grid.len() % (p_cell as usize) - p_grid.len()];

    // match l_contain.as_str() {
    //     OPPONENT_SYMBOL | PLAYER_SYMBOL => {
    //         println!("Already taken");
    //     }
    //     _ => {
    //         println!("OK");
    //     }
    // }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn test_winner(p_grid: &Vec<Vec<String>>) {
    let mut l_counter: i8 = 0;

    // Test in the row
    for i in 0..p_grid.len() {
        for j in 0..p_grid[i].len() {
            if p_grid[i][j] == OPPONENT_SYMBOL {
                l_counter -= 1;
            } else if p_grid[i][j] == PLAYER_SYMBOL {
                l_counter += 1;
            }
        }
    }

    if (l_counter >= 3) || (l_counter <= -3) {
        println!("There is a winner");
    }

    // Test in the column
    for i in 0..p_grid.len() {
        l_counter = 0;
        for j in 0..p_grid[i].len() {
            if p_grid[i][j] == OPPONENT_SYMBOL {
                l_counter -= 1;
            } else if p_grid[i][j] == PLAYER_SYMBOL {
                l_counter += 1;
            }
        }
        if (l_counter >= 3) || (l_counter <= -3) {
            println!("There is a winner");
        }
    }

    l_counter = 0;

    // Check Left-to-Right downward Diagonal:
    for i in 0..p_grid.len() {
        if p_grid[i][i] == OPPONENT_SYMBOL {
            l_counter -= 1;
        } else if p_grid[i][i] == PLAYER_SYMBOL {
            l_counter += 1;
        }
    }

    if (l_counter >= 3) || (l_counter <= -3) {
        println!("There is a winner");
    }
    // TODO Rationalize

    // Check Left-to-Right upward Diagonal
    l_counter = 0;
    for i in 0..p_grid.len() {
        if p_grid[i][(p_grid[i].len() - 1) - i] == OPPONENT_SYMBOL {
            l_counter -= 1;
        } else if p_grid[i][(p_grid[i].len() - 1) - i] == PLAYER_SYMBOL {
            l_counter += 1;
        }
    }

    if (l_counter >= 3) || (l_counter <= -3) {
        println!("There is a winner");
    }
}
