use std::{
    fs,
    str,
    path::Path,
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

    let file_name =  "index.html";
    let status_line = "HTTP/1.1 200 OK";
    let contents: String = if Path::new(file_name).exsts() {
        load_contents(file_name)
    }else{
        "err404 file not found".to_string()
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap()
}

fn load_contents(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap()
}