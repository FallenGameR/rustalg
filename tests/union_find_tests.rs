use rustalg::uf::*;

// Only pub methods can be tested here
#[test]
fn basic_chain() {
    let mut uf = WeightedUnionFind::new(Node::new(5));
    uf.union(Node::new(0), Node::new(1));
    uf.union(Node::new(1), Node::new(2));
    assert!(uf.is_connected(Node::new(0), Node::new(2)));
    assert_eq!(uf.count(), 6 - 2);
}