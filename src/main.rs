#[path = "./com/client.rs"]
mod client;
#[path = "./com/server.rs"]
mod server;


fn main() {
    server::main_server();
    client::main();
}