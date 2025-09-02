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

//use rustalg::sort::*;

//-----------------------------------------------------------------/ structs

#[derive(Debug)]
pub struct Config {
    in_file: String,
}

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
    let mut matches = Command::new("union-find")
        .version("1.0")
        .author("FallenGameR")
        .about("Outputs connection of not yet connected sites, at the end prints total number of connected components")
        .args([
            arg!([INPUT_FILE] "Input file with pairs of connected sites specified by ids, stdin is -").default_value("-"),
        ])
        .get_matches();

    Ok(Config {
        in_file: matches
            .remove_one("INPUT_FILE")
            .expect("Input file not provided"),
    })
}

fn open(path: &str) -> Result<Box<dyn BufRead>> {
    match path {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(path).map_err(|e| anyhow!("{path}: {e}"))?,
        ))),
    }
}


fn run(_config: Config) -> Result<()> {
    println!("Latest alg is to be done, yay!");
    Ok(())
}
