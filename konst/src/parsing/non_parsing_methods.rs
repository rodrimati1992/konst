use super::{ErrorKind, ParseDirection, ParseError, Parser};

use crate::string;

// Putting this impl in a submodule so that it appears before
// the integer parsing methods in the docs
impl<'a> Parser<'a> {
    /// Constructs a Parser from a string.
    #[inline]
    pub const fn new(string: &'a str) -> Self {
        Self {
            parse_direction: ParseDirection::FromStart,
            start_offset: 0,
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

    /// Gets the byte offset of this parser in the str/byte slice that this
    /// was constructed from.
    #[inline(always)]
    pub const fn start_offset(self) -> usize {
        self.start_offset as _
    }

    /// Gets the end byte offset of this parser in the str/byte slice that this
    /// was constructed from.
    #[inline(always)]
    pub const fn end_offset(self) -> usize {
        self.start_offset as usize + self.str.len()
    }

    /// The end the parser was last mutated from.
    pub fn parse_direction(self) -> ParseDirection {
        self.parse_direction
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
