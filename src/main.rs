use rust_web_server::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufRead, BufReader},
    net::{self, TcpStream},
    thread::sleep,
    time::Duration,
};

fn main() {
    let listener =
        net::TcpListener::bind("127.0.0.1:3000").expect("Could not listen to provided address");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        pool.execute(|| match stream {
            Ok(s) => handle_connection(s),
            Err(e) => println!("invalid stream, {:?}", e),
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT_FOUND", "404.html"),
    };
    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
