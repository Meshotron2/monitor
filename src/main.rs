use std::str::FromStr;
use std::usize;

pub struct Config {
    ip: String,
    cluster_port: usize,
    file_transfer_port: usize,
    proc_name: String,
    server_addr: String,
    pcm_endpoint: String,
}

impl Config {
    fn new(
        ip: String,
        cluster_port: usize,
        file_transfer_port: usize,
        proc_name: String,
        server_addr: String,
        pcm_endpoint: String,
    ) -> Self {
        // TODO: Add checks

        Config {
            ip,
            cluster_port,       // 49152
            file_transfer_port, // 49153
            proc_name,          // test_client
            server_addr,        // 127.0.0.1:8888
            pcm_endpoint,       // 127.0.0.1:5000
        }
    }
}

fn main() {
    // ports have 16 bits, range from 0 to 65535
    // 0-1023 – Well known ports
    // 1024-49151 - Registered Port
    // 49152-65535 - free to use

    let args: Vec<String> = std::env::args().collect();

    let ip = args[1].clone();
    let cluster_port = usize::from_str(args[2].as_str()).unwrap();
    let file_transfer_port = usize::from_str(args[3].as_str()).unwrap();
    let proc_name = args[4].clone();
    let server_addr = args[5].clone();
    let pcm_endpoint = args[6].clone();

    let cfg = Config::new(
        ip,
        cluster_port,
        file_transfer_port,
        proc_name,
        server_addr,
        pcm_endpoint,
    );

    monitor::run(
        cfg.ip,
        cfg.cluster_port,
        cfg.file_transfer_port,
        cfg.proc_name,
        cfg.server_addr,
        cfg.pcm_endpoint,
    );
}
