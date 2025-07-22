//! Const equivalent of iterators with a specific `next` function signature.
//!
//! The docs for [`ConstIntoIter`] has more information on
//! const equivalents of IntoIterator and Iterator.
//!

#[macro_use]
mod internal_iter_macros;

use core::mem::ManuallyDrop;

#[doc(no_inline)]
pub use crate::polymorphism::kinds::{IsIntoIterKind, IsIteratorKind, IsStdKind};

#[doc(hidden)]
pub mod collect_const;
mod for_each_macro;
mod iterator_adaptors;
pub mod iterator_dsl;

mod combinator_methods;

pub(crate) mod step;

#[doc(inline)]
pub use for_each_macro::for_each;

#[doc(inline)]
pub use collect_const::collect_const;

#[doc(inline)]
pub use iterator_adaptors::*;

#[doc(hidden)]
pub use self::internal_iter_macros::{
    __assert_item_ty, __cim_method_not_found_err, __cim_preprocess_methods, __get_item_ty,
};

pub use self::step::Step;

include! {"./iter/iter_eval_macro.rs"}

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
/// const fn next(self) -> Option<Item> {
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
/// const fn next_back(self) -> Option<Item> {
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
///     const fn next(&mut self) -> Option<u8> {
///         konst::option::map!(self.0.checked_sub(1), |ret| {
///             self.0 = ret;
///             ret
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
///     const fn next(&mut self) -> Option<u8> {
///         if self.start == self.end {
///             None
///         } else {
///             let ret = self.start;
///             self.start += 1;
///             Some(ret)
///         }
///     }
///
///     const fn next_back(&mut self) -> Option<u8> {
///         if self.start == self.end {
///             None
///         } else {
///             self.end -= 1;
///             Some(self.end)
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
///         Self {..*self}
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
///     const fn next(&mut self) -> Option<u8> {
///         self.0.next_back()
///     }
///
///     const fn next_back(&mut self) -> Option<u8> {
///         self.0.next()
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
pub trait ConstIntoIter {
    /// What kind of type this is:
    /// - [`IsIntoIterKind`]: user-defined types that are convertible to const iterators
    /// - [`IsIteratorKind`]: const iterators
    /// - [`IsStdKind`]: standard library types that are convertible to const iterators
    type Kind;

    /// The item that `Self::IntoIter` yields on `.next()`
    type Item;

    /// The iterator that this can be converted into.
    type IntoIter: ConstIntoIter<Item = Self::Item>;
}

/// Wrapper for `ConstIntoIter` implementors,
/// that defines different methods depending on the
/// value of `K`.
#[repr(transparent)]
pub struct IntoIterWrapper<I, K>
where
    I: ConstIntoIter<Kind = K>,
{
    /// The value to be converted into an iterator
    pub iter: ManuallyDrop<I>,
}

impl<T> IntoIterWrapper<T, IsStdKind>
where
    T: ConstIntoIter<Kind = IsStdKind>,
{
    /// Performs the coercion to a type that has a `const_into_iter` method
    #[inline(always)]
    pub const fn coerce(self) -> Self {
        self
    }
}

impl<T> IntoIterWrapper<T, IsIntoIterKind>
where
    T: ConstIntoIter<Kind = IsIntoIterKind>,
{
    /// Performs the coercion to a type that has a `const_into_iter` method
    #[inline(always)]
    pub const fn coerce(self) -> T {
        ManuallyDrop::into_inner(self.iter)
    }
}

impl<T> IntoIterWrapper<T, IsIteratorKind>
where
    T: ConstIntoIter<Kind = IsIteratorKind>,
{
    /// Performs the coercion to a type that has a `const_into_iter` method
    #[inline(always)]
    pub const fn coerce(self) -> Self {
        self
    }

    /// Converts `T` into a const iterator.
    /// Since `T` is already an iterator, this does nothing.
    #[inline(always)]
    pub const fn const_into_iter(self) -> T
    where
        T: ConstIntoIter<IntoIter = T>,
    {
        ManuallyDrop::into_inner(self.iter)
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<'a, T> ConstIntoIter for &'a mut T
where
    T: ConstIntoIter<IntoIter = T, Kind = IsIteratorKind>,
{
    type Kind = IsIteratorKind;
    type Item = <T as ConstIntoIter>::Item;
    type IntoIter = &'a mut T;
}

////////////////////////////////////////////////////////////////////////////////

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
/// let mut iter: slice::Iter<'_, u8> = iter::into_iter!(&[3, 5, 8]);
///
/// assert_eq!(iter.next().unwrap(), &3);
/// assert_eq!(iter.next().unwrap(), &5);
/// assert_eq!(iter.next().unwrap(), &8);
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
///
/// assert_eq!(iter.next().unwrap(), 2);
/// assert_eq!(iter.next().unwrap(), 1);
/// assert_eq!(iter.next().unwrap(), 0);
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
///     const fn next(&mut self) -> Option<u32> {
///         let next = konst::try_opt!(self.0.checked_sub(1));
///         self.0 = next;
///         Some(next)
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
///
/// assert_eq!(iter.next().unwrap(), "foo");
/// assert_eq!(iter.next().unwrap(), "bar");
/// assert_eq!(iter.next().unwrap(), "baz");
///
/// assert!(iter.next().is_none());
/// ```
///
/// <span id="iterator-example"></span>
/// ### By-ref iterator
///
/// This example demonstrates how passing mutable references to const iterators works.
///
/// ```rust
/// use konst::{iter, string};
///
/// let mut split: string::Split<'_, '_, char> = string::split("foo bar baz", ' ');
///
/// // `iter::into_iter` is an identity function when passed mutable references to iterators
/// let mut iter: &mut string::Split<'_, '_, char> = iter::into_iter!(&mut split);
///
/// assert_eq!(iter.next().unwrap(), "foo");
/// assert_eq!(iter.next().unwrap(), "bar");
///
/// assert_eq!(split.next().unwrap(), "baz");
/// assert!(split.next().is_none());
/// ```
///
///
#[doc(inline)]
pub use crate::__into_iter as into_iter;

#[doc(hidden)]
#[macro_export]
macro_rules! __into_iter {
    ($iter:expr) => {
        $crate::iter::IntoIterWrapper {
            iter: $crate::__::ManuallyDrop::new($iter),
        }
        .coerce()
        .const_into_iter()
    };
}
