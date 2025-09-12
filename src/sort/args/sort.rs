use anyhow::{Result, anyhow};
use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Debug)]
pub enum Algorithm {
    Selection,
    Insertion,
}

#[derive(Debug)]
pub struct Config {
    in_file: String,
    algorithm: Algorithm,
}

impl Config {
    pub fn new(in_file: String, algorithm: Algorithm) -> Self {
        Self { in_file, algorithm }
    }
}

pub fn open(config: &Config) -> Result<Box<dyn BufRead>> {
    let path = config.in_file.as_str();
    match path {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(path).map_err(|e| anyhow!("{path}: {e}"))?,
        ))),
    }
}