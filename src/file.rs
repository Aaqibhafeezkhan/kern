use std::fs;
use std::io;

pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)
}
