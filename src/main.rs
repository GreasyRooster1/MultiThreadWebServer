mod files;
mod uri;
mod defaults;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use MultiThreadWebServer::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
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

    let (status_line,filename) = match &request_line[..] {
        "GET / HTTP/1.1"=>("HTTP/1.1 200 OK","index.html"),
        "GET /sleep HTTP/1.1"=>{
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK",defaults::DEFAULT_PATH)
        }
        _ => ("HTTP/1.1 404 NOT FOUND",defaults::NOT_FOUND_PATH),
    };

    let contents = if files::file_exists(filename) {
        files::load_contents(filename)
    }else{
        files::load_contents(defaults::NOT_FOUND_PATH)
    };
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap()
}

