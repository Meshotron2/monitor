use crate::communication::http_requests::RequestSerializable;
use crate::monitor::stats::{NodeData, ProcData};
/// With help from <https://gist.github.com/ThatsNoMoon/edc16ab072d470d3a7f9d996c8fc9dec>
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::convert::TryInto;
use sysinfo::{System, SystemExt};

/// Starts the TCP server that communicates usage and progress data to the server
///
/// # Arguments
///
/// - `ip`: The ip to start the server on
/// - `port`: The port to bind the server to
/// - `proc_name`: The name of the processes to gather usage data on
///
/// # Acknowledgements
/// Based on <https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo>
pub fn start_server(ip: String, port: usize, proc_name: &str) {
    let mut sys = System::new_all();
    let node = NodeData::new();

    let procs = Arc::new(Mutex::new(ProcData::fetch_all(
        proc_name,
        node.get_id(),
        &mut sys,
    )));
    let sys = Arc::new(Mutex::new(sys));
    let node = Arc::new(Mutex::new(node));

    let listener = TcpListener::bind(ip.to_owned() + ":" + &*port.to_string()).unwrap();
    //listener.set_nonblocking(true).expect("Cannot be blocking");
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

                    handle_client(stream, &mut *ph, &mut *nh, &mut *sh);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    // drop(listener);
}

/// Handles a client connection
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
) {
    // let mut data = [0; 5 + 1 + 7 + 1]; // using 50 byte buffer
    let mut data = [0; 6 * 4]; // PID: i32, percentage: f32, send_t, recv_t, delay_t, scatter_t
    // let node_url = String::from("http://127.0.0.1:8080/monitor/node");
    // let proc_url = String::from("http://127.0.0.1:8080/monitor/proc");
    let server_addr = String::from("127.0.0.1:8888");

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
                println!("Something");
                println!("Post processing: {pid} @ {progress}% (send {send_t}, recv {recv_t}, delay {delay_t}, scatter {scatter_t})");

                // node.update(sys);
                // send_update(node, &server_addr);
                if let Some(p) = procs.get_mut(&pid) {
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

fn read_i32(buff: &[u8]) -> i32 {
    i32::from_le_bytes(buff[..4].try_into().unwrap())
}

fn read_u32(buff: &[u8]) -> u32 {
    u32::from_le_bytes(buff[..4].try_into().unwrap())
}

fn read_f32(buff: &[u8]) -> f32 {
    f32::from_bits(read_u32(buff))
}

/// Validates the input and fetches the PID and progress from it.
///
/// # Parameters
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

    // let input = std::str::from_utf8(input).ok().unwrap();
    // println!("RAW: {}", input);

    // let (str_pid, str_progress) = input.split_once(':').unwrap();

    // println!("PID: {} @ {}%", str_pid, str_progress.trim());

    // return (
    //     str_pid.parse().unwrap_or(0),
    //     str_progress.trim().parse::<f32>().unwrap_or(200.0),
    // );
}

/// Sends the data to the server
///
/// # Parameters
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
/// # Parameters
///
/// - `a`: The byte array to write to
/// - `data`: The string to convert into bytes
fn fetch_message(mut a: &mut [u8], data: &str) {
    write!(a, "{}", data).unwrap();
}
