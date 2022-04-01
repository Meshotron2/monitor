use std::{net::{TcpListener, TcpStream}, thread, fs::File, io::{Read, Write}};

pub fn start_file_server(ip: String, port: usize, file_name: &'static str) {
    let listener = TcpListener::bind(ip.to_owned() + ":" + &*port.to_string()).unwrap();

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				thread::spawn(move || receive_file(stream, file_name));		
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
fn receive_file(mut stream: TcpStream, file_name: &str) {
	if let Ok(mut f) = File::create(file_name) {
		let mut byte = [0u8; 1];
		while let Ok(_) = stream.read(&mut byte) {
			f.write(&byte).unwrap();
		}
	}
}
