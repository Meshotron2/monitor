/// With help from <https://gist.github.com/ThatsNoMoon/edc16ab072d470d3a7f9d996c8fc9dec>

use std::collections::HashMap;
use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use sysinfo::{System, SystemExt};
use proc_macro::bridge::PanicMessage::String;
use crate::monitor::stats::{NodeData, ProcData};
use crate::communication::http_requests::send;

/// Starts the TCP server
///
/// # Acknowledgements
/// Based on <https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo>
pub fn start_server(ip: &str, port: usize, proc_name: &str) {
    let mut sys = System::new_all();

    let procs = Arc::new(Mutex::new(ProcData::new(proc_name, &mut sys)));
    let sys = Arc::new(Mutex::new(sys));
    let node = Arc::new(Mutex::new(NodeData::new()));

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
/// The messages must be on the format `<pid>:<progress>`.
///
/// # Acknowledgements
/// Based on <https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo>
fn handle_client(mut stream: TcpStream, procs: &mut HashMap<i32, ProcData>,
                 node: &mut NodeData, sys: &mut System) {
    let mut data = [0; 9]; // using 50 byte buffer
    let url = String::from("127.0.0.1:500");

    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                node.update(sys);
                send(node, &url);

                let (pid, progress) = process_input(&data[0..size]);

                match procs.get_mut(&pid) {
                    Some(p) => {
                        p.update(progress, &mut *sys);
                        send(p, &url)
                    }
                    None => {}
                }

                // stream.write(&data).unwrap();
                // true
            }
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                // false
                break;
            }
        }
    }
}

/// Validates the input and fetches the PID and progress from it.
///
/// input must be of type `<pid>:<progress>`,
/// where `pid` is a valid process id in `i32` and `progress` is `0 <= progress <= 100` `usize`
/// These values have to be padded so `pid` takes **5** characters and `progress` takes **3**,
/// for a total  of **9** bytes
///
/// If `pid` does not match, `-1` is returned
/// If `progress` does not match, `200` is returned
///
fn process_input(input: &[u8]) -> (i32, usize) {
    let input = std::str::from_utf8(input).ok().unwrap();
    // println!("RAW: {}", input);

    let (str_pid, str_progress) = input.split_once(':').unwrap();

    println!("PID: {} @ {}%", str_pid, str_progress);

    return (str_pid.parse().unwrap_or(0), str_progress.parse().unwrap_or(200));
}