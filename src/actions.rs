use std::thread;
use std::time::Duration;
use crate::files::load_contents;

//--FOR FUTURE REFERENCE--
//when making methods, using &self as a parameter makes the method non-static
//whereas without the &self it acts similarly to a static method in java
//traits that need to act as objects (Action in this case) cant have any of these to be considered as objects
//and non-static structs need a ::new() method, as rust doesnt have an equivalent to "new" in java
//thanks to Iris for helping me out with this lol

pub(crate) trait Action{
    fn identifier(&self)->String;
    fn func(&self)->fn(&str) ->String;
}

pub(crate) struct ActionRegistry;
impl ActionRegistry{
    pub(crate) fn get_registry() -> Vec<Box<dyn Action>>{
        vec![Box::new(Sleep::new())]
    }
}


//Actions
struct Sleep;
impl Action for Sleep {
    fn identifier(&self) -> String { "sleep".parse().unwrap() }
    fn func(&self) -> fn(&str) -> String {
        //creates an anonymous function for only this scope, then returns it
        fn anon(_request: &str) -> String {
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