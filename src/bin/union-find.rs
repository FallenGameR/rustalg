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
            arg!([INPUT_FILE] "Input file with pairs of connected sites specified by ids, stdin is -").default_value("-"),
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

pub fn run(config: Config) -> Result<Vec<(Node, Node)>> {
    let mut connections: Vec<(Node, Node)> = Vec::new();
    let reader = open(&config.in_file)?;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let mut pair = line.split_whitespace();

        let p = pair
            .next()
            .ok_or_else(|| anyhow!("line {i}: missing first index"))?
            .parse()
            .map(|id| Node { id })
            .map_err(|e| anyhow!("line {i}: failed to parse index: {e}"))?;

        let q = pair
            .next()
            .ok_or_else(|| anyhow!("line {i}: missing second index"))?
            .parse()
            .map(|id| Node { id })
            .map_err(|e| anyhow!("line {i}: failed to parse index: {e}"))?;

        if let Some(extra) = pair.next() {
            return Err(anyhow!("line {i}: extra token '{extra}'"));
        }

        connections.push((p, q));
    }

    println!("Total connections: {}", connections.len());
    for (p, q) in &connections {
        println!("  {} <-> {}", p.id, q.id);
    }

    Ok(connections)
}