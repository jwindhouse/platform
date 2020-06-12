use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

mod irc;

fn handle_client(mut stream: TcpStream) {
    let mut request = irc::request::new();
    while match stream.read(request.data()) {
        Ok(_size) => {
            if request.valid() {
                for message in request.messages() {
                    println!("String:    {:?}", message.string());
                    println!("Command:   {:?}", message.command());
                    for p in message.parameters() {
                        println!("Parameter: {:?}", p);
                    }
                }
            }
            request.clear_data();

            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6667").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 6667");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
