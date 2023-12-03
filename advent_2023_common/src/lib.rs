use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// File to parse
    #[arg(default_value = "input")]
    pub path: PathBuf,
}

pub fn get_args() -> Cli {
    Cli::parse()
}
