use std::fs;
use std::path::Path;

pub fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}

pub fn load_contents(file_name: &str) -> Vec<u8> {
    fs::read(file_name).unwrap()
}
