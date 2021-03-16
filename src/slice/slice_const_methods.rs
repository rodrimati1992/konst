use crate::utils::saturating_sub;

#[cfg(feature = "constant_time_slice")]
use crate::utils::{min_usize, Dereference};

macro_rules! slice_up_to_impl{
    (
        $($args:tt)*
    )=>{
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
    };
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

/// A const equivalent of `&slice[..len]`.
///
/// If `slice.len() < len`, this simply returns an empty slice.
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
    let rem = saturating_sub(slice.len(), start);

    if rem == 0 {
        return &[];
    }

    #[cfg(feature = "constant_time_slice")]
    {
        unsafe {
            let raw_slice = core::slice::from_raw_parts(slice.as_ptr().offset(start), rem);
            Dereference { ptr: raw_slice }.reff
        }
    }
    #[cfg(not(feature = "constant_time_slice"))]
    {
        let mut ret = slice;
        let mut to_remove = start;

        slice_up_to_impl! {
            ret, to_remove, next,
            () (next @ ..),
        }
        ret
    }
}

/// A const equivalent of `&slice[..len]`.
///
/// If `slice.len() < len`, this simply returns `slice` back.
///
/// # Performance
///
/// If the "constant_time_slice" feature is disabled,
/// thich takes linear time to remove the trailing elements,
/// proportional to `slice.len() - len`.
///
/// If the "constant_time_slice" feature is enabled, it takes constant time to run,
/// but uses a few nightly features.
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
    let rem = saturating_sub(slice.len(), len);

    if rem == 0 {
        return slice;
    }

    #[cfg(feature = "constant_time_slice")]
    {
        // Doing this to get a slice up to length at compile-time
        unsafe {
            let raw_slice = core::slice::from_raw_parts(slice.as_ptr(), len);
            Dereference { ptr: raw_slice }.reff
        }
    }
    #[cfg(not(feature = "constant_time_slice"))]
    {
        let mut ret = slice;
        let mut to_remove = rem;

        slice_up_to_impl! {
            ret, to_remove, next,
            (next @ ..,) (),
        }
        ret
    }
}

/// A const equivalent of `&slice[start..end]`.
///
/// If `start >= end ` or `slice.len() < start `, this returns an empty slice.
///
/// If `slice.len() < end`, this returns the slice from `start`.
///
///
/// # Performance
///
/// If the "constant_time_slice" feature is disabled,
/// thich takes linear time to remove the trailing elements,
/// proportional to `slice.len() - len`.
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
