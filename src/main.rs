mod files;
mod paths;
mod uri;
mod actions;
mod cryptography;

use std::{io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread};
use std::time::Duration;
use log::{error, warn};
use rsa::pkcs1::{EncodeRsaPrivateKey, LineEnding};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use MultiThreadWebServer::ThreadPool;
use crate::actions::special_cases;
use crate::files::load_contents;
use crate::paths::DEFAULT_PATH;
use crate::uri::*;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8081").unwrap();
    let pool = ThreadPool::new(15);

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
    let buf_reader_next_line = buf_reader.lines().next();
    let request_line_result = match buf_reader_next_line {
        None=> {
            warn!("received empty packet");
            return;
        },
        _=>buf_reader_next_line.unwrap(),
    };
    let request_line = match request_line_result {
        Ok(_) => request_line_result.unwrap(),
        Err(_) => {
            warn!("Error caused on reading from buffer");
            "GET /404 HTTP/1.1".to_string()
        }
    };

    let uri = uri::extract(request_line.as_str());
    let client_addr = stream.local_addr().unwrap().ip().to_string();

    println!("received request from {client_addr} asking for uri {uri}");

    let response =special_cases(uri);

    stream.write_all(response.as_slice()).unwrap();

    println!("responded to request!");
}



fn load_contents_from_uri(uri:&str) -> Vec<u8> {
    let filename = if uri.eq("/") {
        find(DEFAULT_PATH)
    }else{
        parse(find(uri).as_str())
    };
    let (contents,status) = if files::file_exists(filename.as_str()) {
        (load_contents(filename.as_str()),"200 OK")
    }else{
        (load_contents(paths::NOT_FOUND_PATH),"NOT FOUND")
    };
    HTTPResponse::from_bytes(status.to_string(),contents.as_slice())
}