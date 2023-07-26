use ferriscrab::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let request_line = http_request.first().unwrap();

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "examples/resources/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "examples/resources/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "examples/resources/404.html"),
    };

    let body = fs::read_to_string(filename).unwrap();
    let header = format!("Content-Length: {}", body.len());

    let response = format!("{status_line}\r\n{header}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).unwrap();
}
