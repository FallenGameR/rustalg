// page 220

use anyhow::{Result, anyhow};
use clap::{Command, arg};
//use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
//    sync::{
//        Arc,
//        atomic::{AtomicU32, Ordering},
//    },
    usize,
};

#[derive(Debug)]
pub struct Config {
    in_file: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Component {
    id: usize,
}

/// Union find algorithm that can say if two specific nodes are transitive connected.
trait UnionFind {
    /// Returns the number of connected components.
    fn count(&self) -> usize;

    /// Checks if two nodes are connected.
    fn is_connected(&self, p: Node, q: Node) -> bool;

    /// Finds the component a node belongs to.
    fn find(&self, p: Node) -> Component;

    /// Adds new node connection into the algorithm.
    fn union(&mut self, p: Node, q: Node);
}

/// page 222
/// Initialized as a vector of components with the same indexes as node ids.
struct QuickFindSlowUnion {
    components: Vec<Component>,
    components_count: usize,
}

impl QuickFindSlowUnion {
    // Initially, each node is its own component
    pub fn new(max: Node) -> Self {
        let count = max.id + 1;
        Self {
            components: (0..count).map(|id| Component { id }).collect(),
            components_count: count,
        }
    }
}

impl UnionFind for QuickFindSlowUnion {
    fn count(&self) -> usize {
        self.components_count
    }

    fn is_connected(&self, l: Node, r: Node) -> bool {
        self.find(l) == self.find(r)
    }

    fn find(&self, n: Node) -> Component {
        self.components[n.id].clone()
    }

    fn union(&mut self, l: Node, r: Node) {
        let left = self.find(l);
        let right = self.find(r);

        // already connected
        if left == right {
            return;
        }

        // leftmost component supersedes all connections where right component was mentioned
        self.components_count -= 1;
        for component in &mut self.components {
            if *component == right {
                *component = left;
            }
        }
    }
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
        in_file: matches
            .remove_one("INPUT_FILE")
            .expect("Input file not provided"),
    })
}

fn open(path: &str) -> Result<Box<dyn BufRead>> {
    match path {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(path).map_err(|e| anyhow!("{path}: {e}"))?,
        ))),
    }
}

fn run(config: Config) -> Result<Box<dyn UnionFind>> {
    let reader = open(&config.in_file)?;

    let (max, connections) = parse_connections(reader)?;
    let mut alg = QuickFindSlowUnion::new(max);

    println!("Total connections: {}", connections.len());
    for (p, q) in &connections {
        if alg.is_connected(*p, *q) {
            print!("  old");
        }
        else {
            print!("  new");
            alg.union(*p, *q);
        }
        println!(" {} <-> {}", p.id, q.id);
    }

    println!("Total components: {}", alg.count());
    alg.components.sort_by_key(|c| c.id);
    alg.components.dedup_by_key(|c| c.id);
    for component in &alg.components {
        println!("  {}", component.id);
    }

    Ok(Box::new(alg))
}

fn parse_connections<R: BufRead>(reader: R) -> Result<(Node, Vec<(Node, Node)>)> {
    let mut connections: Vec<(Node, Node)> = Vec::new();
    let mut max = Node { id: usize::MIN };

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
        max.id = std::cmp::max(max.id, q.id);
        max.id = std::cmp::max(max.id, p.id);
    }

    Ok((max, connections))
}

// Tests
// cargo test --bin union-find
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use indoc::indoc;

    #[test]
    fn parse_connections_success_case() {
        let text = indoc! {"
            0 1
            1 2
            2 3
        "};
        let reader = Cursor::new(text);
        let (_, connections) = parse_connections(reader).unwrap();

        let expected = vec![
            (Node { id: 0 }, Node { id: 1 }),
            (Node { id: 1 }, Node { id: 2 }),
            (Node { id: 2 }, Node { id: 3 }),
        ];

        assert_eq!(connections, expected);
    }
}
