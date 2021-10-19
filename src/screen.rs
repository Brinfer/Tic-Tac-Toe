use crate::common;
use crate::game;
use std::io::stdin;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn role_selection() -> common::PlayerRole {
    println!("Choose your role by entering one of the following role (press \x1B[1mq\x1B[22m to quit):\n\x1B[94m1 : Host\n2 : Guest\x1B[0m");

    let mut answer: common::PlayerRole = common::PlayerRole::UNKNOWN;

    while answer == common::PlayerRole::UNKNOWN {
        match &*read_keyboard() {
            "1" => {
                println!("\x1B[32mYou are the host\x1B[0m");
                answer = common::PlayerRole::HOST;
            }
            "2" => {
                println!("\x1B[32mYou are the guest\x1B[0m");
                answer = common::PlayerRole::GUEST;
            }
            "q" => {
                println!("\x1B[32mYou choose to quit the game.\x1B[0m");
                break;
            }
            line => {
                println!("\x1B[33mError the entered value \x1B[1m{}\x1B[22m is out of the possibility field, please try again.\x1B[0m",line);
            }
        }
    }
    return answer;
}

pub fn display_grid(p_grid: &Vec<Vec<String>>) {
    println!("{}+", "+-----".repeat(p_grid.len()));
    for i in 0..p_grid.len() {
        print!("|");
        for j in 0..p_grid[i].len() {
            print!("{:^5}|", p_grid[i][j]);
        }
        println!("\n{}+", "+-----".repeat(p_grid[i].len()));
    }
}

pub fn write_in_grid(p_grid: &Vec<Vec<String>>) {
    display_grid(p_grid);

    // TODO Protect against bad enter

    loop {
        println!("Enter the number of the box you wish to fill in");
        let l_number: u8 = read_keyboard().parse().unwrap();
        game::change_cell(p_grid, l_number, "X".to_string());
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn read_keyboard() -> String {
    let mut buf = String::new();

    stdin()
        .read_line(&mut buf)
        .expect("\x1B[31mCouldn't read line\x1B[0m");
    buf.to_lowercase();
    buf.replace("\n", "").replace("\r", "")
}
