// page 256

use anyhow::Result;
use clap::Parser;
use rand::Rng;
use rustalg::sort::{args::Algorithm, Sort, Sorter};

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
    #[arg(short='l', long="length", default_value_t=10_000)]
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
    let mut time_first = std::time::Duration::ZERO;
    let mut time_second = std::time::Duration::ZERO;

    for trial in 0..config.trials {
        println!("Trial {}/{}", trial + 1, config.trials);
        let data: Vec<f64> = (0..config.array_length).map(|_| rng.random()).collect();

        let duration_first = measure_sort(&config.first, data.clone());
        println!("  {:?}: {:?}", config.first, duration_first);
        time_first += duration_first;

        let duration_second = measure_sort(&config.second, data.clone());
        println!("  {:?}: {:?}", config.second, duration_second);
        time_second += duration_second;
    }

    let ratio = time_second.as_secs_f64() / time_first.as_secs_f64();
    println!("Alg {:?} is {:.2}x faster than {:?} on array length {}", config.first, ratio, config.second, config.array_length);

    Ok(())
}

fn measure_sort(alg: &Algorithm, data: Vec<f64>) -> std::time::Duration {
    let mut sorter = Sorter::from_algorithm(alg, data);

    let start = std::time::Instant::now();
    sorter.sort();
    let duration = start.elapsed();

    sorter.is_sorted();
    duration
}
