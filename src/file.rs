use std::fs;
use std::io;
use std::path::PathBuf;

pub fn write_file(path: &PathBuf, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

pub fn read_file(path: &PathBuf) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
