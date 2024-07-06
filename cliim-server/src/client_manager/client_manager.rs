use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread};

pub struct ClientManager {
    pub clients: Arc<Mutex<Vec<TcpStream>>>
}

impl ClientManager { 
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn start(&self, listener: TcpListener) {
        println!("Waiting for connections...");
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut clients = Arc::clone(&self.clients);
            clients.lock().unwrap().push(stream.try_clone().unwrap());
            println!("Client connected : {}", stream.peer_addr().unwrap());
            thread::spawn(move || {
                Self::handle_client(&mut stream, &mut clients);
            });
        }
    }

    fn handle_client(stream: &mut TcpStream, clients: &mut Arc<Mutex<Vec<TcpStream>>>) {
        let mut buffer = [0; 512];
        loop {
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }
                    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("{} from : {}", message, stream.peer_addr().unwrap());
                    let clients = clients.lock().unwrap();
    
                    clients.iter().for_each(|mut client| {
                        client.write_all(message.as_bytes()).unwrap();
                    });
                },
                Err(_) => {println!("Error reading from stream");},
            }
        }
    }
}