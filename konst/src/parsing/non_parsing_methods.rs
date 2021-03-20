use super::{ErrorKind, ParseDirection, ParseError, Parser};

// Putting this impl in a submodule so that it appears before
// the integer parsing methods in the docs
impl<'a> Parser<'a> {
    /// Constructs a Parser from a byte string.
    #[inline]
    pub const fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            parse_direction: ParseDirection::FromStart,
            start_offset: 0,
            bytes,
        }
    }

    /// Constructs a Parser from a string.
    #[inline]
    pub const fn from_str(string: &'a str) -> Self {
        Self {
            parse_direction: ParseDirection::FromStart,
            start_offset: 0,
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
        self.start_offset as usize + self.bytes.len()
    }

    /// Constructs a [`ParseError`] for this point in parsing.
    ///
    /// [`ParseError`]: struct.ParseError.html
    pub const fn into_error(self, kind: ErrorKind) -> ParseError<'a> {
        ParseError::new(self, kind)
    }

    /// Constructs a [`ParseError`] for this point in parsing,
    /// with an [`ErrorKind::Other`] for the kind of error.
    ///
    /// [`ParseError`]: struct.ParseError.html
    /// [`ErrorKind::Other`]: ./enum.ErrorKind.html#variant.Other
    pub const fn into_other_error(self) -> ParseError<'a> {
        ParseError::new(self, ErrorKind::Other)
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
}
