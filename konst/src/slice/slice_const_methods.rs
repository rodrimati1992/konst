#[cfg(feature = "constant_time_slice")]
use crate::utils::{min_usize, Dereference};

macro_rules! slice_from_impl {
    ($slice:ident, $start:ident, $as_ptr:ident, $from_raw_parts:ident, $on_overflow:expr) => {{
        #[allow(unused_variables)]
        let (rem, overflowed) = $slice.len().overflowing_sub($start);

        if overflowed {
            return $on_overflow;
        }

        #[cfg(feature = "constant_time_slice")]
        {
            unsafe { crate::utils::$from_raw_parts($slice.$as_ptr().offset($start as _), rem) }
        }
        #[cfg(not(feature = "constant_time_slice"))]
        {
            let mut ret = $slice;
            let mut to_remove = $start;

            slice_up_to_linear_time_impl! {
                ret, to_remove, next,
                () (next @ ..),
            }
            ret
        }
    }};
}

macro_rules! slice_up_to_impl {
    ($slice:ident, $len:ident, $as_ptr:ident, $from_raw_parts:ident, $on_overflow:expr) => {{
        #[allow(unused_variables)]
        let (rem, overflowed) = $slice.len().overflowing_sub($len);

        if overflowed {
            return $on_overflow;
        }

        #[cfg(feature = "rust_1_56")]
        {
            // Doing this to get a slice up to length at compile-time
            unsafe { crate::utils::$from_raw_parts($slice.$as_ptr(), $len) }
        }
        #[cfg(not(feature = "rust_1_56"))]
        {
            let mut ret = $slice;
            let mut to_remove = rem;

            slice_up_to_linear_time_impl! {
                ret, to_remove, next,
                (next @ ..,) (),
            }
            ret
        }
    }};
}

macro_rules! slice_up_to_linear_time_impl{
    (
        $($args:tt)*
    )=>({
        slice_up_to_impl_inner!{
            $($args)*
            (64, [
                _, _, _, _, _, _,_, _, _, _, _, _,_, _, _, _,
                _, _, _, _, _, _, _, _,_, _, _, _, _, _,_, _,
                _, _, _, _, _, _,_, _, _, _, _, _,_, _, _, _,
                _, _, _, _, _, _, _, _,_, _, _, _, _, _,_, _,
            ])
        }
        slice_up_to_impl_inner!{
            $($args)*
            (8, [_, _, _, _, _, _, _, _,])
        }
        slice_up_to_impl_inner!{
            $($args)*
            (1, [_,])
        }
    });
}
macro_rules! slice_up_to_impl_inner{
    (
        $ret:ident, $to_remove:ident, $next:ident,
        ($($before_ignored:tt)*) ($($after_ignored:tt)*),
        ($ignored_len:expr, [$($ignored:tt)*])
    )=>{
        while $to_remove >= $ignored_len {
            if let [
                $($before_ignored)*
                $($ignored)*
                $($after_ignored)*
            ] = $ret {
                $ret = $next;
            }
            $to_remove -= $ignored_len;
        }
    }
}

/// A const equivalent of `slice.get(index)`
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const FIBB: &[u16] = &[3, 5, 8];
///
/// const ELEM0: Option<&u16> = slice::get(FIBB, 0);
/// const ELEM1: Option<&u16> = slice::get(FIBB, 1);
/// const ELEM2: Option<&u16> = slice::get(FIBB, 2);
/// const ELEM3: Option<&u16> = slice::get(FIBB, 3);
///
/// assert_eq!(ELEM0, Some(&3));
/// assert_eq!(ELEM1, Some(&5));
/// assert_eq!(ELEM2, Some(&8));
/// assert_eq!(ELEM3, None);
///
/// ```
#[inline]
pub const fn get<T>(slice: &[T], index: usize) -> Option<&T> {
    if slice.len() > index {
        Some(&slice[index])
    } else {
        None
    }
}

/// A const equivalent of `slice.get_mut(index)`
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// let mut fibb = [3, 5, 8];
///
/// assert_eq!(slice::get_mut(&mut fibb, 0), Some(&mut 3));
/// assert_eq!(slice::get_mut(&mut fibb, 1), Some(&mut 5));
/// assert_eq!(slice::get_mut(&mut fibb, 2), Some(&mut 8));
/// assert_eq!(slice::get_mut(&mut fibb, 3), None);
///
/// ```
#[inline]
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
pub const fn get_mut<T>(slice: &mut [T], index: usize) -> Option<&mut T> {
    if slice.len() > index {
        Some(&mut slice[index])
    } else {
        None
    }
}

