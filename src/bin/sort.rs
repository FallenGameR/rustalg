// page 245

use anyhow::{Result, anyhow};
use clap::{arg, Command, Parser};

use rustalg::sort::args::{open, sort_bin::*};

/// Sorts a file or input stream with various different algorithms
#[derive(Parser, Debug)]
#[command(name = "sort", version, about = "Sort utility")]
pub struct Config {
    /// Input file with strings to sort on each line, stdin is - to the (positional parameter), this comment is added to help
    #[arg(default_value = "-")]
    in_file: String,

    algorithm: Algorithm,
}

impl Config {
    pub fn new(in_file: String, algorithm: Algorithm) -> Self {
        Self { in_file, algorithm }
    }
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
    let cli = Config::parse();
    if cli.command.is_none() {
        return Ok(());
    }

    let in_file = cli.input_file;
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
    let reader = open(&config.in_file)?;

    println!("Latest alg is to be done, yay!");

    dbg!(config);

    Ok(())
}
