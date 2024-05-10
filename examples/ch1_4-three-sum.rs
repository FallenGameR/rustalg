// https://algs4.cs.princeton.edu/14analysis/
//
// cd V:\src\rust\rustalg\examples\data\ch1_4-three-sum\
// http https://algs4.cs.princeton.edu/14analysis/1Kints.txt > 1Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/2Kints.txt > 2Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/4Kints.txt > 4Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/8Kints.txt > 8Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/16Kints.txt > 16Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/32Kints.txt > 32Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/1Mints.txt > 1Mints.txt

use std::{fs::File, io::{self, BufRead, BufReader}};

use clap::{arg, Command};
use anyhow::{bail, Ok, Result};

#[derive(Debug)]
pub struct Config {
    in_file: String,
}

/// cargo run --release --example ch1_4-three-sum examples\data\ch1_4-three-sum\1Kints.txt
/// hyperfine.exe "cargo run --release --example ch1_4-three-sum examples\data\ch1_4-three-sum\1Kints.txt"
fn main() {
    let config = get_args().unwrap();
    dbg!(&config);
}

pub fn get_args() -> Result<Config> {
    let mut matches = Command::new("three-sum")
        .version("1.0")
        .author("FallenGameR")
        .about("Counts number of triples that sum to 0 in an array of integers")
        .args([
            arg!([INPUT_FILE] "Input file to process, stdin is -").default_value("-"),
        ])
        .get_matches();

    Ok(Config {
        in_file: matches.remove_one("INPUT_FILE").expect("Input file not provided"),
    })
}

pub fn run(config: Config) -> Result<()> {
    match open_read(&config) {
        Err(error) => panic!("Can't open file '{}', error {}", &config.in_file, error),
        Ok(reader) => process_unuque(reader)?,
    }

    Ok(())
}

fn open_read(config: &Config) -> Result<Box<dyn BufRead>> {
    match config.in_file.as_str() {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        path => Ok(Box::new(BufReader::new(File::open(path)?))),
    }
}

fn process_unuque(mut _reader: impl BufRead) -> Result<()> {
    todo!("Implement process_unuque")
}