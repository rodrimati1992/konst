//! `const fn` equivalents of `str` methods.
//!
//! # Removed in 0.3.0
//!
//! These functions were removed in 0.3.0 because there is an equivalent
//! const fn in the standard library:
//!
//! - `from_utf8`: [`core::str::from_utf8`]
//! - `Utf8Error`: [`core::str::Utf8Error`]
//!
//!

pub(crate) mod pattern;

pub use self::pattern::Pattern;

use self::pattern::PatternNorm;

mod splitting;

pub use splitting::*;

mod split_terminator_items;

pub use split_terminator_items::*;

__declare_string_cmp_fns! {
    import_path = "konst",
    equality_fn = eq_str,
    ordering_fn = cmp_str,
    ordering_fn_inner = cmp_str_inner,
}

#[cfg(feature = "cmp")]
__declare_fns_with_docs! {
    (Option<&'a str>, (eq_option_str, cmp_option_str))

    docs(default)

    macro = __impl_option_cmp_fns!(
        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
        for['a,]
        params(l, r)
        eq_comparison = crate::polymorphism::CmpWrapper(l).const_eq(r),
        cmp_comparison = crate::polymorphism::CmpWrapper(l).const_cmp(r),
        parameter_copyability = copy,
    ),
}

/// A const equivalent of
/// [`str::starts_with`](https://doc.rust-lang.org/std/primitive.str.html#method.starts_with)
/// , taking a [`Pattern`] parameter.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// assert!( string::starts_with("foo,bar,baz", "foo,"));
///
/// assert!(!string::starts_with("foo,bar,baz", "bar"));
/// assert!(!string::starts_with("foo,bar,baz", "baz"));
///
/// ```
///
#[inline(always)]
pub const fn starts_with<'a, P>(left: &str, pat: P) -> bool
where
    P: Pattern<'a>,
{
    let pat = PatternNorm::new(pat);
    crate::slice::bytes_start_with(left.as_bytes(), pat.as_bytes())
}

/// A const equivalent of
/// [`str::ends_with`](https://doc.rust-lang.org/std/primitive.str.html#method.ends_with)
/// , taking a [`Pattern`] parameter.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// assert!( string::ends_with("foo,bar,baz", ",baz"));
/// assert!( string::ends_with("abc...z", 'z'));
///
/// assert!(!string::ends_with("foo,bar,baz", "bar"));
/// assert!(!string::ends_with("foo,bar,baz", "foo"));
/// assert!(!string::ends_with("abc", 'z'));
///
/// ```
///
#[inline(always)]
pub const fn ends_with<'a, P>(left: &str, pat: P) -> bool
where
    P: Pattern<'a>,
{
    let pat = PatternNorm::new(pat);
    crate::slice::bytes_end_with(left.as_bytes(), pat.as_bytes())
}

/// A const equivalent of
/// [`str::find`](https://doc.rust-lang.org/std/primitive.str.html#method.find)
/// , taking a [`Pattern`] parameter.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// assert_eq!(string::find("foo-bar-baz", 'q'), None);
/// assert_eq!(string::find("foo-bar-baz", '-'), Some(3));
///
/// assert_eq!(string::find("foo-bar-baz-foo", "qux"), None);
/// assert_eq!(string::find("foo-bar-baz-foo", "foo"), Some(0));
/// assert_eq!(string::find("foo-bar-baz-foo-bar", "bar"), Some(4));
/// assert_eq!(string::find("foo-the-baz-foo-bar", "bar"), Some(16));
///
/// ```
///
#[inline]
pub const fn find<'a, P>(left: &str, pat: P) -> Option<usize>
where
    P: Pattern<'a>,
{
    let pat = PatternNorm::new(pat);
    crate::slice::bytes_find(left.as_bytes(), pat.as_bytes())
}

/// A const equivalent of
/// [`str::contains`](https://doc.rust-lang.org/std/primitive.str.html#method.contains)
/// , taking a [`Pattern`] parameter.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// assert!(string::contains("foo-bar-baz", '-'));
/// assert!(!string::contains("foo-bar-baz", 'q'));
///
/// assert!(string::contains("foo-bar-baz-foo", "foo"));
///
/// assert!( string::contains("foo-bar-baz-foo-bar", "bar"));
/// assert!(!string::contains("foo-he-baz-foo-he", "bar"));
///
/// ```
///
#[inline]
pub const fn contains<'a, P>(left: &str, pat: P) -> bool
where
    P: Pattern<'a>,
{
    let pat = PatternNorm::new(pat);
    matches!(
        crate::slice::bytes_find(left.as_bytes(), pat.as_bytes()),
        Some(_)
    )
}

/// A const equivalent of
/// [`str::rfind`](https://doc.rust-lang.org/std/primitive.str.html#method.rfind)
/// , taking a [`Pattern`] parameter.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// assert_eq!(string::rfind("bar-baz-baz", 'q'), None);
/// assert_eq!(string::rfind("bar-baz-baz", '-'), Some(7));
///
/// assert_eq!(string::rfind("bar-baz", "foo"), None);
/// assert_eq!(string::rfind("bar-baz-foo", "foo"), Some(8));
/// assert_eq!(string::rfind("foo-bar-baz", "foo"), Some(0));
///
/// ```
///
#[inline]
pub const fn rfind<'a, P>(left: &str, pat: P) -> Option<usize>
where
    P: Pattern<'a>,
{
    let pat = PatternNorm::new(pat);
    crate::slice::bytes_rfind(left.as_bytes(), pat.as_bytes())
}

/// A const equivalent of
/// [`str::contains`](https://doc.rust-lang.org/std/primitive.str.html#method.contains)
/// , taking a [`Pattern`] parameter.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// assert!(string::rcontains("foo-bar-baz", '-'));
/// assert!(!string::rcontains("foo-bar-baz", 'q'));
///
/// assert!(!string::rcontains("bar-baz", "foo"));
/// assert!(string::rcontains("foo-bar", "foo"));
///
/// ```
///
#[inline(always)]
pub const fn rcontains<'a, P>(left: &str, pat: P) -> bool
where
    P: Pattern<'a>,
{
    let pat = PatternNorm::new(pat);
    matches!(
        crate::slice::bytes_rfind(left.as_bytes(), pat.as_bytes()),
        Some(_)
    )
}

/// A const equivalent of `&string[..len]`.
///
/// If `string.len() < len`, this simply returns `string` back.
///
/// # Performance
///
/// This has the same performance as
/// [`konst::slice::slice_up_to`](../slice/fn.slice_up_to.html#performance)
///
/// # Panics
///
/// This function panics if `len` is inside the string and doesn't fall on a char boundary.
///
/// # Example
///
/// ```
/// use konst::string::str_up_to;
///
/// const STR: &str = "foo bar baz";
///
/// const SUB0: &str = str_up_to(STR, 3);
/// assert_eq!(SUB0, "foo");
///
/// const SUB1: &str = str_up_to(STR, 7);
/// assert_eq!(SUB1, "foo bar");
///
/// const SUB2: &str = str_up_to(STR, 11);
/// assert_eq!(SUB2, STR);
///
/// const SUB3: &str = str_up_to(STR, 100);
/// assert_eq!(SUB3, STR);
///
///
/// ```
pub const fn str_up_to(string: &str, len: usize) -> &str {
    let bytes = string.as_bytes();
    if is_char_boundary(bytes, len) {
        // Safety: is_char_boundary checks that `len` falls on a char boundary.
        unsafe { core::str::from_utf8_unchecked(crate::slice::slice_up_to(bytes, len)) }
    } else {
        non_char_boundary_panic("index", len)
    }
}

/// A const equivalent of `string.get(..len)`.
///
/// # Performance
///
/// This has the same performance as
/// [`konst::slice::slice_up_to`](../slice/fn.slice_up_to.html#performance)
///
/// # Example
///
/// ```
/// use konst::string;
///
/// const STR: &str = "foo bar baz";
///
/// const SUB0: Option<&str> = string::get_up_to(STR, 3);
/// assert_eq!(SUB0, Some("foo"));
///
/// const SUB1: Option<&str> = string::get_up_to(STR, 7);
/// assert_eq!(SUB1, Some("foo bar"));
///
/// const SUB2: Option<&str> = string::get_up_to(STR, 11);
/// assert_eq!(SUB2, Some(STR));
///
/// const SUB3: Option<&str> = string::get_up_to(STR, 100);
/// assert_eq!(SUB3, None);
///
///
/// ```
pub const fn get_up_to(string: &str, len: usize) -> Option<&str> {
    let bytes = string.as_bytes();

    crate::option::and_then!(
        crate::slice::get_up_to(bytes, len),
        |x| if is_char_boundary_get(bytes, len) {
            // Safety: is_char_boundary_get checks that `len` falls on a char boundary.
            unsafe { Some(core::str::from_utf8_unchecked(x)) }
        } else {
            None
        }
    )
}

/// A const equivalent of `&string[start..]`.
///
/// If `string.len() < start`, this simply returns an empty string` back.
///
/// # Performance
///
/// This has the same performance as
/// [`konst::slice::slice_from`](../slice/fn.slice_from.html#performance)
///
/// # Panics
///
/// This function panics if `start` is inside the string and doesn't fall on a char boundary.
///
/// # Example
///
/// ```
/// use konst::string::str_from;
///
/// const STR: &str = "foo bar baz";
///
/// const SUB0: &str = str_from(STR, 0);
/// assert_eq!(SUB0, STR);
///
/// const SUB1: &str = str_from(STR, 4);
/// assert_eq!(SUB1, "bar baz");
///
/// const SUB2: &str = str_from(STR, 8);
/// assert_eq!(SUB2, "baz");
///
/// const SUB3: &str = str_from(STR, 11);
/// assert_eq!(SUB3, "");
///
/// const SUB4: &str = str_from(STR, 1000);
/// assert_eq!(SUB3, "");
///
///
/// ```
pub const fn str_from(string: &str, start: usize) -> &str {
    let bytes = string.as_bytes();
    if is_char_boundary(bytes, start) {
        // Safety: is_char_boundary checks that `start` falls on a char boundary.
        unsafe { core::str::from_utf8_unchecked(crate::slice::slice_from(bytes, start)) }
    } else {
        non_char_boundary_panic("start", start)
    }
}

/// A const equivalent of `string.get(from..)`.
///
/// # Performance
///
/// This has the same performance as
/// [`konst::slice::slice_from`](../slice/fn.slice_from.html#performance)
///
/// # Example
///
/// ```
/// use konst::string;
///
/// const STR: &str = "foo bar baz";
///
/// const SUB0: Option<&str> = string::get_from(STR, 0);
/// assert_eq!(SUB0, Some(STR));
///
/// const SUB1: Option<&str> = string::get_from(STR, 4);
/// assert_eq!(SUB1, Some("bar baz"));
///
/// const SUB2: Option<&str> = string::get_from(STR, 8);
/// assert_eq!(SUB2, Some("baz"));
///
/// const SUB3: Option<&str> = string::get_from(STR, 100);
/// assert_eq!(SUB3, None);
///
///
/// ```
pub const fn get_from(string: &str, from: usize) -> Option<&str> {
    let bytes = string.as_bytes();

    crate::option::and_then!(
        crate::slice::get_from(bytes, from),
        |x| if is_char_boundary_get(bytes, from) {
            // Safety: is_char_boundary_get checks that `from` falls on a char boundary.
            unsafe { Some(core::str::from_utf8_unchecked(x)) }
        } else {
            None
        }
    )
}

/// A const equivalent of [`str::split_at`]
///
/// If `at > string.len()` this returns `(string, "")`.
///
/// # Performance
///
/// This has the same performance as [`konst::slice::split_at`](crate::slice::split_at)
///
/// # Panics
///
/// This function panics if `at` is inside the string and doesn't fall on a char boundary.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// const IN: &str = "foo bar baz";
///
/// {
///     const SPLIT0: (&str, &str) = string::split_at(IN, 0);
///     assert_eq!(SPLIT0, ("", "foo bar baz"));
/// }
/// {
///     const SPLIT1: (&str, &str) = string::split_at(IN, 4);
///     assert_eq!(SPLIT1, ("foo ", "bar baz"));
/// }
/// {
///     const SPLIT2: (&str, &str) = string::split_at(IN, 8);
///     assert_eq!(SPLIT2, ("foo bar ", "baz"));
/// }
/// {
///     const SPLIT3: (&str, &str) = string::split_at(IN, 11);
///     assert_eq!(SPLIT3, ("foo bar baz", ""));
/// }
/// {
///     const SPLIT4: (&str, &str) = string::split_at(IN, 13);
///     assert_eq!(SPLIT4, ("foo bar baz", ""));
/// }
///
/// ```
///
/// [`str::split_at`]: https://doc.rust-lang.org/std/primitive.str.html#method.split_at
pub const fn split_at(string: &str, at: usize) -> (&str, &str) {
    (str_up_to(string, at), str_from(string, at))
}

/// A const equivalent of `&string[start..end]`.
///
/// If `start >= end ` or `string.len() < start `, this returns an empty string.
///
/// If `string.len() < end`, this returns the string from `start`.
///
/// # Alternatives
///
/// For a const equivalent of `&string[start..]` there's [`str_from`].
///
/// For a const equivalent of `&string[..end]` there's [`str_up_to`].
///
/// [`str_from`]: ./fn.str_from.html
/// [`str_up_to`]: ./fn.str_up_to.html
///
/// # Performance
///
/// This has the same performance as
/// [`konst::slice::slice_range`](../slice/fn.slice_range.html#performance)
///
/// # Panics
///
/// This function panics if either `start` or `end` are inside the string and
/// don't fall on a char boundary.
///
/// # Example
///
/// ```
/// use konst::string::str_range;
///
/// const STR: &str = "foo bar baz";
///
/// const SUB0: &str = str_range(STR, 0, 3);
/// assert_eq!(SUB0, "foo");
///
/// const SUB1: &str = str_range(STR, 0, 7);
/// assert_eq!(SUB1, "foo bar");
///
/// const SUB2: &str = str_range(STR, 4, 11);
/// assert_eq!(SUB2, "bar baz");
///
/// const SUB3: &str = str_range(STR, 0, 1000);
/// assert_eq!(SUB3, STR);
///
///
/// ```
pub const fn str_range(string: &str, start: usize, end: usize) -> &str {
    let bytes = string.as_bytes();
    let start_inbounds = is_char_boundary(bytes, start);
    if start_inbounds && is_char_boundary(bytes, end) {
        // Safety: is_char_boundary checks that `start` and `end` fall on a char boundaries.
        unsafe { core::str::from_utf8_unchecked(crate::slice::slice_range(bytes, start, end)) }
    } else if start_inbounds {
        non_char_boundary_panic("end", end)
    } else {
        non_char_boundary_panic("start", start)
    }
}

/// A const equivalent of `string.get(start..end)`.
///
/// # Alternatives
///
/// For a const equivalent of `string.get(start..)` there's [`get_from`].
///
/// For a const equivalent of `string.get(..end)` there's [`get_up_to`].
///
/// [`get_from`]: ./fn.get_from.html
/// [`get_up_to`]: ./fn.get_up_to.html
///
/// # Performance
///
/// This has the same performance as
/// [`konst::slice::slice_range`](../slice/fn.slice_range.html#performance)
///
/// # Example
///
/// ```
/// use konst::string;
///
/// const STR: &str = "foo bar baz";
///
/// const SUB0: Option<&str> = string::get_range(STR, 0, 3);
/// assert_eq!(SUB0, Some("foo"));
///
/// const SUB1: Option<&str> = string::get_range(STR, 0, 7);
/// assert_eq!(SUB1, Some("foo bar"));
///
/// const SUB2: Option<&str> = string::get_range(STR, 4, 11);
/// assert_eq!(SUB2, Some("bar baz"));
///
/// const SUB3: Option<&str> = string::get_range(STR, 0, 1000);
/// assert_eq!(SUB3, None);
///
///
/// ```
pub const fn get_range(string: &str, start: usize, end: usize) -> Option<&str> {
    let bytes = string.as_bytes();

    crate::option::and_then!(
        crate::slice::get_range(bytes, start, end),
        |x| if is_char_boundary_get(bytes, start) && is_char_boundary_get(bytes, end) {
            // Safety: is_char_boundary_get checks that `start` and `end` fall on a char boundary.
            unsafe { Some(core::str::from_utf8_unchecked(x)) }
        } else {
            None
        }
    )
}

/// A const subset of [`str::strip_prefix`].
///
/// This takes [`Pattern`] implementors as the pattern.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// {
///     const STRIP: Option<&str> = string::strip_prefix("--5 8", '-');
///     assert_eq!(STRIP, Some("-5 8"));
/// }
/// {
///     const STRIP: Option<&str> = string::strip_prefix("--5 8", '_');
///     assert_eq!(STRIP, None);
/// }
///
/// {
///     const STRIP: Option<&str> = string::strip_prefix("33 5 8", "3");
///     assert_eq!(STRIP, Some("3 5 8"));
/// }
/// {
///     const STRIP: Option<&str> = string::strip_prefix("3 5 8", "hello");
///     assert_eq!(STRIP, None);
/// }
///
///
/// ```
///
/// [`str::strip_prefix`]: https://doc.rust-lang.org/std/primitive.str.html#method.strip_prefix
pub const fn strip_prefix<'a, 'p, P>(string: &'a str, pattern: P) -> Option<&'a str>
where
    P: Pattern<'p>,
{
    let pat = PatternNorm::new(pattern);

    // Safety: because `pat` is a `Pattern`, removing it should result in a valid `&str`
    unsafe {
        crate::option::map!(
            crate::slice::bytes_strip_prefix(string.as_bytes(), pat.as_bytes()),
            core::str::from_utf8_unchecked,
        )
    }
}

/// A const subset of [`str::strip_suffix`].
///
/// This takes [`Pattern`] implementors as the pattern.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// {
///     const STRIP: Option<&str> = string::strip_suffix("3 5 8--", '-');
///     assert_eq!(STRIP, Some("3 5 8-"));
/// }
/// {
///     const STRIP: Option<&str> = string::strip_suffix("3 5 8", '_');
///     assert_eq!(STRIP, None);
/// }
///
/// {
///     const STRIP: Option<&str> = string::strip_suffix("3 5 6868", "68");
///     assert_eq!(STRIP, Some("3 5 68"));
/// }
/// {
///     const STRIP: Option<&str> = string::strip_suffix("3 5 8", "hello");
///     assert_eq!(STRIP, None);
/// }
///
///
/// ```
///
pub const fn strip_suffix<'a, 'p, P>(string: &'a str, pattern: P) -> Option<&'a str>
where
    P: Pattern<'p>,
{
    let pat = PatternNorm::new(pattern);

    // Safety: because `suffix` is a `&str`, removing it should result in a valid `&str`
    unsafe {
        crate::option::map!(
            crate::slice::bytes_strip_suffix(string.as_bytes(), pat.as_bytes()),
            core::str::from_utf8_unchecked,
        )
    }
}

const fn is_char_boundary(bytes: &[u8], position: usize) -> bool {
    position >= bytes.len() || (bytes[position] as i8) >= -0x40
}

const fn is_char_boundary_get(bytes: &[u8], position: usize) -> bool {
    let len = bytes.len();

    position == len || (bytes[position] as i8) >= -0x40
}

const fn find_next_char_boundary(bytes: &[u8], mut position: usize) -> usize {
    loop {
        position += 1;

        if is_char_boundary(bytes, position) {
            break position;
        }
    }
}

const fn find_prev_char_boundary(bytes: &[u8], mut position: usize) -> usize {
    position = position.saturating_sub(1);

    while !is_char_boundary(bytes, position) {
        position -= 1;
    }

    position
}

/// A const subset of [`str::trim`] which only removes ascii whitespace.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// const TRIMMED: &str = string::trim("\nhello world  ");
///
/// assert_eq!(TRIMMED, "hello world");
///
/// ```
pub const fn trim(this: &str) -> &str {
    let trimmed = crate::slice::bytes_trim(this.as_bytes());
    // safety: bytes_trim only removes ascii bytes
    unsafe { core::str::from_utf8_unchecked(trimmed) }
}

/// A const subset of [`str::trim_start`] which only removes ascii whitespace.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// const TRIMMED: &str = string::trim_start("\rfoo bar  ");
///
/// assert_eq!(TRIMMED, "foo bar  ");
///
/// ```
pub const fn trim_start(this: &str) -> &str {
    let trimmed = crate::slice::bytes_trim_start(this.as_bytes());
    // safety: bytes_trim_start only removes ascii bytes
    unsafe { core::str::from_utf8_unchecked(trimmed) }
}

/// A const subset of [`str::trim_end`] which only removes ascii whitespace.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// const TRIMMED: &str = string::trim_end("\rfoo bar  ");
///
/// assert_eq!(TRIMMED, "\rfoo bar");
///
/// ```
///
pub const fn trim_end(this: &str) -> &str {
    let trimmed = crate::slice::bytes_trim_end(this.as_bytes());
    // safety: bytes_trim_end only removes ascii bytes
    unsafe { core::str::from_utf8_unchecked(trimmed) }
}

/// A const subset of [`str::trim_matches`].
///
/// This takes [`Pattern`] implementors as the needle.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// const CHAR_TRIMMED: &str = string::trim_matches("---baz qux---", '-');
/// const STR_TRIMMED: &str = string::trim_matches("<>baz qux<><><>", "<>");
///
/// assert_eq!(CHAR_TRIMMED, "baz qux");
/// assert_eq!(STR_TRIMMED, "baz qux");
///
/// ```
pub const fn trim_matches<'a, 'p, P>(this: &'a str, needle: P) -> &'a str
where
    P: Pattern<'p>,
{
    let needle = PatternNorm::new(needle);
    let trimmed = crate::slice::bytes_trim_matches(this.as_bytes(), needle.as_bytes());
    // safety:
    // because bytes_trim_matches was passed `&str`s casted to `&[u8]`s,
    // it returns a valid utf8 sequence.
    unsafe { core::str::from_utf8_unchecked(trimmed) }
}

/// A const subset of [`str::trim_start_matches`].
///
/// This takes [`Pattern`] implementors as the needle.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// const CHAR_TRIMMED: &str = string::trim_start_matches("#####huh###", '#');
/// const STR_TRIMMED: &str = string::trim_start_matches("#####huh###", "##");
///
/// assert_eq!(CHAR_TRIMMED, "huh###");
/// assert_eq!(STR_TRIMMED, "#huh###");
///
/// ```
pub const fn trim_start_matches<'a, 'p, P>(this: &'a str, needle: P) -> &'a str
where
    P: Pattern<'p>,
{
    let needle = PatternNorm::new(needle);
    let trimmed = crate::slice::bytes_trim_start_matches(this.as_bytes(), needle.as_bytes());
    // safety:
    // because bytes_trim_start_matches was passed `&str`s casted to `&[u8]`s,
    // it returns a valid utf8 sequence.
    unsafe { core::str::from_utf8_unchecked(trimmed) }
}

/// A const subset of [`str::trim_end_matches`].
///
/// This takes [`Pattern`] implementors as the needle.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// const CHAR_TRIMMED: &str = string::trim_end_matches("oowowooooo", 'o');
/// const STR_TRIMMED: &str = string::trim_end_matches("oowowooooo", "oo");
///
/// assert_eq!(CHAR_TRIMMED, "oowow");
/// assert_eq!(STR_TRIMMED, "oowowo");
///
/// ```
pub const fn trim_end_matches<'a, 'p, P>(this: &'a str, needle: P) -> &'a str
where
    P: Pattern<'p>,
{
    let needle = PatternNorm::new(needle);
    let trimmed = crate::slice::bytes_trim_end_matches(this.as_bytes(), needle.as_bytes());
    // safety:
    // because bytes_trim_end_matches was passed `&str`s casted to `&[u8]`s,
    // it returns a valid utf8 sequence.
    unsafe { core::str::from_utf8_unchecked(trimmed) }
}

/// Advances `this` past the first instance of `needle`.
///
/// Return `None` if no instance of `needle` is found.
///
/// Return `Some(this)` if `needle` is empty.
///
/// This takes [`Pattern`] implementors as the needle.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// {
///     const FOUND: Option<&str> = string::find_skip("foo bar baz", ' ');
///     assert_eq!(FOUND, Some("bar baz"));
/// }
///
/// {
///     const FOUND: Option<&str> = string::find_skip("foo bar baz", "bar");
///     assert_eq!(FOUND, Some(" baz"));
/// }
/// {
///     const NOT_FOUND: Option<&str> = string::find_skip("foo bar baz", "qux");
///     assert_eq!(NOT_FOUND, None);
/// }
/// ```
pub const fn find_skip<'a, 'p, P>(this: &'a str, needle: P) -> Option<&'a str>
where
    P: Pattern<'p>,
{
    let needle = PatternNorm::new(needle);
    unsafe {
        crate::option::map!(
            crate::slice::bytes_find_skip(this.as_bytes(), needle.as_bytes()),
            // safety:
            // because bytes_find_skip was passed `&str`s casted to `&[u8]`s,
            // it returns a valid utf8 sequence.
            core::str::from_utf8_unchecked,
        )
    }
}

/// Advances `this` up to the first instance of `needle`.
///
/// Return `None` if no instance of `needle` is found.
///
/// Return `Some(this)` if `needle` is empty.
///
/// This takes [`Pattern`] implementors as the needle.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// {
///     const FOUND: Option<&str> = string::find_keep("foo-bar-baz", '-');
///     assert_eq!(FOUND, Some("-bar-baz"));
/// }
///
/// {
///     const FOUND: Option<&str> = string::find_keep("foo bar baz", "bar");
///     assert_eq!(FOUND, Some("bar baz"));
/// }
/// {
///     const NOT_FOUND: Option<&str> = string::find_keep("foo bar baz", "qux");
///     assert_eq!(NOT_FOUND, None);
/// }
/// ```
pub const fn find_keep<'a, 'p, P>(this: &'a str, needle: P) -> Option<&'a str>
where
    P: Pattern<'p>,
{
    let needle = PatternNorm::new(needle);
    unsafe {
        crate::option::map!(
            crate::slice::bytes_find_keep(this.as_bytes(), needle.as_bytes()),
            // safety:
            // because bytes_find_keep was passed `&str`s casted to `&[u8]`s,
            // it returns a valid utf8 sequence.
            core::str::from_utf8_unchecked,
        )
    }
}

/// Truncates `this` to before the last instance of `needle`.
///
/// Return `None` if no instance of `needle` is found.
///
/// Return `Some(this)` if `needle` is empty.
///
/// This takes [`Pattern`] implementors as the needle.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// {
///     const FOUND: Option<&str> = string::rfind_skip("foo bar _ bar baz", '_');
///     assert_eq!(FOUND, Some("foo bar "));
/// }
///
/// {
///     const FOUND: Option<&str> = string::rfind_skip("foo bar _ bar baz", "bar");
///     assert_eq!(FOUND, Some("foo bar _ "));
/// }
/// {
///     const NOT_FOUND: Option<&str> = string::rfind_skip("foo bar baz", "qux");
///     assert_eq!(NOT_FOUND, None);
/// }
/// ```
pub const fn rfind_skip<'a, 'p, P>(this: &'a str, needle: P) -> Option<&'a str>
where
    P: Pattern<'p>,
{
    let needle = PatternNorm::new(needle);
    unsafe {
        crate::option::map!(
            crate::slice::bytes_rfind_skip(this.as_bytes(), needle.as_bytes()),
            // safety:
            // because bytes_rfind_skip was passed `&str`s casted to `&[u8]`s,
            // it returns a valid utf8 sequence.
            core::str::from_utf8_unchecked,
        )
    }
}

/// Truncates `this` to the last instance of `needle`.
///
/// Return `None` if no instance of `needle` is found.
///
/// Return `Some(this)` if `needle` is empty.
///
/// This takes [`Pattern`] implementors as the needle.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// {
///     const FOUND: Option<&str> = string::rfind_keep("foo bar _ bar baz", '_');
///     assert_eq!(FOUND, Some("foo bar _"));
/// }
///
/// {
///     const FOUND: Option<&str> = string::rfind_keep("foo bar _ bar baz", "bar");
///     assert_eq!(FOUND, Some("foo bar _ bar"));
/// }
/// {
///     const NOT_FOUND: Option<&str> = string::rfind_keep("foo bar baz", "qux");
///     assert_eq!(NOT_FOUND, None);
/// }
/// ```
pub const fn rfind_keep<'a, 'p, P>(this: &'a str, needle: P) -> Option<&'a str>
where
    P: Pattern<'p>,
{
    let needle = PatternNorm::new(needle);
    unsafe {
        crate::option::map!(
            crate::slice::bytes_rfind_keep(this.as_bytes(), needle.as_bytes()),
            // safety:
            // because bytes_rfind_keep was passed `&str`s casted to `&[u8]`s,
            // it returns a valid utf8 sequence.
            core::str::from_utf8_unchecked,
        )
    }
}

#[cold]
#[track_caller]
const fn non_char_boundary_panic(extreme: &str, index: usize) -> ! {
    use const_panic::{FmtArg, PanicVal};

    const_panic::concat_panic(&[&[
        PanicVal::write_str(extreme),
        PanicVal::write_str(" `"),
        PanicVal::from_usize(index, FmtArg::DEBUG),
        PanicVal::write_str("` is not on a char boundary"),
    ]])
}
