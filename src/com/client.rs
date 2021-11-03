//!Author : Damien Frissant

//! Ipv4Addr => use to declare port and ipv4 adress
//! TcpStream => Read and write network stream
use std::io::{stdin, Read, Write};
use std::net::{Ipv4Addr, TcpStream};

/// Set up the connection with the server
pub fn main() {
    //Choose IP address and port
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let port = 1234;
    println!("Try to connect to the server...");
    //Connection to the server
    match TcpStream::connect((ip_addr, port)) {
        Ok(stream) => {
            println!("Connection SUCCES !");
            server_exchanges(stream);
        }
        Err(e) => {
            println!(
                "Conction FAIL.\nCouldn't connect to the server with the following ERROR : {}",
                e
            );
        }
    }
}

fn server_exchanges(mut stream: TcpStream) {
    //Add fix symbole to the terminal
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    //Maximum size buffer
    let buf = &mut [0; 1024];
    let mut message: Vec<u8> = Vec::new();

    println!("Entre 'exit' to leave\n");
    loop {
        //The symbole '>' is write into the terminal
        write!(handle, "> ").expect("Couldn't write into handle the caractere '>'");
        //make sure that the buffered content reach its destination
        handle.flush().expect("Couldn’t flush from stdin");
        match &*get_keypad() {
            "exit" => {
                println!("Good bye !");
                return;
            }
            line => {
                write!(stream, "{}\n", line).expect("Couldn't write the line into stream");
                //read incoming buffer from the server
                match stream.read(buf) {
                    Ok(received) => {
                        if received < 1 {
                            println!("Connection lost with the server");
                            return;
                        }
                        let mut x = 0;
                        //TODO Search to convert buf into string in one line without "for"
                        for c in buf.iter_mut() {
                            if x >= received {
                                break;
                            }
                            x += 1;
                            //When it is the end of the incoming message
                            if *c == '\n' as u8 {
                                println!(
                                    "received message from server : {}",
                                    String::from_utf8(message).unwrap()
                                );
                                message = Vec::new();
                            } else {
                                //add element into the variable
                                message.push(*c);
                            }
                        }
                    }

                    Err(_) => {
                        println!("Connection lost with the server");
                        return;
                    }
                }
            }
        }
    }
}

///Return the string entering on the keypad
fn get_keypad() -> String {
    let mut key_entry = String::new();
    stdin()
        .read_line(&mut key_entry)
        .expect("Couldn’t read line from stdin");
    //To don't care about the letter case, every thing is in lowercase
    key_entry.to_lowercase();
    key_entry.replace("\n", "").replace("\r", "")
}
