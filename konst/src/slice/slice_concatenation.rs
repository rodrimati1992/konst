/// Macro equivalent of `<[&[T]]>::concat`, which takes a constant as an argument.
///
/// This acts like a compile-time-evaluated version of this function:
/// ```rust
/// pub const fn slice_concat<T>(slices: &'static [&'static [T]]) -> [T; LEN]
/// where
///     T: Copy
/// # { [] }
/// # const LEN: usize = 0;
/// ```
///
/// Where `LEN` is the summed length of all inner slices.
///
/// # Example
///
/// ```rust
/// use konst::slice::slice_concat;
///
/// const S: &[&[u8]] = &[&[3, 5], &[8, 13, 21, 34]];
/// assert_eq!(slice_concat!(u8, S), [3, 5, 8, 13, 21, 34]);
///
/// assert_eq!(slice_concat!(u8, &[]), []);
///
/// assert_eq!(slice_concat!(u8, &[&[], &[1, 2, 3], &[4, 5]]), [1, 2, 3, 4, 5]);
///
/// ```
pub use konst_kernel::slice_concat;
