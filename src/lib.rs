//! This crate allows for communication between the cluster and server and
//! collection of performance metrics.

use std::thread;

// use crate::monitor::stats::{NodeData, ProcData};
// use sysinfo::{System, SystemExt};
use crate::communication::file_transfer::start_file_server;
use crate::communication::tcp::start_server;

/// Holds all communication interfaces
///
/// - File transfer
/// - HTTP requests
/// - TCP communication
mod communication {
    pub mod file_transfer;
    pub mod http_requests;
    pub mod tcp;
}

/// The code that gathers information on processes 
mod monitor {
    pub mod stats;
}

/// Runs the program
///
/// # Arguments
///
/// - `ip`: The ip to start the servers in
/// - `cluster_port`: The port to bind the cluster server to
/// - `file_transfer_port`: The port to bint the file transfer server to
/// - `proc_name`: The process name to gather usage data on
pub fn run(
    ip: String,
    cluster_port: usize,
    file_transfer_port: usize,
    proc_name: &'static str,
) {
    // communication::http_requests::test();
    let ip1 = ip.clone();
    let node_server_handle = thread::spawn(move || start_server(ip1, cluster_port, proc_name));
    let file_server_handle =
        thread::spawn(move || start_file_server(ip, file_transfer_port, "received"));

    let _ = node_server_handle.join();
    let _ = file_server_handle.join();
}
