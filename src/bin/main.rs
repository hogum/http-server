//! Listens to incoming tcp streams
//!
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::thread;
use std::time::Duration;

use server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7676").unwrap();
    let pool = ThreadPool::new(0).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            process_connection(stream);
        });
    }
}

///Reads data from the TCP stream
///
///# Remarks
///Shows sent data
fn process_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let get_response = b"GET / HTTP/1.1";
    let res = b"GET /favicon.ico HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1";

    stream.read(&mut buffer).unwrap();

    let (status_line, filename) = if buffer.starts_with(get_response) || buffer.starts_with(res) {
        ("HTTP/1.1 200 OK\r\n\r\n", "src/temp/entry.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "src/temp/entry.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "src/temp/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // lossy: Replace invalid sequences with ?
}
