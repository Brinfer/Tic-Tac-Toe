///Ipv4Addr => use to declare port and ipv4 adress
/// TcpStream => Read and write network stream
use std::io::{stdin, Read, Write};
use std::net::{Ipv4Addr, TcpStream};

pub fn main() {
    let ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    let port = 1234;
    println!("Try to connect to the server...");
    match TcpStream::connect((ip_addr, port)) {
        Ok(stream) => {
            println!("Connection SUCCES !");
            server_exchanges(stream);
        }
        Err(e) => {
            println!(
                "Conction FAIL.\nCouldn't connect to the server with the following error : {}",
                e
            );
        }
    }
}

fn server_exchanges(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    let buf = &mut [0; 3];

    println!("Entre 'exit' to leave\n");

    loop {
        write!(handle, "> ").expect("Couldn't write into handle the caractere '>'");
        //Print the following
        handle.flush().expect("Couldn’t flush from stdin");
        match &*get_keypad() {
            "exit" => {
                println!("Good bye !");
                return;
            }
            line => {
                write!(stream, "{}\n", line).expect("Couldn't write the line into stream");
                match stream.read(buf) {
                    Ok(received) => {
                        if received < 1 {
                            println!("Connection lost with the server");
                            return;
                        }
                    }
                    Err(_) => {
                        println!("Connection lost with the server");
                        return;
                    }
                }
                println!("Reply from the server: {:?}", buf);
            }
        }
    }
}

///Return a string
fn get_keypad() -> String {
    let mut key_entry = String::new();

    stdin()
        .read_line(&mut key_entry)
        .expect("Couldn’t read line from stdin");
        key_entry.to_lowercase();
    key_entry.replace("\n", "").replace("\r", "")
}