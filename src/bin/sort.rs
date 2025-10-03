// page 245

use anyhow::Result;
use clap::Parser;
use rustalg::sort::{args::{open, Algorithm}, insertion::InsertionSort, selection::SelectionSort, Sort};

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
// cargo run --release --bin sort -- -a selection .\data\rand\2K_int.txt
// hyperfine.exe --warmup 1 ".\target\release\sort.exe -a selection .\data\rand\32K_int.txt" # 2.4s
// hyperfine.exe --warmup 1 ".\target\release\sort.exe -a insertion .\data\rand\2K_int.txt"  # 1.0 sec
fn main() {
    let config = Config::parse();
    if let Err(error) = run(config) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<()> {
    println!("Reading input file: {:?}", config.in_file);
    let reader = open(&config.in_file)?;
    let mut lines = Vec::new();
    for line in std::io::BufRead::lines(reader) {
        lines.push(line?);
    }
    println!("Finished reading, number of lines: {:?}", lines.len());

    println!("Init algorithm: {:?}", config.algorithm);
    let mut sorter: Box<dyn Sort<Item = String>> = match config.algorithm {
        Algorithm::Selection => Box::new(SelectionSort::new(lines)),
        Algorithm::Insertion => Box::new(InsertionSort::new(lines)),
    };

    sorter.sort();
    println!("Finished sorting");

    sorter.is_sorted();
    println!("All is sorted");

    Ok(())
}
