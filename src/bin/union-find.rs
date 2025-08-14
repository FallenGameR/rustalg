// page 220

use std::{fs::File, io::{BufRead, BufReader}, sync::{atomic::{AtomicU32, Ordering}, Arc}};
use clap::{arg, Command};
use anyhow::{anyhow, Result};
use rayon::prelude::*;
//use crate::shared_function;

#[derive(Debug)]
pub struct Config {
    in_file: String,
}

#[derive(Debug)]
pub struct Node {
    id: usize,
}

#[derive(Debug)]
pub struct Component {
    id: usize,
}

trait UnionFind {
    fn count(&self) -> usize;
    fn is_connected(&self, p: Node, q: Node) -> bool;
    fn find(&self, p: Node) -> Component;
    fn union(&mut self, p: Node, q: Node);
}

// cargo run --release --bin union-find -- .\data\union-find\tinyUF.txt
fn main() {
    if let Err(error) = get_args().and_then(run) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

pub fn get_args() -> Result<Config> {
    let mut matches = Command::new("union-find")
        .version("1.0")
        .author("FallenGameR")
        .about("Outputs connection of not yet connected sites, at the end prints total number of connected components")
        .args([
            arg!([INPUT_FILE] "Input file that starts with number of entries and then proceeds with pairs of connected sites, stdin is -").default_value("-"),
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
    let mut reader = open(&config.in_file)?;

    // Parse number of connections
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    let n: usize = first_line.trim().parse()?; // parse::<i32>, into, from
    println!("Number of connections: {n}");

    // Parse the connections
    for line in reader.lines() {
        let line = line?;
        //let mut pair = line.trim().split_whitespace();
        println!("Processing line: {line}");
    }

    Ok(())
}