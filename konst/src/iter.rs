//! Const equivalent of iterators with a specific `next` function signature.
//!
//! The docs for [`ConstIntoIter`] has more information on
//! const equivalents of IntoIterator and Iterator.
//!

mod iterator_adaptors;
pub mod iterator_dsl;

pub use iterator_adaptors::*;

/// Iterates over all elements of an [iterator](crate::iter::ConstIntoIter),
/// const equivalent of [`Iterator::for_each`]
///
/// # Syntax
///
/// ```text
/// for_each!{
///     $pattern:pat in $iterator:expr
///         $(,$iterator_method:ident ($($method_args:tt)*) )*
///         $(,)?
///     =>
///     $($code:tt)*
/// }
/// ```
///
/// This macro supports emulating iterator methods by expanding to equivalent code.
/// They are documented in the [`iterator_dsl`] module,
/// because they are also supported by other `konst::iter` macros.
///
/// # Examples
///
/// ### Custom iterator
///
/// ```rust
/// use konst::iter::{ConstIntoIter, IsIteratorKind};
///
/// struct Upto10(u8);
///
/// impl ConstIntoIter for Upto10 {
///     type Kind = IsIteratorKind;
///     type IntoIter = Self;
///     type Item = u8;
/// }
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
///
/// ### Summing pairs
///
/// ```rust
/// use konst::iter::for_each;
///     
/// const fn add_pairs<const N: usize>(l: [u32; N], r: [u32; N]) -> [u32; N] {
///     let mut out = [0u32; N];
///
///     for_each!{(i, val) in &l,zip(&r),map(|(l, r)| *l + *r),enumerate() =>
///         out[i] = val;
///     }
///
///     out
/// }
///
/// assert_eq!(add_pairs([], []), []);
/// assert_eq!(add_pairs([3], [5]), [8]);
/// assert_eq!(add_pairs([3, 5], [8, 13]), [11, 18]);
///
/// ```
///
/// [`iterator_dsl`]: crate::iter::iterator_dsl
pub use konst_kernel::for_each;

/// Wrapper for `ConstIntoIter` implementors,
/// that defines different methods depending on the
/// value of `K`.
#[doc(inline)]
pub use konst_kernel::into_iter::IntoIterWrapper;

/// Marker type for proving that `T: ConstIntoIter<Kind = K>`
#[doc(inline)]
pub use konst_kernel::into_iter::IsConstIntoIter;

/// Macro for converting [`ConstIntoIter`] implementors into const iterators.
///
/// # Behavior
///
/// For std types (`ConstIntoIter<Kind = IsStdKind>`),
/// this converts those types to their iterator.
/// [(example below)](#std-type-example)
///
/// For user-defined into-iterators (`ConstIntoIter<Kind = IsIntoIterKind>`),
/// this calls their `const_into_iter` inherent method to convert them to an iterator.
/// [(example below)](#into-iter-example)
///
/// For iterators (`ConstIntoIter<Kind = IsIteratorKind>`),
/// this returns the iterator untouched.
/// [(example below)](#iterator-example)
///
/// # Examples
///
/// <span id="std-type-example"></span>
/// ### Std type
///
/// This example demonstrates passing a `ConstIntoIter<Kind = IsStdKind>` in.
///
/// ```rust
/// use konst::{iter, slice};
///
/// let mut elem;
/// let mut iter: slice::Iter<'_, u8> = iter::into_iter!(&[3, 5, 8]);
///
/// (elem, iter) = iter.next().unwrap();
/// assert_eq!(elem, &3);
///
/// (elem, iter) = iter.next().unwrap();
/// assert_eq!(elem, &5);
///
/// (elem, iter) = iter.next().unwrap();
/// assert_eq!(elem, &8);
///
/// assert!(iter.next().is_none());
///
/// ```
///
/// <span id="into-iter-example"></span>
/// ### IntoIterator type
///
/// This example demonstrates passing a `ConstIntoIter<Kind = IsIntoIterKind>` in.
///
/// ```rust
/// use konst::{iter, string};
///
/// let mut iter: Countdown = iter::into_iter!(Number(3));
/// let mut elem;
///
/// (elem, iter) = iter.next().unwrap();
/// assert_eq!(elem, 2);
///
/// (elem, iter) = iter.next().unwrap();
/// assert_eq!(elem, 1);
///
/// (elem, iter) = iter.next().unwrap();
/// assert_eq!(elem, 0);
///
/// assert!(iter.next().is_none());
///
///
/// struct Number(u32);
///
/// impl iter::ConstIntoIter for Number {
///     type Kind = iter::IsIntoIterKind;
///     type Item = u32;
///     type IntoIter = Countdown;
/// }
///
/// impl Number {
///     const fn const_into_iter(self) -> Countdown {
///         Countdown(self.0)
///     }
/// }
///
/// struct Countdown(u32);
///
/// impl iter::ConstIntoIter for Countdown {
///     type Kind = iter::IsIteratorKind;
///     type Item = u32;
///     type IntoIter = Self;
/// }
///
/// impl Countdown {
///     const fn next(self) -> Option<(u32, Self)> {
///         let next = konst::try_opt!(self.0.checked_sub(1));
///         Some((next, Countdown(next)))
///     }
/// }
///
///
/// ```
///
/// <span id="iterator-example"></span>
/// ### Iterator type
///
/// This example demonstrates passing a `ConstIntoIter<Kind = IsIteratorKind>` in.
///
/// ```rust
/// use konst::{iter, string};
///
/// let iter: string::Split<'_, '_, char> = string::split("foo bar baz", ' ');
///
/// // `iter::into_iter` is an identity function when passed iterators
/// let mut iter: string::Split<'_, '_, char> = iter::into_iter!(iter);
/// let mut elem;
///
/// (elem, iter) = iter.next().unwrap();
/// assert_eq!(elem, "foo");
///
/// (elem, iter) = iter.next().unwrap();
/// assert_eq!(elem, "bar");
///
/// (elem, iter) = iter.next().unwrap();
/// assert_eq!(elem, "baz");
///
/// assert!(iter.next().is_none());
/// ```
///
///
#[doc(inline)]
pub use konst_kernel::into_iter_macro as into_iter;

