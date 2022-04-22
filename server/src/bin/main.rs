use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
    println!("shuting down!!!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let http_version = "HTTP/1.1";
    let (filename, status) = if buffer.starts_with(get) {
        ("../index.html", "200 OK")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("../index.html", "200 OK")
    } else {
        ("../404.html", "404 NOT FOUND")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{} {}\r\nContent-Length: {}\r\n\r\n{}",
        http_version,
        status,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
