fn main() {
    // ports have 16 bits, range from 0 to 65535
    // 0-1023 – Well known ports
    // 1024-49151 - Registered Port
    // 49152-65535 - free to use
    monitor::run("127.0.0.1", 49152, 49153, "test_client");
}
