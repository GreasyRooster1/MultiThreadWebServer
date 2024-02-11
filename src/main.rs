mod actions;
mod files;
mod paths;
mod uri;

use crate::actions::{get_registry, Action};
use std::ptr::copy;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use MultiThreadWebServer::ThreadPool;

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

    let response = if actions::check_action(uri) {
        println!("executing action with identifier {uri}");
        get_registry()
            .iter()
            .find(|&x| x.identifier() == uri)
            .unwrap()
            .func()(lines)
    } else {
        println!("no action was specified, executing default");
        actions::default_action().func()(lines)
    };

    stream.write_all(response.as_bytes()).unwrap();
    println!("stream was written to, connection handel completed");
}
