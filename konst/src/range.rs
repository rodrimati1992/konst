//! `const fn` equivalents of range methods.

/// `const fn`s for comparing range for equality and ordering.
#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

#[cfg(feature = "iter")]
include!{"./range/range_into_iter.rs"}