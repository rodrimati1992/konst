use super::{ErrorKind, ParseDirection, ParseError, Parser};

use crate::string;

// Putting this impl in a submodule so that it appears before
// the integer parsing methods in the docs
impl<'a> Parser<'a> {
    /// Constructs a Parser from a string.
    ///
    /// This parser start with a `start_offset` of `0`,
    /// [`with_start_offset`](Self::with_start_offset)
    /// is preferable for parsing after the start of a string.
    #[inline]
    pub const fn new(string: &'a str) -> Self {
        Self {
            parse_direction: ParseDirection::FromStart,
            start_offset: 0,
            yielded_last_split: false,
            str: string,
        }
    }

    /// Constructs a Parser from `string` which is at `start_offset`
    /// inside some other string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::parsing::{ErrorKind, Parser};
    ///
    /// // indices
    /// //  0   4   8
    /// //  |   |   |
    /// // "foo bar baz"
    /// let substr = konst::string::str_from("foo bar baz", 4);
    ///
    /// let parser = Parser::with_start_offset(substr, 4);
    /// assert_eq!(parser.remainder(), "bar baz");
    ///
    /// let (bar, parser) = parser.split(' ').unwrap();
    /// assert_eq!(bar, "bar");
    ///
    /// let err = parser.split_terminator(' ').unwrap_err();
    ///
    /// assert_eq!(parser.remainder(), "baz");
    /// assert_eq!(err.offset(), 8);
    /// assert_eq!(err.kind(), ErrorKind::DelimiterNotFound);
    ///
    /// ```
    #[inline]
    pub const fn with_start_offset(string: &'a str, start_offset: usize) -> Self {
        Self {
            parse_direction: ParseDirection::FromStart,
            start_offset: start_offset as u32,
            yielded_last_split: false,
            str: string,
        }
    }

    /// Skips `byte_count` bytes from the parsed string,
    /// as well as however many bytes are required to be on a char boundary.
    pub const fn skip(mut self, mut byte_count: usize) -> Self {
        let bytes = self.str.as_bytes();
        if byte_count > bytes.len() {
            byte_count = bytes.len()
        } else {
            use konst_kernel::string::__is_char_boundary_bytes;
            while !__is_char_boundary_bytes(bytes, byte_count) {
                byte_count += 1;
            }
        };
        self.parse_direction = ParseDirection::FromStart;
        self.start_offset += byte_count as u32;
        self.str = string::str_from(self.str, byte_count);
        self
    }

    /// Skips `byte_count` bytes from the back of the parsed string,
    /// as well as however many bytes are required to be on a char boundary.
    pub const fn skip_back(mut self, byte_count: usize) -> Self {
        use konst_kernel::string::__is_char_boundary_bytes;

        let bytes = self.str.as_bytes();
        let mut pos = self.str.len().saturating_sub(byte_count);
        while !__is_char_boundary_bytes(bytes, pos) {
            pos -= 1;
        }
        self.parse_direction = ParseDirection::FromEnd;
        self.str = string::str_up_to(self.str, pos);
        self
    }

    /// Returns the remaining, unparsed string.
    #[inline(always)]
    pub const fn remainder(self) -> &'a str {
        self.str
    }

    /// Gets the byte offset of this parser in the str slice that this
    /// was constructed from.
    #[inline(always)]
    pub const fn start_offset(self) -> usize {
        self.start_offset as _
    }

    /// Gets the end byte offset of this parser in the str slice that this
    /// was constructed from.
    #[inline(always)]
    pub const fn end_offset(self) -> usize {
        self.start_offset as usize + self.str.len()
    }

    /// The direction that the parser was last mutated from.
    pub const fn parse_direction(self) -> ParseDirection {
        self.parse_direction
    }

    /// Constructs a [`ParseError`] for this point in parsing.
    ///
    /// [`ParseError`]: struct.ParseError.html
    pub const fn into_error(self, kind: ErrorKind) -> ParseError<'a> {
        ParseError::new(self, kind)
    }

    /// Constructs a [`ParseError`] for this point in parsing,
    /// for an [`ErrorKind::Other`] with a custom error message.
    ///
    /// [`ParseError`]: struct.ParseError.html
    /// [`ErrorKind::Other`]: ./enum.ErrorKind.html#variant.Other
    pub const fn into_other_error(self, string: &'static &'static str) -> ParseError<'a> {
        ParseError::other_error(self, string)
    }

    /// The amount of unparsed bytes.
    #[inline(always)]
    pub const fn len(self) -> usize {
        self.str.len()
    }

    /// Whether there are any bytes left to parse.
    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.str.is_empty()
    }
}
