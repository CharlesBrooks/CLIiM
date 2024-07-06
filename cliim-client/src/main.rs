use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::thread;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let mut incoming_message_stream = stream.try_clone().unwrap();
    
    thread::spawn(move || {
        let mut buffer = [0; 512];
         loop {
            match incoming_message_stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                    }
                },
                Err(_) => break,
            }
        }
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stream.write_all(input.as_bytes()).unwrap();
    }
}
