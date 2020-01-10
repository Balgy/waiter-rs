 //https://doc.rust-lang.org/book/title-page.html

mod server_threading;

use std::io::prelude::*;
use std::net::TcpListener;
use std::fs;
use chrono::{DateTime, Utc};
use server_threading::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("Connection attempt failed with error: {}", e);
                continue;
            }
        };

        thread_pool.execute(|| {
            if let Err(err) = handle_connection(stream) {
                eprintln!("Handling connection failed with error: {}", err);
            }
        });
    }
}

fn handle_connection(mut stream: impl Read + Write) -> Result<(), std::io::Error> {
    let mut buffer = [0; 512];
    let _amount = stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";
    
    let now: DateTime<Utc> = Utc::now();

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("Views/hello.html")?;
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nDate: {}\r\n\r\n{}", contents.len(),
        now.to_rfc2822(), 
        contents);
        
        stream.write_all(response.as_bytes()).and_then(|_| stream.flush())
    }
    else {
        let contents = fs::read_to_string("Views/notFound.html")?;
        let response = format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nDate: {}\r\n\r\n{}", contents.len(),
        now.to_rfc2822(), 
        contents);
        
        stream.write_all(response.as_bytes()).and_then(|_| stream.flush())
    }
}