/// Const analog of the [`IntoIterator`] trait.
///
/// # Implementor
///
/// Implementors are expected to be one of these:
///
/// - [`IsIntoIterKind` kind](#isintoiterkind)
/// - [`IsIteratorKind` kind](#isiteratorkind)
/// - Standard library types, of the [`IsStdKind`] kind
///
/// ### `IsIntoIterKind`
///
/// These are user-defined types convertible to const iterators.
///
/// These implement `ConstIntoIter<Kind = `[`IsIntoIterKind`]`>`
/// and are expected to define this inherent method for converting to
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
/// These are const iterator types.
///
/// These implement `ConstIntoIter<Kind = `[`IsIteratorKind`]`>`
/// and are expected to define this inherent method:
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
/// // Reverses the iterator, equivalent to `Iterator::rev`
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
/// Where `SomeIteratorRev` should be a `ConstIntoIter<Kind = IsIteratorKind>`
/// which has the same inherent methods for iteration,
/// and returns the same `Item` type.
///
/// [full example below](#iter-example)
///
/// # Examples
///
/// <span id = "non-iter-example"></span>
/// ### Implementing for an into-iterator
///
/// ```rust
/// use konst::{iter, slice};
///
/// struct GetSlice<'a, T>{
///     slice: &'a [T],
///     up_to: usize,
/// }
///
/// impl<'a, T> iter::ConstIntoIter for GetSlice<'a, T> {
///     type Kind = iter::IsIntoIterKind;
///     type IntoIter = konst::slice::Iter<'a, T>;
///     type Item = &'a T;
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
///     iter::eval!(gs,fold(0, |l, &r| l + r))
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
/// use konst::iter::{self, ConstIntoIter};
///
/// struct Countdown(u8);
///
/// impl ConstIntoIter for Countdown {
///     type Kind = iter::IsIteratorKind;
///     type IntoIter = Self;
///     type Item = u8;
/// }
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
///     iter::eval!(Countdown(initial),fold(0u16, |accum, elem| accum + elem as u16))
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
///     iter::for_each!{(i, hour) in 0..6,zip(hours.copy()) =>
///         arr[i] = hour;
///     }
///     iter::for_each!{(i, hour) in 6..12,zip(hours.rev()) =>
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
/// impl iter::ConstIntoIter for Hours {
///     type Kind = iter::IsIteratorKind;
///     type IntoIter = Self;
///     type Item = u8;
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
/// impl iter::ConstIntoIter for HoursRev {
///     type Kind = iter::IsIteratorKind;
///     type IntoIter = Self;
///     type Item = u8;
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
pub use konst_kernel::into_iter::ConstIntoIter;

#[doc(no_inline)]
pub use crate::polymorphism::kinds::{IsIntoIterKind, IsIteratorKind, IsStdKind};

/// Trait for all the types that can be iterated over with ranges.
///
/// This trait is sealed and can only be implemented by `konst`
pub use konst_kernel::step_kk::Step;

include! {"./iter/collect_const.rs"}
include! {"./iter/iter_eval.rs"}
