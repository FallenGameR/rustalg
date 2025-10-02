// page 245

use anyhow::Result;
use clap::Parser;

use rustalg::sort::args::{open, sort_bin::Algorithm};

/// Sorts a file or input stream with various different algorithms
#[derive(Parser, Debug)]
#[command(name = "sort", version, about = "Sort utility")]
pub struct Config {
    /// Input file with strings to sort on each line; use - for stdin
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
    let _reader = open(&config.in_file)?;
    println!("Algorithm: {:?}", config.algorithm);
    Ok(())
}
