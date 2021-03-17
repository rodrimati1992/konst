//! `const fn` equivalents of slice methods.

/// `const fn`s for comparing slices for equality and ordering.
pub mod cmp;

mod slice_const_methods;

pub use slice_const_methods::*;
