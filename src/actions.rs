use std::thread;
use std::time::Duration;
use log::warn;
use rsa::pkcs1::{EncodeRsaPublicKey, LineEnding};
use rsa::pkcs8::EncodePublicKey;
use crate::cryptography::PUBLIC_KEY_PKCS8;
//use crate::cryptography::PUBLIC_KEY;
use crate::files::{load_contents, safe_load};
use crate::logging::{log_debug, log_warn};
use crate::uri::*;

pub fn special_cases(uri:&str) -> Vec<u8> {
    match uri{
        uri if uri.starts_with("/raw_data_request")=>{
            secure_data_request_action(uri)
        }
        "/public_key"=> public_key(uri),
        _ => { HTTPResponse::from_uri(uri)}
    }
}

pub fn secure_data_request_action(uri:&str) ->Vec<u8>{
    let requested_file = uri.strip_prefix("/raw_data_request");
    let file_path = match requested_file {
        None=> {
            log_warn("invalid path requested","worker");
            return HTTPResponse::from_string_as_bytes("200 OK", "FORBIDDEN");
        }
        _=>requested_file.unwrap()
    };
    let contents = match safe_load(find_hidden(file_path).as_str()){
        None => {
            log_warn("file was not found","worker");
            HTTPResponse::from_string_as_bytes("200 OK","System could not locate the file specified\n(raw_data_request::safe_load() -> None)")
        }
        Some(data) => {data}
    };
    HTTPResponse::from_bytes("200 OK".to_string(), contents.as_slice())
}

pub fn public_key(uri: &str) ->Vec<u8>{
    let key =PUBLIC_KEY_PKCS8.as_bytes().to_owned();
    log_debug("served public key via /public_key","worker");
    HTTPResponse::from_bytes("200 OK".to_string(), key.as_slice())
}