use std::path::PathBuf;

use clap::Parser;
use kern::cli::Cli;

#[test]
fn accepts_a_file_path() {
    let cli = Cli::try_parse_from(["kern", "example.txt"]).unwrap();

    assert_eq!(cli.path, PathBuf::from("example.txt"));
}

#[test]
fn rejects_missing_file_path() {
    let result = Cli::try_parse_from(["kern"]);

    assert!(result.is_err());
}

#[test]
fn rejects_additional_arguments() {
    let result = Cli::try_parse_from(["kern", "one.txt", "two.txt"]);

    assert!(result.is_err());
}

#[test]
fn accepts_short_help_flag() {
    let result = Cli::try_parse_from(["kern", "-h"]);

    let error = result.unwrap_err();
    assert_eq!(error.kind(), clap::error::ErrorKind::DisplayHelp);
}

#[test]
fn accepts_long_help_flag() {
    let result = Cli::try_parse_from(["kern", "--help"]);

    let error = result.unwrap_err();
    assert_eq!(error.kind(), clap::error::ErrorKind::DisplayHelp);
}
