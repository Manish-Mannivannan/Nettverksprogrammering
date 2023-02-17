use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};
use meval;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => { eprintln!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {

        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {return Ok(())}

       
        let utf8_res = match std::str::from_utf8(&buf) {
            Ok(v) => v.split("\n").nth(0).unwrap().to_string(),
            Err(v) => v.to_string(),
        };

        if let Ok(result) = meval::eval_str(&utf8_res) {
            println!("{}  = {}", utf8_res, result);
            
            stream.write(format!("{} = {}\n", utf8_res, result).as_bytes())?;
        } else {
            stream.write(format!("'{}' is not a valid expression\n", utf8_res).as_bytes())?;
        }
                  
    }
}