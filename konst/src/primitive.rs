//! `const fn` equivalents of primitive type methods.

/// `const fn`s for comparing primitive types for equality and ordering.
#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

#[cfg(feature = "parsing")]
mod parse;

#[cfg(feature = "parsing")]
pub use parse::*;
