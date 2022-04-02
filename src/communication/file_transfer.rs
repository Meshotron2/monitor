use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

pub fn start_file_server(ip: String, port: usize, file_name: &'static str) {
    let listener = TcpListener::bind(ip.to_owned() + ":" + &*port.to_string()).unwrap();

    let cnt = Arc::new(Mutex::new(0));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let cnt_handle = Arc::clone(&cnt);

                thread::spawn(move || {
                    receive_file(stream, file_name, &mut cnt_handle.lock().unwrap())
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}

///
/// with the help from [SO](https://stackoverflow.com/questions/53826371/how-to-create-a-binary-file-with-rust)
fn receive_file(mut stream: TcpStream, file_name: &str, counter: &mut i32) {
    let file_name = file_name.to_owned() + &counter.to_string() + ".dwm";
    *counter += 1;
    if let Ok(mut f) = File::create(file_name) {
        let mut byte = [0u8; 1];
        while let Ok(_) = stream.read(&mut byte) {
            f.write(&byte).unwrap();
        }
    }
}
