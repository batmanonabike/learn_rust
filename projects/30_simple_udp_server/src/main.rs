// Network Programming in Rust,  Abhishek Chanda
// Page 71
use std::net::UdpSocket;
use std::thread;

// nc -u 127.0.0.1 8888
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Failed to bind to socket");

    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socket");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("Handling connection from {}", src);
                    sock.send_to(&buf, &src).expect("Failed to send a response");
                });
            }
            Err(e) => {
                eprintln!("Failed to receive datagram: {}", e);
            }
        }
    }
}
