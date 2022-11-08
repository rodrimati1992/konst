use konst_kernel::{__slice_from_impl, __slice_up_to_impl};

use crate::slice::{BytesPattern, PatternNorm};

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
pub use konst_kernel::slice::slice_from;

/// A const equivalent of `&slice[..len]`.
///
/// If `slice.len() < len`, this simply returns `slice` back.
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
pub use konst_kernel::slice::slice_up_to;

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
pub use konst_kernel::slice::slice_range;

/// A const equivalent of `slice.get(start..)`.
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
    Some(__slice_from_impl!(
        slice,
        start,
        as_ptr,
        from_raw_parts,
        None
    ))
}

/// A const equivalent of `&mut slice[start..]`.
///
/// If `slice.len() < start`, this simply returns an empty slice.
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
    __slice_from_impl!(slice, start, as_mut_ptr, from_raw_parts_mut, &mut [])
}

/// A const equivalent of `slice.get_mut(start..)`.
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
    Some(__slice_from_impl!(
        slice,
        start,
        as_mut_ptr,
        from_raw_parts_mut,
        None
    ))
}

/// A const equivalent of `slice.get(..len)`.
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
    Some(__slice_up_to_impl!(
        slice,
        len,
        as_ptr,
        from_raw_parts,
        None
    ))
}

/// A const equivalent of `&mut slice[..len]`.
///
/// If `slice.len() < len`, this simply returns `slice` back.
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
    __slice_up_to_impl!(slice, len, as_mut_ptr, from_raw_parts_mut, slice)
}

/// A const equivalent of `slice.get_mut(..len)`.
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
pub const fn get_up_to_mut<T>(slice: &mut [T], len: usize) -> Option<&mut [T]> {
    Some(__slice_up_to_impl!(
        slice,
        len,
        as_mut_ptr,
        from_raw_parts_mut,
        None
    ))
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
#[cfg(feature = "mut_refs")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "mut_refs")))]
pub const fn split_at_mut<T>(slice: &mut [T], at: usize) -> (&mut [T], &mut [T]) {
    use core::slice::from_raw_parts_mut;

    if at > slice.len() {
        return (slice, &mut []);
    }

    let suffix_len = slice.len() - at;

    unsafe {
        let ptr = slice.as_mut_ptr();

        let prefix = from_raw_parts_mut(ptr.offset(0), at);
        let suffix = from_raw_parts_mut(ptr.offset(at as isize), suffix_len);

        (prefix, suffix)
    }
}

/// Whether `pattern` is the start of `left`.
///
/// This is analogous to
/// [`<[u8]>::starts_with`](https://doc.rust-lang.org/std/primitive.slice.html#method.starts_with)
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_start_with;
///
/// assert!( bytes_start_with(b"foo,bar,baz", "foo,"));
/// assert!( bytes_start_with(b"foo,bar,baz", &'f'));
/// assert!( bytes_start_with(b"foo,bar,baz", &[b'f', b'o', b'o']));
/// assert!(!bytes_start_with(b"foo,bar,baz", "bar"));
/// assert!(!bytes_start_with(b"foo,bar,baz", "baz"));
///
/// ```
///
#[inline]
pub const fn bytes_start_with<const N: usize, P>(left: &[u8], pattern: &P) -> bool
where
    P: ?Sized + BytesPattern<N>,
{
    let pattern = PatternNorm::new(pattern);
    __bytes_start_with(left, pattern.as_bytes())
}
#[inline(always)]
pub(crate) const fn __bytes_start_with(left: &[u8], pattern: &[u8]) -> bool {
    matches!(__bytes_strip_prefix(left, pattern), Some(_))
}

/// Remove `prefix` from the start of `left`.
///
/// Returns `None` if `prefix` is not the start of `left`.
///
/// This is analogous to
/// [`<[u8]>::strip_prefix`](https://doc.rust-lang.org/std/primitive.slice.html#method.strip_prefix)
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_strip_prefix;
///
/// assert_eq!(bytes_strip_prefix(b"foo,bar,baz", b"foo,"), Some("bar,baz".as_bytes()));
/// assert_eq!(bytes_strip_prefix(b"foo,bar,baz", "foo,bar,"), Some("baz".as_bytes()));
/// assert_eq!(bytes_strip_prefix(b"foo,bar,baz", &'f'), Some("oo,bar,baz".as_bytes()));
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
pub const fn bytes_strip_prefix<'a, const N: usize, P>(
    left: &'a [u8],
    prefix: &P,
) -> Option<&'a [u8]>
where
    P: ?Sized + BytesPattern<N>,
{
    let prefix = PatternNorm::new(prefix);
    __bytes_strip_prefix(left, prefix.as_bytes())
}
pub(crate) const fn __bytes_strip_prefix<'a>(
    mut left: &'a [u8],
    mut prefix: &[u8],
) -> Option<&'a [u8]> {
    impl_bytes_function! {
        strip_prefix;
        left = left;
        right = prefix;
        on_error = return None,
    }
    Some(left)
}

