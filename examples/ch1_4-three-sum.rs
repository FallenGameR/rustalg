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

use std::{fs::File, io::{BufRead, BufReader}};
use clap::{arg, Command};
use anyhow::{anyhow, Ok, Result};

#[derive(Debug)]
pub struct Config {
    in_file: String,
}

/// cargo run --release --example ch1_4-three-sum examples\data\ch1_4-three-sum\1Kints.txt
/// hyperfine.exe "cargo run --release --example ch1_4-three-sum examples\data\ch1_4-three-sum\1Kints.txt"
pub fn run(config: Config) -> Result<()> {
    dbg!(&config);

    let reader = open(&config.in_file)?;
    let mut numbers = Vec::new();

    // TODO: Write terser code, figure out error propogation
    for line in reader.lines() {
        let line = line?;
        let number = line.trim().parse::<i32>()?;
        numbers.push(number);
    }

    dbg!(&numbers);

    // TODO: the actual tree sum logic here
    // TODO: measure performance on different input sizes

    Ok(())
}

fn main() {
    if let Err(error) = get_args().and_then(run) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

pub fn get_args() -> Result<Config> {
    let mut matches = Command::new("three-sum")
        .version("1.0")
        .author("FallenGameR")
        .about("Counts number of triples that sum to 0 in an array of signed integers")
        .args([
            arg!([INPUT_FILE] "Input file with signed integers to process, stdin is -").default_value("-"),
        ])
        .get_matches();

    Ok(Config {
        in_file: matches.remove_one("INPUT_FILE").expect("Input file not provided"),
    })
}

fn open(path: &str) -> Result<Box<dyn BufRead>> {
    match path {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(path).map_err(|e| anyhow!("{path}: {e}"))?))),
    }
}
