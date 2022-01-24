// use crate::monitor::stats::{NodeData, ProcData};
// use sysinfo::{System, SystemExt};
use crate::communication::tcp::start_server;

mod communication {
    pub mod http_requests;
    pub mod tcp;
}

mod monitor {
    pub mod stats;
}

pub fn run(ip: &str, port: usize, proc_name: &str) {
    // communication::http_requests::test();
    start_server(ip, port, proc_name);
}
