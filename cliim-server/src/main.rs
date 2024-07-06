use std::net::TcpListener;

mod client_manager;
use client_manager::client_manager::ClientManager;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Server started on port 7878");

    let cm = ClientManager::new();
    cm.start(listener);
}