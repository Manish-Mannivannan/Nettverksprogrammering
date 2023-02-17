use oving3::ThreadPool;
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {

    let buff_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buff_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1";
    println!("g = {:?}", get);
    let sleep = b"GET /sleep HTTP/1.1";
    
    let request = http_request.first().unwrap().as_bytes();
    println!("r = {:?}", request);

    let (status_line, filename) = if request == get{
        ("HTTP/1.1 200 OK", "hello.html")

    } else if request == sleep {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")

    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut headers = String::new();

    for request in http_request {
        let line = format!("<li>{}</li>\n", request);
        headers.push_str(&line);
    }

    let contents = fs::read_to_string(filename).unwrap();
    let contents = contents.replace("{{ content }}", headers.as_str());

    let response = format!(
        
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    
        
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
