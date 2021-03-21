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
            let raw_slice = core::ptr::slice_from_raw_parts(slice.as_ptr().offset(start as _), rem);
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
            let raw_slice = core::ptr::slice_from_raw_parts(slice.as_ptr(), len);
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
        on_error = return None;
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
        on_error = return None;
    }
    Some(left)
}

/// Finds the byte offset of `right` inside `&left[from..]`.
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
