// https://algs4.cs.princeton.edu/14analysis/
//
// cd V:\src\rust\rustalg\examples\data\ch1_p173-three-sum\
// http https://algs4.cs.princeton.edu/14analysis/1Kints.txt > 1Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/2Kints.txt > 2Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/4Kints.txt > 4Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/8Kints.txt > 8Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/16Kints.txt > 16Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/32Kints.txt > 32Kints.txt
// http https://algs4.cs.princeton.edu/14analysis/1Mints.txt > 1Mints.txt

use std::{fs::File, io::{BufRead, BufReader}, sync::{atomic::{AtomicU32, Ordering}, Arc}};
use clap::{arg, Command};
use anyhow::{anyhow, Ok, Result};
use rayon::prelude::*;

#[derive(Debug)]
pub struct Config {
    in_file: String,
}

/// cargo run --release --example ch1_p173-three-sum examples\data\rand\1K_int.txt
/// cargo build --release --example ch1_p173-three-sum
/// hyperfine.exe --warmup 1 --export-markdown examples\data\ch1_p173-three-sum\result_single_thread.md --parameter-list SIZE 1,2,4,8,16,32 ".\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\{SIZE}Kints.txt"
/// https://github.com/rayon-rs/rayon/blob/main/README.md
/// https://github.com/rayon-rs/rayon/blob/main/FAQ.md

/*

  .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\1Kints.txt ran
    5.02 ± 0.13 times faster than .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\2Kints.txt
   35.79 ± 0.71 times faster than .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\4Kints.txt
  279.94 ± 5.74 times faster than .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\8Kints.txt
 2234.14 ± 48.41 times faster than .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\16Kints.txt
18151.01 ± 549.46 times faster than .\target\release\examples\ch1_p173-three-sum.exe examples\data\ch1_p173-three-sum\32Kints.txt

1k -  1
x2 - x5
x2 - x7
x2 - x8
x2 - x8
x2 - x8

8 = 2 * 2 * 2

this is O(n^3) algorithm

cargo run --release --example ch1_p173-three-sum examples\data\rand\1K_int.txt
cargo build --release --example ch1_p173-three-sum
hyperfine.exe ".\target\release\examples\ch1_p173-three-sum.exe .\examples\data\rand\8K_int.txt"

for_impl - 7s
ranges_impl - 38s
tuples_impl - 7s - same as for_impl, good cache utilization
tuples_vec_par_filter_impl - out of memory if we collect all indexes
partitions_impl - 18s - perf hit due to worse cache utilization
partitions_par_impl - 2s - parralel version of partitions_impl
ranges_arc_impl - 38s - same as ranges_impl, but with atomic counter, for_each is slow for some reason
atomic_par_impl - 6s - parralel version of ranges_arc_impl, worse then partitions_par_impl since it uses a shared memory variable, bad for caches
atomic_par_par_impl - 6s - rayon is smart about parralelism
atomic_par_par_par_impl - 14s - lots of everhead due to parralelism
atomic_par_trailing_impl - >1min - very inefficient to make small operations parralel

https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/
https://github.com/rayon-rs/rayon/blob/main/README.md
https://github.com/rayon-rs/rayon/blob/main/FAQ.md

https://docs.rs/rayon/latest/rayon/
https://docs.rs/rayon/latest/rayon/iter/index.html
https://docs.rs/rayon/latest/rayon/fn.join.html
https://docs.rs/rayon/latest/rayon/fn.scope.html
*/

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
