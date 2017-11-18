// #[derive(Debug)]
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let binding = "127.0.0.1:8080";
    let listener = TcpListener::bind(binding).unwrap();

    println!("TcpListener bound: {}", binding);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection received!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    println!("reading stream...");
    stream.read(&mut buffer).unwrap();
    println!("REQ:\n{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let mut file = File::open("./html/hello.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK \r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open("./html/404.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

}