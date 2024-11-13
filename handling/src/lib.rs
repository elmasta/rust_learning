use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use std::fs;

pub fn open_or_create(file: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file)
        .unwrap_or_else(|e| panic!("Failed to open or create file '{}': {}", file, e));
    if let Err(e) = file.write_all(content.as_bytes()) {
        panic!("Failed to write to file '{:?}': {}", file, e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_file_content(filename: &str) -> String {
        let mut file = File::open(filename).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        fs::remove_file(filename).unwrap();
        return s;
    }

    #[test]
    fn test_if_file_exists() {
        let filename = "test_existing_file.txt";
        let content = "hello world!";
        File::create(filename).unwrap();
        open_or_create(filename, content);

        assert_eq!(content, get_file_content(filename));
    }
}
