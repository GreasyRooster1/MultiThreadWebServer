mod files;
mod uri;
mod paths;
mod actions;

use std::{io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};
use std::ptr::copy;
use MultiThreadWebServer::ThreadPool;
use crate::actions::{Action, get_registry};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    let pool = ThreadPool::new(4);;

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute( || {
            handel_connection(stream);
        });
    }
    println!("Shutting down!");
}

fn handel_connection(mut stream:TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    let request_line_result =  lines.next().unwrap();
    let request_line = match request_line_result {
        Ok(_) => request_line_result.unwrap(),
        Err(_) => {
            println!("Error caused on reading from buffer");
            "GET /404 HTTP/1.1".to_string()
        }
    };

    let uri = uri::extract(request_line.as_str());
    //let mut client_addr = stream.local_addr().unwrap().ip().to_string();

    //println!("received request from {client_addr} asking for uri {uri}");

    let response = if actions::check_action(uri) {
        println!("executing action with identifier {uri}");
        get_registry().iter().find(|&x|x.identifier()==uri).unwrap().func()(lines)
    }else{
        println!("no action was specified, executing default");
        actions::default_action().func()(lines)
    };

    stream.write_all(response.as_bytes()).unwrap();
    println!("stream was written to, connection handel completed");
}

