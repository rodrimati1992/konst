//! Const equivalent of iterators with a specific `next` function signature.
//!
//! The docs for [`IntoIterKind`] has more information on
//! const equivalents of IntoIterator and Iterator.
//!

/// Iterates over all elements of an [iterator](crate::iter#iterator),
/// const equivalent of [`Iterator::for_each`]
///
/// # Example
///
/// ```rust
/// use konst::iter::{IntoIterKind, IsIteratorKind};
///
/// struct Upto10(u8);
///
/// impl IntoIterKind for Upto10 { type Kind = IsIteratorKind; }
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

/// Const equivalent of [`Iterator::zip`] combined with [`Iterator::for_each`]
///
/// # Example
///
/// ### Enumerate
///
/// ```rust
/// use konst::{
///     iter::{IntoIterKind, IsIteratorKind, for_each_zip},
///     option,
/// };
///
/// struct Mersennes(Option<u64>);
///
/// impl IntoIterKind for Mersennes { type Kind = IsIteratorKind; }
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
///     for_each_zip!{(i, num) in 0.., Mersennes(Some(1)) =>
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
///
/// ### Zip two slices
///
/// This example requires the `"rust_1_51"` feature
///
#[cfg_attr(feature = "rust_1_51", doc = "```rust")]
#[cfg_attr(not(feature = "rust_1_51"), doc = "```ignore")]
/// use konst::iter::for_each_zip;
///     
/// const fn add_pairs<const N: usize>(l: [u32; N], r: [u32; N]) -> [u32; N] {
///     let mut out = [0u32; N];
///     for_each_zip!{(i, &l, &r) in 0.., &l, &r =>
///         out[i] = l + r;
///     }
///
///     out
/// }
///
/// assert_eq!(add_pairs([], []), []);
/// assert_eq!(add_pairs([3], [5]), [8]);
/// assert_eq!(add_pairs([3, 5], [8, 13]), [11, 18]);
///
///
/// ```
pub use konst_macro_rules::for_each_zip;

/// Const equivalent of [`Iterator::all`]
///
/// # Example
///
/// ```rust
/// const fn all_digits(s: &str) -> bool {
///     konst::iter::all!(s.as_bytes(), |c| c.is_ascii_digit())
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
/// use konst::iter;
///
/// const fn contains_pow2(s: &[u64]) -> bool {
///     iter::any!(s, |c| c.is_power_of_two())
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
/// use konst::iter;
///
/// const fn find_num(slice: &[u64], n: u64) -> Option<usize> {
///     iter::position!(slice, |&&elem| elem == n)
/// }
///
/// assert_eq!(find_num(&[3, 5, 8], 0), None);
/// assert_eq!(find_num(&[3, 5, 8], 3), Some(0));
/// assert_eq!(find_num(&[3, 5, 8], 5), Some(1));
/// assert_eq!(find_num(&[3, 5, 8], 8), Some(2));
///
/// ```
pub use konst_macro_rules::iter_position as position;

/// Const equivalent of [`Iterator::rposition`]
///
/// # Example
///
/// ```rust
/// use konst::iter;
///
/// const fn rfind_num(slice: &[u64], n: u64) -> Option<usize> {
///     iter::rposition!(slice, |&&elem| elem == n)
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
/// use konst::iter;
///
/// const fn find_odd(slice: &[u64], n: u64) -> Option<&u64> {
///     iter::find!(slice, |&&elem| elem % 2 == 1)
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
/// use konst::{iter, result};
/// use konst::primitive::parse_u64;
///
/// const fn find_parsable(slice: &[&str]) -> Option<u64> {
///     iter::find_map!(slice, |&s| result::ok!(parse_u64(s)))
/// }
///
/// assert_eq!(find_parsable(&[]), None);
/// assert_eq!(find_parsable(&["foo"]), None);
/// assert_eq!(find_parsable(&["foo", "10"]), Some(10));
/// assert_eq!(find_parsable(&["10", "20"]), Some(10));
///
/// ```
pub use konst_macro_rules::iter_find_map as find_map;

