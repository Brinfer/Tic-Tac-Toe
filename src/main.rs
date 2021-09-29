#[path = "./com/client.rs"]
mod client;

#[path = "./com/server.rs"]
mod server;

#[path = "./state_machine/state_machine.rs"]
mod state_machine;

extern crate colored;
use colored::*;

use std::io::stdin;

fn main() {
    println!("{}", "Choose your role by entering one of the following role:
                        ----------- Server-----------
                        ----------- Client----------- ".blue().bold());
    match &*setup() {
        "server" => {
            println!("You are the server");
            server::main_server();
        }
        "client" => {
            println!("You are the client");
            client::main();
        }
        line => {
            println!("Error {} this is not a type, try again please with 'client or 'server'", line);
        }
    }
}

fn setup() -> String {
    let mut buf = String::new();

    stdin().read_line(&mut buf).expect("Couldn't read line");
    buf.to_lowercase();
    buf.replace("\n", "").replace("\r", "")
}

// fn runStateMachine() {
//     state_machine::init();
//     state_machine::event(state_machine::Event::Initialized);
//     state_machine::event(state_machine::Event::AskForConnection);
//     state_machine::event(state_machine::Event::ContinueGame);
//     state_machine::event(state_machine::Event::Playing);
//     state_machine::event(state_machine::Event::EndTurn);
//     state_machine::event(state_machine::Event::ContinueGame);
//     state_machine::event(state_machine::Event::Waiting);
//     state_machine::event(state_machine::Event::EndTurn);
//     state_machine::event(state_machine::Event::ExitGame);
//     state_machine::event(state_machine::Event::AskForConnection);
//     state_machine::event(state_machine::Event::Error);
// }