/// A const equivalent of `&slice[start..]`.
///
/// If `slice.len() < start`, this simply returns an empty slice.
///
/// # Performance
///
/// If the "constant_time_slice" feature is disabled,
/// thich takes linear time to remove the leading elements,
/// proportional to `start`.
///
/// If the "constant_time_slice" feature is enabled, it takes constant time to run,
/// but uses a few nightly features.
///
/// # Example
///
/// ```rust
/// use konst::slice::slice_from;
///
/// const FIBB: &[u16] = &[3, 5, 8, 13, 21, 34, 55, 89];
///
/// const TWO: &[u16] = slice_from(FIBB, 2);
/// const FOUR: &[u16] = slice_from(FIBB, 4);
/// const ALL: &[u16] = slice_from(FIBB, 0);
/// const NONE: &[u16] = slice_from(FIBB, 1000);
///
/// assert_eq!(TWO, &[8, 13, 21, 34, 55, 89]);
/// assert_eq!(FOUR, &[21, 34, 55, 89]);
/// assert_eq!(ALL, FIBB);
/// assert_eq!(NONE, &[]);
///
/// ```
#[inline]
pub const fn slice_from<T>(slice: &[T], start: usize) -> &[T] {
    slice_from_impl!(slice, start, as_ptr, slice_from_raw_parts, &[])
}

/// A const equivalent of `slice.get(start..)`.
///
/// # Performance
///
/// If the "constant_time_slice" feature is disabled,
/// thich takes linear time to remove the leading elements,
/// proportional to `start`.
///
/// If the "constant_time_slice" feature is enabled, it takes constant time to run,
/// but uses a few nightly features.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const FIBB: &[u16] = &[3, 5, 8, 13, 21, 34, 55, 89];
///
/// const TWO: Option<&[u16]> = slice::get_from(FIBB, 2);
/// const FOUR: Option<&[u16]> = slice::get_from(FIBB, 4);
/// const ALL: Option<&[u16]> = slice::get_from(FIBB, 0);
/// const NONE: Option<&[u16]> = slice::get_from(FIBB, 1000);
///
/// assert_eq!(TWO, Some(&[8, 13, 21, 34, 55, 89][..]));
/// assert_eq!(FOUR, Some(&[21, 34, 55, 89][..]));
/// assert_eq!(ALL, Some(FIBB));
/// assert_eq!(NONE, None);
///
/// ```
#[inline]
pub const fn get_from<T>(slice: &[T], start: usize) -> Option<&[T]> {
    Some(slice_from_impl!(
        slice,
        start,
        as_ptr,
        slice_from_raw_parts,
        None
    ))
}

/// A const equivalent of `&mut slice[start..]`.
///
/// If `slice.len() < start`, this simply returns an empty slice.
///
/// # Performance
///
/// If the "constant_time_slice" feature is disabled,
/// thich takes linear time to remove the leading elements,
/// proportional to `start`.
///
/// If the "constant_time_slice" feature is enabled, it takes constant time to run,
/// but uses a few nightly features.
///
/// # Example
///
/// ```rust
/// use konst::slice::slice_from_mut;
///
/// let mut fibs = [3, 5, 8, 13, 21, 34, 55, 89];
///
/// assert_eq!(slice_from_mut(&mut fibs, 0), &mut [3, 5, 8, 13, 21, 34, 55, 89]);
/// assert_eq!(slice_from_mut(&mut fibs, 1), &mut [5, 8, 13, 21, 34, 55, 89]);
/// assert_eq!(slice_from_mut(&mut fibs, 2), &mut [8, 13, 21, 34, 55, 89]);
/// assert_eq!(slice_from_mut(&mut fibs, 6), &mut [55, 89]);
/// assert_eq!(slice_from_mut(&mut fibs, 7), &mut [89]);
/// assert_eq!(slice_from_mut(&mut fibs, 8), &mut []);
/// assert_eq!(slice_from_mut(&mut fibs, 1000), &mut []);
///
///
/// ```
#[inline]
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
pub const fn slice_from_mut<T>(slice: &mut [T], start: usize) -> &mut [T] {
    slice_from_impl!(slice, start, as_mut_ptr, slice_from_raw_parts_mut, &mut [])
}

/// A const equivalent of `slice.get_mut(start..)`.
///
/// # Performance
///
/// If the "constant_time_slice" feature is disabled,
/// thich takes linear time to remove the leading elements,
/// proportional to `start`.
///
/// If the "constant_time_slice" feature is enabled, it takes constant time to run,
/// but uses a few nightly features.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// let mut fibs = [3, 5, 8, 13, 21, 34, 55];
///
/// assert_eq!(slice::get_from_mut(&mut fibs, 0), Some(&mut [3, 5, 8, 13, 21, 34, 55][..]));
/// assert_eq!(slice::get_from_mut(&mut fibs, 1), Some(&mut [5, 8, 13, 21, 34, 55][..]));
/// assert_eq!(slice::get_from_mut(&mut fibs, 2), Some(&mut [8, 13, 21, 34, 55][..]));
/// assert_eq!(slice::get_from_mut(&mut fibs, 6), Some(&mut [55][..]));
/// assert_eq!(slice::get_from_mut(&mut fibs, 7), Some(&mut [][..]));
/// assert_eq!(slice::get_from_mut(&mut fibs, 8), None);
/// assert_eq!(slice::get_from_mut(&mut fibs, 100), None);
///
///
/// ```
#[inline]
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
pub const fn get_from_mut<T>(slice: &mut [T], start: usize) -> Option<&mut [T]> {
    Some(slice_from_impl!(
        slice,
        start,
        as_mut_ptr,
        slice_from_raw_parts_mut,
        None
    ))
}

