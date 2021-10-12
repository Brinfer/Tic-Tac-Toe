extern crate colored;
use colored::*;

use std::io::stdin;

use crate::tools;

pub fn role_selection() -> tools::PlayerRole {
    println!("{}", "Choose your role by entering one of the following role:
                        ----------- Server-----------
                        ----------- Client----------- ".blue().bold());

    let mut answer: tools::PlayerRole = tools::PlayerRole::UNKNOWN;

    match &*setup() {
        "server" => {
            println!("You are the server");
            answer = tools::PlayerRole::HOST;
        }
        "client" => {
            println!("You are the client");
            answer = tools::PlayerRole::CLIENT;
        }
        line => {
            println!("Error {} this is not a type, try again please with 'client or 'server'", line);
        }
    }

    return answer;
}




fn setup() -> String {
    let mut buf = String::new();

    stdin().read_line(&mut buf).expect("Couldn't read line");
    buf.to_lowercase();
    buf.replace("\n", "").replace("\r", "")
}