/// Whether `pattern` is the end of `left`.
///
/// A const analog of
/// [`<[u8]>::ends_with`](https://doc.rust-lang.org/std/primitive.slice.html#method.ends_with)
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_end_with;
///
/// assert!( bytes_end_with(b"foo,bar,baz", b",baz"));
/// assert!( bytes_end_with(b"foo,bar,baz", "bar,baz"));
/// assert!( bytes_end_with(b"foo,bar,baz", &'z'));
///
/// assert!(!bytes_end_with(b"foo,bar,baz", b"bar"));
/// assert!(!bytes_end_with(b"foo,bar,baz", b"foo"));
///
/// ```
///
#[inline]
pub const fn bytes_end_with<const N: usize, P>(left: &[u8], pattern: &P) -> bool
where
    P: ?Sized + BytesPattern<N>,
{
    let pattern = PatternNorm::new(pattern);
    __bytes_end_with(left, pattern.as_bytes())
}
pub(crate) const fn __bytes_end_with(left: &[u8], pattern: &[u8]) -> bool {
    matches!(__bytes_strip_suffix(left, pattern), Some(_))
}

/// Remove `suffix` from the end of `left`.
///
/// Returns `None` if `suffix` is not the end of `left`.
///
/// A const analog of
/// [`<[u8]>::strip_suffix`](https://doc.rust-lang.org/std/primitive.slice.html#method.strip_suffix)
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_strip_suffix;
///
/// assert_eq!(bytes_strip_suffix(b"foo,bar,baz", b",baz"), Some("foo,bar".as_bytes()));
/// assert_eq!(bytes_strip_suffix(b"foo,bar,baz", ",bar,baz"), Some("foo".as_bytes()));
/// assert_eq!(bytes_strip_suffix(b"foo,bar,baz", &'z'), Some("foo,bar,ba".as_bytes()));
///
/// assert_eq!(bytes_strip_suffix(b"foo,bar,baz", b"bar"), None);
/// assert_eq!(bytes_strip_suffix(b"foo,bar,baz", "foo"), None);
///
/// ```
///
/// [`strip_suffix`]:
/// https://doc.rust-lang.org/std/primitive.slice.html#method.strip_suffix
///
#[inline]
pub const fn bytes_strip_suffix<'a, const N: usize, P>(
    left: &'a [u8],
    suffix: &P,
) -> Option<&'a [u8]>
where
    P: ?Sized + BytesPattern<N>,
{
    let suffix = PatternNorm::new(suffix);
    __bytes_strip_suffix(left, suffix.as_bytes())
}
pub(crate) const fn __bytes_strip_suffix<'a>(
    mut left: &'a [u8],
    mut suffix: &[u8],
) -> Option<&'a [u8]> {
    impl_bytes_function! {
        strip_suffix;
        left = left;
        right = suffix;
        on_error = return None,
    }
    Some(left)
}

