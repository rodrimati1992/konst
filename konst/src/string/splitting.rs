//! string splitting that requires Rust 1.64.0 to be efficient.

use crate::{
    iter::{IntoIterKind, IsIteratorKind},
    string::{self, str_from, str_up_to, Pattern, PatternNorm},
};

use konst_macro_rules::iterator_shared;

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
/// assert_eq!(string::rsplit_once("foo-bar", "-"), Some(("foo", "bar")));
/// assert_eq!(string::rsplit_once("foo-bar-baz", "-"), Some(("foo-bar", "baz")));
///
/// assert_eq!(string::rsplit_once("foo,bar", ','), Some(("foo", "bar")));
/// assert_eq!(string::rsplit_once("foo,bar,baz", ','), Some(("foo,bar", "baz")));
///
/// ```
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

//////////////////////////////////////////////////

/// Const equivalent of [`str::split`].
///
/// This takes [`Pattern`] implementors as the delimiter.
///
/// # Example
///
/// ```rust
/// use konst::string;
/// use konst::iter::collect_const;
///
/// const STRS0: [&str; 3] = collect_const!(&str => string::split("foo-bar-baz", "-"));
/// const STRS1: [&str; 3] = collect_const!(&str => string::split("these are spaced", ' '));
///
/// assert_eq!(STRS0, ["foo", "bar", "baz"]);
/// assert_eq!(STRS1, ["these", "are", "spaced"]);
/// ```
pub const fn split<'a, 'p, P>(this: &'a str, delim: P) -> Split<'a, 'p, P>
where
    P: Pattern<'p>,
{
    let delim = PatternNorm::new(delim);
    Split {
        this,
        state: if delim.as_str().is_empty() {
            State::Empty(EmptyState::Start)
        } else {
            State::Normal { delim }
        },
    }
}

/// Const equivalent of [`str::rsplit`].
///
/// This takes [`Pattern`] implementors as the delimiter.
///
/// # Example
///
/// ```rust
/// use konst::string;
/// use konst::iter::collect_const;
///
/// const STRS0: [&str; 3] = collect_const!(&str => string::rsplit("foo-bar-baz", "-"));
/// const STRS1: [&str; 3] = collect_const!(&str => string::rsplit("these are spaced", ' '));
///
/// assert_eq!(STRS0, ["baz", "bar", "foo"]);
/// assert_eq!(STRS1, ["spaced", "are", "these"]);
/// ```
pub const fn rsplit<'a, 'p, P>(this: &'a str, delim: P) -> RSplit<'a, 'p, P>
where
    P: Pattern<'p>,
{
    split(this, delim).rev()
}

#[derive(Copy, Clone)]
enum State<'p, P: Pattern<'p>> {
    Normal { delim: PatternNorm<'p, P> },
    Empty(EmptyState),
    Finished,
}

#[derive(Copy, Clone)]
enum EmptyState {
    Start,
    Continue,
}

macro_rules! split_shared {
    (is_forward = $is_forward:ident) => {
        const fn next_from_empty(mut self, es: EmptyState) -> Option<(&'a str, Self)> {
            match es {
                EmptyState::Start => {
                    self.state = State::Empty(EmptyState::Continue);
                    Some(("", self))
                }
                EmptyState::Continue => {
                    let this = self.this;

                    if this.is_empty() {
                        self.state = State::Finished;
                    }
                    let next_char = string::find_next_char_boundary(this.as_bytes(), 0);
                    let (next_char, rem) = string::split_at(this, next_char);
                    self.this = rem;
                    Some((next_char, self))
                }
            }
        }

        const fn next_back_from_empty(mut self, es: EmptyState) -> Option<(&'a str, Self)> {
            match es {
                EmptyState::Start => {
                    self.state = State::Empty(EmptyState::Continue);
                    Some(("", self))
                }
                EmptyState::Continue => {
                    let this = self.this;

                    if self.this.is_empty() {
                        self.state = State::Finished;
                    }
                    let next_char = string::find_prev_char_boundary(this.as_bytes(), this.len());
                    let (rem, next_char) = string::split_at(this, next_char);
                    self.this = rem;
                    Some((next_char, self))
                }
            }
        }

        iterator_shared! {
            is_forward = $is_forward,
            item = &'a str,
            iter_forward = Split<'a, 'p, P>,
            iter_reversed = RSplit<'a, 'p, P>,
            next(self){
                let Self {
                    this,
                    state,
                } = self;

                match state {
                    State::Normal{delim} => {
                        let delim = delim.as_str();
                        match string::find(this, delim) {
                            Some(pos) => {
                                self.this = str_from(this, pos + delim.len());
                                Some((str_up_to(this, pos), self))
                            }
                            None => {
                                self.this = "";
                                self.state = State::Finished;
                                Some((this, self))
                            }
                        }
                    }
                    State::Empty(es) => self.next_from_empty(es),
                    State::Finished => None,
                }
            },
            next_back{
                let Self {
                    this,
                    state,
                } = self;
                match state {
                    State::Normal{delim} => {
                        let delim = delim.as_str();
                        match string::rfind(this, delim) {
                            Some(pos) => {
                                self.this = str_up_to(this, pos);
                                Some((str_from(this, pos + delim.len()), self))
                            }
                            None => {
                                self.this = "";
                                self.state = State::Finished;
                                Some((this, self))
                            }
                        }
                    }
                    State::Empty(es) => self.next_back_from_empty(es),
                    State::Finished => None,
                }
            },
            fields = {this, state},
        }
    };
}

/// Const equivalent of `core::str::Split<'a, P>`
///
/// This is constructed with [`split`] like this:
/// ```rust
/// # let string = "";
/// # let delim = "";
/// # let _ =
/// konst::string::split(string, delim)
/// # ;
/// ```
///
pub struct Split<'a, 'p, P: Pattern<'p>> {
    this: &'a str,
    state: State<'p, P>,
}
impl<'p, P: Pattern<'p>> IntoIterKind for Split<'_, 'p, P> {
    type Kind = IsIteratorKind;
}

impl<'a, 'p, P: Pattern<'p>> Split<'a, 'p, P> {
    split_shared! {is_forward = true}

    /// Gets the remainder of the string.
    ///
    /// # Example
    ///
    /// ```rust
    /// let iter = konst::string::split("foo-bar-baz", "-");
    /// assert_eq!(iter.remainder(), "foo-bar-baz");
    ///
    /// let (elem, iter) = iter.next().unwrap();
    /// assert_eq!(elem, "foo");
    /// assert_eq!(iter.remainder(), "bar-baz");
    ///
    /// let (elem, iter) = iter.next().unwrap();
    /// assert_eq!(elem, "bar");
    /// assert_eq!(iter.remainder(), "baz");
    ///
    /// let (elem, iter) = iter.next().unwrap();
    /// assert_eq!(elem, "baz");
    /// assert_eq!(iter.remainder(), "");
    ///
    /// ```
    pub const fn remainder(&self) -> &'a str {
        self.this
    }
}

