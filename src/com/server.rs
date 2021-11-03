//Author: Damien Frissant
//Many functions are identical from "client.rs" and "server.rs". please, see "client.rs" most questions
use std::io::{stdin, Read, Write};
///Ipv4Addr => use to declare port and ipv4 adress
/// TcpStream => Read and write network stream
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::thread;

/// Set up the connection to wait for clients
pub fn set_up() {
    //Choose IP address and port
    let ip_addr = Ipv4Addr::LOCALHOST;
    let port = 1234;
    //Bind : return a new Tcp instance
    let listener = TcpListener::bind((ip_addr, port)).expect("Failed to bind");

    println!("Waiting for client ... Listening on the port {}", port);

    match listener.accept() {
        Ok((_socket, addr)) => {
            println!("New client with at the address {}", addr);
            //The thread read the variable 'stream'
            thread::spawn(move || client_manager(_socket));
        }
        Err(e) => {
            println!("Connection fail : {}", e);
        }
    }
}

//stream is 'mut' because the instance keep a track of what data it returns
fn client_manager(mut stream: TcpStream) {
    //Add fix symbole to the terminal
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    // TODO Handle the good message relative to the project

    let mut message: Vec<u8> = Vec::new();
    loop {
        //request_buffer store the data that will read
        let request_buffer = &mut [0; 1024];
        //We send the request_buffer into stream.read => that will read the bytes from TcpStream and add them into the request_buffer
        match stream.read(request_buffer) {
            //received is the number of bytes read in stream
            Ok(received) => {
                //if 0 byte receive, the client is deconnected
                if received < 1 {
                    println!(
                        "Client with the address {} is disconnected",
                        stream.peer_addr().unwrap()
                    );
                    return;
                }
                let mut x = 0;
                for c in request_buffer {
                    //we can not have more bytes than the buffer size
                    if x >= received {
                        break;
                    }
                    x += 1;
                    if *c == '\n' as u8 {
                        //Convert byte to string and print it
                        println!(
                            "Received message from {} : {}",
                            stream.peer_addr().unwrap(),
                            String::from_utf8(message).expect("Failed to convert bytes to utf8")
                        ); //print address of the sender and convert the buffer to be printable
                           // Writes some prefix of the byte string, not necessarily all of it.
                        write!(handle, ">").expect("Could not write handle");
                        handle.flush().expect("Could not print handle");
                        match &*get_keypad() {
                            "exit" => {
                                println!("Good bye");
                                return;
                            }
                            line => {
                                write!(stream, "{}\n", line)
                                    .expect("Could not write the line into the stream");
                            }
                        }

                        message = Vec::new();
                    } else {
                        //flush => wait (end don't allow the program to go further) till all the bytes are send into the stream
                        message.push(*c);
                    }
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

///Get the keypad entries
fn get_keypad() -> String {
    let mut key_entry = String::new();
    stdin()
        .read_line(&mut key_entry)
        .expect("Couldn’t read line from stdin");
    //don't care about the letter case, every thing is in lowercase
    key_entry.to_lowercase();
    key_entry.replace("\n", "").replace("\r", "")
}
