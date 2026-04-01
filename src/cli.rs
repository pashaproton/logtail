use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "logtail",
    version,
    about = "Stream and filter large log or text files efficiently"
)]
pub struct Args {
    /// File to read. If omitted, reads from stdin.
    pub file: Option<PathBuf>,

    /// Follow file growth like tail -f
    #[arg(short = 'f', long = "follow")]
    pub follow: bool,

    /// Match lines containing this text
    #[arg(short = 'c', long = "contains")]
    pub contains: Option<String>,

    /// Match lines using a regular expression
    #[arg(short = 'r', long = "regex")]
    pub regex: Option<String>,

    /// Perform case-insensitive matching
    #[arg(short = 'i', long = "ignore-case")]
    pub ignore_case: bool,

    /// Invert the match result
    #[arg(short = 'v', long = "invert-match")]
    pub invert_match: bool,

    /// Show line numbers
    #[arg(short = 'n', long = "line-number")]
    pub line_number: bool,

    /// Print only the number of matched lines
    #[arg(long = "count")]
    pub count: bool,
}
