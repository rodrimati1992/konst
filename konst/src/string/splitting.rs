//! string splitting that requires Rust 1.64.0 to be efficient.

use crate::string::{self, str_from, str_up_to};

/// A const-equivalent of the [`str::split_once`] method.
///
/// This only accepts `&str` as the delimiter.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// assert_eq!(string::split_once("", "-"), None);
/// assert_eq!(string::split_once("foo", "-"), None);
/// assert_eq!(string::split_once("foo-", "-"), Some(("foo", "")));
/// assert_eq!(string::split_once("foo-bar", "-"), Some(("foo", "bar")));
/// assert_eq!(string::split_once("foo-bar-baz", "-"), Some(("foo", "bar-baz")));
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
pub const fn split_once<'a>(this: &'a str, delim: &str) -> Option<(&'a str, &'a str)> {
    crate::option::map! {
        string::find(this, delim, 0),
        |pos| (str_up_to(this, pos), str_from(this, pos + delim.len()))
    }
}

/// A const-equivalent of the [`str::rsplit_once`] method.
///
/// This only accepts `&str` as the delimiter.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// assert_eq!(string::rsplit_once("", "-"), None);
/// assert_eq!(string::rsplit_once("foo", "-"), None);
/// assert_eq!(string::rsplit_once("foo-", "-"), Some(("foo", "")));
/// assert_eq!(string::rsplit_once("foo-bar", "-"), Some(("foo", "bar")));
/// assert_eq!(string::rsplit_once("foo-bar-baz", "-"), Some(("foo-bar", "baz")));
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
pub const fn rsplit_once<'a>(this: &'a str, delim: &str) -> Option<(&'a str, &'a str)> {
    crate::option::map! {
        string::rfind(this, delim, this.len()),
        |pos| (str_up_to(this, pos), str_from(this, pos + delim.len()))
    }
}
