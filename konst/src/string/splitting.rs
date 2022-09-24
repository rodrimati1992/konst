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

/// Makes an iterator over the substrings in `this` separated by `delim`.
///
/// # Example
///
/// ```rust
/// use konst::string;
/// use konst::iter::for_each_i;
///
/// const STRS: &[&str] = &{
///     let mut arr = [""; 3];
///     for_each_i!{(i, sub) in string::split("foo-bar-baz", "-") =>
///         arr[i] = sub;
///     }
///     arr
/// };
///
/// assert_eq!(STRS, ["foo", "bar", "baz"]);
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
pub const fn split<'a, 'b>(this: &'a str, delim: &'b str) -> Split<'a, 'b> {
    Split {
        this,
        delim,
        finished: false,
    }
}

/// Makes an iterator over the substrings in `this` separated by `delim`,
/// iterating from the back.
///
/// # Example
///
/// ```rust
/// use konst::string;
/// use konst::iter::for_each_i;
///
/// const STRS: &[&str] = &{
///     let mut arr = [""; 3];
///     for_each_i!{(i, sub) in string::rsplit("foo-bar-baz", "-") =>
///         arr[i] = sub;
///     }
///     arr
/// };
///
/// assert_eq!(STRS, ["baz", "bar", "foo"]);
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
pub const fn rsplit<'a, 'b>(this: &'a str, delim: &'b str) -> RSplit<'a, 'b> {
    RSplit {
        this,
        delim,
        finished: false,
    }
}

macro_rules! split_shared {
    (is_forward = $is_forward:ident) => {
        iterator_shared! {
            is_forward = $is_forward,
            item = &'a str,
            iter_forward = Split<'a, 'b>,
            iter_reversed = RSplit<'a, 'b>,
            next = split_next,
            next_back = split_next_back,
            fields = {this, delim, finished},
        }
    };
}

macro_rules! split_next {
    ($self:ident) => {{
        let Self {
            this,
            delim,
            finished,
        } = $self;
        match string::find(this, delim, 0) {
            Some(pos) => {
                $self.this = str_from(this, pos + delim.len());
                Some((str_up_to(this, pos), $self))
            }
            None if finished => None,
            None => {
                $self.finished = true;
                Some((this, $self))
            }
        }
    }};
}

macro_rules! split_next_back {
    ($self:ident) => {{
        let Self {
            this,
            delim,
            finished,
        } = $self;
        match string::rfind(this, delim, this.len()) {
            Some(pos) => {
                $self.this = str_up_to(this, pos);
                Some((str_from(this, pos + delim.len()), $self))
            }
            None if finished => None,
            None => {
                $self.finished = true;
                Some((this, $self))
            }
        }
    }};
}

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
pub struct Split<'a, 'b> {
    this: &'a str,
    delim: &'b str,
    finished: bool,
}

impl<'a, 'b> Split<'a, 'b> {
    split_shared! {is_forward = true}
}

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
pub struct RSplit<'a, 'b> {
    this: &'a str,
    delim: &'b str,
    finished: bool,
}

impl<'a, 'b> RSplit<'a, 'b> {
    split_shared! {is_forward = false}
}
