use std::fs;
use std::path::Path;
use log::warn;

pub fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}

pub fn load_contents(file_name: &str) -> Vec<u8> {
    fs::read(file_name).unwrap()
}

pub fn safe_load(filename: &str)->Option<Vec<u8>>{
    if file_exists(filename){
        let data = match fs::read(filename){
            Ok(data) => { data }
            Err(err) => {
                warn!("Error occurred in safe_load(): {err}");
                return None;
            }
        };
        Some(data)
    }else{
        None
    }
}