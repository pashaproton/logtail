use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::error::LogtailError;

pub fn open_input(path: Option<&Path>) -> Result<Box<dyn BufRead>, LogtailError> {
    match path {
        Some(path) => {
            let file = File::open(path)?;
            Ok(Box::new(BufReader::new(file)))
        }
        None => {
            let stdin = io::stdin();
            Ok(Box::new(BufReader::new(stdin)))
        }
    }
}
