// Export these submodules as they are
pub mod sort;

// Re-export modules with shorter names
mod union_find;
pub mod uf {
    #[doc(inline)]
    pub use super::union_find::*;
}

