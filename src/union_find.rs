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
    components_count: usize,
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

impl Node {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

impl From<Component> for Node {
    fn from(c: Component) -> Self {
        Self { id: c.id }
    }
}

impl From<Node> for Component {
    fn from(node: Node) -> Self {
        Self { id: node.id }
    }
}

impl WeightedUnionFind {
    /// Initially every node is connected to itself and every height is 1
    pub fn new(max: Node) -> Self {
        let count = max.id + 1;
        Self {
            links: (0..count).map(|id| Node::new(id)).collect(),
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

        Component::from(cursor)
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
            self.links[left.id] = Node::from(right);
            self.heights[right.id] += self.heights[left.id];
        }
        else {
            self.links[right.id] = Node::from(left);
            self.heights[left.id] += self.heights[right.id];
        }
    }
}

// --------------------------------------------------------------------/ tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state() {
        let uf = WeightedUnionFind::new(Node::new(4));

        // There are 0..=4 components in total, none are connected yet
        assert_eq!(uf.count(), 5);

        // Each one has itself as its root
        assert_eq!(uf.find(Node::new(3)), Component::from(Node::new(3)));
    }

    #[test]
    fn union_connects_and_decreases_count() {
        // 4 components
        let mut uf = WeightedUnionFind::new(Node::new(3));

        // connect 0,1 -> 3 components
        uf.union(Node::new(0), Node::new(1));
        assert!(uf.is_connected(Node::new(0), Node::new(1)));
        assert!(!uf.is_connected(Node::new(0), Node::new(2)));
        assert_eq!(uf.count(), 3);

        // connect 2,3 -> 2 components
        uf.union(Node::new(2), Node::new(3));
        assert!(uf.is_connected(Node::new(2), Node::new(3)));
        assert_eq!(uf.count(), 2);

        // repeated connection doesn't change component count
        uf.union(Node::new(0), Node::new(1));
        assert_eq!(uf.count(), 2);
    }

    #[test]
    fn transitive_connection() {
        let mut uf = WeightedUnionFind::new(Node::new(4));
        uf.union(Node::new(0), Node::new(1));
        uf.union(Node::new(1), Node::new(2));
        assert!(uf.is_connected(Node::new(0), Node::new(2)));
        assert!(!uf.is_connected(Node::new(0), Node::new(3)));
    }
}