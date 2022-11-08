//! `const fn` equivalents of `str` methods.
//!

#[cfg(feature = "iter")]
mod chars_methods;

#[cfg(feature = "iter")]
pub use chars_methods::*;

mod concatenation;

pub use concatenation::*;

#[cfg(test)]
mod priv_string_tests;

mod pattern;

use core::fmt::{self, Debug, Display};

pub use self::pattern::Pattern;

pub(crate) use self::pattern::PatternNorm;

mod split_once;

pub use split_once::*;

#[cfg(feature = "iter")]
mod splitting;

#[cfg(feature = "iter")]
pub use splitting::*;

#[cfg(feature = "iter")]
mod split_terminator_items;

#[cfg(feature = "iter")]
pub use split_terminator_items::*;

use konst_kernel::string::__is_char_boundary_bytes;

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
        eq_comparison = crate::cmp::CmpWrapper(l).const_eq(r),
        cmp_comparison = crate::cmp::CmpWrapper(l).const_cmp(r),
        parameter_copyability = copy,
    ),
}

/// Delegates to [`core::str::from_utf8`],
/// wrapping the error to provide a `panic` method for use in [`unwrap_ctx`]
///
/// # Example
///
/// ### Basic
///
/// ```rust
/// use konst::{
///     result::unwrap_ctx,
///     string,
/// };
///
/// const STR: &str = unwrap_ctx!(string::from_utf8(b"foo bar"));
///
/// assert_eq!(STR, "foo bar")
/// ```
///
/// ### Compile-time error
///
/// ```compile_fail
/// use konst::{
///     result::unwrap_ctx,
///     string,
/// };
///
/// const _: &str = unwrap_ctx!(string::from_utf8(&[255, 255, 255]));
/// ```
///
/// ```text
/// error[E0080]: evaluation of constant value failed
///  --> src/string.rs:88:17
///   |
/// 9 | const _: &str = unwrap_ctx!(string::from_utf8(&[255, 255, 255]));
///   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'invalid utf-8 sequence of 1 bytes from index 0', src/string.rs:9:17
///   |
///   = note: this error originates in the macro `unwrap_ctx` (in Nightly builds, run with -Z macro-backtrace for more info)
///
/// ```
///
/// [`unwrap_ctx`]: crate::result::unwrap_ctx
pub const fn from_utf8(slice: &[u8]) -> Result<&str, Utf8Error> {
    match core::str::from_utf8(slice) {
        Ok(x) => Ok(x),
        Err(e) => Err(Utf8Error(e)),
    }
}

/// Wrapper around [`core::str::Utf8Error`]
/// to provide a `panic` method for use in [`unwrap_ctx`],
/// returned by [`from_utf8`](crate::string::from_utf8).
///
/// [`unwrap_ctx`]: crate::result::unwrap_ctx
#[derive(Copy, Clone)]
pub struct Utf8Error(pub core::str::Utf8Error);

impl Utf8Error {
    /// Panics with a `Display` formatted error message
    #[track_caller]
    pub const fn panic(self) -> ! {
        let pvs = const_panic::StdWrapper(&self.0).to_panicvals(const_panic::FmtArg::DISPLAY);
        const_panic::concat_panic(&[&pvs])
    }
}

impl Debug for Utf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Utf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
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
    crate::slice::__bytes_start_with(left.as_bytes(), pat.as_bytes())
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
    crate::slice::__bytes_end_with(left.as_bytes(), pat.as_bytes())
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
    crate::slice::__bytes_find(left.as_bytes(), pat.as_bytes())
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
        crate::slice::__bytes_find(left.as_bytes(), pat.as_bytes()),
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
    crate::slice::__bytes_rfind(left.as_bytes(), pat.as_bytes())
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
        crate::slice::__bytes_rfind(left.as_bytes(), pat.as_bytes()),
        Some(_)
    )
}

/// A const equivalent of `&string[..len]`.
///
/// If `string.len() < len`, this simply returns `string` back.
///
/// # Panics
///
/// This function panics if `len` is inside the string but doesn't fall on a char boundary.
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
#[doc(inline)]
pub use konst_kernel::string::str_up_to;

/// A const equivalent of `&string[start..]`.
///
/// If `string.len() < start`, this simply returns an empty string` back.
///
/// # Panics
///
/// This function panics if `start` is inside the string but doesn't fall on a char boundary.
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
#[doc(inline)]
pub use konst_kernel::string::str_from;

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
#[doc(inline)]
pub use konst_kernel::string::str_range;

/// Const equivalent of [`str::is_char_boundary`].
///
/// # Example
///
/// ```
/// use konst::string::is_char_boundary;
///
/// let string =  "锈 is 🧠";
///
/// // Start of "锈"
/// assert!(is_char_boundary(string, 0));
/// assert!(!is_char_boundary(string, 1));
/// assert!(!is_char_boundary(string, 2));
///
/// // start of " "
/// assert!(is_char_boundary(string, 3));
///
/// // start of "🧠"
/// assert!(is_char_boundary(string, 7));
/// assert!(!is_char_boundary(string, 8));
///
/// // end of string
/// assert!(is_char_boundary(string, string.len()));
///
/// // after end of string
/// assert!(!is_char_boundary(string, string.len() + 1));
///
///
/// ```
#[doc(inline)]
pub use konst_kernel::string::is_char_boundary;

