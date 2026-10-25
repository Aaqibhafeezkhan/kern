use std::fs;
use std::io;
use std::path::PathBuf;

pub fn write_file(path: &PathBuf, content: &str) -> io::Result<()> {
    fs::write(path, content)
}
