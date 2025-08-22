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

//-----------------------------------------------------------------/ structs

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

//-------------------------------------------------------------------/ traits

/// Union find algorithm that can say if two specific nodes are transitive connected.
trait UnionFind {
    /// Returns the number of connected components.
    fn count(&self) -> usize;

    /// Checks if two nodes are connected.
    fn is_connected(&self, l: Node, r: Node) -> bool;

    /// Finds the component a node belongs to.
    fn find(&self, n: Node) -> Component;

    /// Adds new node connection into the algorithm.
    fn union(&mut self, l: Node, r: Node);
}

//---------------------------------------------------------/ QuickFindSlowUnion

/// Page 222
/// Initialized as a vector of components with the same indexes as node ids.
struct QuickFindSlowUnion {
    components: Vec<Component>,
    components_count: usize,
}

impl QuickFindSlowUnion {
    /// Initially, each node is its own component
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

//---------------------------------------------------/ QuickUnionSlowFind

/// Page 224
/// Initialized as a vecotor of Node links that are connected to themselves.
struct QuickUnionSlowFind {
    links: Vec<Node>,
    components_count: usize,
}

impl QuickUnionSlowFind {
    /// Initially every node is connected to itself
    pub fn new(max: Node) -> Self {
        let count = max.id + 1;
        Self {
            links: (0..count).map(|id| Node { id }).collect(),
            components_count: count,
        }
    }
}

impl UnionFind for QuickUnionSlowFind {
    fn count(&self) -> usize {
        self.components_count
    }

    fn is_connected(&self, l: Node, r: Node) -> bool {
        self.find(l) == self.find(r)
    }

    /// Component is identified as root Node that points to itself
    fn find(&self, n: Node) -> Component {
        let mut cursor = n;

        while self.links[cursor.id] != cursor {
            cursor = self.links[cursor.id];
        }

        Component { id: cursor.id }
    }

    fn union(&mut self, l: Node, r: Node) {
        let left = self.find(l);
        let right = self.find(r);

        // already connected
        if left == right {
            return;
        }

        // leftmost root gets rightmost root as its parent
        self.components_count -= 1;
        self.links[left.id] = Node { id: right.id };
    }
}

/// Page 227
/// Initialized as a vecotor of Node links that are connected to themselves
/// plus vector of heights for all the roots.
struct WeightedUnionFind {
    links: Vec<Node>,
    heights: Vec<usize>,
    components_count: usize,
}

impl WeightedUnionFind {
    /// Initially every node is connected to itself and every height is 1
    pub fn new(max: Node) -> Self {
        let count = max.id + 1;
        Self {
            links: (0..count).map(|id| Node { id }).collect(),
            heights: vec![1; count],
            components_count: count,
        }
    }
}

impl UnionFind for WeightedUnionFind {
    fn count(&self) -> usize {
        self.components_count
    }

    fn is_connected(&self, l: Node, r: Node) -> bool {
        self.find(l) == self.find(r)
    }

    fn find(&self, n: Node) -> Component {
        let mut cursor = n;

        while self.links[cursor.id] != cursor {
            cursor = self.links[cursor.id];
        }

        Component { id: cursor.id }
    }

    fn union(&mut self, l: Node, r: Node) {
        let left = self.find(l);
        let right = self.find(r);

        // already connected
        if left == right {
            return;
        }

        // connect smaller height tree to the larger tree
        self.components_count -= 1;
        if self.heights[left.id] <= self.heights[right.id] {
            self.links[left.id] = Node { id: right.id };
            self.heights[right.id] += self.heights[left.id];
        }
        else {
            self.links[right.id] = Node { id: left.id };
            self.heights[left.id] += self.heights[right.id];
        }
    }
}

//--------------------------------------------------------------/ functions
// $env:RUSTFLAGS="-Awarnings"
// cargo run --release --bin union-find -- .\data\union-find\tinyUF.txt

fn main() {
    if let Err(error) = get_args().and_then(run) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn get_args() -> Result<Config> {
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
    //let mut alg = QuickFindSlowUnion::new(max);
    //let mut alg = QuickUnionSlowFind::new(max);
    let mut alg = WeightedUnionFind::new(max);

    println!("Total connections: {}", connections.len());
    println!("Total componenets: {}", alg.components_count);
    for (p, q) in &connections {
        if alg.is_connected(*p, *q) {
            //print!("  old");
        }
        else {
            //print!("  new");
            alg.union(*p, *q);
        }
        //println!(" {} <-> {}, {}", p.id, q.id, alg.components_count);
    }

    println!("Total components: {}", alg.count());

    //alg.components.sort_by_key(|c| c.id);
    //alg.components.dedup_by_key(|c| c.id);
    //for component in &alg.components {
    //    println!("  {}", component.id);
    //}

    //for (id, link) in alg.links.iter().enumerate() {
    //    println!("  {}: {}", id, link.id);
    //}

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

//---------------------------------------------------------------/ Tests
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
