/// Page 227

//-----------------------------------------------------------------/ imports

use std::{
    usize,
};

//-----------------------------------------------------------------/ structs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node {
    pub id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Component {
    id: usize,
}

/// Initialized as a vecotor of Node links that are connected to themselves
/// plus vector of heights for all the roots.
pub struct WeightedUnionFind {
    links: Vec<Node>,
    heights: Vec<usize>,
    pub components_count: usize,
}

//-------------------------------------------------------------------/ traits

/// Union find algorithm that can say if two specific nodes are transitive connected.
pub trait UnionFind {
    /// Returns the number of connected components.
    fn count(&self) -> usize;

    /// Checks if two nodes are connected.
    fn is_connected(&self, l: Node, r: Node) -> bool;

    /// Finds the component a node belongs to.
    fn find(&self, n: Node) -> Component;

    /// Adds new node connection into the algorithm.
    fn union(&mut self, l: Node, r: Node);
}

//-------------------------------------------------------------------/ implementations

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
