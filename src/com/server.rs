//! TODO
//!
//! # Author
//! Damien FRISSANT
//! Pierre-Louis GAUTIER

use crate::{common, ERROR, TRACE};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::thread;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Public
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn set_up() -> Result<TcpStream, ()> {
    let l_ip_addr = Ipv4Addr::LOCALHOST;
    let l_port = 1234;

    let l_listener = TcpListener::bind((l_ip_addr, l_port)).expect("Failed to bind");

    println!("Waiting for client ... Listening on the port {}", l_port);

    match l_listener.accept() {
        Ok((l_stream, l_addr)) => {
            println!("New client at the address {}", l_addr);
            Ok(l_stream)
        }
        Err(e) => {
            println!("Fail to connect to a client");
            ERROR!("Couldn't connect, error : {}",e);
            Err(())
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                              Private
//
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn run(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    // TODO Handle the good message relative to the project

    let mut message: common::Message;

    loop {
        let l_buffer = &mut [0; std::mem::size_of::<common::Message>()];

        match stream.read_exact(l_buffer) {
            Ok(_) => {
                let l_nb_read = std::mem::size_of::<common::Message>();
                if l_nb_read < 1 {
                    println!(
                        "Client at the address {} is disconnected",
                        stream.peer_addr().unwrap()
                    );
                    break;
                }

                let mut x = 0;
                for c in l_buffer {
                    //we can not have more bytes than the buffer size
                    if x >= l_nb_read {
                        break;
                    }
                    x += 1;
                    // if *c == '\n' as u8 {
                    //     //Convert byte to string and print it
                    //     println!(
                    //         "Received message from {} : {}",
                    //         stream.peer_addr().unwrap(),
                    //         String::from_utf8(message.msg).expect("Failed to convert bytes to utf8")
                    //     ); //print address of the sender and convert the buffer to be printable
                    //        // Writes some prefix of the byte string, not necessarily all of it.
                    //     write!(handle, ">").expect("Could not write handle");
                    //     handle.flush().expect("Could not print handle");

                    //     message.msg = Vec::new();
                    // } else {
                    //     //flush => wait (end don't allow the program to go further) till all the bytes are send into the stream
                    //     message.msg.push(*c);
                    // }
                }
            }
            Err(_) => {
                println!(
                    "Client with the address {} is disconnected",
                    stream.peer_addr().unwrap()
                );
                return;
            }
        }
    }
}
