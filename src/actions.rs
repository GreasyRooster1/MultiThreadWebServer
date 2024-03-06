use std::thread;
use std::time::Duration;
use log::warn;
use crate::files::{load_contents, safe_load};
use crate::load_contents_from_uri;
use crate::uri::*;

pub fn special_cases(uri:&str) -> Vec<u8> {
    match uri{
        uri if uri.starts_with("/raw_data_request")=>{
            secure_data_request_action(uri)
        }
        _ => { load_contents_from_uri(uri)}
    }
}

pub fn secure_data_request_action(uri:&str) ->Vec<u8>{
    let requested_file = uri.strip_prefix("/raw_data_request");
    let file_path = match requested_file {
        None=> {
            warn!("Invalid path");
            return HTTPResponse::from_string_as_bytes("200 OK", "FORBIDDEN");
        }
        _=>requested_file.unwrap()
    };
    let contents = match safe_load(find_hidden(file_path).as_str()){
        None => {
            HTTPResponse::from_string_as_bytes("200 OK","System could not locate the file specified\n(raw_data_request::safe_load() -> None)")
        }
        Some(data) => {data}
    };
    HTTPResponse::from_bytes("200 OK".to_string(), contents.as_slice())
}