//! `const fn` equivalents of range methods.

/// `const fn`s for comparing range for equality and ordering.
#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

/// Const-iterator for [`Range`](core::ops::Range)
///
/// This is constructed like this:
/// ```rust
/// # let _ =
/// konst::iter::into_iter!(0..10)
/// # ;
/// ```
#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::into_iter::range_into_iter::RangeIter;

/// Reversed const-iterator for [`Range`](core::ops::Range)
///
///
/// This is constructed like this:
/// ```rust
/// # let _ =
/// konst::iter::into_iter!(0..10).rev()
/// # ;
/// ```
#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::into_iter::range_into_iter::RangeIterRev;

/// Const-iterator for [`RangeInclusive`](core::ops::RangeInclusive)
///
/// This is constructed like this:
/// ```rust
/// # let _ =
/// konst::iter::into_iter!(0..=10)
/// # ;
/// ```
#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::into_iter::range_into_iter::RangeInclusiveIter;

/// Reversed const-iterator for [`RangeInclusive`](core::ops::RangeInclusive)
///
/// This is constructed like this:
/// ```rust
/// # let _ =
/// konst::iter::into_iter!(0..=10).rev()
/// # ;
/// ```
#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::into_iter::range_into_iter::RangeInclusiveIterRev;

/// Const-iterator for [`RangeFrom`](core::ops::RangeFrom)
///
/// This is constructed like this:
/// ```rust
/// # let _ =
/// konst::iter::into_iter!(0..)
/// # ;
/// ```
#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::into_iter::range_into_iter::RangeFromIter;
