//! The monitor program is the bridge between every DWM instance in a node and the partitioner
//! server.
//!
//! It receives the progress and telemetry data from the DWM processes,
//! determines their system usage and sends it to the server.
//! Also deals with file transfer of the partitions from the server to the node and the excitation
//! sound files from the node to the server.

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
pub fn run(ip: String, cluster_port: usize, file_transfer_port: usize, proc_name: String) {
    // communication::http_requests::test();
    let ip1 = ip.clone();
    let node_server_handle = thread::spawn(move || start_server(ip1, cluster_port, proc_name));
    let file_server_handle =
        thread::spawn(move || start_file_server(ip, file_transfer_port, "received"));

    let _ = node_server_handle.join();
    let _ = file_server_handle.join();
}
