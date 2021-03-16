mod primitive_parsing;

pub use primitive_parsing::ParseIntError;

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

    /// Gets the next unparsed byte.
    #[inline]
    pub const fn next_byte(mut self) -> Option<(u8, Self)> {
        if let [byte, rem @ ..] = self.bytes {
            self.bytes = rem;
            Some((*byte, self))
        } else {
            None
        }
    }

    /// For skipping the first `bytes` bytes.
    ///
    /// # Performance
    ///
    /// If the "constant_time_slice" feature is disabled,
    /// thich takes linear time to remove the leading elements,
    /// proportional to `bytes`.
    ///
    /// If the "constant_time_slice" feature is enabled, it takes constant time to run,
    /// but uses a few nightly features.
    ///
    pub const fn skip(mut self, bytes: usize) -> Self {
        self.bytes = crate::slice::slice_from(self.bytes, bytes);
        self
    }

    /// Checks that the parsed bytes start with `matched`,
    /// returning the remainder of the bytes.
    ///
    /// # Examples
    ///
    /// ### Basic
    ///
    /// ```
    /// use konst::{Parser, assign_if};
    ///
    /// let mut parser = Parser::from_str("foo;bar;baz;");
    ///
    /// assert!(parser.strip_prefix("aaa").is_none());
    ///
    /// assign_if!{Some(parser) = parser.strip_prefix("foo;")};
    /// assert_eq!(parser.bytes(), "bar;baz;".as_bytes());
    ///
    /// assign_if!{Some(parser) = parser.strip_prefix("bar;")};
    /// assert_eq!(parser.bytes(), "baz;".as_bytes());
    ///
    /// assign_if!{Some(parser) = parser.strip_prefix("baz;")};
    /// assert_eq!(parser.bytes(), "".as_bytes());
    ///
    ///
    /// ```
    ///
    /// ### Use case
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

    /// Equivalent to [`strip_prefix`], but takes a single byte.
    ///
    /// [`strip_prefix`]: #method.strip_prefix
    pub const fn strip_prefix_u8(mut self, matched: u8) -> Option<Self> {
        match self.bytes {
            [byte, rem @ ..] if *byte == matched => {
                self.bytes = rem;
                Some(self)
            }
            _ => None,
        }
    }

    /// Checks that the parsed bytes end with `matched`,
    /// returning the remainder of the bytes.
    ///
    /// # Examples
    ///
    /// ### Basic
    ///
    /// ```
    /// use konst::{Parser, assign_if};
    ///
    /// let mut parser = Parser::from_str("foo;bar;baz;");
    ///
    /// assert!(parser.strip_suffix("aaa").is_none());
    ///
    /// assign_if!{Some(parser) = parser.strip_suffix("baz;")};
    /// assert_eq!(parser.bytes(), "foo;bar;".as_bytes());
    ///
    /// assign_if!{Some(parser) = parser.strip_suffix("bar;")};
    /// assert_eq!(parser.bytes(), "foo;".as_bytes());
    ///
    /// assign_if!{Some(parser) = parser.strip_suffix("foo;")};
    /// assert_eq!(parser.bytes(), "".as_bytes());
    ///
    /// ```
    ///
    #[inline]
    pub const fn strip_suffix(self, matched: &str) -> Option<Self> {
        self.strip_suffix_b(matched.as_bytes())
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
