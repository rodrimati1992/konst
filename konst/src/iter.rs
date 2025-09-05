//! Const equivalent of iterators with a specific `next` function signature.
//!
//! The docs for [`ConstIntoIter`] has more information on
//! const equivalents of [`IntoIterator`] and [`Iterator`].
//!
//! [`IntoIterator`]: core::iter::IntoIterator
//! [`Iterator`]: core::iter::Iterator

use core::marker::PhantomData;
use core::mem::ManuallyDrop;

use typewit::TypeEq;

#[macro_use]
mod internal_iter_macros;

#[doc(no_inline)]
pub use crate::polymorphism::kinds::{IsIntoIterKind, IsIteratorKind, IsStdKind};

#[doc(hidden)]
pub mod collect_const;
mod for_each_macro;
mod iter_eval_macro;
mod iterator_adaptors;
pub mod iterator_dsl;

pub(crate) mod step;

#[doc(inline)]
pub use for_each_macro::for_each;

#[doc(inline)]
pub use collect_const::collect_const;

#[doc(inline)]
pub use self::iter_eval_macro::eval;

#[doc(hidden)]
pub use self::internal_iter_macros::{
    __assert_item_ty, __get_item_ty, __infer_option_ty, __items_needs_drop,
};

pub use iterator_adaptors::*;

#[doc(hidden)]
pub use self::iter_eval_macro::{__eval2_lowering, iter_eval_helpers::__StepByVars};

pub use self::step::Step;

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
/// ### Dropping
///
/// Because dropping in const is not supported, the `konst::iter` macros:
/// - drop the remaining items for iterators that impl `ConstIntoIter<ITEMS_NEED_DROP = true>`
/// - `mem::forget` the iterator
///
/// because of this,
/// iterators cannot manage heap allocations or other resources in their destructors.
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
/// const fn next(&mut self) -> Option<Item> {
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
/// const fn next_back(&mut self) -> Option<Item> {
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
///     const ITEMS_NEED_DROP: bool = false;
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
///     const ITEMS_NEED_DROP: bool = false;
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
///     const ITEMS_NEED_DROP: bool = false;
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
///     const ITEMS_NEED_DROP: bool = false;
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
    type Kind: ConstIntoIterKind;

    /// The item that `Self::IntoIter` yields on `.next()`
    type Item;

    /// The iterator that this can be converted into.
    type IntoIter: ConstIntoIter<Item = Self::Item>;

    /// Whether the iterator items need to be dropped.
    ///
    /// Define this as:
    /// - `false`: if the items don't need dropping
    /// - `true`: if the items do need dropping
    /// - `core::mem::needs_drop::<Self::Item>()`: if the you're not sure
    const ITEMS_NEED_DROP: bool;
}

/// Wrapper for `ConstIntoIter` implementors,
/// that defines different methods depending on the
/// value of `K`.
///
/// You're not intended to use this directly, the intended way to convert a type
/// into a const iterator is with the [`into_iter`] macro.
#[repr(transparent)]
pub struct IntoIterWrapper<I, K> {
    /// The value to be converted into an iterator
    pub(crate) iter: ManuallyDrop<I>,
    _phantom: PhantomData<K>,
}

////////////////////////////////////////////////////////////////////////////////