/// Checks that the start and end are valid utf8 char boundaries
/// when the `"debug"` feature is enabled.
///
/// When the `"debug"` feature is disabled,
/// this is equivalent to calling `core::str::from_utf8_unchecled`
///
/// # Safety
///
/// The input byte slice must be a subslice of a `&str`,
/// so that only the start and end need to be checked.
#[doc(inline)]
pub use konst_kernel::string::__from_u8_subslice_of_str;

/// A const equivalent of `string.get(..len)`.
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
        |x| if __is_char_boundary_bytes(bytes, len) {
            // Safety: __is_char_boundary_bytes checks that `len` falls on a char boundary.
            unsafe { Some(__from_u8_subslice_of_str(x)) }
        } else {
            None
        }
    )
}

/// A const equivalent of `string.get(from..)`.
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
        |x| if __is_char_boundary_bytes(bytes, from) {
            // Safety: __is_char_boundary_bytes checks that `from` falls on a char boundary.
            unsafe { Some(__from_u8_subslice_of_str(x)) }
        } else {
            None
        }
    )
}

/// A const equivalent of [`str::split_at`]
///
/// If `at > string.len()` this returns `(string, "")`.
///
/// # Panics
///
/// This function panics if `at` is inside the string but doesn't fall on a char boundary.
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

    crate::option::and_then!(crate::slice::get_range(bytes, start, end), |x| {
        if __is_char_boundary_bytes(bytes, start) && __is_char_boundary_bytes(bytes, end) {
            // Safety: __is_char_boundary_bytes checks that `start` and `end` fall on a char boundary.
            unsafe { Some(__from_u8_subslice_of_str(x)) }
        } else {
            None
        }
    })
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
            crate::slice::__bytes_strip_prefix(string.as_bytes(), pat.as_bytes()),
            __from_u8_subslice_of_str,
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
            crate::slice::__bytes_strip_suffix(string.as_bytes(), pat.as_bytes()),
            __from_u8_subslice_of_str,
        )
    }
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
    unsafe { __from_u8_subslice_of_str(trimmed) }
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
    unsafe { __from_u8_subslice_of_str(trimmed) }
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
    unsafe { __from_u8_subslice_of_str(trimmed) }
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
    let trimmed = crate::slice::__bytes_trim_matches(this.as_bytes(), needle.as_bytes());
    // safety:
    // because bytes_trim_matches was passed `&str`s casted to `&[u8]`s,
    // it returns a valid utf8 sequence.
    unsafe { __from_u8_subslice_of_str(trimmed) }
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
    let trimmed = crate::slice::__bytes_trim_start_matches(this.as_bytes(), needle.as_bytes());
    // safety:
    // because bytes_trim_start_matches was passed `&str`s casted to `&[u8]`s,
    // it returns a valid utf8 sequence.
    unsafe { __from_u8_subslice_of_str(trimmed) }
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
    let trimmed = crate::slice::__bytes_trim_end_matches(this.as_bytes(), needle.as_bytes());
    // safety:
    // because bytes_trim_end_matches was passed `&str`s casted to `&[u8]`s,
    // it returns a valid utf8 sequence.
    unsafe { __from_u8_subslice_of_str(trimmed) }
}

/// Advances `this` past the first instance of `needle`.
///
/// Returns `None` if no instance of `needle` is found.
///
/// Returns `Some(this)` if `needle` is empty.
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
            crate::slice::__bytes_find_skip(this.as_bytes(), needle.as_bytes()),
            // safety:
            // because bytes_find_skip was passed `&str`s casted to `&[u8]`s,
            // it returns a valid utf8 sequence.
            __from_u8_subslice_of_str,
        )
    }
}

/// Advances `this` up to the first instance of `needle`.
///
/// Returns `None` if no instance of `needle` is found.
///
/// Returns `Some(this)` if `needle` is empty.
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
            crate::slice::__bytes_find_keep(this.as_bytes(), needle.as_bytes()),
            // safety:
            // because bytes_find_keep was passed `&str`s casted to `&[u8]`s,
            // it returns a valid utf8 sequence.
            __from_u8_subslice_of_str,
        )
    }
}

/// Truncates `this` to before the last instance of `needle`.
///
/// Returns `None` if no instance of `needle` is found.
///
/// Returns `Some(this)` if `needle` is empty.
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
            crate::slice::__bytes_rfind_skip(this.as_bytes(), needle.as_bytes()),
            // safety:
            // because bytes_rfind_skip was passed `&str`s casted to `&[u8]`s,
            // it returns a valid utf8 sequence.
            __from_u8_subslice_of_str,
        )
    }
}

/// Truncates `this` to the last instance of `needle`.
///
/// Returns `None` if no instance of `needle` is found.
///
/// Returns `Some(this)` if `needle` is empty.
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
            crate::slice::__bytes_rfind_keep(this.as_bytes(), needle.as_bytes()),
            // safety:
            // because bytes_rfind_keep was passed `&str`s casted to `&[u8]`s,
            // it returns a valid utf8 sequence.
            __from_u8_subslice_of_str,
        )
    }
}
