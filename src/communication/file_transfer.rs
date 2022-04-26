//! Holds methods to transfer and receive files.
//! File reception is handled through a TCP server

use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};
use std::io::Seek;
use byteorder::{ByteOrder, LittleEndian, NativeEndian, WriteBytesExt};

/// Starts a server that receives files from the partitioner.
///
/// Since the monitor should only receive room description files, the file extension is assumed to be .dwm.
///
/// # Arguments
///
/// - `ip`: The ip to start the server on
/// - `port`: The port to bind the server to
/// - `file_name`: The base name of the file that will be received.
/// All the files received will have the same name with a number appended, representing the arrival
/// order of the file.
pub fn start_file_server(ip: String, port: usize, file_name: &'static str) {
    let listener = TcpListener::bind(ip + ":" + &*port.to_string()).unwrap();

    let cnt = Arc::new(Mutex::new(0));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let cnt_handle = Arc::clone(&cnt);

                thread::spawn(move || {
                    receive_file(stream, file_name, &mut cnt_handle.lock().unwrap())
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}

/// Handles an incoming TCP byte stream containing a file
///
/// # Arguments
/// - `stream`: The incoming file TCP stream
/// - `file_name`: The name to write the file to
/// - `counter`: A counter with the number of files received
///
/// # Protocol
///
/// The bytes stream should only contain the file
///
/// # Acknowledgements
/// With the help from [Stack Overflow](HTTPS://Stackoverflow.com/questions/53826371/how-to-create-a-binary-file-with-rust)
fn receive_file(mut stream: TcpStream, file_name: &str, counter: &mut i32) {
    let file_name = file_name.to_owned() + &counter.to_string() + ".dwm";
	println!("Writing to {}", file_name);
    
    *counter += 1;
    
    if let Ok(mut f) = File::create(file_name) {
        let mut byte = [0u8; 1];
        while let Ok(n) = stream.read(&mut byte) {
            if n == 0 {
                break;
            }
            
            f.write_all(&byte).unwrap();
        }
    }
}

/// Sends a file over a TCP stream
/// It is assumed only pcm files will be transmited and transmissions are only to the partitioner
/// server.
///
/// # Arguments
/// - `endpoint`: A string in the format `<ip>:<port>` that tells where to send the file to
/// - `file_name`: The name of the file to send
/// - `node_number`: The number of the node this process is running on.
/// It is necessary to know this due to the way the merger deals with the files
pub fn send_file(endpoint: &str, file_name: &str, node_number: u8) {
    println!("node {}", node_number as i32);
    match TcpStream::connect(endpoint) {
        Ok(mut stream) => {

            let mut file = File::open(file_name).unwrap();
            let mut buff = Vec::<u8>::new();

            file.read_to_end(&mut buff).unwrap();
            let _ = stream.write_u8(node_number);
            let _ = stream.write_u64::<LittleEndian>(file.metadata().unwrap().len());
            stream.write_all(&*buff).unwrap();

            println!("Done! {} bytes", buff.len())
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
