use std::{
    fs,
    str,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handel_connection(stream);
    }
}

fn handel_connection(mut stream:TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line|!line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = load_contents("index.html");
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap()
}

fn load_contents(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap()
}