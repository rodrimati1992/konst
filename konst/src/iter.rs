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

/// Const equivalent of [`Iterator::all`]
///
/// # Example
///
/// ```rust
/// use konst::{iter, slice};
///
/// const fn all_digits(s: &str) -> bool {
///     iter::all!(slice::iter(s.as_bytes()), |c| c.is_ascii_digit())
/// }
///
/// assert!(all_digits("123456"));
/// assert!(!all_digits("0x123456"));
///
/// ```
pub use konst_macro_rules::iter_all as all;

/// Const equivalent of [`Iterator::any`]
///
/// # Example
///
/// ```rust
/// use konst::{iter, slice};
///
/// const fn contains_pow2(s: &[u64]) -> bool {
///     iter::any!(slice::iter(s), |c| c.is_power_of_two())
/// }
///
/// assert!(contains_pow2(&[2, 3, 5]));
/// assert!(!contains_pow2(&[13, 21, 34]));
///
/// ```
pub use konst_macro_rules::iter_any as any;

/// Const equivalent of [`Iterator::position`]
///
/// # Example
///
/// ```rust
/// use konst::{iter, slice};
///
/// const fn find_num(slice: &[u64], n: u64) -> Option<usize> {
///     iter::position!(slice::iter(slice), |&&elem| elem == n)
/// }
///
/// assert_eq!(find_num(&[3, 5, 8], 0), None);
/// assert_eq!(find_num(&[3, 5, 8], 3), Some(0));
/// assert_eq!(find_num(&[3, 5, 8], 5), Some(1));
/// assert_eq!(find_num(&[3, 5, 8], 8), Some(2));
///
/// ```
pub use konst_macro_rules::iter_position as position;

/// Const equivalent of [`DoubleEndedIterator::rposition`]
///
/// # Example
///
/// ```rust
/// use konst::{iter, slice};
///
/// const fn rfind_num(slice: &[u64], n: u64) -> Option<usize> {
///     iter::rposition!(slice::iter(slice), |&&elem| elem == n)
/// }
///
/// assert_eq!(rfind_num(&[3, 5, 8], 0), None);
/// assert_eq!(rfind_num(&[3, 5, 8], 3), Some(2));
/// assert_eq!(rfind_num(&[3, 5, 8], 5), Some(1));
/// assert_eq!(rfind_num(&[3, 5, 8], 8), Some(0));
///
/// ```
pub use konst_macro_rules::iter_rposition as rposition;

/// Const equivalent of [`Iterator::find`]
///
/// # Example
///
/// ```rust
/// use konst::{iter, slice};
///
/// const fn find_odd(slice: &[u64], n: u64) -> Option<&u64> {
///     iter::find!(slice::iter(slice), |&&elem| elem % 2 == 1)
/// }
///
/// assert_eq!(find_odd(&[], 0), None);
/// assert_eq!(find_odd(&[2, 4], 0), None);
/// assert_eq!(find_odd(&[3, 5, 8], 3), Some(&3));
/// assert_eq!(find_odd(&[8, 12, 13], 3), Some(&13));
///
/// ```
pub use konst_macro_rules::iter_find as find;

/// Const equivalent of [`Iterator::count`]
///
/// # Example
///
/// This example requires the `"rust_1_64"` crate feature.
///
#[cfg_attr(not(feature = "rust_1_64"), doc = "```ignore")]
#[cfg_attr(feature = "rust_1_64", doc = "```rust")]
/// use konst::{iter, string};
///
/// const fn count_csv(s: &str) -> usize {
///     iter::count!(string::split(s, ","))
/// }
///
/// assert_eq!(count_csv("foo"), 1);
/// assert_eq!(count_csv("foo,bar"), 2);
/// assert_eq!(count_csv("foo,bar,baz"), 3);
///
/// ```
pub use konst_macro_rules::iter_count as count;

/// Const equivalent of [`Iterator::nth`]
///
/// # Example
///
/// This example requires the `"rust_1_64"` crate feature.
///
#[cfg_attr(not(feature = "rust_1_64"), doc = "```ignore")]
#[cfg_attr(feature = "rust_1_64", doc = "```rust")]
/// use konst::{iter, string};
///
/// const fn nth_csv(s: &str, nth: usize) -> Option<&str> {
///     iter::nth!(string::split(s, ","), nth)
/// }
///
/// assert_eq!(nth_csv("foo,bar,baz", 0), Some("foo"));
/// assert_eq!(nth_csv("foo,bar,baz", 1), Some("bar"));
/// assert_eq!(nth_csv("foo,bar,baz", 2), Some("baz"));
/// assert_eq!(nth_csv("foo,bar,baz", 3), None);
///
/// ```
pub use konst_macro_rules::iter_nth as nth;

/// Const equivalent of [`Iterator::find_map`]
///
/// # Example
///
/// This example requires the `"parsing_no_proc"` feature.
///
#[cfg_attr(not(feature = "parsing_no_proc"), doc = "```ignore")]
#[cfg_attr(feature = "parsing_no_proc", doc = "```rust")]
/// use konst::{iter, result, slice};
/// use konst::primitive::parse_u64;
///
/// const fn find_parsable(slice: &[&str]) -> Option<u64> {
///     iter::find_map!(slice::iter(slice), |&s| result::ok!(parse_u64(s)))
/// }
///
/// assert_eq!(find_parsable(&[]), None);
/// assert_eq!(find_parsable(&["foo"]), None);
/// assert_eq!(find_parsable(&["foo", "10"]), Some(10));
/// assert_eq!(find_parsable(&["10", "20"]), Some(10));
///
/// ```
pub use konst_macro_rules::iter_find_map as find_map;

/// Const equivalent of [`Iterator::rfind`]
///
/// # Example
///
/// ```rust
/// use konst::{iter, slice};
///
/// const fn sum_u64(slice: &[u64]) -> Option<&u64> {
///     iter::rfind!(slice::iter(slice), |&elem| elem.is_power_of_two())
/// }
///
/// assert_eq!(sum_u64(&[]), None);
/// assert_eq!(sum_u64(&[2]), Some(&2));
/// assert_eq!(sum_u64(&[2, 5, 8]), Some(&8));
///
///
/// ```
pub use konst_macro_rules::iter_rfind as rfind;

/// Const equivalent of [`Iterator::fold`]
///
/// # Example
///
/// ```rust
/// use konst::{iter, slice};
///
/// const fn sum_u64(slice: &[u64]) -> u64 {
///     iter::fold!(slice::iter(slice), 0, |accum, &rhs| accum + rhs)
/// }
///
/// assert_eq!(sum_u64(&[]), 0);
/// assert_eq!(sum_u64(&[3]), 3);
/// assert_eq!(sum_u64(&[3, 5]), 8);
/// assert_eq!(sum_u64(&[3, 5, 8]), 16);
///
///
/// ```
pub use konst_macro_rules::iter_fold as fold;

/// Const equivalent of [`DoubleEndedIterator::rfold`]
///
/// # Example
///
/// ```rust
/// use konst::{iter, slice};     
///
/// const fn concat_u16s(slice: &[u16]) -> u128 {
///     iter::rfold!(
///         slice::iter(slice),
///         0,
///         |accum, &rhs| (accum << 16) + (rhs as u128)
///     )
/// }
///
/// assert_eq!(concat_u16s(&[1, 2, 3]), 0x0003_0002_0001);
/// assert_eq!(concat_u16s(&[3, 5, 8]), 0x0008_0005_0003);
///
///
/// ```
pub use konst_macro_rules::iter_rfold as rfold;
