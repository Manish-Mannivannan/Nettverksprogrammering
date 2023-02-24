use std::net::UdpSocket;
use std::io;

fn main() -> std::io::Result<()> {
    // Create a UDP socket bound to any available address
    let socket = UdpSocket::bind("0.0.0.0:8888")?;

    // Send a message to a UDP server
    let server_address = "127.0.0.1:8080";

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");
        socket.send_to(input.as_bytes(), server_address)?;
    
        // Receive a response from the server
        let mut buffer = [0; 1024];
        let (received_bytes, _src_address) = socket.recv_from(&mut buffer)?;
        let response = std::str::from_utf8(&buffer[..received_bytes]).unwrap();
        println!("Received response from server: {}", response);
    }
}