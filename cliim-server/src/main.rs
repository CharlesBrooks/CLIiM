use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Read, Write};

fn handle_client(stream: &mut TcpStream, clients: &mut Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                let clients = clients.lock().unwrap();

                clients.iter().for_each(|mut client| {
                    client.write_all(message.as_bytes()).unwrap();
                });
            },
            Err(_) => break,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let clients = Arc::new(Mutex::new(Vec::new()));

    println!("Server started on port 7878");
    println!("Waiting for connections...");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut clients = Arc::clone(&clients);
        clients.lock().unwrap().push(stream.try_clone().unwrap());
        println!("Client connected : {}", stream.peer_addr().unwrap());
        thread::spawn(move || {
            handle_client(&mut stream, &mut clients);
        });
    }
}