/// A const equivalent of `&slice[..len]`.
///
/// If `slice.len() < len`, this simply returns `slice` back.
///
/// # Performance
///
/// If the "rust_1_56" feature is disabled,
/// thich takes linear time to remove the trailing elements,
/// proportional to `slice.len() - len`.
///
/// If the "rust_1_56" feature is enabled, it takes constant time to run,
/// but requires Rust 1.56.0 .
///
/// # Example
///
/// ```rust
/// use konst::slice::slice_up_to;
///
/// const FIBB: &[u16] = &[3, 5, 8, 13, 21, 34, 55, 89];
///
/// const TWO: &[u16] = slice_up_to(FIBB, 2);
/// const FOUR: &[u16] = slice_up_to(FIBB, 4);
/// const NONE: &[u16] = slice_up_to(FIBB, 0);
/// const ALL: &[u16] = slice_up_to(FIBB, 1000);
///
/// assert_eq!(TWO, &[3, 5]);
/// assert_eq!(FOUR, &[3, 5, 8, 13]);
/// assert_eq!(NONE, &[]);
/// assert_eq!(ALL, FIBB);
///
/// ```
#[inline]
pub const fn slice_up_to<T>(slice: &[T], len: usize) -> &[T] {
    slice_up_to_impl!(slice, len, as_ptr, slice_from_raw_parts, slice)
}

/// A const equivalent of `slice.get(..len)`.
///
/// # Performance
///
/// If the "rust_1_56" feature is disabled,
/// thich takes linear time to remove the trailing elements,
/// proportional to `slice.len() - len`.
///
/// If the "rust_1_56" feature is enabled, it takes constant time to run,
/// but requires Rust 1.56.0 .
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const FIBB: &[u16] = &[3, 5, 8, 13, 21, 34, 55, 89];
///
/// const TWO: Option<&[u16]> = slice::get_up_to(FIBB, 2);
/// const FOUR: Option<&[u16]> = slice::get_up_to(FIBB, 4);
/// const NONE: Option<&[u16]> = slice::get_up_to(FIBB, 0);
/// const ALL: Option<&[u16]> = slice::get_up_to(FIBB, 1000);
///
/// assert_eq!(TWO, Some(&[3, 5][..]));
/// assert_eq!(FOUR, Some(&[3, 5, 8, 13][..]));
/// assert_eq!(NONE, Some(&[][..]));
/// assert_eq!(ALL, None);
///
/// ```
#[inline]
pub const fn get_up_to<T>(slice: &[T], len: usize) -> Option<&[T]> {
    Some(slice_up_to_impl!(
        slice,
        len,
        as_ptr,
        slice_from_raw_parts,
        None
    ))
}

/// A const equivalent of `&mut slice[..len]`.
///
/// If `slice.len() < len`, this simply returns `slice` back.
///
/// # Performance
///
/// This takes constant time to run.
///
/// # Example
///
/// ```rust
/// use konst::slice::slice_up_to_mut;
///
/// let mut fibs = [3, 5, 8, 13, 21, 34, 55, 89];
///
/// assert_eq!(slice_up_to_mut(&mut fibs, 100), &mut [3, 5, 8, 13, 21, 34, 55, 89]);
/// assert_eq!(slice_up_to_mut(&mut fibs, 8), &mut [3, 5, 8, 13, 21, 34, 55, 89]);
/// assert_eq!(slice_up_to_mut(&mut fibs, 7), &mut [3, 5, 8, 13, 21, 34, 55]);
/// assert_eq!(slice_up_to_mut(&mut fibs, 6), &mut [3, 5, 8, 13, 21, 34]);
/// assert_eq!(slice_up_to_mut(&mut fibs, 3), &mut [3, 5, 8]);
/// assert_eq!(slice_up_to_mut(&mut fibs, 2), &mut [3, 5]);
/// assert_eq!(slice_up_to_mut(&mut fibs, 1), &mut [3]);
/// assert_eq!(slice_up_to_mut(&mut fibs, 0), &mut []);
///
///
/// ```
#[inline]
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
pub const fn slice_up_to_mut<T>(slice: &mut [T], len: usize) -> &mut [T] {
    slice_up_to_impl!(slice, len, as_mut_ptr, slice_from_raw_parts_mut, slice)
}

/// A const equivalent of `slice.get_mut(..len)`.
///
/// # Performance
///
/// This takes constant time to run.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// let mut fibs = [3, 5, 8, 13, 21, 34, 55, 89];
///
/// assert_eq!(slice::get_up_to_mut(&mut fibs, 100), None);
/// assert_eq!(slice::get_up_to_mut(&mut fibs, 9), None);
/// assert_eq!(slice::get_up_to_mut(&mut fibs, 8), Some(&mut [3, 5, 8, 13, 21, 34, 55, 89][..]));
/// assert_eq!(slice::get_up_to_mut(&mut fibs, 7), Some(&mut [3, 5, 8, 13, 21, 34, 55][..]));
/// assert_eq!(slice::get_up_to_mut(&mut fibs, 6), Some(&mut [3, 5, 8, 13, 21, 34][..]));
/// assert_eq!(slice::get_up_to_mut(&mut fibs, 3), Some(&mut [3, 5, 8][..]));
/// assert_eq!(slice::get_up_to_mut(&mut fibs, 2), Some(&mut [3, 5][..]));
/// assert_eq!(slice::get_up_to_mut(&mut fibs, 1), Some(&mut [3][..]));
/// assert_eq!(slice::get_up_to_mut(&mut fibs, 0), Some(&mut [][..]));
///
/// ```
#[inline]
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
#[inline]
pub const fn get_up_to_mut<T>(slice: &mut [T], len: usize) -> Option<&mut [T]> {
    Some(slice_up_to_impl!(
        slice,
        len,
        as_mut_ptr,
        slice_from_raw_parts_mut,
        None
    ))
}

