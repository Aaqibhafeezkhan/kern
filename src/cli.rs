use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "kern", version, about = "A terminal text editor")]
pub struct Cli {
    /// File to open
    pub path: PathBuf,
}
