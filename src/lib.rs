// Export these submodules as they are
pub mod sort;

// Export modules with shorter names
mod union_find;
pub mod uf {
    #[doc(inline)]
    pub use super::union_find::*;
}