/// A const equivalent of `&slice[start..end]`.
///
/// If `start >= end ` or `slice.len() < start `, this returns an empty slice.
///
/// If `slice.len() < end`, this returns the slice from `start`.
///
/// # Alternatives
///
/// For a const equivalent of `&slice[start..]` there's [`slice_from`].
///
/// For a const equivalent of `&slice[..end]` there's [`slice_up_to`].
///
/// [`slice_from`]: ./fn.slice_from.html
/// [`slice_up_to`]: ./fn.slice_up_to.html
///
/// # Performance
///
/// If the "constant_time_slice" feature is disabled,
/// thich takes linear time to remove the leading and trailing elements,
/// proportional to `start + (slice.len() - end)`.
///
/// If the "constant_time_slice" feature is enabled, it takes constant time to run,
/// but uses a few nightly features.
///
/// # Example
///
/// ```rust
/// use konst::slice::slice_range;
///
/// const FIBB: &[u16] = &[3, 5, 8, 13, 21, 34, 55, 89];
///
/// const TWO: &[u16] = slice_range(FIBB, 2, 4);
/// const FOUR: &[u16] = slice_range(FIBB, 4, 7);
/// const NONE: &[u16] = slice_range(FIBB, 0, 0);
/// const ALL: &[u16] = slice_range(FIBB, 0, 1000);
///
/// assert_eq!(TWO, &[8, 13]);
/// assert_eq!(FOUR, &[21, 34, 55]);
/// assert_eq!(NONE, &[]);
/// assert_eq!(ALL, FIBB);
///
/// ```
pub const fn slice_range<T>(slice: &[T], start: usize, end: usize) -> &[T] {
    slice_from(slice_up_to(slice, end), start)
}

/// A const equivalent of `slice.get(start..end)`.
///
/// # Alternatives
///
/// For a const equivalent of `slice.get(start..)` there's [`get_from`].
///
/// For a const equivalent of `slice.get(..end)` there's [`get_up_to`].
///
/// [`get_from`]: ./fn.get_from.html
/// [`get_up_to`]: ./fn.get_up_to.html
///
/// # Performance
///
/// If the "constant_time_slice" feature is disabled,
/// thich takes linear time to remove the leading and trailing elements,
/// proportional to `start + (slice.len() - end)`.
///
/// If the "constant_time_slice" feature is enabled, it takes constant time to run,
/// but uses a few nightly features.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const FIBB: &[u16] = &[3, 5, 8, 13, 21, 34, 55, 89];
///
/// const TWO: Option<&[u16]> = slice::get_range(FIBB, 2, 4);
/// const FOUR: Option<&[u16]> = slice::get_range(FIBB, 4, 7);
/// const ALL: Option<&[u16]> = slice::get_range(FIBB, 0, 8);
/// const EMPTY: Option<&[u16]> = slice::get_range(FIBB, 0, 0);
/// const NONE: Option<&[u16]> = slice::get_range(FIBB, 0, 1000);
///
/// assert_eq!(TWO, Some(&[8, 13][..]));
/// assert_eq!(FOUR, Some(&[21, 34, 55][..]));
/// assert_eq!(ALL, Some(FIBB));
/// assert_eq!(EMPTY, Some(&[][..]));
/// assert_eq!(NONE, None);
///
/// ```
pub const fn get_range<T>(slice: &[T], start: usize, end: usize) -> Option<&[T]> {
    let x = crate::try_opt!(get_up_to(slice, end));
    get_from(x, start)
}

/// A const equivalent of `&mut slice[start..end]`.
///
/// If `start >= end ` or `slice.len() < start `, this returns an empty slice.
///
/// If `slice.len() < end`, this returns the slice from `start`.
///
///
/// # Alternatives
///
/// For a const equivalent of `&mut slice[start..]` there's [`slice_from_mut`].
///
/// For a const equivalent of `&mut slice[..end]` there's [`slice_up_to_mut`].
///
/// [`slice_from_mut`]: ./fn.slice_from_mut.html
/// [`slice_up_to_mut`]: ./fn.slice_up_to_mut.html
///
/// # Performance
///
/// If the "constant_time_slice" feature is disabled,
/// thich takes linear time to remove the leading and trailing elements,
/// proportional to `start + (slice.len() - end)`.
///
/// If the "constant_time_slice" feature is enabled, it takes constant time to run,
/// but uses a few nightly features.
///
/// # Example
///
/// ```rust
/// use konst::slice::slice_range_mut;
///
/// let mut fibb = [3, 5, 8, 13, 21, 34, 55, 89];
///
/// assert_eq!(slice_range_mut(&mut fibb, 2, 4), &mut [8, 13]);
/// assert_eq!(slice_range_mut(&mut fibb, 4, 7), &mut [21, 34, 55]);
/// assert_eq!(slice_range_mut(&mut fibb, 0, 0), &mut []);
/// assert_eq!(slice_range_mut(&mut fibb, 0, 1000), &mut [3, 5, 8, 13, 21, 34, 55, 89]);
///
/// ```
#[inline]
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
pub const fn slice_range_mut<T>(slice: &mut [T], start: usize, end: usize) -> &mut [T] {
    slice_from_mut(slice_up_to_mut(slice, end), start)
}

