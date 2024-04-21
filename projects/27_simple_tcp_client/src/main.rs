// Network Programming in Rust,  Abhishek Chanda
// Page 66
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Failed to connect to server");
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
