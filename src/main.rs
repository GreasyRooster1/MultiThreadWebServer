mod files;
mod uri;
mod paths;
mod actions;

use std::{io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};
use MultiThreadWebServer::ThreadPool;
use crate::actions::{Action, ActionRegistry};
use crate::paths::DEFAULT_PATH;
use crate::uri::{find, parse};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    let pool = ThreadPool::new(4);;

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

    let request_line_result = buf_reader.lines().next().unwrap();
    let request_line = match request_line_result {
        Ok(_) => request_line_result.unwrap(),
        Err(_) => {
            println!("Error caused on reading from buffer");
            "GET /404 HTTP/1.1".to_string()
        }
    };

    let uri = uri::extract(request_line.as_str());
    let client_addr = stream.local_addr().unwrap().ip().to_string();

    println!("received request from {client_addr} asking for uri {uri}");

    let response = if actions::check_action(uri) {
        println!("executing action with identifier {uri}");
        actions::get_action(uri).func()(buf_reader.lines())
    }else{
        println!("no action was specified, executing default");
        ActionRegistry::default_action().func()(buf_reader.lines())
    };

    stream.write_all(response.as_bytes()).unwrap();
    println!("stream was written to, connection handel completed");
}

