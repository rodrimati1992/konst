//! `const fn` equivalents of `str` methods.

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

#[doc(hidden)]
pub use konst_macro_rules::string::check_utf8 as __priv_check_utf8;

/// A const equivalent of [`std::str::from_utf8`],
/// usable *only in `const`s and `static`s.
///
/// \* This can be used in `const fn`s when the
/// `"rust_1_55"` feature is enabled.
///
/// # Example
///
/// ```rust
/// use konst::{string, unwrap_ctx};
///
/// const OK: &str = unwrap_ctx!(string::from_utf8!(b"foo bar"));
/// assert_eq!(OK, "foo bar");
///
/// const ERR: Result<&str, string::Utf8Error> = string::from_utf8!(b"what\xFA");
/// assert_eq!(ERR.unwrap_err().valid_up_to(), 4);
///
/// ```
///
/// [`std::str::from_utf8`]: https://doc.rust-lang.org/std/str/fn.from_utf8.html
pub use konst_macro_rules::from_utf8_macro as from_utf8;

/// A const equivalent of [`std::str::from_utf8`],
/// requires Rust 1.55 and the `"rust_1_55"` feature.
///
/// For an alternative that works in Rust 1.46.0,
/// there is the [`from_utf8`](./macro.from_utf8.html) macro,
/// but it can only be used in `const`s, not in `const fn`s .
///
/// # Example
///
/// ```rust
/// use konst::{string, unwrap_ctx};
///
/// const OK: &str = unwrap_ctx!(string::from_utf8(b"hello world"));
/// assert_eq!(OK, "hello world");
///
/// const ERR: Result<&str, string::Utf8Error> = string::from_utf8(&[32, 34, 255]);
/// assert_eq!(ERR.unwrap_err().valid_up_to(), 2);
///
/// ```
///
/// [`std::str::from_utf8`]: https://doc.rust-lang.org/std/str/fn.from_utf8.html
#[cfg(feature = "rust_1_55")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_55")))]
pub use konst_macro_rules::string::from_utf8_fn as from_utf8;

/// Error returned by the `from_utf8` [function](fn.from_utf8.html) and
/// [macro](macro.from_utf8.html) when the
/// input byte slice isn't valid utf8.
pub use konst_macro_rules::string::Utf8Error;

/// A const equivalent of
/// [`str::starts_with`](https://doc.rust-lang.org/std/primitive.str.html#method.starts_with)
/// , taking a `&str` parameter.
///
/// # Example
///
/// ```rust
/// use konst::string::str_starts_with;
///
/// assert!( str_starts_with("foo,bar,baz", "foo,"));
///
/// assert!(!str_starts_with("foo,bar,baz", "bar"));
/// assert!(!str_starts_with("foo,bar,baz", "baz"));
///
/// ```
///
#[inline(always)]
pub const fn str_starts_with(left: &str, right: &str) -> bool {
    crate::slice::bytes_start_with(left.as_bytes(), right.as_bytes())
}

/// A const equivalent of
/// [`str::ends_with`](https://doc.rust-lang.org/std/primitive.str.html#method.ends_with)
/// , taking a `&str` parameter.
///
/// # Example
///
/// ```rust
/// use konst::string::str_ends_with;
///
/// assert!( str_ends_with("foo,bar,baz", ",baz"));
///
/// assert!(!str_ends_with("foo,bar,baz", "bar"));
/// assert!(!str_ends_with("foo,bar,baz", "foo"));
///
/// ```
///
#[inline(always)]
pub const fn str_ends_with(left: &str, right: &str) -> bool {
    crate::slice::bytes_end_with(left.as_bytes(), right.as_bytes())
}

/// A const equivalent of
/// [`str::find`](https://doc.rust-lang.org/std/primitive.str.html#method.find)
/// , taking a `&str` parameter, searching in `&left[from..]`.
///
/// # Example
///
/// ```rust
/// use konst::string::str_find;
///
/// assert_eq!(str_find("foo-bar-baz-foo", "foo", 0), Some(0));
/// assert_eq!(str_find("foo-bar-baz-foo", "foo", 4), Some(12));
///
/// assert_eq!(str_find("foo-bar-baz-foo-bar", "bar", 0), Some(4));
/// assert_eq!(str_find("foo-bar-baz-foo-bar", "bar", 4), Some(4));
/// assert_eq!(str_find("foo-bar-baz-foo-bar", "bar", 5), Some(16));
/// assert_eq!(str_find("foo-bar-baz-foo-bar", "bar", 16), Some(16));
/// assert_eq!(str_find("foo-bar-baz-foo-bar", "bar", 17), None);
///
/// ```
///
#[inline]
pub const fn str_find(left: &str, right: &str, from: usize) -> Option<usize> {
    crate::slice::bytes_find(left.as_bytes(), right.as_bytes(), from)
}

