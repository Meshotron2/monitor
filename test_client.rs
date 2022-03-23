///!
/// With help from <https://gist.github.com/ThatsNoMoon/edc16ab072d470d3a7f9d996c8fc9dec>
use std::io::Write;
use std::net::TcpStream;
use std::process;

fn start() -> TcpStream {
    TcpStream::connect("127.0.0.1:49152").unwrap()
}

fn send_to_open(i: f32, mut stream: &TcpStream) {
    let mut a = [0; 13];
    fetch_message(&mut a, i);

    stream.write(&a).unwrap();
}

fn main() {
    let mut n: u64 = 99999999;
    let iter = 56;

    let stream = start();

    for i in 1..=iter {
        let mut is_prime = false;

        while !is_prime {
            n += 1;

            for j in 2..n {
                if n % j == 0 {
                    is_prime = false;
                    break;
                }

                is_prime = true;
            }

            if is_prime {
                println!("[{: >7.3}%] {}", 100.0*i as f32/iter as f32, n);
                //send(100.0*i as f32/iter as f32);
                send_to_open(100.0*i as f32/iter as f32, &stream);
            }
        }
    }
}

fn send(percent: f32) {
    match TcpStream::connect("127.0.0.1:49152") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 49152");

            let mut a = [0; 13];
            fetch_message(&mut a, percent);

            stream.write(&a).unwrap();
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn fetch_message(mut a: &mut [u8], percent: f32) {
    write!(a, "{:0>5}:{:0>7.3}", process::id(), percent).unwrap();
}
