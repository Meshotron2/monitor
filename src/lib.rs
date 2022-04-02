use std::thread;

// use crate::monitor::stats::{NodeData, ProcData};
// use sysinfo::{System, SystemExt};
use crate::communication::file_transfer::start_file_server;
use crate::communication::tcp::start_server;

mod communication {
    pub mod file_transfer;
    pub mod http_requests;
    pub mod tcp;
}

mod monitor {
    pub mod stats;
}

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