/// Const equivalent of [`DoubleEndedIterator::rfind`]
///
/// # Example
///
/// ```rust
/// use konst::iter;
///
/// const fn sum_u64(slice: &[u64]) -> Option<&u64> {
///     iter::rfind!(slice, |&elem| elem.is_power_of_two())
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
/// use konst::iter;
///
/// const fn sum_u64(slice: &[u64]) -> u64 {
///     iter::fold!(slice, 0, |accum, &rhs| accum + rhs)
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
/// use konst::iter;     
///
/// const fn concat_u16s(slice: &[u16]) -> u128 {
///     iter::rfold!(slice, 0, |accum, &rhs| (accum << 16) + (rhs as u128))
/// }
///
/// assert_eq!(concat_u16s(&[1, 2, 3]), 0x0003_0002_0001);
/// assert_eq!(concat_u16s(&[3, 5, 8]), 0x0008_0005_0003);
///
///
/// ```
pub use konst_macro_rules::iter_rfold as rfold;

/// Wrapper for `IntoIterKind` implementors,
/// that defines different methods depending on the
/// value of `<T as IntoIterKind>::Kind`.
#[doc(inline)]
pub use konst_macro_rules::into_iter::IntoIterWrapper;

/// Marker type for proving that `T: IntoIterKind<Kind = K>`
#[doc(inline)]
pub use konst_macro_rules::into_iter::IsIntoIterKind;

/// Macro for converting [`IntoIterKind`] implementors into const iterators.
///
#[doc(inline)]
pub use konst_macro_rules::into_iter_macro as into_iter;

