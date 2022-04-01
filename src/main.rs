fn main() {
    // ports have 16 bits, range from 0 to 65535
    // 0-1023 – Well known ports
    // 1024-49151 - Registered Port
    // 49152-65535 - free to use

    let args: Vec<String> = std::env::args().collect();

    let ip = args[1].clone();

    monitor::run(ip, 49152, 49153, "test_client");
}