/// A const equivalent of `slice.get_mut(start..end)`.
///
///
/// # Alternatives
///
/// For a const equivalent of `slice.get_mut(start..)` there's [`get_from_mut`].
///
/// For a const equivalent of `slice.get_mut(..end)` there's [`get_up_to_mut`].
///
/// [`get_from_mut`]: ./fn.get_from_mut.html
/// [`get_up_to_mut`]: ./fn.get_up_to_mut.html
///
/// # Performance
///
/// If the "constant_time_slice" feature is disabled,
/// thich takes linear time to remove the leading and trailing elements,
/// proportional to `start + (slice.len() - end)`.
///
/// If the "constant_time_slice" feature is enabled, it takes constant time to run,
/// but uses a few nightly features.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// let mut fibb = [3, 5, 8, 13, 21, 34, 55];
///
/// assert_eq!(slice::get_range_mut(&mut fibb, 0, 0), Some(&mut [][..]));
/// assert_eq!(slice::get_range_mut(&mut fibb, 2, 4), Some(&mut [8, 13][..]));
/// assert_eq!(slice::get_range_mut(&mut fibb, 4, 7), Some(&mut [21, 34, 55][..]));
/// assert_eq!(slice::get_range_mut(&mut fibb, 0, 7), Some(&mut [3, 5, 8, 13, 21, 34, 55][..]));
/// assert_eq!(slice::get_range_mut(&mut fibb, 0, 1000), None);
///
/// ```
#[inline]
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
pub const fn get_range_mut<T>(slice: &mut [T], start: usize, end: usize) -> Option<&mut [T]> {
    let x = crate::try_opt!(get_up_to_mut(slice, end));
    get_from_mut(x, start)
}

/// A const equivalent of
/// [`<[T]>::split_at`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at)
///
/// If `at > slice.len()`, this returns a `slice`, empty slice pair.
///
/// # Example
///
/// ```rust
/// use konst::slice::split_at;
///
/// let arr = [3, 5, 8, 13, 21, 34];
///
/// assert_eq!(split_at(&arr, 0), (&[][..], &[3, 5, 8, 13, 21, 34][..]));
///
/// assert_eq!(split_at(&arr, 1), (&[3][..], &[5, 8, 13, 21, 34][..]));
///
/// assert_eq!(split_at(&arr, 2), (&[3, 5][..], &[8, 13, 21, 34][..]));
///
/// assert_eq!(split_at(&arr, 5), (&[3, 5, 8, 13, 21][..], &[34][..]));
///
/// assert_eq!(split_at(&arr, 6), (&[3, 5, 8, 13, 21, 34][..], &[][..]));
///
/// assert_eq!(split_at(&arr, 7), (&[3, 5, 8, 13, 21, 34][..], &[][..]));
///
/// ```
///
pub const fn split_at<T>(slice: &[T], at: usize) -> (&[T], &[T]) {
    (slice_up_to(slice, at), slice_from(slice, at))
}

/// A const equivalent of
/// [`<[T]>::split_at_mut`
/// ](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut)
///
/// If `at > slice.len()`, this returns a `slice`, empty slice pair.
///
/// # Example
///
/// ```rust
/// use konst::slice::split_at_mut;
///
/// let mut arr = [3, 5, 8, 13, 21, 34];
///
/// assert_eq!(split_at_mut(&mut arr, 0), (&mut [][..], &mut [3, 5, 8, 13, 21, 34][..]));
///
/// assert_eq!(split_at_mut(&mut arr, 1), (&mut [3][..], &mut [5, 8, 13, 21, 34][..]));
///
/// assert_eq!(split_at_mut(&mut arr, 2), (&mut [3, 5][..], &mut [8, 13, 21, 34][..]));
///
/// assert_eq!(split_at_mut(&mut arr, 5), (&mut [3, 5, 8, 13, 21][..], &mut [34][..]));
///
/// assert_eq!(split_at_mut(&mut arr, 6), (&mut [3, 5, 8, 13, 21, 34][..], &mut [][..]));
///
/// assert_eq!(split_at_mut(&mut arr, 7), (&mut [3, 5, 8, 13, 21, 34][..], &mut [][..]));
///
/// ```
///
#[inline]
#[cfg(all(feature = "mut_refs", feature = "constant_time_slice"))]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(all(feature = "mut_refs", feature = "constant_time_slice")))
)]
pub const fn split_at_mut<T>(slice: &mut [T], at: usize) -> (&mut [T], &mut [T]) {
    use crate::utils::slice_from_raw_parts_mut;

    if at > slice.len() {
        return (slice, &mut []);
    }

    let suffix_len = slice.len() - at;

    unsafe {
        let ptr = slice.as_mut_ptr();

        let prefix = slice_from_raw_parts_mut(ptr.offset(0), at);
        let suffix = slice_from_raw_parts_mut(ptr.offset(at as isize), suffix_len);

        (prefix, suffix)
    }
}

