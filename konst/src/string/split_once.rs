use crate::string::{self, str_from, str_up_to, Pattern, PatternNorm};

/// A const-equivalent of the [`str::split_once`] method.
///
/// This takes [`Pattern`] implementors as the delimiter.
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
/// assert_eq!(string::split_once("foo,bar", ","), Some(("foo", "bar")));
/// assert_eq!(string::split_once("foo,bar,baz", ","), Some(("foo", "bar,baz")));
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub const fn split_once<'a, 'p, P>(this: &'a str, delim: P) -> Option<(&'a str, &'a str)>
where
    P: Pattern<'p>,
{
    let delim = PatternNorm::new(delim);
    let delim = delim.as_str();

    if delim.is_empty() {
        // using split_at so that the pointer points within the string
        Some(string::split_at(this, 0))
    } else {
        crate::option::map! {
            string::find(this, delim),
            |pos| (str_up_to(this, pos), str_from(this, pos + delim.len()))
        }
    }
}

/// A const-equivalent of the [`str::rsplit_once`] method.
///
/// This takes [`Pattern`] implementors as the delimiter.
///
/// # Example
///
/// ```rust
/// use konst::string;
///
/// assert_eq!(string::rsplit_once("", "-"), None);
/// assert_eq!(string::rsplit_once("foo", "-"), None);
/// assert_eq!(string::rsplit_once("foo-", "-"), Some(("foo", "")));
/// assert_eq!(string::rsplit_once("-foo", "-"), Some(("","foo")));
/// assert_eq!(string::rsplit_once("foo-bar", "-"), Some(("foo", "bar")));
/// assert_eq!(string::rsplit_once("foo-bar-baz", "-"), Some(("foo-bar", "baz")));
///
/// assert_eq!(string::rsplit_once("foo,bar", ','), Some(("foo", "bar")));
/// assert_eq!(string::rsplit_once("foo,bar,baz", ','), Some(("foo,bar", "baz")));
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub const fn rsplit_once<'a, 'p, P>(this: &'a str, delim: P) -> Option<(&'a str, &'a str)>
where
    P: Pattern<'p>,
{
    let delim = PatternNorm::new(delim);
    let delim = delim.as_str();

    if delim.is_empty() {
        // using split_at so that the pointer points within the string
        Some(string::split_at(this, this.len()))
    } else {
        crate::option::map! {
            string::rfind(this, delim),
            |pos| (str_up_to(this, pos), str_from(this, pos + delim.len()))
        }
    }
}
