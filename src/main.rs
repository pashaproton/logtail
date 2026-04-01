mod cli;
mod error;
mod filter;
mod follow;
mod input;

use std::io::{self, Write};

use clap::Parser;
use cli::Args;
use error::LogtailError;
use filter::LineFilter;
use follow::{FollowOptions, follow_file, process_file, validate_follow_args};
use input::open_input;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), LogtailError> {
    let args = Args::parse();

    let filter = LineFilter::build(
        args.contains,
        args.regex,
        args.ignore_case,
        args.invert_match,
    )?;

    let stdout = io::stdout();
    let mut writer = stdout.lock();

    if args.follow {
        validate_follow_args(args.file.as_deref(), args.count)?;

        let options = FollowOptions {
            show_line_numbers: args.line_number,
            count_only: args.count,
        };

        let path = args.file.as_deref().expect("validated above");
        let _ = follow_file(path, &mut writer, &filter, &options)?;
        return Ok(());
    }

    let mut reader = open_input(args.file.as_deref())?;
    let mut line_number: usize = 0;
    let mut matched_count: usize = 0;

    process_file(
        &mut reader,
        &mut writer,
        &filter,
        args.line_number,
        args.count,
        &mut line_number,
        &mut matched_count,
    )?;

    if args.count {
        writeln!(writer, "{matched_count}")?;
    }

    Ok(())
}
