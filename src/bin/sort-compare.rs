//! Temporary placeholder binary.
//! The earlier experimental comparison driver was fully commented out,
//! which caused a build error (no `main`). Reintroduce a minimal `main`
//! so the workspace compiles. When ready, replace this with an actual
//! benchmarking / comparison harness (e.g. using `criterion` or manual timing).

fn main() {
    eprintln!(
        "sort-compare binary is currently a stub. Run `cargo run --bin sort -- --help` for the main sorting CLI."
    );
}