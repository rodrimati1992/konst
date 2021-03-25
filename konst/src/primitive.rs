//! `const fn` equivalents of primitive type methods.

/// `const fn`s for comparing primitive types for equality and ordering.
#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

mod parse;

pub use parse::*;
