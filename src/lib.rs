use sysinfo::{System, SystemExt};
use crate::communication::tcp::start_server;
use crate::monitor::stats::{NodeData, ProcData};

mod communication {
    pub mod tcp;
}

mod monitor {
    pub mod stats;
}

pub fn run(ip: &str, port: usize, proc_name: &str) {
    start_server(ip, port, proc_name);
}