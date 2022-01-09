use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex}; //, MutexGuard};
use std::thread;
use sysinfo::{System, SystemExt};
use crate::monitor::stats::{NodeData, ProcData};
// use crate::NodeData; // TODO: make this import the unix one

/// Starts the TCP server
///
/// # Acknowledgements
/// Based on https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo
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

                    handle_client(stream, &mut *ph,&mut *nh, &mut *sh);
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
/// # Protocol
/// The messages must be on the format `<pid>:<progress>`.
///
/// # Acknowledgements
/// Based on https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo
fn handle_client(mut stream: TcpStream, procs: &mut HashMap<i32, ProcData>,
                 node: &mut NodeData, sys: &mut System) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(_size) => {
            // echo everything!
            // stream.write(&data[0..size]).unwrap();

            node.update(sys);

            let (pid, progress) = process_input(&data[0.._size]);

            match procs.get_mut(&pid) {
                Some(p) => { p.update(progress, &mut *sys); }
                None => {}
            }

            stream.write(b"I got your message!\n").unwrap();
            true
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

/// Validates the input and fetches the PID and progress from it.
///
/// input must be of type `<pid>:<progress>`,
/// where `pid` is a valid process id in `i32` and `progress` is `0 <= progress <= 100` `usize`
///
/// If `pid` does not match, `-1` is returned
/// If `progress` does not match, `200` is returned
///
fn process_input(input: &[u8]) -> (i32, usize) {
    let mut str_pid = String::new();
    let mut str_progress = String::new();

    let mut is_pid = 1;
    for c in input {
        if is_pid == 1 {
            if *c == ':' as u8 {
                is_pid = 0;
                continue;
            }

            str_pid += &*String::from(*c as char);
            continue;
        }

        str_progress += &*String::from(*c as char);
    }

    println!("PID: {} @ {}%", str_pid, str_progress);

    return (str_pid.parse().unwrap_or(-1), str_progress.parse().unwrap_or(200));
}