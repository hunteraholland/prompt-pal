use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A tool for analyzing and processing prompt files"
)]
pub struct Cli {
    /// Directory to scan for prompt files
    #[arg(short, long, value_name = "DIR")]
    pub directory: PathBuf,

    /// Generate XML output
    #[arg(short, long)]
    pub xml: bool,

    /// Check token count
    #[arg(short, long)]
    pub tokens: bool,

    /// Output file for results (optional)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    pub debug: u8,
}
