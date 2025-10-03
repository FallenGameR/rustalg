// page 256

use anyhow::Result;
use clap::Parser;
use rand::Rng;
use rustalg::sort::{args::{Algorithm}, insertion::InsertionSort, selection::SelectionSort, Sort};

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
    #[arg(short='l', long="length", default_value_t=1_000)]
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

    let mut rng = rand::rng();

    for trial in 0..config.trials {
        println!("Trial {}/{}: array length {}", trial + 1, config.trials, config.array_length);
        let data: Vec<f64> = (0..config.array_length).map(|_| rng.random()).collect();

        let duration_first = measure_sort(&config.first, data.clone());
        println!("  {:?}: {:?}", config.first, duration_first);

        let duration_second = measure_sort(&config.second, data.clone());
        println!("  {:?}: {:?}", config.second, duration_second);
    }

    Ok(())
}

fn construct_alg(alg: &Algorithm, data: Vec<f64>) -> Box<dyn Sort<Item = f64>> {
    match alg {
        Algorithm::Selection => Box::new(SelectionSort::new(data)),
        Algorithm::Insertion => Box::new(InsertionSort::new(data)),
    }
}


fn measure_sort(alg: &Algorithm, data: Vec<f64>) -> std::time::Duration {
    let mut sorter = construct_alg(alg, data.clone());

    let start = std::time::Instant::now();
    sorter.sort();
    let duration = start.elapsed();

    sorter.is_sorted();
    duration
}
