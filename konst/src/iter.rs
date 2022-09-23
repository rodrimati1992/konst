//! Const equivalent of iterators with a specific `next` function signature.
//!
//! # Iterator
//!
//! Iterators are expected to have a `next` method with this signature:
//! ```rust
//! # struct Iter;
//! # type Item = u8;
//! # impl Iter {
//! const fn next(self) -> Option<(Item, Self)> {
//!     // ...
//! #   None
//! }
//! # }
//! ```
//! Where `Item` is whatever type the iterator is over.
//!

/// Iterates over all elements of an [iterator](crate::iter#iterator),
/// const equivalent of [`Iterator::for_each`]
///
/// # Example
///
/// ```rust
/// struct Upto10(u8);
///
/// impl Upto10 {
///     const fn next(mut self) -> Option<(u8, Self)> {
///         if self.0 < 10 {
///             let ret = self.0;
///             self.0 += 1;
///             Some((ret, self))
///         } else {
///             None
///         }
///     }
/// }
///
/// const N: u32 = {
///     let mut n = 0u32;
///     konst::iter::for_each!{elem in Upto10(7) =>
///         n = n * 10 + elem as u32;
///     }
///     n
/// };
///
/// assert_eq!(N, 789);
///
/// ```
pub use konst_macro_rules::for_each;

/// Iterates over all elements of an [iterator](crate::iter#iterator)
/// along with an iteration index,
/// const equivalent of [`Iterator::enumerate`] combined with [`Iterator::for_each`]
///
/// # Example
///
/// ```rust
/// use konst::iter::for_each_i;
/// use konst::{option, slice};
///
/// struct Mersennes(Option<u64>);
///
/// impl Mersennes {
///     const fn next(mut self) -> Option<(u64, Self)> {
///         option::map!(self.0, |number| {
///             let newer = (number << 1) | 1;
///             self.0 = if newer == number { None } else { Some(newer) };
///             (number, self)
///         })
///     }
/// }
///
/// const LEN: usize = 5;
/// const MERSES: [u64; LEN] = {
///     let mut arr = [0u64; LEN];
///     for_each_i!{(i, num) in Mersennes(Some(1)) =>
///         if i >= LEN { break }
///         arr[i] = num;
///     }
///     arr
/// };
///
/// assert_eq!(MERSES, [1, 3, 7, 15, 31])
///
///
/// ```
pub use konst_macro_rules::for_each_i;
