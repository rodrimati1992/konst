/// For parsing and byte string splitting in const contexts.
#[derive(Copy, Clone)]
pub struct Parser<'a> {
    bytes: &'a [u8],
}

impl<'a> Parser<'a> {
    /// Constructs a Parser from a byte string.
    #[inline]
    pub const fn from_bytes(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    /// Constructs a Parser from a string.
    #[inline]
    pub const fn from_str(string: &'a str) -> Self {
        Self {
            bytes: string.as_bytes(),
        }
    }

    /// Returns the remaining, unparsed bytes.
    #[inline(always)]
    pub const fn bytes(self) -> &'a [u8] {
        self.bytes
    }

    /// Returns amount of unparsed bytes.
    #[inline(always)]
    pub const fn len(self) -> usize {
        self.bytes.len()
    }

    /// Returns whether there's any bytes left to parse.
    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.bytes.is_empty()
    }

    /// Checks that the parsed bytes start with `matched`,
    /// returning the remainder of the bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, assign_if};
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Flags {
    ///     foo: bool,
    ///     bar: bool,
    /// }
    ///
    /// const fn parse_flags(mut parser: Parser<'_>) -> (Parser<'_>, Flags) {
    ///     let mut flags = Flags{foo: false, bar: false};
    ///     assign_if!{Some(parser) = parser.strip_prefix("foo;") => {
    ///         flags.foo = true;
    ///     }}
    ///     assign_if!{Some(parser) = parser.strip_prefix("bar;") => {
    ///         flags.bar = true;
    ///     }}
    ///     (parser, flags)
    /// }
    ///
    /// const VALUES: &[Flags] = &[
    ///     parse_flags(Parser::from_str("")).1,
    ///     parse_flags(Parser::from_str("foo;")).1,
    ///     parse_flags(Parser::from_str("bar;")).1,
    ///     parse_flags(Parser::from_str("foo;bar;")).1,
    /// ];
    ///
    /// assert_eq!(VALUES[0], Flags{foo: false, bar: false});
    /// assert_eq!(VALUES[1], Flags{foo: true, bar: false});
    /// assert_eq!(VALUES[2], Flags{foo: false, bar: true});
    /// assert_eq!(VALUES[3], Flags{foo: true, bar: true});
    ///
    /// ```
    #[inline]
    pub const fn strip_prefix(self, matched: &str) -> Option<Self> {
        self.strip_prefix_b(matched.as_bytes())
    }

    /// Equivalent to [`strip_prefix`], but takes a byte slice.
    ///
    /// [`strip_prefix`]: #method.strip_prefix
    pub const fn strip_prefix_b(mut self, mut matched: &[u8]) -> Option<Self> {
        if self.bytes.len() < matched.len() {
            return None;
        }

        while let ([lb, rem_slice @ ..], [rb, rem_matched @ ..]) = (self.bytes, matched) {
            self.bytes = rem_slice;
            matched = rem_matched;

            if *lb != *rb {
                return None;
            }
        }

        Some(self)
    }

    /// Checks that the parsed bytes end with `matched`,
    /// returning the remainder of the bytes.
    #[inline]
    pub const fn strip_suffix(self, matched: &str) -> Option<Self> {
        self.strip_prefix_b(matched.as_bytes())
    }

    /// Equivalent to [`strip_suffix`], but takes a byte slice.
    ///
    /// [`strip_suffix`]: #method.strip_suffix
    pub const fn strip_suffix_b(mut self, mut matched: &[u8]) -> Option<Self> {
        if self.bytes.len() < matched.len() {
            return None;
        }

        while let ([rem_slice @ .., lb], [rem_matched @ .., rb]) = (self.bytes, matched) {
            self.bytes = rem_slice;
            matched = rem_matched;

            if *lb != *rb {
                return None;
            }
        }

        Some(self)
    }
}