/// A const equivalent of
/// [`str::contains`](https://doc.rust-lang.org/std/primitive.str.html#method.contains)
/// , taking a `&str` parameter, searching in `&left[from..]`.
///
/// # Example
///
/// ```rust
/// use konst::string::str_contains;
///
/// assert!(str_contains("foo-bar-baz-foo", "foo", 0));
/// assert!(str_contains("foo-bar-baz-foo", "foo", 4));
///
/// assert!( str_contains("foo-bar-baz-foo-bar", "bar", 0));
/// assert!( str_contains("foo-bar-baz-foo-bar", "bar", 4));
/// assert!( str_contains("foo-bar-baz-foo-bar", "bar", 5));
/// assert!( str_contains("foo-bar-baz-foo-bar", "bar", 16));
/// assert!(!str_contains("foo-bar-baz-foo-bar", "bar", 17));
///
/// ```
///
#[inline(always)]
pub const fn str_contains(left: &str, right: &str, from: usize) -> bool {
    matches!(
        crate::slice::bytes_find(left.as_bytes(), right.as_bytes(), from),
        Some(_)
    )
}

/// A const equivalent of
/// [`str::rfind`](https://doc.rust-lang.org/std/primitive.str.html#method.rfind)
/// , taking a `&str` parameter, searching in `&left[..=from]`.
///
/// You can pass `usize::MAX` as the `from` argument to search from the end of `left`
/// regardless of its length.
///
/// # Example
///
/// ```rust
/// use konst::string::str_rfind;
///
/// assert_eq!(str_rfind("foo-bar-baz-foo", "foo", 0), None);
/// assert_eq!(str_rfind("foo-bar-baz-foo", "foo", 1), None);
///
/// assert_eq!(str_rfind("foo-bar-baz-foo", "foo", 2), Some(0));
/// assert_eq!(str_rfind("foo-bar-baz-foo", "foo", 3), Some(0));
/// assert_eq!(str_rfind("foo-bar-baz-foo", "foo", 4), Some(0));
///
/// assert_eq!(str_rfind("foo-bar-baz-foo", "foo", 15), Some(12));
/// assert_eq!(str_rfind("foo-bar-baz-foo", "foo", 20000), Some(12));
///
/// ```
///
#[inline]
pub const fn str_rfind(left: &str, right: &str, from: usize) -> Option<usize> {
    crate::slice::bytes_rfind(left.as_bytes(), right.as_bytes(), from)
}

