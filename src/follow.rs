use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::error::LogtailError;
use crate::filter::LineFilter;

const FOLLOW_SLEEP_MS: u64 = 200;

pub struct FollowOptions {
    pub show_line_numbers: bool,
    pub count_only: bool,
}

pub fn process_file<R: BufRead, W: Write>(
    reader: &mut R,
    writer: &mut W,
    filter: &LineFilter,
    show_line_numbers: bool,
    count_only: bool,
    line_number: &mut usize,
    matched_count: &mut usize,
) -> Result<(), LogtailError> {
    let mut line = String::new();

    loop {
        line.clear();

        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        *line_number += 1;

        let content = line.strip_suffix('\n').unwrap_or(&line);
        let content = content.strip_suffix('\r').unwrap_or(content);

        if filter.matches(content) {
            *matched_count += 1;

            if !count_only {
                if show_line_numbers {
                    writeln!(writer, "{}:{}", *line_number, content)?;
                } else {
                    writeln!(writer, "{content}")?;
                }
            }
        }
    }

    Ok(())
}

pub fn follow_file<W: Write>(
    path: &Path,
    writer: &mut W,
    filter: &LineFilter,
    options: &FollowOptions,
) -> Result<usize, LogtailError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut line_number = 0usize;
    let mut matched_count = 0usize;

    process_file(
        &mut reader,
        writer,
        filter,
        options.show_line_numbers,
        options.count_only,
        &mut line_number,
        &mut matched_count,
    )?;

    loop {
        let current_position = reader.stream_position()?;
        let metadata = std::fs::metadata(path)?;
        let file_len = metadata.len();

        if file_len < current_position {
            reader = BufReader::new(File::open(path)?);
            line_number = 0;

            process_file(
                &mut reader,
                writer,
                filter,
                options.show_line_numbers,
                options.count_only,
                &mut line_number,
                &mut matched_count,
            )?;
        } else if file_len > current_position {
            reader.seek(SeekFrom::Start(current_position))?;

            process_file(
                &mut reader,
                writer,
                filter,
                options.show_line_numbers,
                options.count_only,
                &mut line_number,
                &mut matched_count,
            )?;

            writer.flush()?;
        } else {
            thread::sleep(Duration::from_millis(FOLLOW_SLEEP_MS));
        }
    }
}

pub fn validate_follow_args(file: Option<&Path>, count: bool) -> Result<(), LogtailError> {
    if file.is_none() {
        return Err(LogtailError::InvalidInput(
            "--follow requires a file path and cannot be used with stdin".to_string(),
        ));
    }

    if count {
        return Err(LogtailError::InvalidInput(
            "--count cannot be used together with --follow".to_string(),
        ));
    }

    Ok(())
}
