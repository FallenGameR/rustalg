pub mod sort_bin;

use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::{Result, anyhow};

pub fn open(path: &str) -> Result<Box<dyn BufRead>> {
    match path {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(path).map_err(|e| anyhow!("failed to open {path}: {e}"))?,
        ))),
    }
}