/// A const equivalent of
/// [`str::contains`](https://doc.rust-lang.org/std/primitive.str.html#method.contains)
/// , taking a `&str` parameter, searching in `&left[..=from]` from the end.
///
/// You can pass `usize::MAX` as the `from` argument to search from the end of `left`
/// regardless of its length.
///
/// # Example
///
/// ```rust
/// use konst::string::str_rcontains;
///
/// assert!(!str_rcontains("foo-bar-baz-foo", "foo", 0));
/// assert!(!str_rcontains("foo-bar-baz-foo", "foo", 1));
///
/// assert!(str_rcontains("foo-bar-baz-foo", "foo", 2));
/// assert!(str_rcontains("foo-bar-baz-foo", "foo", 3));
/// assert!(str_rcontains("foo-bar-baz-foo", "foo", 4));
///
/// assert!(str_rcontains("foo-bar-baz-foo", "foo", 15));
/// assert!(str_rcontains("foo-bar-baz-foo", "foo", 20000));
///
/// ```
///
#[inline(always)]
pub const fn str_rcontains(left: &str, right: &str, from: usize) -> bool {
    matches!(
        crate::slice::bytes_rfind(left.as_bytes(), right.as_bytes(), from),
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
/// [`crate::slice::slice_up_to`](../slice/fn.slice_up_to.html#performance)
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
#[cfg(feature = "rust_1_55")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_55")))]
pub const fn str_up_to(string: &str, len: usize) -> &str {
    let bytes = string.as_bytes();
    if is_char_boundary(bytes, len) {
        // Safety: is_char_boundary checks that `len` falls on a char boundary.
        unsafe { core::str::from_utf8_unchecked(crate::slice::slice_up_to(bytes, len)) }
    } else {
        [/* len is not on a char boundary */][len]
    }
}

/// A const equivalent of `string.get(..len)`.
///
/// # Performance
///
/// This has the same performance as
/// [`crate::slice::slice_up_to`](../slice/fn.slice_up_to.html#performance)
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
#[cfg(feature = "rust_1_55")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_55")))]
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
/// If `string.len() < start`, this simply returns `string` back.
///
/// # Performance
///
/// This has the same performance as
/// [`crate::slice::slice_from`](../slice/fn.slice_from.html#performance)
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
#[cfg(feature = "rust_1_55")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_55")))]
pub const fn str_from(string: &str, start: usize) -> &str {
    let bytes = string.as_bytes();
    if is_char_boundary(bytes, start) {
        // Safety: is_char_boundary checks that `start` falls on a char boundary.
        unsafe { core::str::from_utf8_unchecked(crate::slice::slice_from(bytes, start)) }
    } else {
        [/* start is not on a char boundary */][start]
    }
}

/// A const equivalent of `string.get(from..)`.
///
/// # Performance
///
/// This has the same performance as
/// [`crate::slice::slice_from`](../slice/fn.slice_from.html#performance)
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
#[cfg(feature = "rust_1_55")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_55")))]
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
#[cfg(feature = "rust_1_55")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_55")))]
pub const fn split_at(string: &str, at: usize) -> (&str, &str) {
    (str_up_to(string, at), str_from(string, at))
}

/// A const equivalent of `&string[start..end]`.
///
/// If `string.len() < start` or `string.len() < end`, this simply returns `string` back.
///
/// # Performance
///
/// This has the same performance as
/// [`crate::slice::slice_range`](../slice/fn.slice_range.html#performance)
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
#[cfg(feature = "rust_1_55")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_55")))]
pub const fn str_range(string: &str, start: usize, end: usize) -> &str {
    let bytes = string.as_bytes();
    let start_inbounds = is_char_boundary(bytes, start);
    if start_inbounds && is_char_boundary(bytes, end) {
        // Safety: is_char_boundary checks that `start` and `end` fall on a char boundaries.
        unsafe { core::str::from_utf8_unchecked(crate::slice::slice_range(bytes, start, end)) }
    } else if start_inbounds {
        [/* end is not on a char boundary */][end]
    } else {
        [/* start is not on a char boundary */][start]
    }
}

/// A const equivalent of `string.get(start..end)`.
///
/// # Performance
///
/// This has the same performance as
/// [`crate::slice::slice_range`](../slice/fn.slice_range.html#performance)
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
#[cfg(feature = "rust_1_55")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_55")))]
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

/// A const subset of [`str::strip_prefix`], this only takes a `&str` pattern.
///
/// # Example
///
/// ```rust
/// use konst::string::str_strip_prefix;
///
/// {
///     const STRIP: Option<&str> = str_strip_prefix("3 5 8", "3");
///     assert_eq!(STRIP, Some(" 5 8"));
/// }
/// {
///     const STRIP: Option<&str> = str_strip_prefix("3 5 8", "3 5 ");
///     assert_eq!(STRIP, Some("8"));
/// }
/// {
///     const STRIP: Option<&str> = str_strip_prefix("3 5 8", "hello");
///     assert_eq!(STRIP, None);
/// }
///
///
/// ```
///
/// [`str::strip_prefix`]: https://doc.rust-lang.org/std/primitive.str.html#method.strip_prefix
#[cfg(feature = "rust_1_55")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_55")))]
pub const fn str_strip_prefix<'a>(string: &'a str, prefix: &str) -> Option<&'a str> {
    // Safety: because `prefix` is a `&str`, removing it should result in a valid `&str`
    unsafe {
        crate::option::map!(
            crate::slice::bytes_strip_prefix(string.as_bytes(), prefix.as_bytes()),
            core::str::from_utf8_unchecked,
        )
    }
}

/// A const subset of [`str::strip_suffix`], this only takes a `&str` pattern.
///
/// # Example
///
/// ```rust
/// use konst::string::str_strip_suffix;
///
/// {
///     const STRIP: Option<&str> = str_strip_suffix("3 5 8", "8");
///     assert_eq!(STRIP, Some("3 5 "));
/// }
/// {
///     const STRIP: Option<&str> = str_strip_suffix("3 5 8", " 5 8");
///     assert_eq!(STRIP, Some("3"));
/// }
/// {
///     const STRIP: Option<&str> = str_strip_suffix("3 5 8", "hello");
///     assert_eq!(STRIP, None);
/// }
///
///
/// ```
///
#[cfg(feature = "rust_1_55")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_55")))]
pub const fn str_strip_suffix<'a>(string: &'a str, suffix: &str) -> Option<&'a str> {
    // Safety: because `suffix` is a `&str`, removing it should result in a valid `&str`
    unsafe {
        crate::option::map!(
            crate::slice::bytes_strip_suffix(string.as_bytes(), suffix.as_bytes()),
            core::str::from_utf8_unchecked,
        )
    }
}

#[cfg(feature = "rust_1_55")]
const fn is_char_boundary(bytes: &[u8], position: usize) -> bool {
    position >= bytes.len() || (bytes[position] as i8) >= -0x40
}

#[cfg(feature = "rust_1_55")]
const fn is_char_boundary_get(bytes: &[u8], position: usize) -> bool {
    let len = bytes.len();

    position == len || (bytes[position] as i8) >= -0x40
}