/// Finds the byte offset of `pattern` in `left`.
///
/// Returns `None` if `pattern` isn't inside `left`
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_find;
///
/// assert_eq!(bytes_find(b"foo-bar-baz", &'q'), None);
/// assert_eq!(bytes_find(b"foo-bar-baz", "foo"), Some(0));
/// assert_eq!(bytes_find(b"foo-bar-baz", b"bar"), Some(4));
/// assert_eq!(bytes_find(b"foo-bar-baz", b"baz"), Some(8));
///
/// ```
///
#[inline]
pub const fn bytes_find<const N: usize, P>(left: &[u8], pattern: &P) -> Option<usize>
where
    P: ?Sized + BytesPattern<N>,
{
    let pattern = PatternNorm::new(pattern);
    __bytes_find(left, pattern.as_bytes())
}
pub(crate) const fn __bytes_find(left: &[u8], pattern: &[u8]) -> Option<usize> {
    let mut matching = pattern;

    crate::for_range! {i in 0..left.len() =>
        match matching {
            [mb, m_rem @ ..] => {
                let b = left[i];

                matching = if b == *mb {
                    m_rem
                } else {
                    match pattern {
                        // For when the string is "lawlawn" and we are trying to find "lawn"
                        [mb2, m_rem2 @ ..] if b == *mb2 => m_rem2,
                        _ => pattern,
                    }
                };
            }
            [] => {
                return Some(i - pattern.len())
            }
        }
    }

    if matching.is_empty() {
        Some(left.len() - pattern.len())
    } else {
        None
    }
}

/// Whether `pattern` is inside `left`.
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_contain;
///
/// assert!(bytes_contain(b"foo-bar", b"foo"));
/// assert!(bytes_contain(b"bar-foo", "foo"));
///
/// assert!(!bytes_contain(b"foo-bar-baz", &'q'));
///
/// ```
///
#[inline]
pub const fn bytes_contain<const N: usize, P>(left: &[u8], pattern: &P) -> bool
where
    P: ?Sized + BytesPattern<N>,
{
    let pattern = PatternNorm::new(pattern);
    __bytes_contain(left, pattern.as_bytes())
}
#[inline(always)]
const fn __bytes_contain(left: &[u8], pattern: &[u8]) -> bool {
    matches!(__bytes_find(left, pattern), Some(_))
}

/// Finds the byte offset of `pattern` inside `left`, searching in reverse.
///
/// Returns `None` if `pattern` isn't inside `left`.
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_rfind;
///
/// assert_eq!(bytes_rfind(b"foo-bar-baz", &'q'), None);
/// assert_eq!(bytes_rfind(b"foo-bar-baz", b"foo"), Some(0));
/// assert_eq!(bytes_rfind(b"foo-bar-baz", "bar"), Some(4));
/// assert_eq!(bytes_rfind(b"foo-bar-baz", b"baz"), Some(8));
///
/// ```
///
#[inline]
pub const fn bytes_rfind<const N: usize, P>(left: &[u8], pattern: &P) -> Option<usize>
where
    P: ?Sized + BytesPattern<N>,
{
    let pattern = PatternNorm::new(pattern);
    __bytes_rfind(left, pattern.as_bytes())
}
pub(crate) const fn __bytes_rfind(left: &[u8], pattern: &[u8]) -> Option<usize> {
    let mut matching = pattern;

    let llen = left.len();

    let mut i = llen;

    while i != 0 {
        i -= 1;

        match matching {
            [m_rem @ .., mb] => {
                let b = left[i];

                matching = if b == *mb {
                    m_rem
                } else {
                    match pattern {
                        // For when the string is "lawlawn" and we are trying to find "lawn"
                        [m_rem2 @ .., mb2] if b == *mb2 => m_rem2,
                        _ => pattern,
                    }
                };
            }
            [] => return Some(i + (!pattern.is_empty()) as usize),
        }
    }

    if matching.is_empty() {
        Some(i)
    } else {
        None
    }
}

/// Returns whether `pattern` is contained inside `left`, searching in reverse.
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_rcontain;
///
/// assert!(bytes_rcontain(b"foo-bar", b"foo"));
/// assert!(bytes_rcontain(b"bar-foo", "foo"));
///
/// assert!(!bytes_rcontain(b"foo-bar-baz", &'q'));
///
/// ```
///
#[inline]
pub const fn bytes_rcontain<const N: usize, P>(left: &[u8], pattern: &P) -> bool
where
    P: ?Sized + BytesPattern<N>,
{
    let pattern = PatternNorm::new(pattern);
    __bytes_rcontain(left, pattern.as_bytes())
}
#[inline(always)]
pub(crate) const fn __bytes_rcontain(left: &[u8], pattern: &[u8]) -> bool {
    matches!(bytes_rfind(left, pattern), Some(_))
}

macro_rules! matches_space {
    ($b:ident) => {
        matches!($b, b'\t' | b'\n' | b'\r' | b' ')
    };
}

