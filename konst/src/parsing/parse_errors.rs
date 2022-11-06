use crate::Parser;

use core::{
    fmt::{self, Display},
    marker::PhantomData,
};

/// Error returned by all parsing methods that return Result.
///
/// This error type knows [`where`](#method.offset) the error happened,
/// in what [`direction`](#method.error_direction) the string was being parsed,
/// and the [`kind`](#method.kind) of error that happened.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseError<'a> {
    start_offset: u32,
    end_offset: u32,
    direction: ParseDirection,
    kind: ErrorKind,
    extra_message: &'static &'static str,
    // Just in case that it goes back to storing the parser
    _lifetime: PhantomData<&'a [u8]>,
}

impl<'a> ParseError<'a> {
    /// Constructs a `ParseError`.
    #[inline(always)]
    pub const fn new(parser: Parser<'a>, kind: ErrorKind) -> Self {
        Self {
            start_offset: parser.start_offset,
            end_offset: parser.start_offset + parser.str.len() as u32,
            direction: parser.parse_direction,
            kind,
            extra_message: &"",
            _lifetime: PhantomData,
        }
    }

    /// Constructs a `ParseError`  for an `ErrorKind::Other` error with
    /// a customized error message.
    pub const fn other_error(parser: Parser<'a>, extra_message: &'static &'static str) -> Self {
        Self {
            start_offset: parser.start_offset,
            end_offset: parser.start_offset + parser.str.len() as u32,
            direction: parser.parse_direction,
            kind: ErrorKind::Other,
            extra_message,
            _lifetime: PhantomData,
        }
    }

    /// A const fn equivalent of a clone method.
    pub const fn copy(&self) -> Self {
        Self {
            start_offset: self.start_offset,
            end_offset: self.end_offset,
            direction: self.direction,
            kind: self.kind,
            extra_message: self.extra_message,
            _lifetime: PhantomData,
        }
    }

    /// Gets the byte offset of this error in the parsed string that the
    /// [`Parser`] was constructed from.
    #[inline(always)]
    pub const fn offset(&self) -> usize {
        (match self.direction {
            ParseDirection::FromStart | ParseDirection::FromBoth => self.start_offset,
            ParseDirection::FromEnd => self.end_offset,
        }) as usize
    }

    /// The direction that this error happened from,
    /// either from the start or the end.
    pub const fn error_direction(&self) -> ParseDirection {
        self.direction
    }

    /// The kind of parsing error that this is.
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    const fn extra_message(&self) -> &str {
        self.extra_message
    }

    /// For panicking with an error message,
    /// this is called by the [`unwrap_ctx`] macro.
    ///
    /// [`unwrap_ctx`]: ../result/macro.unwrap_ctx.html
    #[track_caller]
    pub const fn panic(&self) -> ! {
        use const_panic::{FmtArg, PanicVal};

        const_panic::concat_panic(&[&[
            PanicVal::write_str(self.error_for_direction()),
            PanicVal::from_usize(self.offset(), FmtArg::DEBUG),
            PanicVal::write_str(" byte offset"),
            PanicVal::write_str(self.error_suffix()),
            PanicVal::write_str(self.extra_message()),
        ]])
    }

    const fn error_for_direction(&self) -> &'static str {
        match self.direction {
            ParseDirection::FromStart => "error from the start at the ",
            ParseDirection::FromEnd => "error from the end at the ",
            ParseDirection::FromBoth => "error from the start and end at the ",
        }
    }
    const fn error_suffix(&self) -> &'static str {
        match self.kind {
            ErrorKind::ParseInteger => " while parsing an integer",
            ErrorKind::ParseBool => " while parsing a bool",
            ErrorKind::Find => " while trying to find and skip a pattern",
            ErrorKind::Strip => " while trying to strip a pattern",
            ErrorKind::SplitExhausted => ": called split on empty parser",
            ErrorKind::DelimiterNotFound => ": delimiter (for splitting) could not be found",
            ErrorKind::Other => {
                if self.extra_message.is_empty() {
                    " other error"
                } else {
                    ": "
                }
            }
        }
    }
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.error_for_direction())?;
        Display::fmt(&self.offset(), f)?;
        f.write_str(" byte offset")?;
        f.write_str(self.error_suffix())?;
        f.write_str(self.extra_message())?;
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

/// The direction that a parser was parsing from when an error happened.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ParseDirection {
    /// Parsing was attempted from the start of the string
    FromStart = 0,
    /// Parsing was attempted from the end of the string
    FromEnd = 1,
    /// Parsing was attempted from both the start and end of the string
    FromBoth = 2,
}

////////////////////////////////////////////////////////////////////////////////

/// What kind of parsing error this is.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ErrorKind {
    /// Returned from integer parsing methods
    ParseInteger,
    /// Returned from `parse_bool`
    ParseBool,
    /// Returned from `*find*` methods
    Find,
    /// Returned from `strip_*` methods
    Strip,
    /// Returned from `split` when the last delimiter-separated/terminated string
    /// has already been returned
    SplitExhausted,
    /// Returned from `split_terminator` when the delimiter could not be found
    DelimiterNotFound,
    /// For user-defined types
    Other,
}

////////////////////////////////////////////////////////////////////////////////

/// Result alias for functions that mutate the parser fallibly.
pub type ParserResult<'a, E = ParseError<'a>> = Result<Parser<'a>, E>;

/// Result alias for functions that parse values.
pub type ParseValueResult<'a, T, E = ParseError<'a>> = Result<(T, Parser<'a>), E>;