/// Coerces the argument to a type that has a `.const_into_iter()` method
///
/// There's 3 different ways to use this function
/// - [passing std types](#std-example): which wraps them in
///   [`IntoIterWrapper<T, IsStdKind>`](crate::iter::IntoIterWrapper)
/// - [passing non-iterator user-defined types](#user-example): returning the argument back
/// - [passing iterator types](#iterator-example): which wraps them in
///   [`IntoIterWrapper<T, IsIteratorKind>`](crate::iter::IntoIterWrapper)
///
/// Note: `konst::iter::coerce(value).const_into_iter()` is equivalent to
/// `konst::iter::into_iter!(value)`
///
///
/// # Examples
///
/// <span id="std-example"></span>
/// ### Std argument
///
/// ```rust
/// use konst::{
///     iter::{self, IsStdKind, IntoIterWrapper},
///     range::RangeIter
/// };
/// use std::ops::Range;
///
/// // Coerce wraps `ConstIntoIter<Kind = IsStdKind>` types in an `IntoIterWrapper`
/// let wrapper: IntoIterWrapper<Range<u32>, IsStdKind> = iter::coerce(0..3);
///
/// // ... which then converts the std type into an iterator with `.const_into_iter()`
/// let mut iter: RangeIter<u32> = wrapper.const_into_iter();
///
/// assert_eq!(iter.next(), Some(0));
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), None);
///
/// ```
///
/// <span id="user-example"></span>
/// ### User-defined argument
///
/// ```rust
/// use konst::{
///     iter::{self, ConstIntoIter, IsIntoIterKind},
///     range::RangeInclusiveIter as RangeIncIter,
/// };
///
/// struct Double(u32);
///
/// impl ConstIntoIter for Double {
///     type Kind = IsIntoIterKind;
///     type Item = u32;
///     type IntoIter = RangeIncIter<u32>;
///     
///     // items are Copy, so they don't need to be dropped
///     const ITEMS_NEED_DROP: bool = false;
/// }
///
/// impl Double {
///     pub const fn const_into_iter(self) -> RangeIncIter<u32> {
///         iter::into_iter!(0 ..= self.0 * 2)
///     }
/// }
///
/// // `coerce` is an identity function for `ConstIntoIter<Kind = IsStdKind>` arguments
/// let double: Double = iter::coerce(Double(2));
///
/// // ... then it's converted into an iterator with `.const_into_iter()`
/// let mut iter: RangeIncIter<u32> = double.const_into_iter();
///
/// assert_eq!(iter.next(), Some(0));
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), Some(4));
/// assert_eq!(iter.next(), None);
///
/// ```
///
/// <span id="iterator-example"></span>
/// ### Iterator argument
///
/// ```rust
/// use konst::{
///     iter::{self, ConstIntoIter, IsIteratorKind, IntoIterWrapper},
///     range::RangeFromIter,
/// };
///
/// let mut iter: RangeFromIter<u32> = iter::into_iter!(3..);
///
/// // Coerce wraps `ConstIntoIter<Kind = IsIteratorKind>` types in a `IntoIterWrapper`
/// let wrapper: IntoIterWrapper<RangeFromIter<u32>, IsIteratorKind> = iter::coerce(iter);
///
/// // ... which then unwraps back into the iterator with `.const_into_iter()`
/// let mut iter: RangeFromIter<u32> = wrapper.const_into_iter();
///
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), Some(4));
/// assert_eq!(iter.next(), Some(5));
/// assert_eq!(iter.next(), Some(6));
/// ```
///
#[inline(always)]
pub const fn coerce<T>(into_iter: T) -> CoerceTo<T>
where
    T: ConstIntoIter,
{
    match const { <T::Kind as ConstIntoIterKind>::__KIND_WITNESS.to_coercion_witness::<T>() } {
        __CoercionWitness::Unwrapped(te) => te.to_left(into_iter),
        __CoercionWitness::Wrapped(te) => te.to_left(IntoIterWrapper {
            iter: ManuallyDrop::new(into_iter),
            _phantom: PhantomData,
        }),
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<T> IntoIterWrapper<T, IsIteratorKind> {
    /// Converts `T` into a const iterator.
    /// Since `T` is already an iterator, this does nothing.
    pub const fn const_into_iter(self) -> T
    where
        T: ConstIntoIter<Kind = IsIteratorKind, IntoIter = T>,
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
    const ITEMS_NEED_DROP: bool = false;
}

/////////////////////////////////////////////////////////////////////////////

/// Trait for the types that are assignable to [`ConstIntoIter::Kind`]
///
pub trait ConstIntoIterKind: Sized {
    /// Computes the type returned by [`coerce`]:
    /// - if `Self == `[`IsStdKind`]:
    ///   this evaluates to `IntoIterWrapper<T, IsStdKind>`.
    /// - if `Self == `[`IsIntoIterKind`]:
    ///   this evaluates to `T`.
    /// - if `Self == `[`IsIteratorKind`]:
    ///   this evaluates to `IntoIterWrapper<T, IsIteratorKind>`.
    type CoerceTo<T>;

    #[doc(hidden)]
    const __KIND_WITNESS: __ConstIntoIterKindWitness<Self>;
}

impl ConstIntoIterKind for IsStdKind {
    type CoerceTo<T> = IntoIterWrapper<T, Self>;

    #[doc(hidden)]
    const __KIND_WITNESS: __ConstIntoIterKindWitness<Self> =
        __ConstIntoIterKindWitness::IsStdKind(TypeEq::NEW);
}

impl ConstIntoIterKind for IsIntoIterKind {
    type CoerceTo<T> = T;

    #[doc(hidden)]
    const __KIND_WITNESS: __ConstIntoIterKindWitness<Self> =
        __ConstIntoIterKindWitness::IsIntoIterKind(TypeEq::NEW);
}

impl ConstIntoIterKind for IsIteratorKind {
    type CoerceTo<T> = IntoIterWrapper<T, Self>;

    #[doc(hidden)]
    const __KIND_WITNESS: __ConstIntoIterKindWitness<Self> =
        __ConstIntoIterKindWitness::IsIteratorKind(TypeEq::NEW);
}

/// Computes the type returned by [`coerce`]:
/// - if [`T::Kind`](ConstIntoIter::Kind)` == `[`IsStdKind`]:
///   this evaluates to `IntoIterWrapper<T, IsStdKind>`.
/// - if [`T::Kind`](ConstIntoIter::Kind)` == `[`IsIntoIterKind`]:
///   this evaluates to `T`.
/// - if [`T::Kind`](ConstIntoIter::Kind)` == `[`IsIteratorKind`]:
///   this evaluates to `IntoIterWrapper<T, IsIteratorKind>`.
pub type CoerceTo<T> = <<T as ConstIntoIter>::Kind as ConstIntoIterKind>::CoerceTo<T>;

#[doc(hidden)]
pub enum __ConstIntoIterKindWitness<Kind> {
    IsStdKind(TypeEq<Kind, IsStdKind>),
    IsIntoIterKind(TypeEq<Kind, IsIntoIterKind>),
    IsIteratorKind(TypeEq<Kind, IsIteratorKind>),
}

impl<Kind> Copy for __ConstIntoIterKindWitness<Kind> {}

impl<Kind> Clone for __ConstIntoIterKindWitness<Kind> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<Kind> __ConstIntoIterKindWitness<Kind> {
    const fn to_coercion_witness<T>(self) -> __CoercionWitness<T>
    where
        T: ConstIntoIter<Kind = Kind>,
        Kind: ConstIntoIterKind,
    {
        typewit::type_fn! {
            struct CoerceToFn<T>;
            impl<K: ConstIntoIterKind> K => <K as ConstIntoIterKind>::CoerceTo<T>
        }
        typewit::type_fn! {
            struct IntoIterWrapperFn<T>;
            impl<K: ConstIntoIterKind> K => IntoIterWrapper<T, K>
        }

        match self {
            Self::IsStdKind(te) => __CoercionWitness::Wrapped(
                te.project::<CoerceToFn<T>>()
                    .join(te.project::<IntoIterWrapperFn<T>>().flip()),
            ),
            Self::IsIntoIterKind(te) => __CoercionWitness::Unwrapped(te.project::<CoerceToFn<T>>()),
            Self::IsIteratorKind(te) => __CoercionWitness::Wrapped(
                te.project::<CoerceToFn<T>>()
                    .join(te.project::<IntoIterWrapperFn<T>>().flip()),
            ),
        }
    }
}

#[doc(hidden)]
enum __CoercionWitness<T: ConstIntoIter> {
    Wrapped(TypeEq<CoerceTo<T>, IntoIterWrapper<T, T::Kind>>),
    Unwrapped(TypeEq<CoerceTo<T>, T>),
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
///     const ITEMS_NEED_DROP: bool = false;
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
///     const ITEMS_NEED_DROP: bool = false;
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
        $crate::iter::coerce($iter).const_into_iter()
    };
}