/// Removes ascii whitespace from the start and end of `this`.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const TRIMMED: &[u8] = slice::bytes_trim(b"\nhello world  ");
///
/// assert_eq!(TRIMMED, b"hello world");
///
/// ```
pub const fn bytes_trim(this: &[u8]) -> &[u8] {
    bytes_trim_start(bytes_trim_end(this))
}

/// Removes ascii whitespace from the start of `this`.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const TRIMMED: &[u8] = slice::bytes_trim_start(b"\tfoo bar  ");
///
/// assert_eq!(TRIMMED, b"foo bar  ");
///
/// ```
pub const fn bytes_trim_start(mut this: &[u8]) -> &[u8] {
    loop {
        match this {
            [b, rem @ ..] if matches_space!(b) => this = rem,
            _ => return this,
        }
    }
}

/// Removes ascii whitespace from the end of `this`.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const TRIMMED: &[u8] = slice::bytes_trim_end(b"\rfoo bar  ");
///
/// assert_eq!(TRIMMED, b"\rfoo bar");
///
/// ```
pub const fn bytes_trim_end(mut this: &[u8]) -> &[u8] {
    loop {
        match this {
            [rem @ .., b] if matches_space!(b) => this = rem,
            _ => return this,
        }
    }
}

/// Removes all instances of `needle` from the start and end of `this`.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const TRIMMED0: &[u8] = slice::bytes_trim_matches(b"<>baz qux<><><>", b"<>");
/// assert_eq!(TRIMMED0, b"baz qux");
///
/// const TRIMMED1: &[u8] = slice::bytes_trim_matches(b"{}foo bar{}{}", "{}");
/// assert_eq!(TRIMMED1, b"foo bar");
///
/// const TRIMMED2: &[u8] = slice::bytes_trim_matches(b"-----soming----", &'-');
/// assert_eq!(TRIMMED2, b"soming");
///
///
/// ```
pub const fn bytes_trim_matches<'a, const N: usize, P>(this: &'a [u8], needle: &P) -> &'a [u8]
where
    P: ?Sized + BytesPattern<N>,
{
    let needle = PatternNorm::new(needle);
    __bytes_trim_matches(this, needle.as_bytes())
}
pub(crate) const fn __bytes_trim_matches<'a>(this: &'a [u8], needle: &[u8]) -> &'a [u8] {
    let ltrim = __bytes_trim_start_matches(this, needle);
    __bytes_trim_end_matches(ltrim, needle)
}

/// Removes all instances of `needle` from the start of `this`.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const TRIMMED0: &[u8] = slice::bytes_trim_start_matches(b"#####huh###", b"##");
/// const TRIMMED1: &[u8] = slice::bytes_trim_start_matches(b"[][]nice[][]", "[][]");
/// const TRIMMED2: &[u8] = slice::bytes_trim_start_matches(b"(((woah", &'(');
///
/// assert_eq!(TRIMMED0, b"#huh###");
/// assert_eq!(TRIMMED1, b"nice[][]");
/// assert_eq!(TRIMMED2, b"woah");
///
/// ```
pub const fn bytes_trim_start_matches<'a, const N: usize, P>(this: &'a [u8], needle: &P) -> &'a [u8]
where
    P: ?Sized + BytesPattern<N>,
{
    let needle = PatternNorm::new(needle);
    __bytes_trim_start_matches(this, needle.as_bytes())
}
pub(crate) const fn __bytes_trim_start_matches<'a>(mut this: &'a [u8], needle: &[u8]) -> &'a [u8] {
    if needle.is_empty() {
        return this;
    }

    let mut matched = needle;

    loop {
        let at_start = this;

        match (this, matched) {
            ([b, rem @ ..], [bm, remm @ ..]) if *b == *bm => {
                this = rem;
                matched = remm;
            }
            _ => return this,
        }

        'inner: loop {
            match (this, matched) {
                ([], [_, ..]) => return at_start,
                ([b, rem @ ..], [bm, remm @ ..]) => {
                    if *b == *bm {
                        this = rem;
                        matched = remm;
                    } else {
                        return at_start;
                    }
                }
                _ => break 'inner,
            }
        }

        matched = needle;
    }
}

