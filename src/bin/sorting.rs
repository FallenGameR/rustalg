// page 245

use anyhow::{Result, anyhow};
use clap::{Command, arg};
//use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    //    sync::{
    //        Arc,
    //        atomic::{AtomicU32, Ordering},
    //    },
    usize,
};

use rustalg::args::*;
use rustalg::sort::*;


//--------------------------------------------------------------/ functions
// $env:RUSTFLAGS="-Awarnings"
// cargo run --release --bin union-find -- .\data\union-find\tinyUF.txt
// hyperfine.exe --warmup 1 ".\target\release\union-find.exe .\data\union-find\largeUF.txt" # 338ms for WeightedUnionFind
// hyperfine.exe --warmup 1 ".\target\release\union-find.exe .\data\union-find\largeUF.txt" # 328ms for WeightedUnionFindWithPathCompression (incremental)
fn main() {
    if let Err(error) = get_args().and_then(run) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn get_args() -> Result<Config> {
    let mut matches = Command::new("sorting")
        .version("1.0")
        .author("FallenGameR")
        .about("Sorts a file or input stream")
        .args([
            arg!([INPUT_FILE] "Input file with strings to sort on each line, stdin is -").default_value("-"),
        ])
        .get_matches();

    let in_file = matches
        .remove_one("INPUT_FILE")
        .expect("Input file not provided");

    Ok(Config::new(in_file))
}

fn run(config: Config) -> Result<()> {
    let reader = open(&config)?;

    println!("Latest alg is to be done, yay!");

    Ok(())
}
