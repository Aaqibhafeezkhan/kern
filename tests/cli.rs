use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use kern::cli::Cli;

#[test]
fn accepts_a_file_path() {
    let cli = Cli::try_parse_from(["kern", "example.txt"]).unwrap();

    assert_eq!(cli.path, Some(PathBuf::from("example.txt")));
}

#[test]
fn rejects_missing_file_path_or_subcommand() {
    let result = Cli::try_parse_from(["kern"]).unwrap();
    let exit_code = result.run();

    assert_eq!(exit_code, ExitCode::FAILURE);
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

#[test]
fn accepts_update_command() {
    let result = Cli::try_parse_from(["kern", "update"]);

    assert!(result.is_ok());
}

#[test]
fn accepts_update_check_flag() {
    let result = Cli::try_parse_from(["kern", "update", "--check"]);

    assert!(result.is_ok());
}
