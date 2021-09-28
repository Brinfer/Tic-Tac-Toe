use std::io::{stdin, Read, Write};
///Ipv4Addr => use to declare port and ipv4 adress
/// TcpStream => Read and write network stream
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::thread;

pub fn main_server() {
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let port = 1234;

    let listener = TcpListener::bind((ip_addr, port)).unwrap();

    println!("Waiting for client...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let address = match stream.peer_addr() {
                    Ok(addr) => format!("Host Address is [{}]", addr),
                    Err(_) => "unknown".to_owned(),
                };
                println!("New client with the following address {}", address);
                thread::spawn(move || client_manager(stream, &*address));
            }
            Err(e) => {
                println!("Connection FAIL due to error : {}", e);
            }
        }
        println!("Waiting for another client...");
    }
}

fn client_manager(mut stream: TcpStream, address: &str) {
    let mut message: Vec<u8> = Vec::new();
    loop {
        let request_buffer = &mut [0; 10];
        match stream.read(request_buffer) {
            //received is the number of bytes read in stream
            Ok(received) => {
                //if 0 byte receive, the client is deconnected
                if received < 1 {
                    println!("Client with the address {} is disconnected", address);
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
                        println!(
                            "Received message from {} : {}",
                            address,
                            String::from_utf8(message).unwrap()
                        ); //print address of the sender and convert the buffer to be printable
                        
                        // Writes some prefix of the byte string, not necessarily all of it.
                        stream.write(b"ok\n").expect("Couldn’t write from stdin");
                        message = Vec::new();
                    } else {
                        message.push(*c);
                    }
                }
            }
            Err(_) => {
                println!("Client with the address {} is disconnected", address);
                return;
            }
        }
    }
}

/* fn get_keypad() -> String {
    let mut key_entry = String::new();

    stdin()
        .read_line(&mut key_entry)
        .expect("Couldn’t read line from stdin");
        key_entry.to_lowercase();
    key_entry.replace("\n", "").replace("\r", "")
} */