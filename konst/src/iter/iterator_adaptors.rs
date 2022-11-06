/// Const analog of [`core::iter::repeat`],
/// except that this requires the repeated value to impl `Copy`
/// (instead of `Clone`).
///
/// # Example
///
/// ```rust
/// use konst::iter::{self, collect_const};
///
/// const ARR: &[u8] = &collect_const!(u8 => iter::repeat(3),take(5));
///
/// assert_eq!(ARR, &[3, 3, 3, 3, 3]);
/// ```
pub use konst_kernel::iter::iter_adaptors::repeat;

/// Const analog of [`core::iter::Repeat`],
/// constructed by [`repeat`](crate::iter::repeat).
pub use konst_kernel::iter::iter_adaptors::Repeat;