/// Removes all instances of `needle` from the end of `this`.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const TRIMMED0: &[u8] = slice::bytes_trim_end_matches(b"oowowooooo", b"oo");
/// const TRIMMED1: &[u8] = slice::bytes_trim_end_matches(b"gooooo", "oo");
/// const TRIMMED2: &[u8] = slice::bytes_trim_end_matches(b"yesssssss", &'s');
///
/// assert_eq!(TRIMMED0, b"oowowo");
/// assert_eq!(TRIMMED1, b"go");
/// assert_eq!(TRIMMED2, b"ye");
///
/// ```
pub const fn bytes_trim_end_matches<'a, const N: usize, P>(this: &'a [u8], needle: &P) -> &'a [u8]
where
    P: ?Sized + BytesPattern<N>,
{
    let needle = PatternNorm::new(needle);
    __bytes_trim_end_matches(this, needle.as_bytes())
}
pub(crate) const fn __bytes_trim_end_matches<'a>(mut this: &'a [u8], needle: &[u8]) -> &'a [u8] {
    if needle.is_empty() {
        return this;
    }

    let mut matched = needle;

    loop {
        let at_start = this;

        match (this, matched) {
            ([rem @ .., b], [remm @ .., bm]) if *b == *bm => {
                this = rem;
                matched = remm;
            }
            _ => return this,
        }

        'inner: loop {
            match (this, matched) {
                ([], [.., _]) => return at_start,
                ([rem @ .., b], [remm @ .., bm]) => {
                    if *b == *bm {
                        this = rem;
                        matched = remm;
                    } else {
                        return at_start;
                    }
                }
                _ => break 'inner,
            }
        }

        matched = needle;
    }
}

macro_rules! elem_then_rem {
    ($elem:ident, $($rem:tt)*) => { [$elem, $($rem)*] };
}

macro_rules! rem_then_elem {
    ($elem:ident, $($rem:tt)*) => { [$($rem)*, $elem] };
}

macro_rules! byte_find_then {
    ($slice_order:ident, $this:ident, $needle:ident, |$next:ident| $then:block) => ({
        if $needle.is_empty() {
            return Some($this);
        }

        let mut matching = $needle;

        let mut $next = $this;

        while let $slice_order!(mb, ref m_rem @ ..) = *matching {
            matching = m_rem;

            if let $slice_order!(b, ref rem @ ..) = *$next {
                if b != mb {
                    matching = match *$needle {
                        // For when the string is "lawlawn" and we are skipping "lawn"
                        $slice_order!(mb2, ref m_rem2 @ ..) if b == mb2 => {
                            // This is considered used in half of the macro invocations
                            #[allow(unused_assignments)]
                            {$this = $next;}
                            m_rem2
                        },
                        _ => {
                            // This is considered used in half of the macro invocations
                            #[allow(unused_assignments)]
                            {$this = rem;}
                            $needle
                        },
                    };
                }
                $next = rem;
            } else {
                return None;
            }
        }

        $then

        Some($this)
    });
}

/// Advances `this` past the first instance of `needle`.
///
/// Return `None` if no instance of `needle` is found.
///
/// Return `Some(this)` if `needle` is empty.
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_find_skip;
///
/// {
///     const FOUND: Option<&[u8]> = bytes_find_skip(b"foo bar baz", b"bar");
///     assert_eq!(FOUND, Some(&b" baz"[..]));
/// }
/// {
///     const NOT_FOUND: Option<&[u8]> = bytes_find_skip(b"foo bar baz", &'q');
///     assert_eq!(NOT_FOUND, None);
/// }
/// {
///     const EMPTY_NEEDLE: Option<&[u8]> = bytes_find_skip(b"foo bar baz", "");
///     assert_eq!(EMPTY_NEEDLE, Some(&b"foo bar baz"[..]));
/// }
/// ```
pub const fn bytes_find_skip<'a, const N: usize, P>(this: &'a [u8], needle: &P) -> Option<&'a [u8]>
where
    P: ?Sized + BytesPattern<N>,
{
    let needle = PatternNorm::new(needle);
    __bytes_find_skip(this, needle.as_bytes())
}
pub(crate) const fn __bytes_find_skip<'a>(mut this: &'a [u8], needle: &[u8]) -> Option<&'a [u8]> {
    byte_find_then! {elem_then_rem, this, needle, |next| {this = next}}
}

