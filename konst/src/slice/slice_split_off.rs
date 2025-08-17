#![expect(
    clippy::mem_replace_with_default,
    reason = "Default::default() isn't usable in const"
)]

use crate::range::{self, OneSidedRange, OneSidedRangeBound};

use core::mem;

/// Const equivalent of
/// [`<[T]>::split_off`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_off)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const SLICES: &[&[u8]] = &{
///     let mut slice = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10].as_slice();
///     
///     assert!(slice::split_off(&mut slice, 100..).is_none());
///     
///     [
///         slice::split_off(&mut slice, ..2).unwrap(),
///         slice::split_off(&mut slice, ..=3).unwrap(),
///         slice::split_off(&mut slice, 1..).unwrap(),
///         slice,
///     ]
/// };
///
/// assert_eq!(
///     SLICES,
///     &[
///         &[0, 1][..],
///         &[2, 3, 4, 5],
///         &[7, 8, 9, 10],
///         &[6]
///     ][..]
/// );
///
/// ```
///
pub const fn split_off<'a, T, R>(this: &mut &'a [T], range: R) -> Option<&'a [T]>
where
    R: OneSidedRange<Item = usize>,
{
    let (bound, index) = crate::try_opt!(range::to_bound(range));

    let (before, after) = crate::try_opt!(this.split_at_checked(index));

    let (ret, assign) = match bound {
        OneSidedRangeBound::FromInclusive => (after, before),
        OneSidedRangeBound::ToExclusive => (before, after),
    };

    *this = assign;

    Some(ret)
}

/// Const equivalent of
/// [`<[T]>::split_off_mut`](
/// https://doc.rust-lang.org/std/primitive.slice.html#method.split_off_mut)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const _: () = {
///     let mut slice: &mut [_] = &mut [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
///     
///     assert!(slice::split_off_mut(&mut slice, 100..).is_none());
///     
///     assert!(matches!(slice::split_off_mut(&mut slice, ..2), Some(&mut [0, 1])));
///     assert!(matches!(slice::split_off_mut(&mut slice, ..=3), Some(&mut [2, 3, 4, 5])));
///     assert!(matches!(slice::split_off_mut(&mut slice, 1..), Some(&mut [7, 8, 9, 10])));
///     assert!(matches!(slice, &mut [6]));
/// };
///
/// ```
///
pub const fn split_off_mut<'a, T, R>(this: &mut &'a mut [T], range: R) -> Option<&'a mut [T]>
where
    R: OneSidedRange<Item = usize>,
{
    let (bound, index) = crate::try_opt!(range::to_bound(range));

    // note on modification:
    // make sure that out-of-bounds ranges don't cause this method to
    // both return `None` and `this` to go from having items to being empty
    if index > this.len() {
        return None;
    }

    let (before, after) = mem::replace(this, &mut []).split_at_mut(index);

    let (ret, assign) = match bound {
        OneSidedRangeBound::FromInclusive => (after, before),
        OneSidedRangeBound::ToExclusive => (before, after),
    };

    *this = assign;

    Some(ret)
}

/// Const equivalent of
/// [`<[T]>::split_off_first`](
/// https://doc.rust-lang.org/std/primitive.slice.html#method.split_off_first)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const _: () = {
///     let mut slice: &[_] = &[0, 1, 2, 3];
///     
///     assert!(slice::split_off_first(&mut [0u8; 0].as_slice()).is_none());
///     
///     assert!(matches!(slice::split_off_first(&mut slice), Some(&0)));
///     assert!(matches!(slice::split_off_first(&mut slice), Some(&1)));
///     assert!(matches!(slice, &[2, 3]));
/// };
///
/// ```
///
pub const fn split_off_first<'a, T>(this: &mut &'a [T]) -> Option<&'a T> {
    if let [first, rem @ ..] = *this {
        *this = rem;
        Some(first)
    } else {
        None
    }
}

/// Const equivalent of
/// [`<[T]>::split_off_first_mut`](
/// https://doc.rust-lang.org/std/primitive.slice.html#method.split_off_first_mut)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const _: () = {
///     let mut slice: &mut [_] = &mut [0, 1, 2, 3];
///     
///     assert!(slice::split_off_first_mut(&mut [0u8; 0].as_mut_slice()).is_none());
///     
///     assert!(matches!(slice::split_off_first_mut(&mut slice), Some(&mut 0)));
///     assert!(matches!(slice::split_off_first_mut(&mut slice), Some(&mut 1)));
///     assert!(matches!(slice, &mut [2, 3]));
/// };
///
/// ```
///
pub const fn split_off_first_mut<'a, T>(this: &mut &'a mut [T]) -> Option<&'a mut T> {
    if let [first, rem @ ..] = mem::replace(this, &mut []) {
        *this = rem;
        Some(first)
    } else {
        None
    }
}

/// Const equivalent of
/// [`<[T]>::split_off_last`](
/// https://doc.rust-lang.org/std/primitive.slice.html#method.split_off_last)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const _: () = {
///     let mut slice: &[_] = &[0, 1, 2, 3];
///     
///     assert!(slice::split_off_last(&mut [0u8; 0].as_slice()).is_none());
///     
///     assert!(matches!(slice::split_off_last(&mut slice), Some(&3)));
///     assert!(matches!(slice::split_off_last(&mut slice), Some(&2)));
///     assert!(matches!(slice, &[0, 1]));
/// };
///
/// ```
///
pub const fn split_off_last<'a, T>(this: &mut &'a [T]) -> Option<&'a T> {
    if let [rem @ .., last] = *this {
        *this = rem;
        Some(last)
    } else {
        None
    }
}

/// Const equivalent of
/// [`<[T]>::split_off_last_mut`](
/// https://doc.rust-lang.org/std/primitive.slice.html#method.split_off_last_mut)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const _: () = {
///     let mut slice: &mut [_] = &mut [0, 1, 2, 3];
///     
///     assert!(slice::split_off_last_mut(&mut [0u8; 0].as_mut_slice()).is_none());
///     
///     assert!(matches!(slice::split_off_last_mut(&mut slice), Some(&mut 3)));
///     assert!(matches!(slice::split_off_last_mut(&mut slice), Some(&mut 2)));
///     assert!(matches!(slice, &mut [0, 1]));
/// };
///
/// ```
///
pub const fn split_off_last_mut<'a, T>(this: &mut &'a mut [T]) -> Option<&'a mut T> {
    if let [rem @ .., last] = mem::replace(this, &mut []) {
        *this = rem;
        Some(last)
    } else {
        None
    }
}
