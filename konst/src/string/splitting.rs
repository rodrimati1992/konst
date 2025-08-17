use crate::{
    iter::{ConstIntoIter, IsIteratorKind},
    string::{self, Pattern, PatternNorm, str_from, str_up_to},
};

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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub const fn rsplit<'a, 'p, P>(this: &'a str, delim: P) -> RSplit<'a, 'p, P>
where
    P: Pattern<'p>,
{
    let Split { this, state } = split(this, delim);

    RSplit { this, state }
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct Split<'a, 'p, P: Pattern<'p>> {
    this: &'a str,
    state: State<'p, P>,
}
impl<'a, 'p, P: Pattern<'p>> ConstIntoIter for Split<'a, 'p, P> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = &'a str;
    const ITEMS_NEED_DROP: bool = false;
}

impl<'a, 'p, P: Pattern<'p>> Split<'a, 'p, P> {
    iterator_shared! {
        is_forward = true,
        item = &'a str,
        iter_forward = Split<'a, 'p, P>,
        next(self) {
            let Self {
                this,
                state,
            } = *self;

            match state {
                State::Normal{delim} => {
                    let delim = delim.as_str();
                    match string::find(this, delim) {
                        Some(pos) => {
                            self.this = str_from(this, pos + delim.len());
                            Some(str_up_to(this, pos))
                        }
                        None => {
                            self.this = "";
                            self.state = State::Finished;
                            Some(this)
                        }
                    }
                }
                State::Empty(es) => self.next_from_empty(es),
                State::Finished => None,
            }
        },
        fields = {this, state},
    }

    pub(crate) const fn is_finished(&self) -> bool {
        matches!(self.state, State::Finished)
    }

    const fn next_from_empty(&mut self, es: EmptyState) -> Option<&'a str> {
        match es {
            EmptyState::Start => {
                self.state = State::Empty(EmptyState::Continue);
                Some("")
            }
            EmptyState::Continue => {
                use crate::string::__find_next_char_boundary;

                let this = self.this;

                if this.is_empty() {
                    self.state = State::Finished;
                }

                let next_char = __find_next_char_boundary(this.as_bytes(), 0);
                let (next_char, rem) = string::split_at(this, next_char);
                self.this = rem;
                Some(next_char)
            }
        }
    }

    /// Gets the remainder of the string.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut iter = konst::string::split("foo-bar-baz", "-");
    /// assert_eq!(iter.remainder(), "foo-bar-baz");
    ///
    /// assert_eq!(iter.next().unwrap(), "foo");
    /// assert_eq!(iter.remainder(), "bar-baz");
    ///
    /// assert_eq!(iter.next().unwrap(), "bar");
    /// assert_eq!(iter.remainder(), "baz");
    ///
    /// assert_eq!(iter.next().unwrap(), "baz");
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct RSplit<'a, 'p, P: Pattern<'p>> {
    this: &'a str,
    state: State<'p, P>,
}
impl<'a, 'p, P: Pattern<'p>> ConstIntoIter for RSplit<'a, 'p, P> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = &'a str;
    const ITEMS_NEED_DROP: bool = false;
}

impl<'a, 'p, P: Pattern<'p>> RSplit<'a, 'p, P> {
    iterator_shared! {
        is_forward = true,
        item = &'a str,
        iter_forward = RSplit<'a, 'p, P>,
        next(self) {
            let Self {
                this,
                state,
            } = *self;
            match state {
                State::Normal{delim} => {
                    let delim = delim.as_str();
                    match string::rfind(this, delim) {
                        Some(pos) => {
                            self.this = str_up_to(this, pos);
                            Some(str_from(this, pos + delim.len()))
                        }
                        None => {
                            self.this = "";
                            self.state = State::Finished;
                            Some(this)
                        }
                    }
                }
                State::Empty(es) => self.next_back_from_empty(es),
                State::Finished => None,
            }
        },
        fields = {this, state},
    }

    const fn next_back_from_empty(&mut self, es: EmptyState) -> Option<&'a str> {
        match es {
            EmptyState::Start => {
                self.state = State::Empty(EmptyState::Continue);
                Some("")
            }
            EmptyState::Continue => {
                use crate::string::__find_prev_char_boundary;

                let this = self.this;

                if self.this.is_empty() {
                    self.state = State::Finished;
                }
                let next_char = __find_prev_char_boundary(this.as_bytes(), this.len());
                let (rem, next_char) = string::split_at(this, next_char);
                self.this = rem;
                Some(next_char)
            }
        }
    }

    /// Gets the remainder of the string.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut iter = konst::string::rsplit("foo-bar-baz", "-");
    /// assert_eq!(iter.remainder(), "foo-bar-baz");
    ///
    /// assert_eq!(iter.next().unwrap(), "baz");
    /// assert_eq!(iter.remainder(), "foo-bar");
    ///
    /// assert_eq!(iter.next().unwrap(), "bar");
    /// assert_eq!(iter.remainder(), "foo");
    ///
    /// assert_eq!(iter.next().unwrap(), "foo");
    /// assert_eq!(iter.remainder(), "");
    ///
    /// ```
    pub const fn remainder(&self) -> &'a str {
        self.this
    }
}
