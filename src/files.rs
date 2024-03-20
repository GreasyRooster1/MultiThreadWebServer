use std::fs;
use std::path::Path;
use log::warn;
use crate::logging::log_warn;

pub fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}

pub fn load_contents(file_name: &str) -> Vec<u8> {
    match fs::read(file_name){
        Ok(res) => {
            res
        }
        Err(_) => {
            log_warn(format!("could not load file: {file_name}").as_str(),"load_contents");
            vec![]
        }
    }
}

pub fn safe_load(filename: &str)->Option<Vec<u8>>{
    if file_exists(filename){
        let data = match fs::read(filename){
            Ok(data) => { data }
            Err(err) => {
                log_warn(format!("Error occurred in safe_load(): {err}").as_str(),"files::safe_load");
                return None;
            }
        };
        Some(data)
    }else{
        None
    }
}