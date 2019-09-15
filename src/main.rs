//! Listens to incoming tcp streams
//!
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7676").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        process_connection(stream);
    }
}

///Reads data from the TCP stream
///
///# Remarks
///Shows sent data
fn process_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let contents = fs::read_to_string("src/temp/entry.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    let get_response = b"GET / HTTP/1.1";
    let res = b"GET /favicon.ico HTTP/1.1";

    stream.read(&mut buffer).unwrap();

    if buffer.starts_with(get_response) || buffer.starts_with(res) {
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // lossy: Replace invalid sequences with ?
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("src/temp/404.html").unwrap();

        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
