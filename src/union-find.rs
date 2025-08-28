use anyhow::{Result, anyhow};
use std::{
    fs::File,
    io::{BufRead, BufReader},
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

/// Page 227
/// Initialized as a vecotor of Node links that are connected to themselves
/// plus vector of heights for all the roots.
///
/// The overall winner. If it used the trait can remove mut from find and is_connected
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

    fn is_connected(&mut self, l: Node, r: Node) -> bool {
        self.find(l) == self.find(r)
    }

    fn find(&mut self, n: Node) -> Component {
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
