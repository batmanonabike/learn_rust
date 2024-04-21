// Network Programming in Rust,  Abhishek Chanda
// Page 69
use std::io::{self, BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};
use std::str;
use std::time::Duration;

fn main() {
    let timeout = Duration::from_secs(3);
    let remote: SocketAddr = "127.0.0.1:8888".parse().unwrap();
    let mut stream =
        TcpStream::connect_timeout(&remote, timeout).expect("Failed to connect to server");
    stream
        .set_read_timeout(Some(timeout))
        .expect("Failed to set read timeout");

    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        stream
            .write(input.as_bytes())
            .expect("Failed to write to server");

        let mut reader = BufReader::new(&stream);
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Failed to read into buffer");

        let utf8: &str = str::from_utf8(&buffer).expect("Failed to write buffer as string");
        print!("{}", utf8);
    }
}
