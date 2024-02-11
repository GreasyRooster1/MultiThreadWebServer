use std::thread;
use std::time::Duration;
use crate::files::load_contents;

trait Action{
    fn identifier()->String;
    fn func()->fn(&str) ->String;
}
pub(crate) trait Registry{
    fn get_registry() -> Vec<Box<dyn Action>>;
}
pub(crate) struct ActionRegistry();
impl Registry for ActionRegistry{
    fn get_registry() -> Vec<Box<dyn Action>>{
        vec![Box::new(Sleep::new())]
    }
}


//Actions
struct Sleep();
impl Action for Sleep {
    fn identifier() -> String { "sleep".parse().unwrap() }
    fn func() -> fn(&str) -> &str {
        //creates an anonymous function for only this scope, then returns it
        fn anon(request: &str) -> &str {
            thread::sleep(Duration::from_secs(5));
            let contents = load_contents("index.html");
            let length = contents.len();
            format!("HTTP/1.1 200 OK\r\nContent-Length:{length}\r\n\r\n{contents}").as_str()
        }
        anon
    }
}