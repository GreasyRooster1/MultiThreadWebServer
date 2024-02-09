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
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line,filename) = if request_line=="GET / HTTP/1.1"{
        ("HTTP/1.1 200 OK","index.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };

    let contents = check_error404(filename);
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap()
}

fn check_error404(filename: &str) -> String{
    if Path::new(filename).exists() {
        load_contents(filename)
    } else {
        load_contents("404.html")
    }
}

fn load_contents(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap()
}