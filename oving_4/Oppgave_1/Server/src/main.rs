use std::net::UdpSocket;

fn handle_client(socket: &UdpSocket){
    let mut buffer = [0; 1024];

    let (message, addr) = socket.recv_from(&mut buffer).unwrap();

    let line = std::str::from_utf8(&buffer[0..message]);

    match line{
        Ok(line) => {
            let calculation = meval::eval_str(line);
            match calculation {
                Ok(calculation) => {
                    socket.send_to(format!("Result: {}", calculation).as_bytes(), addr).unwrap();
                }
                Err(_) => {
                    socket.send_to(b"Eval failed", addr).unwrap();
                }
            }
        }
        Err(_) => {
            socket.send_to(b"Your input was not evaluated", addr).unwrap();
        }
    }
}

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8080").expect("could not bind");

    loop{
        handle_client(&socket);
    }
}