/// A const equivalent of
/// [`<[u8]>::starts_with`](https://doc.rust-lang.org/std/primitive.slice.html#method.starts_with)
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_start_with;
///
/// assert!( bytes_start_with(b"foo,bar,baz", b"foo,"));
///
/// assert!(!bytes_start_with(b"foo,bar,baz", b"bar"));
/// assert!(!bytes_start_with(b"foo,bar,baz", b"baz"));
///
/// ```
///
#[inline]
pub const fn bytes_start_with(left: &[u8], right: &[u8]) -> bool {
    matches!(bytes_strip_prefix(left, right), Some(_))
}

/// A const equivalent of
/// [`<[u8]>::strip_prefix`](https://doc.rust-lang.org/std/primitive.slice.html#method.strip_prefix)
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_strip_prefix;
///
/// assert_eq!(bytes_strip_prefix(b"foo,bar,baz", b"foo,"), Some("bar,baz".as_bytes()));
///
/// assert_eq!(bytes_strip_prefix(b"foo,bar,baz", b"bar"), None);
/// assert_eq!(bytes_strip_prefix(b"foo,bar,baz", b"baz"), None);
///
/// ```
///
/// [`strip_prefix`]:
/// https://doc.rust-lang.org/std/primitive.slice.html#method.strip_prefix
///
#[inline]
pub const fn bytes_strip_prefix<'a>(mut left: &'a [u8], mut prefix: &[u8]) -> Option<&'a [u8]> {
    impl_bytes_function! {
        strip_prefix;
        left = left;
        right = prefix;
        on_error = return None,
    }
    Some(left)
}

/// A const equivalent of
/// [`<[u8]>::ends_with`](https://doc.rust-lang.org/std/primitive.slice.html#method.ends_with)
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_end_with;
///
/// assert!( bytes_end_with(b"foo,bar,baz", b",baz"));
///
/// assert!(!bytes_end_with(b"foo,bar,baz", b"bar"));
/// assert!(!bytes_end_with(b"foo,bar,baz", b"foo"));
///
/// ```
///
#[inline]
pub const fn bytes_end_with(left: &[u8], right: &[u8]) -> bool {
    matches!(bytes_strip_suffix(left, right), Some(_))
}

/// A const equivalent of
/// [`<[u8]>::strip_suffix`](https://doc.rust-lang.org/std/primitive.slice.html#method.strip_suffix)
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_strip_suffix;
///
/// assert_eq!(bytes_strip_suffix(b"foo,bar,baz", b",baz"), Some("foo,bar".as_bytes()));
///
/// assert_eq!(bytes_strip_suffix(b"foo,bar,baz", b"bar"), None);
/// assert_eq!(bytes_strip_suffix(b"foo,bar,baz", b"foo"), None);
///
/// ```
///
/// [`strip_suffix`]:
/// https://doc.rust-lang.org/std/primitive.slice.html#method.strip_suffix
///
#[inline]
pub const fn bytes_strip_suffix<'a>(mut left: &'a [u8], mut suffix: &[u8]) -> Option<&'a [u8]> {
    impl_bytes_function! {
        strip_suffix;
        left = left;
        right = suffix;
        on_error = return None,
    }
    Some(left)
}

/// Finds the byte offset of `right` in `left`, starting from the `from` index.
///
/// Returns `None` if `right` isn't inside `&left[from..]`
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_find;
///
/// assert_eq!(bytes_find(b"foo-bar-baz-foo", b"foo", 0), Some(0));
/// assert_eq!(bytes_find(b"foo-bar-baz-foo", b"foo", 4), Some(12));
///
/// assert_eq!(bytes_find(b"foo-bar-baz-foo-bar", b"bar", 0), Some(4));
/// assert_eq!(bytes_find(b"foo-bar-baz-foo-bar", b"bar", 4), Some(4));
/// assert_eq!(bytes_find(b"foo-bar-baz-foo-bar", b"bar", 5), Some(16));
/// assert_eq!(bytes_find(b"foo-bar-baz-foo-bar", b"bar", 16), Some(16));
/// assert_eq!(bytes_find(b"foo-bar-baz-foo-bar", b"bar", 17), None);
///
/// ```
///
#[inline]
pub const fn bytes_find(left: &[u8], right: &[u8], from: usize) -> Option<usize> {
    let mut matching = right;

    for_range! {i in from..left.len() =>
        match matching {
            [mb, m_rem @ ..] => {
                let b = left[i];

                matching = if b == *mb {
                    m_rem
                } else {
                    match right {
                        // For when the string is "lawlawn" and we are trying to find "lawn"
                        [mb2, m_rem2 @ ..] if b == *mb2 => m_rem2,
                        _ => right,
                    }
                };
            }
            [] => {
                return Some(i - right.len())
            }
        }
    }

    if matching.is_empty() {
        Some(left.len() - right.len())
    } else {
        None
    }
}

