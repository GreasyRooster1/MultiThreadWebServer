use std::thread;
use std::time::Duration;
use crate::load_contents_from_uri;
use crate::uri::{create_const_http, create_http_response};

pub fn special_cases(uri:&str) -> Vec<u8> {
    match uri{
        uri if uri.starts_with("/raw_data_request")=>{
            let requested_file = uri.strip_prefix("/raw_data_request");
            let requested_bytes = requested_file.unwrap().as_bytes().to_owned();
            create_http_response("200 OK".to_string(), &*requested_bytes)
        }
        _ => { load_contents_from_uri(uri)}
    }
}