#[path = "./com/client.rs"]
mod client;

#[path = "./com/server.rs"]
mod server;

#[path = "./state_machine/state_machine.rs"]
mod state_machine;

// extern crate colored;
// use colored::*;

use std::io::stdin;

fn main() {
    let game_state_machine : state_machine::StateMachine;

    game_state_machine = state_machine::new();
    state_machine::ask_for_select_role(&game_state_machine);
    state_machine::ask_for_connection(&game_state_machine);
    state_machine::signal_connection_established(&game_state_machine);
    state_machine::signal_to_continue_the_game(&game_state_machine);

    state_machine::signal_to_play(&game_state_machine);
    state_machine::signal_finish_turn(&game_state_machine);
    state_machine::signal_to_continue_the_game(&game_state_machine);
    state_machine::ask_for_wait_opponent(&game_state_machine);
    state_machine::signal_finish_turn(&game_state_machine);
    state_machine::signal_game_finish(&game_state_machine);
    // state_machine::free();

    // println!("{}", "Choose your role by entering one of the following role:
    //                     ----------- Server-----------
    //                     ----------- Client----------- ".blue().bold());
    // match &*setup() {
    //     "server" => {
    //         println!("You are the server");
    //         server::main_server();
    //     }
    //     "client" => {
    //         println!("You are the client");
    //         client::main();
    //     }
    //     line => {
    //         println!("Error {} this is not a type, try again please with 'client or 'server'", line);
    //     }
    // }
}

fn setup() -> String {
    let mut buf = String::new();

    stdin().read_line(&mut buf).expect("Couldn't read line");
    buf.to_lowercase();
    buf.replace("\n", "").replace("\r", "")
}
