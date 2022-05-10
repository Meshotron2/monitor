//! Handles the lowest lever of TCP communications of the monitor.
//! Provides methods to start a TCP server and to send data over TCP.
//!
//! Note that this module is not very generic, so if new functionality is needed it will probably
//! require new methods.
//!
//! With help from [ThatsNoMoon](https://gist.github.com/ThatsNoMoon/edc16ab072d470d3a7f9d996c8fc9dec)

use crate::communication::file_transfer::{send_all_pcm, send_file};
use crate::communication::http_requests::RequestSerializable;
use crate::monitor::stats::{NodeData, ProcData};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use sysinfo::{System, SystemExt};

/// Starts the TCP server that communicates usage and progress data to the server
///
/// # Arguments
///
/// - `ip`: The ip to start the server on
/// - `port`: The port to bind the server to
/// - `proc_name`: The name of the processes to gather usage data on
/// - `server_addr`: The address of the room partitioner server
///
/// # Acknowledgements
/// Based on <https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo>
pub fn start_server(ip: String, port: usize, proc_name: String, server_addr: String) {
    let mut sys = System::new_all();
    let node = NodeData::new();

    let procs = Arc::new(Mutex::new(ProcData::fetch_all(
        &proc_name,
        node.get_id(),
        &mut sys,
    )));
    let sys = Arc::new(Mutex::new(sys));
    let node = Arc::new(Mutex::new(node));

    let listener = TcpListener::bind(ip.to_owned() + ":" + &*port.to_string()).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                let procs_handle = Arc::clone(&procs);
                let node_handle = Arc::clone(&node);
                let sys_handle = Arc::clone(&sys);

                thread::spawn(move || {
                    // connection succeeded
                    println!("Handling stuff");

                    let mut ph = procs_handle.lock().unwrap();
                    let mut nh = node_handle.lock().unwrap();
                    let mut sh = sys_handle.lock().unwrap();

                    handle_client(stream, &mut *ph, &mut *nh, &mut *sh, server_addr.clone());
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

/// Handles a client connection
///
/// # Arguments
/// -`stream`: The client's TCP stream
/// -`procs`: The processes' object's list
/// -`node`: The node's object
/// -`sys`: [Sys] instance to fetch process data from
/// - `server_addr`: The address of the room partitioner server
///
/// # Protocol
/// To see details on the protocol refer to [process_input]
///
/// # Acknowledgements
/// Based on <https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo>
fn handle_client(
    mut stream: TcpStream,
    procs: &mut HashMap<i32, ProcData>,
    node: &mut NodeData,
    sys: &mut System,
    server_addr: String,
) {
    // let mut data = [0; 5 + 1 + 7 + 1]; // using 50 byte buffer
    let mut data = [0; 6 * 4]; // PID: i32, percentage: f32, send_t, recv_t, delay_t, scatter_t
                               // let server_addr = String::from("127.0.0.1:8888");

    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                node.update(sys);
                send_update(node, &server_addr);
                println!("{:?}", node);

                println!("Size: {size}");
                let (pid, progress, send_t, recv_t, delay_t, scatter_t) =
                    process_input(&data[0..size]);
                println!("Post processing: {pid} @ {progress}% (send {send_t}, recv {recv_t}, delay {delay_t}, scatter {scatter_t})");
                node.set_id(pid as u8);

                if progress == -1.0 {
                    // signals the end of the transmission

                    let mut i = 1;
                    let mut name = format!("receiver_{}.pcm", i);

                    send_all_pcm("127.0.0.1:5000", node.get_id());

                    // while File::open(&name).is_ok() {
                    //     println!("Found {}", name);
                    //     send_file("127.0.0.1:5000", &name, node.get_id());
                    //
                    //     name = format!("receiver_{}.pcm", i);
                    //     i += 1;
                    // }
                } else if let Some(p) = procs.get_mut(&pid) {
                    // the process is valid

                    p.update(progress, send_t, recv_t, delay_t, scatter_t, &mut *sys);

                    println!("SEND: {}", &p.serialize());

                    send_update(p, &server_addr);
                } else {
                    let p = ProcData::new(pid, node.get_id(), sys);

                    println!("SEND: {}", &p.serialize());

                    send_update(&p, &server_addr);
                    procs.insert(pid, p);
                }

                // stream.write(&data).unwrap();
                // true
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                // false
                break;
            }
        }
    }
}

/// Converts a byte array into an i32
fn read_i32(buff: &[u8]) -> i32 {
    i32::from_le_bytes(buff[..4].try_into().unwrap())
}

/// Converts a byte array into an u32
fn read_u32(buff: &[u8]) -> u32 {
    u32::from_le_bytes(buff[..4].try_into().unwrap())
}

/// Converts a byte array into an f32
fn read_f32(buff: &[u8]) -> f32 {
    f32::from_bits(read_u32(buff))
}

/// Validates the bytes received from the stream and returns all data in the correct data types.
///
/// # Arguments
///
/// - `input`: the bytes received from the server
///
/// # Protocol
/// Messages should be a stream of bytes with the following elements, in this specific order:
///
/// 1. pid: i32, the communicating process' id
/// 1. progress: f32, the percentage of progress towards the end. Should be `0 <= progress <= 100`
/// 1. send time: f32, the time it took to send the scatter pass data to the neighbor nodes
/// 1. receive time: f32, the time it took to receive the required data from the neighbor nodes after the scatter pass
/// 1. delay time: f32, the time the delay pass took
/// 1. scatter time: f32, the time the scatter pass took
fn process_input(input: &[u8]) -> (i32, f32, f32, f32, f32, f32) {
    (
        read_i32(&input[0..4]),
        read_f32(&input[4..8]),
        read_f32(&input[8..12]),
        read_f32(&input[12..16]),
        read_f32(&input[16..20]),
        read_f32(&input[20..24]),
    )
}

/// Sends the data to the server
///
/// # Arguments
///
/// - `request`: The `RequestSerializable` to be sent to the server
/// - `endpoint`: A string in the form `<ip>:<port>` that contains the ip and port to send the data to
fn send_update(request: &dyn RequestSerializable, endpoint: &str) {
    match TcpStream::connect(endpoint) {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 49152");

            let mut a = [0; 256];
            fetch_message(&mut a, &request.serialize());

            stream.write_all(&a).unwrap();
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

/// Converts a string into a byte array
///
/// # Arguments
///
/// - `a`: The byte array to write to
/// - `data`: The string to convert into bytes
fn fetch_message(mut a: &mut [u8], data: &str) {
    write!(a, "{}", data).unwrap();
}
