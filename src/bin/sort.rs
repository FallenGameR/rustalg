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

use rustalg::sort::*;
use rustalg::sort::args::sort_bin::*;


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
            arg!(-a --algorithm <ALGORITHM> "Sorting algorithm to use: selection, insertion")
                .value_parser(clap::builder::EnumValueParser::<Algorithm>::new())
                .default_value("insertion"),
        ])
        .get_matches();

    let in_file = matches
        .remove_one("INPUT_FILE")
        .expect("Input file not provided");
    let algorithm = matches
        .remove_one("algorithm")
        .expect("Algorithm not provided");

    Ok(Config::new(in_file, algorithm))
}

fn run(config: Config) -> Result<()> {
    let reader = open(&config)?;

    println!("Latest alg is to be done, yay!");

    dbg!(config);

    Ok(())
}
