extern crate webserver;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;
use std::thread;
use std::time::Duration;
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // 스레드를 무한정 생성하는 것은 시스템의 과부하 유의.
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down...");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // Ruquest Packet 첫번째 라인을 잘라서 Log로 활용.
    let buffer_string = String::from_utf8_lossy(&buffer[..]);
    let mut buffer_iter = buffer_string.split("\r\n");
    if let Some(first_line) = buffer_iter.next() {
        println!("{}", first_line);
    }

    let (status_line, filename) = if buffer.starts_with(b"GET / ") {
        ("HTTP/1.1 200 OK", "index.html")
    }  else if buffer.starts_with(b"GET /slow ") {
        thread::sleep(Duration::from_secs(1));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}