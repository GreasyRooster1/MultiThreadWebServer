mod files;
mod uri;
mod paths;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use MultiThreadWebServer::ThreadPool;
use crate::paths::DEFAULT_PATH;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(|| {
            handel_connection(stream);
        });
    }
    println!("Shutting down!");
}

fn handel_connection(mut stream:TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let uri = uri::extract(request_line.as_str());
    let client_addr = stream.local_addr().unwrap().ip().to_string();

    println!("received request from {client_addr} asking for uri {uri}");

    let filename = if uri.eq("/") {
        uri::find(DEFAULT_PATH)
    }else{
        uri::parse(uri::find(uri).as_str())
    };

    let (contents,status_line) = if files::file_exists(filename.as_str()) {
        (files::load_contents(filename.as_str()),"HTTP/1.1 200 OK")
    }else{
        (files::load_contents(&paths::NOT_FOUND_PATH),"HTTP/1.1 404 NOT FOUND")
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    println!("responded with status line {status_line}, at length {length}, with content from {filename}");
}

