use crate::{
    iter::{IntoIterKind, IsIteratorKind},
    string::{self, str_from, str_up_to, Pattern, PatternNorm},
};

use konst_macro_rules::iterator_shared;

/// Const equivalent of [`str::split_terminator`], which only takes a `&str` delimiter.
///
/// The same as [`split`](crate::string::split),
/// except that, if the string after the last delimiter is empty, it is skipped.
///
/// This takes [`Pattern`] implementors as the delimiter.
///
/// # Example
///
/// ```rust
/// use konst::string;
/// use konst::iter::for_each;
///
/// const STRS: &[&str] = &{
///     let mut arr = [""; 3];
///     for_each!{(i, sub) in string::split_terminator("foo,bar,baz,", ','),enumerate() =>
///         arr[i] = sub;
///     }
///     arr
/// };
///
/// assert_eq!(STRS, ["foo", "bar", "baz"]);
/// ```
pub const fn split_terminator<'a, 'p, P>(this: &'a str, delim: P) -> SplitTerminator<'a, 'p, P>
where
    P: Pattern<'p>,
{
    let delim = PatternNorm::new(delim);

    SplitTerminator {
        this,
        state: if delim.as_str().is_empty() {
            State::Empty(EmptyState::Start)
        } else {
            State::Normal { delim }
        },
    }
}

/// Const equivalent of [`str::rsplit_terminator`].
///
/// The same as [`rsplit`](crate::string::rsplit),
/// except that, if the string before the first delimiter is empty, it is skipped.
///
/// This takes [`Pattern`] implementors as the delimiter.
///
/// # Example
///
/// ```rust
/// use konst::string;
/// use konst::iter::for_each;
///
/// const STRS: &[&str] = &{
///     let mut arr = [""; 3];
///     for_each!{(i, sub) in string::rsplit_terminator(":foo:bar:baz", ":"),enumerate() =>
///         arr[i] = sub;
///     }
///     arr
/// };
///
/// assert_eq!(STRS, ["baz", "bar", "foo"]);
/// ```
pub const fn rsplit_terminator<'a, 'p, P>(this: &'a str, delim: P) -> RSplitTerminator<'a, 'p, P>
where
    P: Pattern<'p>,
{
    let SplitTerminator { this, state } = split_terminator(this, delim);
    RSplitTerminator { this, state }
}

#[derive(Copy, Clone)]
enum State<'p, P: Pattern<'p>> {
    Normal { delim: PatternNorm<'p, P> },
    Empty(EmptyState),
}

#[derive(Copy, Clone)]
enum EmptyState {
    Start,
    Continue,
}

/// Const equivalent of `core::str::SplitTerminator<'a, P>`
///
/// This is constructed with [`split_terminator`] like this:
/// ```rust
/// # let string = "";
/// # let delim = "";
/// # let _: konst::string::SplitTerminator<'_, '_, &str> =
/// konst::string::split_terminator(string, delim)
/// # ;
/// ```
///
pub struct SplitTerminator<'a, 'p, P: Pattern<'p>> {
    this: &'a str,
    state: State<'p, P>,
}
impl<'p, P: Pattern<'p>> IntoIterKind for SplitTerminator<'_, 'p, P> {
    type Kind = IsIteratorKind;
}

impl<'a, 'p, P: Pattern<'p>> SplitTerminator<'a, 'p, P> {
    iterator_shared! {
        is_forward = true,
        item = &'a str,
        iter_forward = SplitTerminator<'a, 'p, P>,
        next(self){
            let Self {
                this,
                state,
            } = self;

            match state {
                State::Empty(EmptyState::Start) => {
                    self.state = State::Empty(EmptyState::Continue);
                    Some(("", self))
                }
                _ if this.is_empty() => {
                    None
                }
                State::Normal{delim} => {
                    let delim = delim.as_str();
                    let (next, ret) = match string::find(this, delim) {
                        Some(pos) => (pos + delim.len(), pos),
                        None => (this.len(), this.len()),
                    };
                    self.this = str_from(this, next);
                    Some((str_up_to(this, ret), self))
                }
                State::Empty(EmptyState::Continue) => {
                    let next_char = string::find_next_char_boundary(self.this.as_bytes(), 0);
                    let (next_char, rem) = string::split_at(self.this, next_char);
                    self.this = rem;
                    Some((next_char, self))
                }
            }
        },
        fields = {this, state},
    }

    /// Gets the remainder of the string.
    ///
    /// # Example
    ///
    /// ```rust
    /// let iter = konst::string::split_terminator("foo,bar,baz,", ",");
    /// assert_eq!(iter.remainder(), "foo,bar,baz,");
    ///
    /// let (elem, iter) = iter.next().unwrap();
    /// assert_eq!(elem, "foo");
    /// assert_eq!(iter.remainder(), "bar,baz,");
    ///
    /// let (elem, iter) = iter.next().unwrap();
    /// assert_eq!(elem, "bar");
    /// assert_eq!(iter.remainder(), "baz,");
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

/// Const equivalent of `core::str::RSplitTerminator<'a, P>`
///
/// This is constructed with [`rsplit_terminator`] like this:
/// ```rust
/// # let string = "";
/// # let delim = "";
/// # let _: konst::string::RSplitTerminator<'_, '_, &str> =
/// konst::string::rsplit_terminator(string, delim)
/// # ;
/// ```
///
pub struct RSplitTerminator<'a, 'p, P: Pattern<'p>> {
    this: &'a str,
    state: State<'p, P>,
}
impl<'p, P: Pattern<'p>> IntoIterKind for RSplitTerminator<'_, 'p, P> {
    type Kind = IsIteratorKind;
}

impl<'a, 'p, P: Pattern<'p>> RSplitTerminator<'a, 'p, P> {
    iterator_shared! {
        is_forward = true,
        item = &'a str,
        iter_forward = RSplitTerminator<'a, 'p>,
        next(self){
            let Self {
                this,
                state,
            } = self;

            match state {
                State::Empty(EmptyState::Start) => {
                    self.state = State::Empty(EmptyState::Continue);
                    Some(("", self))
                }
                _ if this.is_empty() => {
                    None
                }
                State::Normal{delim} => {
                    let delim = delim.as_str();
                    let (next, ret) = match string::rfind(this, delim) {
                        Some(pos) => (pos, pos + delim.len()),
                        None => (0, 0),
                    };
                    self.this = str_up_to(this, next);
                    Some((str_from(this, ret), self))
                }
                State::Empty(EmptyState::Continue) => {
                    let bytes = self.this.as_bytes();
                    let next_char = string::find_prev_char_boundary(bytes, bytes.len());
                    let (rem, next_char) = string::split_at(self.this, next_char);
                    self.this = rem;
                    Some((next_char, self))
                }
            }
        },
        fields = {this, state},
    }

    /// Gets the remainder of the string.
    ///
    /// # Example
    ///
    /// ```rust
    /// let iter = konst::string::rsplit_terminator("=foo=bar=baz", "=");
    /// assert_eq!(iter.remainder(), "=foo=bar=baz");
    ///
    /// let (elem, iter) = iter.next().unwrap();
    /// assert_eq!(elem, "baz");
    /// assert_eq!(iter.remainder(), "=foo=bar");
    ///
    /// let (elem, iter) = iter.next().unwrap();
    /// assert_eq!(elem, "bar");
    /// assert_eq!(iter.remainder(), "=foo");
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