/// Const equivalent of `core::str::RSplit<'a, P>`
///
/// This is constructed with [`rsplit`] like this:
/// ```rust
/// # let string = "";
/// # let delim = "";
/// # let _ =
/// konst::string::rsplit(string, delim)
/// # ;
/// ```
///
pub struct RSplit<'a, 'p, P: Pattern<'p>> {
    this: &'a str,
    state: State<'p, P>,
}
impl<'p, P: Pattern<'p>> IntoIterKind for RSplit<'_, 'p, P> {
    type Kind = IsIteratorKind;
}

impl<'a, 'p, P: Pattern<'p>> RSplit<'a, 'p, P> {
    split_shared! {is_forward = false}

    /// Gets the remainder of the string.
    ///
    /// # Example
    ///
    /// ```rust
    /// let iter = konst::string::rsplit("foo-bar-baz", "-");
    /// assert_eq!(iter.remainder(), "foo-bar-baz");
    ///
    /// let (elem, iter) = iter.next().unwrap();
    /// assert_eq!(elem, "baz");
    /// assert_eq!(iter.remainder(), "foo-bar");
    ///
    /// let (elem, iter) = iter.next().unwrap();
    /// assert_eq!(elem, "bar");
    /// assert_eq!(iter.remainder(), "foo");
    ///
    /// let (elem, iter) = iter.next().unwrap();
    /// assert_eq!(elem, "foo");
    /// assert_eq!(iter.remainder(), "");
    ///
    /// ```
    pub const fn remainder(&self) -> &'a str {
        self.this
    }
}
