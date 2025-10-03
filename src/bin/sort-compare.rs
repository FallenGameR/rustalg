// page 256

use anyhow::Result;
use clap::Parser;
use rustalg::sort::{args::{open, Algorithm}, insertion::InsertionSort, selection::SelectionSort, Sort};

/// Compare two sort alg implementations runtime-wise
#[derive(Parser, Debug)]
#[command(version)]
pub struct Config {
    /// First sorting algorithm to use
    #[arg(short='s', long="first", value_enum, default_value_t=Algorithm::Selection)]
    first: Algorithm,

    /// Second sorting algorithm to use
    #[arg(short='b', long="second", value_enum, default_value_t=Algorithm::Insertion)]
    second: Algorithm,

    /// Length of a random array of doubles to sort
    #[arg(short='l', long="length", default_value_t=1_000_000)]
    array_length: usize,

    /// Number of trials to run
    #[arg(short='t', long="trials", default_value_t=3)]
    trials: usize,
}

//--------------------------------------------------------------/ functions
// $env:RUSTFLAGS="-Awarnings"
// cargo run --release --bin sort-compare -- --help
// hyperfine.exe --warmup 1 ".\target\release\sort-compare.exe -a selection .\data\rand\32K_int.txt" # 2.4s
// hyperfine.exe --warmup 1 ".\target\release\sort-compare.exe -a insertion .\data\rand\2K_int.txt"  # 1.0 sec
fn main() {
    let config = Config::parse();
    if let Err(error) = run(config) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<()> {
    /*
    println!("Finished reading, number of lines: {:?}", lines.len());

    println!("Init algorithm: {:?}", config.first);
    let mut sorter: Box<dyn Sort<Item = String>> = match config.first {
        Algorithm::Selection => Box::new(SelectionSort::new(lines)),
        Algorithm::Insertion => Box::new(InsertionSort::new(lines)),
    };

    sorter.sort();
    println!("Finished sorting");

    sorter.is_sorted();
    println!("All is sorted");
    */

    Ok(())
}
