//! TODO
//!
//! # Author
//! Damien FRISSANT

use std::io::{stdin, Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use crate::{common, TRACE, ERROR};

/// Set up the connection with the server
pub fn set_up() -> Result<TcpStream, ()>{
    //Choose IP address and port
    let ip_addr = Ipv4Addr::LOCALHOST;
    let port = 1234;

    println!("Try to connect to the server ... {}:{}", ip_addr, port);

    match TcpStream::connect((ip_addr, port)) {
        Ok(l_stream) => {
            println!("Connection success !");
            Ok(l_stream)
        }
        Err(e) => {
            println!("Fail to connect to the server");
            ERROR!("Couldn't connect, error : {}",e);
            Err(())
        }
    }
}

fn server_exchanges(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut terminal = stdout.lock();

    //Maximum size buffer
    let buf = &mut [0; std::mem::size_of::<common::Message>()];
    let mut message: Vec<u8> = Vec::new();

    println!("Entre 'q' to leave\n");

    loop {
        //The symbole '>' is write into the terminal
        write!(terminal, "> ").expect("Couldn't write into handle the caractere '>'");
        //make sure that the buffered content reach its destination
        terminal.flush().expect("Couldn’t flush from stdin");
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
