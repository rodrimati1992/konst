use crate::{
    iter::{ConstIntoIter, IsIteratorKind},
    string::{self, Pattern, PatternNorm, RSplit, Split},
};

/// Const equivalent of [`str::split_terminator`], which only takes a `&str` delimiter.
///
/// This does the same as [`split`](crate::string::split),
/// except that, if the string after the last delimiter is empty, it is skipped.
///
/// This takes [`Pattern`] implementors as the delimiter.
///
/// # Example
///
/// ```rust
/// use konst::string;
/// use konst::iter::collect_const;
///
/// const STRS: [&str; 3] = collect_const!(&str =>  
///     string::split_terminator("foo,bar,baz,", ',')
/// );
///
/// assert_eq!(STRS, ["foo", "bar", "baz"]);
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub const fn split_terminator<'a, 'p, P>(this: &'a str, delim: P) -> SplitTerminator<'a, 'p, P>
where
    P: Pattern<'p>,
{
    let delim_norm = PatternNorm::new(delim);

    SplitTerminator {
        inner: string::split(this, delim),
        terminator_visited: !string::ends_with(this, delim),
        skip_iterating: this.is_empty() && !delim_norm.as_str().is_empty(),
    }
}

/// Const equivalent of [`str::rsplit_terminator`].
///
/// This does the same as [`rsplit`](crate::string::rsplit),
/// except that, if the string before the first delimiter is empty, it is skipped.
///
/// This takes [`Pattern`] implementors as the delimiter.
///
/// # Example
///
/// ```rust
/// use konst::string;
/// use konst::iter::collect_const;
///
/// const STRS: [&str; 3] = collect_const!(&str =>
///     string::rsplit_terminator("foo:bar:baz:", ":")
/// );
///
/// assert_eq!(STRS, ["baz", "bar", "foo"]);
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub const fn rsplit_terminator<'a, 'p, P>(this: &'a str, delim: P) -> RSplitTerminator<'a, 'p, P>
where
    P: Pattern<'p>,
{
    let delim_norm = PatternNorm::new(delim);

    RSplitTerminator {
        inner: string::rsplit(this, delim),
        terminator_visited: !string::ends_with(this, delim),
        skip_iterating: this.is_empty() && !delim_norm.as_str().is_empty(),
    }
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct SplitTerminator<'a, 'p, P: Pattern<'p>> {
    inner: Split<'a, 'p, P>,
    terminator_visited: bool,
    skip_iterating: bool,
}
impl<'a, 'p, P: Pattern<'p>> ConstIntoIter for SplitTerminator<'a, 'p, P> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = &'a str;
    const ITEMS_NEED_DROP: bool = false;
}

impl<'a, 'p, P: Pattern<'p>> SplitTerminator<'a, 'p, P> {
    iterator_shared! {
        is_forward = true,
        item = &'a str,
        iter_forward = SplitTerminator<'a, 'p, P>,

        next(self){
            if self.skip_iterating {
                return None
            }

            let mut ret = self.inner.next();

            if self.inner.is_finished() {
                if let Some(x) = ret && x.is_empty() && !self.terminator_visited {
                    ret = None;
                }

                self.terminator_visited = true;
            }

            ret
        },

        fields = {inner.copy(), terminator_visited, skip_iterating},
    }

    /// Gets the remainder of the string.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut iter = konst::string::split_terminator("foo,bar,baz,", ",");
    /// assert_eq!(iter.remainder(), "foo,bar,baz,");
    ///
    /// assert_eq!(iter.next().unwrap(), "foo");
    /// assert_eq!(iter.remainder(), "bar,baz,");
    ///
    /// assert_eq!(iter.next().unwrap(), "bar");
    /// assert_eq!(iter.remainder(), "baz,");
    ///
    /// assert_eq!(iter.next().unwrap(), "baz");
    /// assert_eq!(iter.remainder(), "");
    ///
    /// ```
    pub const fn remainder(&self) -> &'a str {
        self.inner.remainder()
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct RSplitTerminator<'a, 'p, P: Pattern<'p>> {
    inner: RSplit<'a, 'p, P>,
    terminator_visited: bool,
    skip_iterating: bool,
}
impl<'a, 'p, P: Pattern<'p>> ConstIntoIter for RSplitTerminator<'a, 'p, P> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = &'a str;
    const ITEMS_NEED_DROP: bool = false;
}

impl<'a, 'p, P: Pattern<'p>> RSplitTerminator<'a, 'p, P> {
    iterator_shared! {
        is_forward = true,
        item = &'a str,
        iter_forward = RSplitTerminator<'a, 'p, P>,

        next(self) {
            if self.skip_iterating {
                return None
            }

            if !self.terminator_visited {
                _ = self.inner.next();
                self.terminator_visited = true;
            }
            self.inner.next()
        },
        fields = {inner.copy(), terminator_visited, skip_iterating},
    }

    /// Gets the remainder of the string.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut iter = konst::string::rsplit_terminator("=foo=bar=baz", "=");
    /// assert_eq!(iter.remainder(), "=foo=bar=baz");
    ///
    /// assert_eq!(iter.next().unwrap(), "baz");
    /// assert_eq!(iter.remainder(), "=foo=bar");
    ///
    /// assert_eq!(iter.next().unwrap(), "bar");
    /// assert_eq!(iter.remainder(), "=foo");
    ///
    /// assert_eq!(iter.next().unwrap(), "foo");
    /// assert_eq!(iter.remainder(), "");
    ///
    /// ```
    pub const fn remainder(&self) -> &'a str {
        self.inner.remainder()
    }
}
