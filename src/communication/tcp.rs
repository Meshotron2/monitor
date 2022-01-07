use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread; // TODO: make this import the unix one

/// Starts the TCP server
///
/// # Acknowledgements
/// Based on https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo
pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:6789").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 6789");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    println!("Handling stuff");
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

/// Handles a client connection
///
/// # Acknowledgements
/// Based on https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo
fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(_size) => {
            // echo everything!
            // stream.write(&data[0..size]).unwrap();
            println!("Printing number of cores and threads");
            crate::monitor::stats::NodeData::new();
            stream.write(b"I got your message!\n").unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}