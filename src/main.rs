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

pub const THREAD_POOL_SIZE:usize = 32;

#[tokio::main]
async fn main() {
    log_info("starting...","main");

    let listener = TcpListener::bind("0.0.0.0:8081").unwrap();
    let pool = ThreadPool::new(THREAD_POOL_SIZE);

    log_info(format!("create thread pool with {THREAD_POOL_SIZE} threads").as_str(),"main");

    let _async_input = start_async_input();
    log_info("console commands are now available","main");

    log_info("begin TCP listen","main");

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(_) => {
                stream.unwrap()
            }
            Err(_) => {
                log_warn("error occurred when unwrapping stream","main");
                continue;
            }
        };
        pool.execute(|| {
            handel_connection(stream);
        });
    }
    log_warn("Stopped listening for connections, quitting","main");
}

fn handel_connection(mut stream: TcpStream) {
    //get the BufReader from TcpStream
    let buf_reader = BufReader::new(&mut stream);
    let buf_reader_next_line = buf_reader.lines().next();
    let request_line_result = match buf_reader_next_line {
        None=> {
            log_warn("received empty packet","worker");
            return;
        },
        _=>buf_reader_next_line.unwrap(),
    };
    let request_line = match request_line_result {
        Ok(_) => request_line_result.unwrap(),
        Err(_) => {
            log_warn("Error caused on reading from buffer","worker");
            "GET /404 HTTP/1.1".to_string()
        }
    };

    let uri = uri::extract(request_line.as_str());
    let client_addr = stream.local_addr().unwrap().ip().to_string();

    log_info(format!("received request from {client_addr} asking for uri {uri}").as_str(),"worker");

    let response =special_cases(uri);

    stream.write_all(response.as_slice()).unwrap();

    log_info("responded to request!","worker");
}



