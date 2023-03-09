use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::Command;
use std::thread;

fn main() {
    let listener_result = TcpListener::bind("127.0.0.1:8080");

    let listener = match listener_result {
        Ok(listener) => listener,
        Err(error) => panic!("Culd not connet to server becaus of error {:?}", error),
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream).unwrap();
                });
            },
            Err(_) => panic!("sothing wrong with the stream") // error propegation
        };
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), ()>{
  
    let mut buff = [0u8;1024];

    let length = stream.read(&mut buff).unwrap();

    let http_request = String::from_utf8(buff[..length].to_vec()).unwrap();
    println!("{http_request}");

    let code = http_request.split("\r\n\r\n").nth(1).unwrap();

    let (status, header) = ("HTTP/1.1 200 OK", "Content-Type: text/plain\r\nAccess-Control-Allow-Origin: *");

    let result = run_code(code);

    let response = format!("{}\r\n{}\r\n\r\n{}",status, header, result );
    stream.write_all(response.as_bytes()).unwrap();

    Ok(())
}

fn run_code(code : &str) -> String {
    println!("kjører run code function {} dette er etter koden",code);
    let command = format!("printf {:?} > main.rs && rustc main.rs && ./main", code);

    let result = Command::new("docker")
        .arg("run")
        .arg("--rm")
        .arg("rust:latest")
        .arg("bash")
        .arg("-c")
        .arg(command)
        .stdout(std::process::Stdio::piped())
        .output()
        .expect("Error when running docker");

    let error = result.stderr.iter().map(|&x| x as char).collect::<String>();
    let output = result.stdout.iter().map(|&x| x as char).collect::<String>();

    if error.len() > 0 {
        return error;
    }

    return output;
}
