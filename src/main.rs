use std::{
    fs,
    io::{prelude::*, BufRead, BufReader},
    net::{self, TcpStream},
};

fn main() {
    let listener =
        net::TcpListener::bind("127.0.0.1:3000").expect("Could not listen to provided address");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_connection(s),
            Err(e) => println!("invalid stream, {:?}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
