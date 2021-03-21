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