/// Advances `this` up to the first instance of `needle`.
///
/// Return `None` if no instance of `needle` is found.
///
/// Return `Some(this)` if `needle` is empty.
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_find_keep;
///
/// {
///     const FOUND: Option<&[u8]> = bytes_find_keep(b"foo bar baz", b"bar");
///     assert_eq!(FOUND, Some(&b"bar baz"[..]));
/// }
/// {
///     const NOT_FOUND: Option<&[u8]> = bytes_find_keep(b"foo bar baz", &'q');
///     assert_eq!(NOT_FOUND, None);
/// }
/// {
///     const EMPTY_NEEDLE: Option<&[u8]> = bytes_find_keep(b"foo bar baz", "");
///     assert_eq!(EMPTY_NEEDLE, Some(&b"foo bar baz"[..]));
/// }
/// ```
pub const fn bytes_find_keep<'a, const N: usize, P>(this: &'a [u8], needle: &P) -> Option<&'a [u8]>
where
    P: ?Sized + BytesPattern<N>,
{
    let needle = PatternNorm::new(needle);
    __bytes_find_keep(this, needle.as_bytes())
}
pub(crate) const fn __bytes_find_keep<'a>(mut this: &'a [u8], needle: &[u8]) -> Option<&'a [u8]> {
    byte_find_then! {elem_then_rem, this, needle, |next| {}}
}

/// Truncates `this` to before the last instance of `needle`.
///
/// Return `None` if no instance of `needle` is found.
///
/// Return `Some(this)` if `needle` is empty.
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_rfind_skip;
///
/// {
///     const FOUND: Option<&[u8]> = bytes_rfind_skip(b"foo bar _ bar baz", b"bar");
///     assert_eq!(FOUND, Some(&b"foo bar _ "[..]));
/// }
/// {
///     const NOT_FOUND: Option<&[u8]> = bytes_rfind_skip(b"foo bar baz", &'q');
///     assert_eq!(NOT_FOUND, None);
/// }
/// {
///     const EMPTY_NEEDLE: Option<&[u8]> = bytes_rfind_skip(b"foo bar baz", "");
///     assert_eq!(EMPTY_NEEDLE, Some(&b"foo bar baz"[..]));
/// }
/// ```
pub const fn bytes_rfind_skip<'a, const N: usize, P>(this: &'a [u8], needle: &P) -> Option<&'a [u8]>
where
    P: ?Sized + BytesPattern<N>,
{
    let needle = PatternNorm::new(needle);
    __bytes_rfind_skip(this, needle.as_bytes())
}
pub(crate) const fn __bytes_rfind_skip<'a>(mut this: &'a [u8], needle: &[u8]) -> Option<&'a [u8]> {
    byte_find_then! {rem_then_elem, this, needle, |next| {this = next}}
}

/// Truncates `this` to the last instance of `needle`.
///
/// Return `None` if no instance of `needle` is found.
///
/// Return `Some(this)` if `needle` is empty.
///
/// # Example
///
/// ```rust
/// use konst::slice::bytes_rfind_keep;
///
/// {
///     const FOUND: Option<&[u8]> = bytes_rfind_keep(b"foo bar _ bar baz", b"bar");
///     assert_eq!(FOUND, Some(&b"foo bar _ bar"[..]));
/// }
/// {
///     const NOT_FOUND: Option<&[u8]> = bytes_rfind_keep(b"foo bar baz", &'q');
///     assert_eq!(NOT_FOUND, None);
/// }
/// {
///     const EMPTY_NEEDLE: Option<&[u8]> = bytes_rfind_keep(b"foo bar baz", "");
///     assert_eq!(EMPTY_NEEDLE, Some(&b"foo bar baz"[..]));
/// }
/// ```
pub const fn bytes_rfind_keep<'a, const N: usize, P>(this: &'a [u8], needle: &P) -> Option<&'a [u8]>
where
    P: ?Sized + BytesPattern<N>,
{
    let needle = PatternNorm::new(needle);
    __bytes_rfind_keep(this, needle.as_bytes())
}
pub(crate) const fn __bytes_rfind_keep<'a>(mut this: &'a [u8], needle: &[u8]) -> Option<&'a [u8]> {
    byte_find_then! {rem_then_elem, this, needle, |next| {}}
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