/// Whether `right` is inside `&left[from..]`.
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_contain;
///
/// assert!(bytes_contain(b"foo-bar-baz-foo", b"foo", 0));
/// assert!(bytes_contain(b"foo-bar-baz-foo", b"foo", 4));
///
/// assert!( bytes_contain(b"foo-bar-baz-foo-bar", b"bar", 0));
/// assert!( bytes_contain(b"foo-bar-baz-foo-bar", b"bar", 4));
/// assert!( bytes_contain(b"foo-bar-baz-foo-bar", b"bar", 5));
/// assert!( bytes_contain(b"foo-bar-baz-foo-bar", b"bar", 16));
/// assert!(!bytes_contain(b"foo-bar-baz-foo-bar", b"bar", 17));
///
/// ```
///
#[inline(always)]
pub const fn bytes_contain(left: &[u8], right: &[u8], from: usize) -> bool {
    matches!(bytes_find(left, right, from), Some(_))
}

/// Finds the byte offset of `right` inside `&left[..=from]`, searching in reverse.
///
/// Returns `None` if `right` isn't inside `&left[..=from]`.
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_rfind;
///
/// assert_eq!(bytes_rfind(b"foo-bar-baz-foo", b"foo", 0), None);
/// assert_eq!(bytes_rfind(b"foo-bar-baz-foo", b"foo", 1), None);
///
/// assert_eq!(bytes_rfind(b"foo-bar-baz-foo", b"foo", 2), Some(0));
/// assert_eq!(bytes_rfind(b"foo-bar-baz-foo", b"foo", 3), Some(0));
/// assert_eq!(bytes_rfind(b"foo-bar-baz-foo", b"foo", 4), Some(0));
///
/// assert_eq!(bytes_rfind(b"foo-bar-baz-foo", b"foo", 15), Some(12));
/// assert_eq!(bytes_rfind(b"foo-bar-baz-foo", b"foo", 20000), Some(12));
///
/// ```
///
#[inline]
pub const fn bytes_rfind(left: &[u8], right: &[u8], from: usize) -> Option<usize> {
    let mut matching = right;

    let llen = left.len();

    let mut i = if from >= llen { llen } else { from + 1 };

    while i != 0 {
        i -= 1;

        match matching {
            [m_rem @ .., mb] => {
                let b = left[i];

                matching = if b == *mb {
                    m_rem
                } else {
                    match right {
                        // For when the string is "lawlawn" and we are trying to find "lawn"
                        [m_rem2 @ .., mb2] if b == *mb2 => m_rem2,
                        _ => right,
                    }
                };
            }
            [] => return Some(i + (!right.is_empty()) as usize),
        }
    }

    if matching.is_empty() {
        Some(i)
    } else {
        None
    }
}

/// Returns whether `right` is contained inside `&left[..=from]` searching in reverse.
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_rcontain;
///
/// assert!(!bytes_rcontain(b"foo-bar-baz-foo", b"foo", 0));
/// assert!(!bytes_rcontain(b"foo-bar-baz-foo", b"foo", 1));
///
/// assert!(bytes_rcontain(b"foo-bar-baz-foo", b"foo", 2));
/// assert!(bytes_rcontain(b"foo-bar-baz-foo", b"foo", 3));
/// assert!(bytes_rcontain(b"foo-bar-baz-foo", b"foo", 4));
///
/// assert!(bytes_rcontain(b"foo-bar-baz-foo", b"foo", 15));
/// assert!(bytes_rcontain(b"foo-bar-baz-foo", b"foo", 20000));
///
/// ```
///
#[inline(always)]
pub const fn bytes_rcontain(left: &[u8], right: &[u8], from: usize) -> bool {
    matches!(bytes_rfind(left, right, from), Some(_))
}

/// A const equivalent of
/// [`<[T]>::first`](https://doc.rust-lang.org/std/primitive.slice.html#method.first)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// assert_eq!(slice::first(&[8, 5, 3]), Some(&8));
///
/// assert_eq!(slice::first(&[5, 3]), Some(&5));
///
/// assert_eq!(slice::first(&[3]), Some(&3));
///
/// assert_eq!(slice::first::<u8>(&[]), None);
///
/// ```
///
pub const fn first<T>(slice: &[T]) -> Option<&T> {
    if let [first, ..] = slice {
        Some(first)
    } else {
        None
    }
}

/// A const equivalent of
/// [`<[T]>::first_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.first_mut)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// assert_eq!(slice::first_mut(&mut [8, 5, 3]), Some(&mut 8));
///
/// assert_eq!(slice::first_mut(&mut [5, 3]), Some(&mut 5));
///
/// assert_eq!(slice::first_mut(&mut [3]), Some(&mut 3));
///
/// assert_eq!(slice::first_mut::<u8>(&mut []), None);
///
/// ```
///
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
pub const fn first_mut<T>(slice: &mut [T]) -> Option<&mut T> {
    if let [first, ..] = slice {
        Some(first)
    } else {
        None
    }
}

