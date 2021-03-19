mod parse_errors;
mod primitive_parsing;

pub use self::{
    parse_errors::{ParseDirection, ParseError},
    primitive_parsing::ParseIntError,
};

/// For parsing and byte string splitting in const contexts.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Parser<'a> {
    start_offset: u32,
    end_offset: u32,
    bytes: &'a [u8],
}

impl<'a> Parser<'a> {
    /// Constructs a Parser from a byte string.
    #[inline]
    pub const fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            start_offset: 0,
            end_offset: bytes.len() as u32,
            bytes,
        }
    }

    /// Constructs a Parser from a string.
    #[inline]
    pub const fn from_str(string: &'a str) -> Self {
        Self {
            start_offset: 0,
            end_offset: string.len() as u32,
            bytes: string.as_bytes(),
        }
    }

    /// Returns the remaining, unparsed bytes.
    #[inline(always)]
    pub const fn bytes(self) -> &'a [u8] {
        self.bytes
    }

    /// Gets the byte offset of this parser in the str/byte slice that this
    /// was constructed from.
    #[inline(always)]
    pub const fn start_offset(self) -> usize {
        self.start_offset as _
    }

    /// Gets the byte offset of this parser in the str/byte slice that this
    /// was constructed from.
    #[inline(always)]
    pub const fn end_offset(self) -> usize {
        self.end_offset as _
    }

    /// Constructs a [`ParseError`] for this point in parsing.
    ///
    /// [`ParseError`]: struct.ParseError.html
    pub const fn into_error(self, direction: ParseDirection) -> ParseError<'a> {
        ParseError::new(self, direction)
    }

    /// TODO
    pub const fn advance_to_remainder_from_start(mut self, to: &'a [u8]) -> Self {
        parsing! {self, FromStart;
            self.bytes = to;
        }
    }
    /// TODO
    pub const fn advance_to_remainder_from_end(mut self, to: &'a [u8]) -> Self {
        parsing! {self, FromEnd;
            self.bytes = to;
        }
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
    pub const fn next_byte(mut self) -> Result<(u8, Self), ParseError<'a>> {
        try_parsing! {self, FromStart, ret;
            if let [byte, rem @ ..] = self.bytes {
                self.bytes = rem;
                *byte
            } else {
                throw!()
            }
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
        parsing! {self, FromStart;
            self.bytes = crate::slice::slice_from(self.bytes, bytes);
        }
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
    /// assert!(parser.strip_prefix("aaa").is_err());
    ///
    /// assign_if!{Ok(parser) = parser.strip_prefix("foo;")};
    /// assert_eq!(parser.bytes(), "bar;baz;".as_bytes());
    ///
    /// assign_if!{Ok(parser) = parser.strip_prefix("bar;")};
    /// assert_eq!(parser.bytes(), "baz;".as_bytes());
    ///
    /// assign_if!{Ok(parser) = parser.strip_prefix("baz;")};
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
    ///     assign_if!{Ok(parser) = parser.strip_prefix("foo;") => {
    ///         flags.foo = true;
    ///     }}
    ///     assign_if!{Ok(parser) = parser.strip_prefix("bar;") => {
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
    pub const fn strip_prefix(self, matched: &str) -> Result<Self, ParseError<'a>> {
        self.strip_prefix_b(matched.as_bytes())
    }

    /// Equivalent to [`strip_prefix`], but takes a byte slice.
    ///
    /// [`strip_prefix`]: #method.strip_prefix
    pub const fn strip_prefix_b(mut self, mut matched: &[u8]) -> Result<Self, ParseError<'a>> {
        try_parsing! {self, FromStart;
            if self.bytes.len() < matched.len() {
                throw!()
            }

            while let ([lb, rem_slice @ ..], [rb, rem_matched @ ..]) = (self.bytes, matched) {
                self.bytes = rem_slice;
                matched = rem_matched;

                if *lb != *rb {
                    throw!()
                }
            }
        }
    }

    /// Equivalent to [`strip_prefix`], but takes a single byte.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::from_str("abcde");
    ///
    /// assert!(parser.strip_prefix_u8(1).is_err());
    ///
    /// parser = unwrap_ctx!(parser.strip_prefix_u8(b'a'));
    /// assert_eq!(parser.bytes(), "bcde".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.strip_prefix_u8(b'b'));
    /// assert_eq!(parser.bytes(), "cde".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.strip_prefix_u8(b'c'));
    /// assert_eq!(parser.bytes(), "de".as_bytes());
    ///
    /// ```
    ///
    /// [`strip_prefix`]: #method.strip_prefix
    pub const fn strip_prefix_u8(mut self, matched: u8) -> Result<Self, ParseError<'a>> {
        try_parsing! {self, FromStart;
            match self.bytes {
                [byte, rem @ ..] if *byte == matched => {
                    self.bytes = rem;
                }
                _ => throw!(),
            }
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
    /// assert!(parser.strip_suffix("aaa").is_err());
    ///
    /// assign_if!{Ok(parser) = parser.strip_suffix("baz;")};
    /// assert_eq!(parser.bytes(), "foo;bar;".as_bytes());
    ///
    /// assign_if!{Ok(parser) = parser.strip_suffix("bar;")};
    /// assert_eq!(parser.bytes(), "foo;".as_bytes());
    ///
    /// assign_if!{Ok(parser) = parser.strip_suffix("foo;")};
    /// assert_eq!(parser.bytes(), "".as_bytes());
    ///
    /// ```
    ///
    #[inline]
    pub const fn strip_suffix(self, matched: &str) -> Result<Self, ParseError<'a>> {
        self.strip_suffix_b(matched.as_bytes())
    }

    /// Equivalent to [`strip_suffix`], but takes a byte slice.
    ///
    /// [`strip_suffix`]: #method.strip_suffix
    pub const fn strip_suffix_b(mut self, mut matched: &[u8]) -> Result<Self, ParseError<'a>> {
        try_parsing! {self, FromEnd;
            if self.bytes.len() < matched.len() {
                throw!()
            }

            while let ([rem_slice @ .., lb], [rem_matched @ .., rb]) = (self.bytes, matched) {
                self.bytes = rem_slice;
                matched = rem_matched;

                if *lb != *rb {
                    throw!()
                }
            }
        }
    }

    /// Equivalent to [`strip_suffix`], but takes a single byte.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::from_str("edcba");
    ///
    /// assert!(parser.strip_suffix_u8(1).is_err());
    ///
    /// parser = unwrap_ctx!(parser.strip_suffix_u8(b'a'));
    /// assert_eq!(parser.bytes(), "edcb".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.strip_suffix_u8(b'b'));
    /// assert_eq!(parser.bytes(), "edc".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.strip_suffix_u8(b'c'));
    /// assert_eq!(parser.bytes(), "ed".as_bytes());
    ///
    /// ```
    ///
    /// [`strip_suffix`]: #method.strip_suffix
    pub const fn strip_suffix_u8(mut self, matched: u8) -> Result<Self, ParseError<'a>> {
        try_parsing! {self,  FromEnd;
            match self.bytes {
                [rem @ .., byte] if *byte == matched => {
                    self.bytes = rem;
                }
                _ => throw!(),
            }
        }
    }

    /////////////////////////////////////////
    //           *trim* methods            //
    /////////////////////////////////////////

    /// Removes whitespace from the start of the parsed bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::from_str("    foo\n\t bar");
    ///
    /// parser = parser.trim_start();
    /// assert_eq!(parser.bytes(), "foo\n\t bar".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.strip_prefix("foo")).trim_start();
    /// assert_eq!(parser.bytes(), "bar".as_bytes());
    ///
    /// ```
    pub const fn trim_start(mut self) -> Self {
        parsing! {self, FromStart;
            while let [b, rem @ ..] = self.bytes {
                if matches!(b, b'\t' | b'\n' | b'\r' | b' ') {
                    self.bytes = rem;
                } else {
                    break;
                }
            }
        }
    }

    /// Removes whitespace from the end of the parsed bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::from_str("foo,\n    bar,\n    ");
    ///
    /// parser = parser.trim_end();
    /// assert_eq!(parser.bytes(), "foo,\n    bar,".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.strip_suffix("bar,")).trim_end();
    /// assert_eq!(parser.bytes(), "foo,".as_bytes());
    ///
    /// ```
    pub const fn trim_end(mut self) -> Self {
        parsing! {self, FromEnd;
            while let [rem @ .., b] = self.bytes {
                if matches!(b, b'\t' | b'\n' | b'\r' | b' ') {
                    self.bytes = rem;
                } else {
                    break;
                }
            }
        }
    }

    /// Repeatedly removes all instances of `needle` from the start of the parsed bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// {
    ///     let mut parser = Parser::from_str("HelloHelloHello world!");
    ///     parser = parser.trim_start_matches("Hello");
    ///     assert_eq!(parser.bytes(), " world!".as_bytes());
    /// }
    /// {
    ///     let mut parser = Parser::from_str("        Hi!");
    ///     parser = parser.trim_start_matches("    ");
    ///     assert_eq!(parser.bytes(), "Hi!".as_bytes());
    /// }
    /// {
    ///     let mut parser = Parser::from_str("------Bye!");
    ///     parser = parser.trim_start_matches("----");
    ///     assert_eq!(parser.bytes(), "--Bye!".as_bytes());
    /// }
    ///
    /// ```
    ///
    pub const fn trim_start_matches(self, needle: &str) -> Self {
        self.trim_start_matches_b(needle.as_bytes())
    }

    /// Equivalent to [`trim_start_matches`], but takes a byte slice.
    ///
    /// [`trim_start_matches`]: #method.trim_start_matches
    pub const fn trim_start_matches_b(mut self, needle: &[u8]) -> Self {
        parsing! {self, FromStart;
            if needle.is_empty() {
                ret_!();
            }

            let mut matched = needle;

            loop {
                let at_start = self;

                match (self.bytes, matched) {
                    ([b, rem @ ..], [bm, remm @ ..]) if *b == *bm => {
                        self.bytes = rem;
                        matched = remm;
                    }
                    _ => break,
                }

                'inner: loop {
                    match (self.bytes, matched) {
                        ([], [_, ..]) => ret_!(self = at_start),
                        ([b, rem @ ..], [bm, remm @ ..]) => {
                            if *b == *bm {
                                self.bytes = rem;
                                matched = remm;
                            } else {
                                ret_!(self = at_start);
                            }
                        }
                        _ => break 'inner,
                    }
                }

                matched = needle;
            }
        }
    }

    /// Equivalent to [`trim_start_matches`], but takes a single byte.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// let mut parser = Parser::from_str("    ----world");
    ///
    /// parser = parser.trim_start_matches_u8(b' ');
    /// assert_eq!(parser.bytes(), "----world".as_bytes());
    ///
    /// parser = parser.trim_start_matches_u8(b'-');
    /// assert_eq!(parser.bytes(), "world".as_bytes());
    ///
    /// parser = parser.trim_start_matches_u8(b'-');
    /// assert_eq!(parser.bytes(), "world".as_bytes());
    ///
    /// ```
    ///
    /// [`trim_start_matches`]: #method.trim_start_matches
    pub const fn trim_start_matches_u8(mut self, needle: u8) -> Self {
        parsing! {self, FromStart;
            while let [b, rem @ ..] = self.bytes {
                if *b == needle {
                    self.bytes = rem;
                } else {
                    break;
                }
            }
        }
    }

    /// Repeatedly removes all instances of `needle` from the start of the parsed bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// {
    ///     let mut parser = Parser::from_str("Hello world!world!world!");
    ///     parser = parser.trim_end_matches("world!");
    ///     assert_eq!(parser.bytes(), "Hello ".as_bytes());
    /// }
    /// {
    ///     let mut parser = Parser::from_str("Hi!        ");
    ///     parser = parser.trim_end_matches("    ");
    ///     assert_eq!(parser.bytes(), "Hi!".as_bytes());
    /// }
    /// {
    ///     let mut parser = Parser::from_str("Bye!------");
    ///     parser = parser.trim_end_matches("----");
    ///     assert_eq!(parser.bytes(), "Bye!--".as_bytes());
    /// }
    ///
    /// ```
    ///
    pub const fn trim_end_matches(self, needle: &str) -> Self {
        self.trim_end_matches_b(needle.as_bytes())
    }

    /// Equivalent to [`trim_end_matches`], but takes a byte slice.
    ///
    /// [`trim_end_matches`]: #method.trim_end_matches
    pub const fn trim_end_matches_b(mut self, needle: &[u8]) -> Self {
        parsing! {self, FromEnd;
            if needle.is_empty() {
                ret_!();
            }

            let mut matched = needle;

            loop {
                let at_start = self;

                match (self.bytes, matched) {
                    ([rem @ .., b], [remm @ .., bm]) if *b == *bm => {
                        self.bytes = rem;
                        matched = remm;
                    }
                    _ => break,
                }

                'inner: loop {
                    match (self.bytes, matched) {
                        ([], [.., _]) => ret_!(self = at_start),
                        ([rem @ .., b], [remm @ .., bm]) => {
                            if *b == *bm {
                                self.bytes = rem;
                                matched = remm;
                            } else {
                                ret_!(self = at_start);
                            }
                        }
                        _ => break 'inner,
                    }
                }

                matched = needle;
            }
        }
    }

    /// Equivalent to [`trim_end_matches`], but takes a single byte.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// let mut parser = Parser::from_str("world----    ");
    ///
    /// parser = parser.trim_end_matches_u8(b' ');
    /// assert_eq!(parser.bytes(), "world----".as_bytes());
    ///
    /// parser = parser.trim_end_matches_u8(b'-');
    /// assert_eq!(parser.bytes(), "world".as_bytes());
    ///
    /// parser = parser.trim_end_matches_u8(b'-');
    /// assert_eq!(parser.bytes(), "world".as_bytes());
    ///
    /// ```
    ///
    /// [`trim_end_matches`]: #method.trim_end_matches
    pub const fn trim_end_matches_u8(mut self, needle: u8) -> Self {
        parsing! {self, FromEnd;
            while let [rem @ .., b] = self.bytes {
                if *b == needle {
                    self.bytes = rem;
                } else {
                    break;
                }
            }
        }
    }

    //////////////////////////////////////////////
    //           *find_skip* methods            //
    //////////////////////////////////////////////

    /// Skips the parser after the first instance of `needle`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::from_str("foo--bar,baz--qux");
    ///
    /// parser = unwrap_ctx!(parser.find_skip("--"));
    /// assert_eq!(parser.bytes(), "bar,baz--qux".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.find_skip("bar,"));
    /// assert_eq!(parser.bytes(), "baz--qux".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.find_skip("--"));
    /// assert_eq!(parser.bytes(), "qux".as_bytes());
    ///
    /// assert!(parser.find_skip("--").is_err());
    ///
    /// ```
    pub const fn find_skip(self, needle: &str) -> Result<Self, ParseError<'a>> {
        self.find_skip_b(needle.as_bytes())
    }

    /// Equivalent to [`find_skip`], but takes a byte slice.
    ///
    /// [`find_skip`]: #method.find_skip
    pub const fn find_skip_b(mut self, needle: &[u8]) -> Result<Self, ParseError<'a>> {
        try_parsing! {self, FromStart;
            if needle.is_empty() {
                ret_!();
            }

            let mut matching = needle;
            while let ([b, rem @ ..], [mb, m_rem @ ..]) = (self.bytes, matching) {
                self.bytes = rem;
                matching = m_rem;

                if *b != *mb {
                    matching = match needle {
                        // For when the string is "lawlawn" and we are skipping "lawn"
                        [mb2, m_rem2 @ ..] if *b == *mb2 => m_rem2,
                        _ => needle,
                    };
                }
            }

            if !matching.is_empty() {
                throw!()
            }
        }
    }

    /// Equivalent to [`find_skip`], but takes a single byte.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::from_str("foo-bar,baz");
    ///
    /// parser = unwrap_ctx!(parser.find_skip_u8(b'-'));
    /// assert_eq!(parser.bytes(), "bar,baz".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.find_skip_u8(b','));
    /// assert_eq!(parser.bytes(), "baz".as_bytes());
    ///
    /// ```
    ///
    /// [`find_skip`]: #method.find_skip
    pub const fn find_skip_u8(mut self, needle: u8) -> Result<Self, ParseError<'a>> {
        try_parsing! {self, FromStart;
            while let [byte, rem @ ..] = self.bytes {
                self.bytes = rem;

                if *byte == needle {
                    ret_!();
                }
            }
            throw!()
        }
    }

    /// Truncates the parsed bytes to before the last instance of `needle`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::from_str("foo--bar,baz--qux");
    ///
    /// parser = unwrap_ctx!(parser.rfind_skip("--"));
    /// assert_eq!(parser.bytes(), "foo--bar,baz".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.rfind_skip(",baz"));
    /// assert_eq!(parser.bytes(), "foo--bar".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.rfind_skip("--"));
    /// assert_eq!(parser.bytes(), "foo".as_bytes());
    ///
    /// assert!(parser.rfind_skip("--").is_err());
    ///
    /// ```
    pub const fn rfind_skip(self, needle: &str) -> Result<Self, ParseError<'a>> {
        self.rfind_skip_b(needle.as_bytes())
    }

    /// Equivalent to [`find_skip`], but takes a byte slice.
    ///
    /// [`find_skip`]: #method.find_skip
    pub const fn rfind_skip_b(mut self, needle: &[u8]) -> Result<Self, ParseError<'a>> {
        try_parsing! {self, FromEnd;
            if needle.is_empty() {
                ret_!();
            }

            let mut matching = needle;
            while let ([rem @ .., b], [m_rem @ .., mb]) = (self.bytes, matching) {
                self.bytes = rem;
                matching = m_rem;

                if *b != *mb {
                    matching = match needle {
                        // For when the string is "lawnawn" and we are skipping "lawn"
                        [m_rem2 @ .., mb2] if *b == *mb2 => m_rem2,
                        _ => needle,
                    };
                }
            }

            if !matching.is_empty() {
                throw!()
            }
        }
    }

    /// Equivalent to [`find_skip`], but takes a single byte.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::from_str("foo,bar-baz");
    ///
    /// parser = unwrap_ctx!(parser.rfind_skip_u8(b'-'));
    /// assert_eq!(parser.bytes(), "foo,bar".as_bytes());
    ///
    /// parser = unwrap_ctx!(parser.rfind_skip_u8(b','));
    /// assert_eq!(parser.bytes(), "foo".as_bytes());
    ///
    /// ```
    ///
    /// [`find_skip`]: #method.find_skip
    pub const fn rfind_skip_u8(mut self, needle: u8) -> Result<Self, ParseError<'a>> {
        try_parsing! {self, FromEnd;
            while let [rem @ .., byte] = self.bytes {
                self.bytes = rem;

                if *byte == needle {
                    ret_!();
                }
            }
            throw!()
        }
    }
}
