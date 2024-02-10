mod files {
    use std::fs;
    use std::path::Path;

    pub fn file_exists(filename: &str) -> bool {
        Path::new(filename).exists()
    }

    pub fn load_contents(file_name: &str) -> String {
        fs::read_to_string(file_name).unwrap()
    }
}