// page 245

use std::io::BufRead;

use anyhow::Result;
use clap::Parser;

use rustalg::sort::args::{open, Algorithm};

/// Sorts a file or input stream with various different algorithms
#[derive(Parser, Debug)]
#[command(version)]
pub struct Config {
    /// Input file to sort with sorted strings on each line; use - for stdin
    #[arg(value_name="INPUT", default_value="-")]
    in_file: String,

    /// Sorting algorithm to use
    #[arg(short='a', long="algorithm", value_enum, default_value_t=Algorithm::Insertion)]
    algorithm: Algorithm,
}


//--------------------------------------------------------------/ functions
// $env:RUSTFLAGS="-Awarnings"
// cargo run --release --bin union-find -- .\data\union-find\tinyUF.txt
// hyperfine.exe --warmup 1 ".\target\release\union-find.exe .\data\union-find\largeUF.txt" # 338ms for WeightedUnionFind
// hyperfine.exe --warmup 1 ".\target\release\union-find.exe .\data\union-find\largeUF.txt" # 328ms for WeightedUnionFindWithPathCompression (incremental)
fn main() {
    let config = Config::parse();
    if let Err(error) = run(config) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<()> {
    let reader = open(&config.in_file)?;
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    println!("Algorithm: {:?}", config.algorithm);
    match config.algorithm {
        Algorithm::Selection => todo!(),
        Algorithm::Insertion => todo!(),
    }


    Ok(())
};
