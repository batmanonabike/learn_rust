// Network Programming in Rust,  Abhishek Chanda
// Page 68
extern crate rand;

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        };

        let mut rng: ThreadRng = thread_rng();
        let secs_array: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
        let secs: u32 = *secs_array.choose(&mut rng).unwrap();
        let sleep = Duration::from_secs(secs as u64);

        println!("Sleeping for {:?}", sleep);
        std::thread::sleep(sleep);
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Failed to bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("Failed: {}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
