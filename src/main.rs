use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Connection closed");
                break;
            }
            Ok(n) => {
                if let Ok(text) = std::str::from_utf8(&buffer[..n]) {
                    println!("Received: {}", text.trim_end());
                } else {
                    println!("Received non-UTF8 data");
                }

                if let Err(e) = stream.write_all(&buffer[..n]) {
                    eprintln!("Failed to send response: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind");

    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
      match stream {
          Ok(stream) => {
              println!("New connection: {}", stream.peer_addr().unwrap());
  
              thread::spawn(move || {
                  handle_client(stream);
              });
          }
          Err(e) => {
              eprintln!("Failed to accept connection: {}", e);
          }
      }
  }
  
}
