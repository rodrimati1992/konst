//! `const fn` equivalents of slice methods.

/// `const fn`s for comparing slices for equality and ordering.
#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

mod slice_const_methods;

pub use slice_const_methods::*;
