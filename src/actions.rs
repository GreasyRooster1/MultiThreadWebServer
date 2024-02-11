use std::{ptr, thread, vec};
use std::io::{BufRead, BufReader, Lines};
use std::net::TcpStream;
use std::time::Duration;
use crate::{actions, files, paths, uri};
use crate::files::load_contents;
use crate::paths::DEFAULT_PATH;
use crate::uri::{find, parse};

//--FOR FUTURE REFERENCE--
//when making methods, using &self as a parameter makes the method non-static
//whereas without the &self it acts similarly to a static method in java
//traits that need to act as objects (Action in this case) cant have any of these to be considered as objects
//and non-static structs need a ::new() method, as rust doesnt have an equivalent to "new" in java
//thanks to Iris for helping me out with this lol


//structs
pub(crate) trait Action{
    fn identifier(&self)->String;
    fn func(&self)->fn(Vec<String>) ->String;
}

pub(crate) fn get_registry() -> Vec<Box<dyn Action>>{
    vec![Box::new(Sleep::new())]
}
pub(crate) fn default_action() -> Box<dyn Action>{
    Box::new(Page::new())
}
//Helper functions
pub(crate) fn check_action(uri:&str) -> bool {
    for action in get_registry().iter(){
        if action.identifier().eq(uri){
            return true
        }
    }
    false
}

//Actions
struct Page;
impl Action for Page {
    fn identifier(&self) -> String { "/".parse().unwrap() }
    fn func(&self) -> fn(Vec<String>) -> String {
        //creates an anonymous function for only this scope, then returns it
        fn anon(request: Vec<String>) -> String {
            let mut req = request;

            let request_line = req.iter().nth(0).unwrap();
            println!("req {request_line}");
            let uri = uri::extract(request_line.as_str());

            let filename = if uri.eq("/") {
                find(DEFAULT_PATH)
            }else{
                parse(find(uri).as_str())
            };
            let (contents,status_line) = if files::file_exists(filename.as_str()) {
                (load_contents(filename.as_str()),"HTTP/1.1 200 OK")
            }else{
                (load_contents(&paths::NOT_FOUND_PATH),"HTTP/1.1 404 NOT FOUND")
            };
            let length = contents.len();
            format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}")
        }
        anon
    }
}
impl Page {
    fn new()->Self{
        Page{}
    }
}

struct Sleep;
impl Action for Sleep {
    fn identifier(&self) -> String { "/sleep".parse().unwrap() }
    fn func(&self) -> fn(Vec<String>) -> String {
        //creates an anonymous function for only this scope, then returns it
        fn anon(_request: Vec<String>) -> String {
            thread::sleep(Duration::from_secs(5));
            let contents = load_contents("index.html");
            let length = contents.len();
            format!("HTTP/1.1 200 OK\r\nContent-Length:{length}\r\n\r\n{contents}")
        }
        anon
    }
}
impl Sleep {
    fn new()->Self{
        Sleep{}
    }
}