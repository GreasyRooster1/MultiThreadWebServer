mod files;
mod paths;
mod uri;
mod actions;
mod cryptography;
mod console;
mod threadlib;
mod logging;

use std::{io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread};
use std::time::Duration;
use log::{error, warn};
use rsa::pkcs1::{EncodeRsaPrivateKey, LineEnding};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use threadlib::ThreadPool;
use logging::*;
use crate::actions::special_cases;
use crate::console::start_async_input;
use crate::files::load_contents;
use crate::paths::DEFAULT_PATH;
use crate::uri::*;

#[tokio::main]
async fn main() {
    log_debug("debugging...","main");
    log_info("wow this is a log!","main");
    log_warn("im warning you...","main");
    log_error("uh oh!","main");
    log_critical("some thing really bad happened","main");
    let listener = TcpListener::bind("0.0.0.0:8081").unwrap();
    let pool = ThreadPool::new(15);

    let _async_input = start_async_input();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handel_connection(stream);
        });
    }
    println!("Main process ended");
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



