// page 220

use anyhow::{anyhow, Result};
use clap::{arg, Command};
use indoc::indoc;
use rayon::prelude::*;
use std::{fs::File, io::{BufRead, BufReader}, sync::{atomic::{AtomicU32, Ordering}, Arc}};

#[derive(Debug)]
pub struct Config {
    in_file: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

struct RegularUnionFind {
    connections: Vec<(Node, Node)>,
    components: Vec<Component>,
}

// $env:RUSTFLAGS="-Awarnings"
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
    let reader = open(&config.in_file)?;
    let connections: Vec<(Node, Node)> = parse_connections(reader)?;

    println!("Total connections: {}", connections.len());
    for (p, q) in &connections {
        println!("  {} <-> {}", p.id, q.id);
    }

    Ok(connections)
}

pub fn parse_connections<R: BufRead>(reader: R) -> Result<Vec<(Node, Node)>> {
    let mut connections: Vec<(Node, Node)> = Vec::new();

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

    Ok(connections)
}

// Tests
// cargo test --bin union-find
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn parse_connections_success_case() {
        let text = indoc! {"
            0 1
            1 2
            2 3
        "};
        let reader = Cursor::new(text);
        let connections = parse_connections(reader).unwrap();

        let expected = vec![
            (Node { id: 0 }, Node { id: 1 }),
            (Node { id: 1 }, Node { id: 2 }),
            (Node { id: 2 }, Node { id: 3 }),
        ];

        assert_eq!(connections, expected);
    }
}
