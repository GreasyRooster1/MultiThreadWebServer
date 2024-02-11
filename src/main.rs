mod actions;
mod files;
mod paths;
mod uri;

use std::ptr::copy;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use MultiThreadWebServer::ThreadPool;
use crate::actions::load_page;
use crate::files::load_contents;
use crate::paths::DEFAULT_PATH;
use crate::uri::{find, parse};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handel_connection(stream);
        });
    }
    println!("Shutting down!");
}

fn handel_connection(mut stream: TcpStream) {
    //get the BufReader from TcpStream
    let buf_reader = BufReader::new(&mut stream);
    let lines: Vec<_> = buf_reader.lines().collect::<Result<_, _>>().unwrap();

    let request_line =lines[0].clone();

    let uri = uri::extract(request_line.as_str());

    println!("received request from asking for uri {uri}");

    //let mut req = request;

    let request_line = lines.iter().nth(0).unwrap();
    println!("req {request_line}");
    let uri = uri::extract(request_line.as_str());

    let filename = if uri.eq("/") {
        find(DEFAULT_PATH)
    } else {
        parse(find(uri).as_str())
    };
    let (contents, status_line) = if files::file_exists(filename.as_str()) {
        (load_contents(filename.as_str()), "HTTP/1.1 200 OK")
    } else {
        (
            load_contents(&paths::NOT_FOUND_PATH),
            "HTTP/1.1 404 NOT FOUND",
        )
    };
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    println!("stream was written to, connection handel completed");
}
