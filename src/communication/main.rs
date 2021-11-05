//! Author:
//! Damien Frissant

#[path = "./client.rs"]
mod client;
#[path = "./server.rs"]
mod server;



use std::io::stdin;

fn main() {
    println!("{}", "Choose your role by entering one of the following role:
                        ----------- Server-----------
                        ----------- Client----------- ");
    match &*setup() {
        "server" => {
            println!("You are the server");
            server::main();
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