use crate::common;
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
    //To don't care about the letter case, every thing is in lowercase
    buf.to_lowercase();
    buf.replace("\n", "").replace("\r", "")
}
