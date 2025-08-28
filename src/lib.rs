// Reference nested modules but don't export them
mod union_find;

// Export modules with shorter names
pub mod uf {
    #[doc(inline)]
    pub use super::union_find::*;
}