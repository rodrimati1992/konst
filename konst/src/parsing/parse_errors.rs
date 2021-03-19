use crate::Parser;

use core::{
    fmt::{self, Display},
    marker::PhantomData,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseError<'a> {
    start_offset: u32,
    end_offset: u32,
    direction: ParseDirection,
    // Just in case that it goes back to storing the parser
    _lifetime: PhantomData<&'a [u8]>,
}

impl<'a> ParseError<'a> {
    /// Constructs a `ParseError`.
    #[inline(always)]
    pub const fn new(parser: Parser<'a>, direction: ParseDirection) -> Self {
        Self {
            start_offset: parser.start_offset,
            end_offset: parser.end_offset,
            direction,
            _lifetime: PhantomData,
        }
    }

    /// Gets the byte offset of this error in the parsed bytes that the
    /// [`Parser`] was constructed from.
    #[inline(always)]
    pub const fn offset(&self) -> usize {
        (match self.direction {
            ParseDirection::FromStart => self.start_offset,
            ParseDirection::FromEnd => self.end_offset,
        }) as usize
    }

    /// The direction that this error happened from,
    /// either from the start or the end.
    pub const fn error_direction(&self) -> ParseDirection {
        self.direction
    }

    /// For erroring with an error message,
    /// this is called by the [`unwrap_ctx`] macro.
    ///
    /// [`unwrap_ctx`]: ../macro.unwrap_ctx.html
    #[track_caller]
    pub const fn panic(&self) -> ! {
        match self.direction {
            ParseDirection::FromStart => [/*parse error from start offset*/][self.offset()],
            ParseDirection::FromEnd => [/*parse error from end offset*/][self.offset()],
        }
    }
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.direction {
            ParseDirection::FromStart => "parsing the bytes from the start failed at the ",
            ParseDirection::FromEnd => "parsing the bytes from the end failed at the ",
        })?;
        Display::fmt(&self.offset(), f)?;
        f.write_str(" byte offset (in the str/byte slice that the Parser was constructed from)")
    }
}

/// The direction that a parser was parsing from when an error happened.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ParseDirection {
    FromStart = 0,
    FromEnd = 1,
}
