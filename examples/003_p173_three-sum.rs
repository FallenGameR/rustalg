
use std::{fs::File, io::{BufRead, BufReader}, sync::{atomic::{AtomicU32, Ordering}, Arc}};
use clap::{arg, Command};
use anyhow::{anyhow, Ok, Result};
use rayon::prelude::*;

#[derive(Debug)]
pub struct Config {
    in_file: String,
}

fn main() {
    if let Err(error) = get_args().and_then(run) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

pub fn get_args() -> Result<Config> {
    let mut matches = Command::new("three-sum")
        .version("1.0")
        .author("FallenGameR")
        .about("Counts number of triples that sum to 0 in an array of signed integers")
        .args([
            arg!([INPUT_FILE] "Input file with signed integers to process, stdin is -").default_value("-"),
        ])
        .get_matches();

    Ok(Config {
        in_file: matches.remove_one("INPUT_FILE").expect("Input file not provided"),
    })
}

fn open(path: &str) -> Result<Box<dyn BufRead>> {
    match path {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(path).map_err(|e| anyhow!("{path}: {e}"))?))),
    }
}

pub fn run(config: Config) -> Result<()> {

    // Parse numbers from file
    let mut numbers = Vec::new();
    let reader = open(&config.in_file)?;
    for line in reader.lines() {
        let line = line?;
        let number = line.trim().parse::<i32>()?;
        numbers.push(number);
    }

    // Find three sum combinations that are zero
    // for_impl, ranges_impl, tuples_impl
    // tuples_vec_par_filter_impl
    // partitions_impl, partitions_par_impl
    // ranges_arc_impl, atomic_par_impl
    let result = atomic_par_trailing_impl(numbers);

    // Print the output
    println!("{}", result);

    Ok(())
}

fn atomic_par_trailing_impl(numbers: Vec<i32>) -> usize {
    let n = numbers.len();
    let result = Arc::new(AtomicU32::new(0));

    (0..n).for_each(|a|
        (a+1..n).for_each(|b|
            (b+1..n).into_par_iter().for_each(|c|
                if numbers[a] + numbers[b] + numbers[c] == 0 {
                    result.fetch_add(1, Ordering::Relaxed);
                }
            )
        )
    );

    result.load(Ordering::Relaxed) as usize
}


fn atomic_par_par_par_impl(numbers: Vec<i32>) -> usize {
    let n = numbers.len();
    let result = Arc::new(AtomicU32::new(0));

    (0..n).into_par_iter().for_each(|a|
        (a+1..n).into_par_iter().for_each(|b|
            (b+1..n).into_par_iter().for_each(|c|
                if numbers[a] + numbers[b] + numbers[c] == 0 {
                    //let counter = Arc::clone(&counter);
                    result.fetch_add(1, Ordering::Relaxed);
                }
            )
        )
    );

    result.load(Ordering::Relaxed) as usize
}

fn atomic_par_par_impl(numbers: Vec<i32>) -> usize {
    let n = numbers.len();
    let result = Arc::new(AtomicU32::new(0));

    (0..n).into_par_iter().for_each(|a|
        (a+1..n).into_par_iter().for_each(|b|
            (b+1..n).for_each(|c|
                if numbers[a] + numbers[b] + numbers[c] == 0 {
                    result.fetch_add(1, Ordering::Relaxed);
                }
            )
        )
    );

    result.load(Ordering::Relaxed) as usize
}

fn atomic_par_impl(numbers: Vec<i32>) -> usize {
    let n = numbers.len();
    let result = Arc::new(AtomicU32::new(0));

    (0..n).into_par_iter().for_each(|a|
        (a+1..n).for_each(|b|
            (b+1..n).for_each(|c|
                if numbers[a] + numbers[b] + numbers[c] == 0 {
                    result.fetch_add(1, Ordering::Relaxed);
                }
            )
        )
    );

    result.load(Ordering::Relaxed) as usize
}

fn ranges_arc_impl(numbers: Vec<i32>) -> usize {
    let n = numbers.len();
    let result = Arc::new(AtomicU32::new(0));

    (0..n).for_each(|a|
        (a+1..n).for_each(|b|
            (b+1..n).for_each(|c|
                if numbers[a] + numbers[b] + numbers[c] == 0 {
                    result.fetch_add(1, Ordering::Relaxed);
                }
            )
        )
    );

    result.load(Ordering::Relaxed) as usize
}

fn partitions_par_impl(numbers: Vec<i32>) -> usize {
    let n = numbers.len();

    let partitions =
        (0..n).into_par_iter().map(move |a| -> usize {
            (a+1..n).flat_map(move |b|
                (b+1..n).map(move |c|
                    (a,b,c)
                )).filter(|(a,b,c)| numbers[*a] + numbers[*b] + numbers[*c] == 0 ).count()
            }
        );

    partitions.sum()
}

fn partitions_impl(numbers: Vec<i32>) -> usize {
    let n = numbers.len();

    let partitions =
        (0..n).map(move |a| -> usize {
            (a+1..n).flat_map(move |b|
                (b+1..n).map(move |c|
                    (a,b,c)
                )).filter(|(a,b,c)| numbers[*a] + numbers[*b] + numbers[*c] == 0 ).count()
            }
        );

    partitions.sum()
}

fn tuples_impl(numbers: Vec<i32>) -> usize {
    let n = numbers.len();

    let tuples =
        (0..n).flat_map(move |a|
            (a+1..n).flat_map(move |b|
                (b+1..n).map(move |c|
                    (a,b,c)
                )
            )
        );

    tuples.filter(|(a,b,c)| numbers[*a] + numbers[*b] + numbers[*c] == 0 ).count()
}

fn ranges_impl(numbers: Vec<i32>) -> usize {
    let mut result = 0;

    (0..numbers.len()).into_iter().for_each(|a| {
        (a+1..numbers.len()).into_iter().for_each(|b| {
            (b+1..numbers.len()).into_iter().for_each(|c| {
                if numbers[a] + numbers[b] + numbers[c] == 0 {
                    result += 1;
                }
            });
        });
    });

    result
}

fn for_impl(numbers: Vec<i32>) -> usize {
    let mut result = 0;

    for a in 0..numbers.len() {
        for b in a+1..numbers.len() {
            for c in b+1..numbers.len() {
                if numbers[a] + numbers[b] + numbers[c] == 0 {
                    result += 1;
                }
            }
        }
    }

    result
}
