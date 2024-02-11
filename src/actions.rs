use std::thread;
use std::time::Duration;
use crate::files::load_contents;
use crate::paths::DEFAULT_PATH;
use crate::{files, paths, uri};
use crate::uri::{find, parse};

pub fn load_page(request:Vec<String>)-> String {
    let mut req = request;

    let request_line = req.iter().nth(0).unwrap();
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
    format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}")
}
pub fn sleep(request:Vec<String>)-> String{
    thread::sleep(Duration::from_secs(5));
    let contents = load_contents("index.html");
    let length = contents.len();
    format!("HTTP/1.1 200 OK\r\nContent-Length:{length}\r\n\r\n{contents}")
}