/// A const equivalent of
/// [`<[T]>::last`](https://doc.rust-lang.org/std/primitive.slice.html#method.last)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// assert_eq!(slice::last(&[3, 5, 8]), Some(&8));
///
/// assert_eq!(slice::last(&[3, 5]), Some(&5));
///
/// assert_eq!(slice::last(&[3]), Some(&3));
///
/// assert_eq!(slice::last::<u8>(&[]), None);
///
/// ```
///
pub const fn last<T>(slice: &[T]) -> Option<&T> {
    if let [.., last] = slice {
        Some(last)
    } else {
        None
    }
}

/// A const equivalent of
/// [`<[T]>::last_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.last_mut)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// assert_eq!(slice::last_mut(&mut [3, 5, 8]), Some(&mut 8));
///
/// assert_eq!(slice::last_mut(&mut [3, 5]), Some(&mut 5));
///
/// assert_eq!(slice::last_mut(&mut [3]), Some(&mut 3));
///
/// assert_eq!(slice::last_mut::<u8>(&mut []), None);
///
/// ```
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
pub const fn last_mut<T>(slice: &mut [T]) -> Option<&mut T> {
    if let [.., last] = slice {
        Some(last)
    } else {
        None
    }
}

/// A const equivalent of
/// [`<[T]>::split_first`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_first)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const fn add_up(mut slice: &[u32]) -> u64 {
///     let mut ret = 0u64;
///     while let Some((first, rem)) = slice::split_first(slice) {
///         ret += *first as u64;
///
///         // advances the slice
///         slice = rem;
///     }
///     ret
/// }
///
/// assert_eq!(add_up(&[1]), 1);
/// assert_eq!(add_up(&[1, 2]), 3);
/// assert_eq!(add_up(&[1, 2, 3]), 6);
/// assert_eq!(add_up(&[1, 2, 3, 4]), 10);
///
/// ```
///
pub const fn split_first<T>(slice: &[T]) -> Option<(&T, &[T])> {
    if let [first, rem @ ..] = slice {
        Some((first, rem))
    } else {
        None
    }
}

/// A const equivalent of
/// [`<[T]>::split_first_mut`
/// ](https://doc.rust-lang.org/std/primitive.slice.html#method.split_first_mut)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// assert_eq!(slice::split_first_mut(&mut [5, 8, 13, 21]), Some((&mut 5, &mut [8, 13, 21][..])));
/// assert_eq!(slice::split_first_mut(&mut [8, 13, 21]), Some((&mut 8, &mut [13, 21][..])));
/// assert_eq!(slice::split_first_mut(&mut [13, 21]), Some((&mut 13, &mut [21][..])));
/// assert_eq!(slice::split_first_mut(&mut [21]), Some((&mut 21, &mut [][..])));
/// assert_eq!(slice::split_first_mut::<()>(&mut []), None);
///
/// ```
///
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
pub const fn split_first_mut<T>(slice: &mut [T]) -> Option<(&mut T, &mut [T])> {
    if let [first, rem @ ..] = slice {
        Some((first, rem))
    } else {
        None
    }
}

/// A const equivalent of
/// [`<[T]>::split_last`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_last)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const fn find_last_even(mut slice: &[u32]) -> Option<usize> {
///     let mut ret = 0u32;
///     while let Some((last, rem)) = slice::split_last(slice) {
///         if *last % 2 == 0 {
///             return Some(rem.len());
///         }
///
///         // advances the slice
///         slice = rem;
///     }
///     None
/// }
///
/// assert_eq!(find_last_even(&[3, 5]), None);
///
/// assert_eq!(find_last_even(&[3, 5, 8, 13, 21]), Some(2));
///
/// assert_eq!(find_last_even(&[3, 5, 8, 13, 21, 34, 55]), Some(5));
///
/// assert_eq!(find_last_even(&[3, 5, 8, 13, 21, 34, 55, 89, 144]), Some(8));
///
/// ```
///
pub const fn split_last<T>(slice: &[T]) -> Option<(&T, &[T])> {
    if let [rem @ .., last] = slice {
        Some((last, rem))
    } else {
        None
    }
}

/// A const equivalent of
/// [`<[T]>::split_last_mut`
/// ](https://doc.rust-lang.org/std/primitive.slice.html#method.split_last_mut)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// assert_eq!(slice::split_last_mut(&mut [8, 13, 21, 5]), Some((&mut 5, &mut [8, 13, 21][..])));
/// assert_eq!(slice::split_last_mut(&mut [13, 21, 8]), Some((&mut 8, &mut [13, 21][..])));
/// assert_eq!(slice::split_last_mut(&mut [21, 13]), Some((&mut 13, &mut [21][..])));
/// assert_eq!(slice::split_last_mut(&mut [21]), Some((&mut 21, &mut [][..])));
/// assert_eq!(slice::split_last_mut::<()>(&mut []), None);
///
/// ```
///
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
pub const fn split_last_mut<T>(slice: &mut [T]) -> Option<(&mut T, &mut [T])> {
    if let [rem @ .., last] = slice {
        Some((last, rem))
    } else {
        None
    }
}