/// Const analog of the [`IntoIterator`] trait.
///
/// # Implementor
///
/// Implementors are expected to be:
///
/// - [Types that have an associated iterator](#isnoniteratorkind),
///   that have [`IsNonIteratorKind`](crate::iter::IsNonIteratorKind)
///   as the [`IntoIterKind::Kind`] associated type.
///
/// - [Iterators themselves](#isiteratorkind),
/// that have [`IsIteratorKind`](crate::iter::IsIteratorKind)
/// as the [`IntoIterKind::Kind`] associated type.
///
/// - Standard library types, of the [`IsStdKind`] kind
///
/// ### `IsNonIteratorKind`
///
/// These types are expected to define this inherent method for converting to
/// a const iterator:
///
/// ```rust
/// # struct II;
/// # struct SomeIterator;
/// # impl II {
/// const fn const_into_iter(self) -> SomeIterator {
/// #   loop{}
/// # }
/// # }
/// ```
///
/// [full example below](#non-iter-example)
///
/// ### `IsIteratorKind`
///
/// These types are expected to have this inherent method:
///
/// ```rust
/// # struct SomeIterator;
/// # type Item = u8;
/// # impl SomeIterator {
/// // Equivalent to `Iterator::next`
/// const fn next(self) -> Option<(Item, Self)> {
/// #   loop{}
/// # }
/// # }
/// ```
/// Where `Item` can be any type.
///
/// These are other methods that you can optionaly define,
/// which most iterators from the `konst` crate define:
/// ```rust
/// # struct SomeIterator;
/// # struct SomeIteratorRev;
/// # type Item = u8;
/// # impl SomeIterator {
/// // equivalent to `DoubleEndedÌterator::mext_back`
/// const fn next_back(self) -> Option<(Item, Self)> {
/// #   loop{}
///     // ... some code...
/// }
///
/// // Reverses the itereator, equivalent to `Iterator::rev`
/// const fn rev(self) -> SomeIteratorRev {
/// #   loop{}
///     // ... some code...
/// }
///
/// // Clones the iterator, equivalent to `Clone::clone`
/// const fn copy(&self) -> Self {
/// #   loop{}
///     // ... some code...
/// }
/// # }
/// ```
/// Where `SomeIteratorRev` should be a `IntoIterKind<Kind = IsIteratorKind>`
/// which has the same inherent methods for iteration.
///
/// [full example below](#iter-example)
///
/// # Examples
///
/// <span id = "non-iter-example"></span>
/// ### Implementing for a non-iterator
///
/// ```rust
/// use konst::{iter, slice};
///
/// struct GetSlice<'a, T>{
///     slice: &'a [T],
///     up_to: usize,
/// }
///
/// impl<T> iter::IntoIterKind for GetSlice<'_, T> {
///     type Kind = iter::IsNonIteratorKind;
/// }
///
/// impl<'a, T> GetSlice<'a, T> {
///     const fn const_into_iter(self) -> konst::slice::Iter<'a, T> {
///         slice::iter(slice::slice_up_to(self.slice, self.up_to))
///     }
/// }
///
/// const fn sum_powers(up_to: usize) -> u64 {
///     let gs = GetSlice{slice: &[1, 2, 4, 8, 16, 32, 64, 128], up_to};
///
///     iter::fold!(gs, 0, |l, &r| l + r)
/// }
///
/// assert_eq!(sum_powers(0), 0);
/// assert_eq!(sum_powers(1), 1);
/// assert_eq!(sum_powers(2), 3);
/// assert_eq!(sum_powers(3), 7);
/// assert_eq!(sum_powers(4), 15);
/// assert_eq!(sum_powers(5), 31);
///
/// ```
///
/// <span id = "iter-example"></span>
/// ### Implementing for an iterator
///
/// ```rust
/// use konst::iter::{self, IntoIterKind};
///
/// struct Countdown(u8);
///
/// impl IntoIterKind for Countdown { type Kind = iter::IsIteratorKind; }
///
/// impl Countdown {
///     const fn next(mut self) -> Option<(u8, Self)> {
///         konst::option::map!(self.0.checked_sub(1), |ret| {
///             self.0 = ret;
///             (ret, self)
///         })
///     }
/// }
///
/// const fn sum(initial: u8) -> u16 {
///     iter::fold!(Countdown(initial), 0u16, |accum, elem| accum + elem as u16)
/// }
///
/// assert_eq!(sum(0), 0);
/// assert_eq!(sum(1), 0);
/// assert_eq!(sum(2), 1);
/// assert_eq!(sum(3), 3);
/// assert_eq!(sum(4), 6);
/// assert_eq!(sum(5), 10);
///
/// ```
///
/// ### Implementing for a double-ended iterator
///
/// ```rust
/// use konst::iter;
///
/// assert_eq!(HOURS, [1, 2, 3, 4, 5, 6, 12, 11, 10, 9, 8, 7]);
///
/// const HOURS: [u8; 12] = {
///     let mut arr = [0; 12];
///     let hours = Hours::new();
///
///     iter::for_each_zip!{(i, hour) in 0..6, hours.copy() =>
///         arr[i] = hour;
///     }
///     iter::for_each_zip!{(i, hour) in 6..12, hours.rev() =>
///         arr[i] = hour;
///     }
///
///     arr
/// };
///
///
/// struct Hours{
///     start: u8,
///     end: u8,
/// }
///
/// impl iter::IntoIterKind for Hours {
///     type Kind = iter::IsIteratorKind;
/// }
///
/// impl Hours {
///     const fn new() -> Self {
///         Self {start: 1, end: 13}
///     }
///
///     const fn next(mut self) -> Option<(u8, Self)> {
///         if self.start == self.end {
///             None
///         } else {
///             let ret = self.start;
///             self.start += 1;
///             Some((ret, self))
///         }
///     }
///
///     const fn next_back(mut self) -> Option<(u8, Self)> {
///         if self.start == self.end {
///             None
///         } else {
///             self.end -= 1;
///             Some((self.end, self))
///         }
///     }
///
///     const fn rev(self) -> HoursRev {
///         HoursRev(self)
///     }
///
///     /// Since `Clone::clone` isn't const callable on stable,
///     /// clonable iterators must define an inherent method to be cloned
///     const fn copy(&self) -> Self {
///         let Self{start, end} = *self;
///         Self{start, end}
///     }
/// }
///
/// struct HoursRev(Hours);
///
/// impl iter::IntoIterKind for HoursRev {
///     type Kind = iter::IsIteratorKind;
/// }
///
/// impl HoursRev {
///     const fn next(self) -> Option<(u8, Self)> {
///         konst::option::map!(self.0.next_back(), |(a, h)| (a, HoursRev(h)))
///     }
///
///     const fn next_back(self) -> Option<(u8, Self)> {
///         konst::option::map!(self.0.next(), |(a, h)| (a, HoursRev(h)))
///     }
///
///     const fn rev(self) -> Hours {
///         self.0
///     }
///
///     const fn copy(&self) -> Self {
///         Self(self.0.copy())
///     }
/// }
///
///
/// ```
///
#[doc(inline)]
pub use konst_macro_rules::into_iter::IntoIterKind;

/// For marking some type as being from std
/// in its [`IntoIterKind::Kind`] associated type.
#[doc(inline)]
pub use konst_macro_rules::into_iter::IsStdKind;

/// For marking some type as being convertible to an iterator
/// in its [`IntoIterKind::Kind`] associated type.
#[doc(inline)]
pub use konst_macro_rules::into_iter::IsNonIteratorKind;

/// For marking some type as being an iterator
/// in its [`IntoIterKind::Kind`] associated type.
#[doc(inline)]
pub use konst_macro_rules::into_iter::IsIteratorKind;
