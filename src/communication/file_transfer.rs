//! Holds methods to transfer and receive files.
//! File reception is handled through a TCP server

use byteorder::{BigEndian, WriteBytesExt};
use std::{
    fs,
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

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
/// It is assumed only pcm files will be transmitted and transmissions are only to the partitioner
/// server.
///
/// # Arguments
/// - `endpoint`: A string in the format `<ip>:<port>` that tells where to send the file to
/// - `file_name`: The name of the file to send
/// - `node_number`: The number of the node this process is running on.
/// It is necessary to know this due to the way the merger deals with the files
// pub fn send_file(endpoint: &str, file_name: &str, node_number: u8) {
//     println!("node {}", node_number as i32);
//     match TcpStream::connect(endpoint) {
//         Ok(mut stream) => {
//             let mut file = File::open(file_name).unwrap();
//             let mut buff = Vec::<u8>::new();
//
//             file.read_to_end(&mut buff).unwrap();
//             let _ = stream.write_u8(node_number);
//             let _ = stream.write_u64::<LittleEndian>(file.metadata().unwrap().len());
//             stream.write_all(&*buff).unwrap();
//
//             println!("Done! {} bytes", buff.len())
//         }
//         Err(e) => {
//             println!("Failed to connect: {}", e);
//         }
//     }
// }

/// Sends all the pcm files over a TCP connection to the room partitioner.
/// It is assumed only pcm files will be transmitted and transmissions are only to the partitioner
/// server.
///
/// # Arguments
/// - `endpoint`: A string in the format `<ip>:<port>` that tells where to send the file to
/// - `node_number`: The number of the node this process is running on.
/// It is necessary to know this due to the way the merger deals with the files
pub fn send_all_pcm(endpoint: &String, node_number: u8) {
    let mut files: Vec<String> = fs::read_dir("./")
        .unwrap()
        .filter(|dir_entry| {
            dir_entry.as_ref().unwrap().file_type().unwrap().is_file()
                && dir_entry
                    .as_ref()
                    .unwrap()
                    .file_name()
                    .into_string()
                    .unwrap()
                    .ends_with(".pcm")
        })
        .map(|dir_entry| dir_entry.unwrap().file_name().into_string().unwrap())
        .collect();

    files.sort();

    let n_files: u32 = files.len() as u32;

    let f_size = if let Some(f) = files.get(0) {
        fs::File::open(f).unwrap().metadata().unwrap().len() as u32
    } else {
        0
    };

    println!("Sending {} files of {} bytes", n_files, f_size);

    match TcpStream::connect(endpoint) {
        Ok(mut stream) => {
            let _ = stream.write_u8(node_number);
            let _ = stream.write_u32::<BigEndian>(n_files);
            let _ = stream.write_u32::<BigEndian>(f_size);
            for f in files {
                let mut buff = Vec::<u8>::new();

                File::open(f).unwrap().read_to_end(&mut buff).unwrap();

                stream.write_all(&*buff).unwrap();

                println!("Done! {} bytes", buff.len())
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
