mod common;

use common::temp_file_path;
use kern::file::read_file;
use std::fs;

#[test]
fn error_no_such_file() {
    let path = temp_file_path();
    let result = read_file(&path);

    assert!(result.is_err());
}

#[test]
fn reads_file_contents() {
    let path = temp_file_path();
    let expected = "Hello, kern!\nHello from the 2nd line!";

    fs::write(&path, expected).unwrap();

    let result = read_file(&path);

    assert_eq!(result.unwrap(), expected);

    fs::remove_file(&path).unwrap();
}
