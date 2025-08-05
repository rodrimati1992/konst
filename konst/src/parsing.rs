//! Parsing using `const fn` methods.
//!
//! You can use the [`Parser`] type to parse from string,
//! more information in its documentation.
//!
//! If you're looking for functions to parse some type from an entire string
//! (instead of only part of it),
//! then you want to look in the module for that type, eg: [`primitive::parse_bool`].
//!
//! If you do want to parse a type from only part of a string, then you can use
//! [`Parser`]'s `parse_*` methods, or the [`parse_type`] macro.
//!
//! [`Parser`]: crate::parsing::Parser
//! [`primitive::parse_bool`]: crate::primitive::parse_bool
//! [`parse_type`]: self::parse_type
//!

mod get_parser;
mod non_parsing_methods;
mod parse_errors;
mod parsing_polymorphism_macros;
mod primitive_parsing;

#[cfg(feature = "parsing_proc")]
mod parser_method_macro;

/////////////////////////////////////////////////////////////////////////////////

pub use self::{
    get_parser::{HasParser, StdParser},
    parse_errors::{ErrorKind, ParseDirection, ParseError, ParseValueResult},
};

#[cfg(feature = "parsing_proc")]
#[doc(inline)]
pub use self::parser_method_macro::parser_method;

#[doc(inline)]
pub use self::parsing_polymorphism_macros::parse_type;

use crate::string::{self, Pattern};

/// For parsing and traversing over strings in const contexts.
///
/// If you're looking for functions to parse some type from an entire string
/// (instead of only part of it),
/// then you want to look in the module for that type, eg: [`primitive`](crate::primitive).
///
/// # Examples
///
/// ### Parsing a variable-length array
///
/// Parses a variable-length array, requires the length to appear before the array.
///
/// This example requires the "parsing_proc" feature (enabled by default)
/// because it uses the  [`parser_method`] macro.
///
#[cfg_attr(feature = "parsing_proc", doc = "```rust")]
#[cfg_attr(not(feature = "parsing_proc"), doc = "```ignore")]
/// use konst::{
///     parsing::{Parser, ParseValueResult, parser_method},
///     result,
///     for_range, try_,
/// };
///
/// // We need to parse the length into a separate const to use it as the length of the array.
/// const LEN_AND_PARSER: (usize, Parser<'_>) = {
///     let input = "\
///         6;
///         up, 0, 90, down, left, right,
///     ";
///     
///     let mut parser = Parser::new(input);
///     let len = result::unwrap!(parser.parse_usize());
///     result::unwrap!(parser.strip_prefix(';'));
///     (len, parser)
/// };
///
/// const ANGLES: [Angle; LEN_AND_PARSER.0] =
///     result::unwrap!(Angle::parse_array(&mut LEN_AND_PARSER.1));
///
/// fn main() {
///     assert_eq!(
///         ANGLES,
///         [Angle::UP, Angle::UP, Angle::RIGHT, Angle::DOWN, Angle::LEFT, Angle::RIGHT]
///     );
/// }
///
///
///
/// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// struct Angle(u16);
///
/// impl Angle {
///     pub const UP: Self = Self(0);
///     pub const RIGHT: Self = Self(90);
///     pub const DOWN: Self = Self(180);
///     pub const LEFT: Self = Self(270);
///
///     pub const fn new(n: u64) -> Angle {
///         Angle((n % 360) as u16)
///     }
///
///     const fn parse_array<'p, const LEN: usize>(
///         parser: &mut Parser<'p>
///     ) -> ParseValueResult<'p, [Angle; LEN]> {
///         let mut ret = [Angle::UP; LEN];
///         
///         for_range!{i in 0..LEN =>
///             ret[i] = try_!(Angle::parse(parser.trim_start()));
///             
///             parser.trim_start();
///             if !parser.is_empty() {
///                 try_!(parser.strip_prefix(','));
///             }
///         }
///         Ok(ret)
///     }
///
///     pub const fn parse<'p>(parser: &mut Parser<'p>) -> ParseValueResult<'p, Angle> {
///         if let Ok(angle) = parser.parse_u64() {
///             return Ok(Self::new(angle))
///         }
///         
///         let angle = parser_method!{parser, strip_prefix;
///             "up" => Self::UP,
///             "right" => Self::RIGHT,
///             "down" => Self::DOWN,
///             "left" => Self::LEFT,
///             _ => return Err(parser.to_other_error(&"could not parse Direction"))
///         };
///         Ok(angle)
///     }
/// }
///
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Parser<'a> {
    parse_direction: ParseDirection,
    // this allows split methods to return the empty string after
    // the last delimiter, but only once.
    yielded_last_split: bool,
    /// The offset of `str` in the string that this was created from.
    start_offset: u32,
    str: &'a str,
}

impl<'a> Parser<'a> {
    /// Gets the string up to (but not including) `delimiter`.
    ///
    /// This is like [`Parser::split`],
    /// except that it always requires that the delimiter can be found.
    ///
    /// # Return value
    ///
    /// If either the string is empty or the delimiter can't be found,
    /// this return an error.
    ///
    /// If the delimiter can be found and the string is non-empty.
    /// this returns the string before the delimiter,
    /// moving the parser to after the delimiter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, result};
    ///
    /// assert_eq!(VARS, ["foo", "bar", "baz"]);
    ///
    /// const VARS: [&str; 3] = {
    ///     let mut parser = Parser::new("foo,bar,baz");
    ///     
    ///     let foo = result::unwrap!(parser.split_terminator(','));
    ///     let bar = result::unwrap!(parser.split_terminator(','));
    ///     
    ///     // `.split_terminator(',')` errors here
    ///     // because there's no `,` in the remainder of the string,
    ///     assert!(parser.split_terminator(',').is_err());
    ///     
    ///     [foo, bar, parser.remainder()]
    /// };
    ///
    /// ```
    pub const fn split_terminator<'p, P>(&mut self, delimiter: P) -> Result<&'a str, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing! {self, FromStart, ret;
            if self.str.is_empty() || self.yielded_last_split {
                throw!(if self.yielded_last_split {
                    ErrorKind::SplitExhausted
                } else {
                    ErrorKind::DelimiterNotFound
                })
            }

            match string::split_once(self.str, delimiter) {
                Some((before, after)) => {
                    self.yielded_last_split = after.is_empty();
                    self.str = after;
                    before
                }
                None => throw!(ErrorKind::DelimiterNotFound),
            }
        }
    }

    /// Gets the string after `delimiter`.
    ///
    /// This is like [`Parser::rsplit`],
    /// except that it always requires that the delimiter can be found.
    ///
    /// # Return value
    ///
    /// If either the string is empty or the delimiter can't be found,
    /// this return an error.
    ///
    /// If the delimiter can be found and the string is non-empty.
    /// this returns the string after the delimiter,
    /// moving the parser to before the delimiter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, result};
    ///
    /// assert_eq!(VARS, ["baz", "bar", "foo"]);
    ///
    /// const VARS: [&str; 3] = {
    ///     let mut parser = Parser::new("foo,bar,baz");
    ///     
    ///     let baz = result::unwrap!(parser.rsplit_terminator(','));
    ///     let bar = result::unwrap!(parser.rsplit_terminator(','));
    ///     
    ///     // `.rsplit_terminator(',')` errors here
    ///     // because there's no `,` in the remainder of the string,
    ///     assert!(parser.rsplit_terminator(',').is_err());
    ///     
    ///     [baz, bar, parser.remainder()]
    /// };
    ///
    /// ```
    pub const fn rsplit_terminator<'p, P>(
        &mut self,
        delimiter: P,
    ) -> Result<&'a str, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing! {self, FromEnd, ret;
            if self.str.is_empty() || self.yielded_last_split {
                throw!(if self.yielded_last_split {
                    ErrorKind::SplitExhausted
                } else {
                    ErrorKind::DelimiterNotFound
                })
            }

            match string::rsplit_once(self.str, delimiter) {
                Some((after, before)) => {
                    self.yielded_last_split = after.is_empty();
                    self.str = after;
                    before
                }
                None => throw!(ErrorKind::DelimiterNotFound),
            }
        }
    }

    /// Gets the string up to (but not including) `delimiter`.
    ///
    /// # Return value
    ///
    /// If the last delimiter-separated string has already been returned,
    /// this return an error.
    ///
    /// If the delimiter can't be found.
    /// this returns the remainder of the string.
    ///
    /// If the delimiter can be found.
    /// this returns the string before the delimiter,
    /// moving the parser to after the delimiter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, result};
    ///
    /// assert_eq!(VARS, ["foo", "bar", ""]);
    ///
    /// const VARS: [&str; 3] = {
    ///     let mut parser = Parser::new("foo,bar,");
    ///     
    ///     let foo = result::unwrap!(parser.split(','));
    ///     let bar = result::unwrap!(parser.split(','));
    ///     let empty = result::unwrap!(parser.split(','));
    ///     
    ///     assert!(parser.split(',').is_err());
    ///     assert!(parser.remainder().is_empty());
    ///     
    ///     [foo, bar, empty]
    /// };
    ///
    /// ```
    pub const fn split<'p, P>(&mut self, delimiter: P) -> Result<&'a str, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing! {self, FromStart, ret;
            if self.yielded_last_split {
                throw!(ErrorKind::SplitExhausted)
            }

            let (before, after) = match string::split_once(self.str, delimiter) {
                Some(pair) => pair,
                None => {
                    self.yielded_last_split = true;
                    (self.str, string::str_from(self.str, self.str.len()))
                }
            };

            self.str = after;

            before
        }
    }

    /// Gets the string after `delimiter`.
    ///
    /// # Return value
    ///
    /// If the last delimiter-separated string has already been returned,
    /// this return an error.
    ///
    /// If the delimiter can't be found.
    /// this returns the remainder of the string.
    ///
    /// If the delimiter can be found.
    /// this returns the string after the delimiter,
    /// moving the parser to before the delimiter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, result};
    ///
    /// assert_eq!(VARS, ["baz", "bar", ""]);
    ///
    /// const VARS: [&str; 3] = {
    ///     let mut parser = Parser::new(",bar,baz");
    ///     
    ///     let baz = result::unwrap!(parser.rsplit(','));
    ///     let bar = result::unwrap!(parser.rsplit(','));
    ///     let empty = result::unwrap!(parser.rsplit(','));
    ///     
    ///     assert!(parser.rsplit(',').is_err());
    ///     assert!(parser.remainder().is_empty());
    ///     
    ///     [baz, bar, empty]
    /// };
    ///
    /// ```
    pub const fn rsplit<'p, P>(&mut self, delimiter: P) -> Result<&'a str, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing! {self, FromEnd, ret;
            if self.yielded_last_split {
                throw!(ErrorKind::SplitExhausted)
            }

            let (after, before) = match string::rsplit_once(self.str, delimiter) {
                Some(pair) => pair,
                None => {
                    self.yielded_last_split = true;
                    (string::str_up_to(self.str, 0), self.str)
                }
            };

            self.str = after;

            before
        }
    }

    /// Gets the string up to (but not including) `delimiter`.
    ///
    /// # Return value
    ///
    /// This behaves the same as [`Parser::split`],
    /// except that it keeps the delimiter in the parser,
    /// rather than skip it.
    ///
    /// # Example
    ///
    /// This example requires the `"parsing_proc"` feature.
    ///
    #[cfg_attr(feature = "parsing_proc", doc = "```rust")]
    #[cfg_attr(not(feature = "parsing_proc"), doc = "```ignore")]
    ///
    /// use konst::{
    ///     parsing::{Parser, ParseValueResult, parser_method},
    ///     result,
    ///     eq_str, for_range, try_,
    /// };
    ///
    /// assert_eq!(VALS, [
    ///     Value::Str("hello"),
    ///     Value::U64(3),
    ///     Value::U64(5),
    ///     Value::Str("world"),
    /// ]);
    ///
    /// const VALS: [Value<'_>; 4] = {
    ///     let mut arr = [Value::Str(""); 4];
    ///     let parser = &mut Parser::new("shello,i3,i5,sworld");
    ///     
    ///     for_range!{i in 0..arr.len() =>
    ///         arr[i] = result::unwrap!(parse_value(parser));
    ///         if !parser.is_empty() {
    ///             result::unwrap!(parser.strip_prefix(','));
    ///         }
    ///     }
    ///     
    ///     arr
    /// };
    ///
    ///
    /// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    /// enum Value<'a> {
    ///     Str(&'a str),
    ///     U64(u64),
    /// }
    ///
    /// pub const fn parse_value<'p>(parser: &mut Parser<'p>) -> ParseValueResult<'p, Value<'p>> {
    ///     let val = parser_method!{parser, strip_prefix;
    ///         "s" => {
    ///             let string = try_!(parser.split_keep(','));
    ///             Value::Str(string)
    ///         }
    ///         "i" => {
    ///             let integer = try_!(parser.parse_u64());
    ///             Value::U64(integer)
    ///         }
    ///         _ => return Err(parser.to_other_error(&"expected either `s` or `ì`"))
    ///     };
    ///     Ok(val)
    /// }
    /// ```
    ///
    pub const fn split_keep<'p, P>(&mut self, delimiter: P) -> Result<&'a str, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing! {self, FromStart, ret;
            if self.yielded_last_split {
                throw!(ErrorKind::SplitExhausted)
            }

            let (before, after) = match string::find(self.str, delimiter) {
                Some(pos) => string::split_at(self.str, pos),
                None => {
                    self.yielded_last_split = true;
                    (self.str, string::str_from(self.str, self.str.len()))
                }
            };

            self.str = after;

            before
        }
    }

    /// Checks that the parsed string starts with `matched`,
    /// returning the remainder of the str.
    ///
    /// For calling `strip_prefix` with multiple alternative `matched` string literals,
    /// you can use the [`parser_method`] macro,
    /// [example](self::parser_method#parsing-enum-example)
    ///
    /// # Examples
    ///
    /// ### Basic
    ///
    /// ```
    /// use konst::Parser;
    ///
    /// let mut parser = Parser::new("foo;bar;baz;");
    ///
    /// assert!(parser.strip_prefix("aaa").is_err());
    ///
    /// _ = parser.strip_prefix("foo;");
    /// assert_eq!(parser.remainder(), "bar;baz;");
    ///
    /// _ = parser.strip_prefix("bar;");
    /// assert_eq!(parser.remainder(), "baz;");
    ///
    /// _ = parser.strip_prefix("baz;");
    /// assert_eq!(parser.remainder(), "");
    ///
    ///
    /// ```
    ///
    /// ### `char` argument
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// let mut parser = Parser::new("abcde");
    ///
    /// _ = parser.strip_prefix('a');
    /// assert_eq!(parser.remainder(), "bcde");
    ///
    /// _ = parser.strip_prefix('b');
    /// assert_eq!(parser.remainder(), "cde");
    ///
    /// _ = parser.strip_prefix('c');
    /// assert_eq!(parser.remainder(), "de");
    ///
    /// ```
    ///
    #[inline]
    pub const fn strip_prefix<'p, P>(&mut self, matched: P) -> Result<&mut Self, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing_ret_parser! {self, FromStart;
            match string::strip_prefix(self.str, matched) {
                Some(x) => self.str = x,
                None => throw!(ErrorKind::Strip),
            }
        }
    }

    /// Checks that the parsed string ends with `matched`,
    /// returning the remainder of the string.
    ///
    /// For calling `strip_suffix` with multiple alternative `matched` string literals,
    /// you can use the [`parser_method`] macro.
    ///
    /// # Examples
    ///
    /// ### `&str` argument
    ///
    /// ```
    /// use konst::Parser;
    ///
    /// let mut parser = Parser::new("foo;bar;baz;");
    ///
    /// assert!(parser.strip_suffix("aaa").is_err());
    ///
    /// _ = parser.strip_suffix("baz;");
    /// assert_eq!(parser.remainder(), "foo;bar;");
    ///
    /// _ = parser.strip_suffix("bar;");
    /// assert_eq!(parser.remainder(), "foo;");
    ///
    /// _ = parser.strip_suffix("foo;");
    /// assert_eq!(parser.remainder(), "");
    ///
    /// ```
    ///
    /// ### `char` argument
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// let mut parser = Parser::new("edcba");
    ///
    /// _ = parser.strip_suffix('a');
    /// assert_eq!(parser.remainder(), "edcb");
    ///
    /// _ = parser.strip_suffix('b');
    /// assert_eq!(parser.remainder(), "edc");
    ///
    /// _ = parser.strip_suffix('c');
    /// assert_eq!(parser.remainder(), "ed");
    ///
    /// ```
    ///
    #[inline]
    pub const fn strip_suffix<'p, P>(&mut self, matched: P) -> Result<&mut Self, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing_ret_parser! {self, FromEnd;
            match string::strip_suffix(self.str, matched) {
                Some(x) => self.str = x,
                None => throw!(ErrorKind::Strip),
            }
        }
    }

    /// Removes whitespace from the start and end of the parsed string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, result};
    ///
    /// let mut parser = Parser::new("    foo\n\t bar    ");
    ///
    /// parser.trim();
    /// assert_eq!(parser.remainder(), "foo\n\t bar");
    ///
    /// ```
    pub const fn trim(&mut self) -> &mut Self {
        parsing! {self, FromBoth;
            self.str = self.str.trim_ascii();
        }
        self
    }

    /// Removes whitespace from the start of the parsed string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, result};
    ///
    /// let mut parser = Parser::new("    foo\n\t bar");
    ///
    /// parser.trim_start();
    /// assert_eq!(parser.remainder(), "foo\n\t bar");
    ///
    /// result::unwrap!(parser.strip_prefix("foo")).trim_start();
    /// assert_eq!(parser.remainder(), "bar");
    ///
    /// ```
    pub const fn trim_start(&mut self) -> &mut Self {
        parsing! {self, FromStart;
            self.str = self.str.trim_ascii_start();
        }
        self
    }

    /// Removes whitespace from the end of the parsed string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, result};
    ///
    /// let mut parser = Parser::new("foo,\n    bar,\n    ");
    ///
    /// parser.trim_end();
    /// assert_eq!(parser.remainder(), "foo,\n    bar,");
    ///
    /// result::unwrap!(parser.strip_suffix("bar,")).trim_end();
    /// assert_eq!(parser.remainder(), "foo,");
    ///
    /// ```
    pub const fn trim_end(&mut self) -> &mut Self {
        parsing! {self, FromEnd;
            self.str = self.str.trim_ascii_end();
        }
        self
    }

    /// Repeatedly removes all instances of `needle` from
    /// both the start and end of the parsed string.
    ///
    /// # Example
    ///
    /// ### `&str`
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// let mut parser = Parser::new("<><>hello<><>");
    ///
    /// parser.trim_matches("<>");
    /// assert_eq!(parser.remainder(), "hello");
    /// ```
    ///
    /// ### `char` argument
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// let mut parser = Parser::new("    world   ");
    ///
    /// parser.trim_matches(' ');
    /// assert_eq!(parser.remainder(), "world");
    /// ```
    ///
    pub const fn trim_matches<'p, P>(&mut self, needle: P) -> &mut Self
    where
        P: Pattern<'p>,
    {
        parsing! {self, FromBoth;
            self.str = crate::string::trim_matches(self.str, needle);
        }
        self
    }

    /// Repeatedly removes all instances of `needle` from the start of the parsed string.
    ///
    /// For trimming with multiple `needle`s, you can use the [`parser_method`] macro,
    /// [example](self::parser_method#trimming-example)
    ///
    /// # Example
    ///
    /// ### `&str`
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// {
    ///     let mut parser = Parser::new("HelloHelloHello world!");
    ///     parser.trim_start_matches("Hello");
    ///     assert_eq!(parser.remainder(), " world!");
    /// }
    /// {
    ///     let mut parser = Parser::new("        Hi!");
    ///     parser.trim_start_matches("    ");
    ///     assert_eq!(parser.remainder(), "Hi!");
    /// }
    /// {
    ///     let mut parser = Parser::new("------Bye!");
    ///     parser.trim_start_matches("----");
    ///     assert_eq!(parser.remainder(), "--Bye!");
    /// }
    ///
    /// ```
    ///
    /// ### `char` argument
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// let mut parser = Parser::new("    ----world");
    ///
    /// parser.trim_start_matches(' ');
    /// assert_eq!(parser.remainder(), "----world");
    ///
    /// parser.trim_start_matches('-');
    /// assert_eq!(parser.remainder(), "world");
    ///
    /// parser.trim_start_matches('-');
    /// assert_eq!(parser.remainder(), "world");
    ///
    /// ```
    ///
    pub const fn trim_start_matches<'p, P>(&mut self, needle: P) -> &mut Self
    where
        P: Pattern<'p>,
    {
        parsing! {self, FromStart;
            self.str = crate::string::trim_start_matches(self.str, needle);
        }
        self
    }

    /// Repeatedly removes all instances of `needle` from the start of the parsed string.
    ///
    /// For trimming with multiple `needle`s, you can use the [`parser_method`] macro,
    /// [example](self::parser_method#trimming-example)
    ///
    /// # Example
    ///
    /// ### `&str`
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// {
    ///     let mut parser = Parser::new("Hello world!world!world!");
    ///     parser.trim_end_matches("world!");
    ///     assert_eq!(parser.remainder(), "Hello ");
    /// }
    /// {
    ///     let mut parser = Parser::new("Hi!        ");
    ///     parser.trim_end_matches("    ");
    ///     assert_eq!(parser.remainder(), "Hi!");
    /// }
    /// {
    ///     let mut parser = Parser::new("Bye!------");
    ///     parser.trim_end_matches("----");
    ///     assert_eq!(parser.remainder(), "Bye!--");
    /// }
    ///
    /// ```
    ///
    /// ### `char` argument
    ///
    /// ```rust
    /// use konst::Parser;
    ///
    /// let mut parser = Parser::new("world----    ");
    ///
    /// parser.trim_end_matches(' ');
    /// assert_eq!(parser.remainder(), "world----");
    ///
    /// parser.trim_end_matches('-');
    /// assert_eq!(parser.remainder(), "world");
    ///
    /// parser.trim_end_matches('-');
    /// assert_eq!(parser.remainder(), "world");
    ///
    /// ```
    ///
    pub const fn trim_end_matches<'p, P>(&mut self, needle: P) -> &mut Self
    where
        P: Pattern<'p>,
    {
        parsing! {self, FromEnd;
            self.str = crate::string::trim_end_matches(self.str, needle);
        }
        self
    }

    /// Skips the parser after the first instance of `needle`.
    ///
    /// For calling `find_skip` with multiple alternative `needle` string literals,
    /// you can use the [`parser_method`] macro,
    /// [example](self::parser_method#find-example)
    ///
    /// # Example
    ///
    /// ### `&str` argument
    ///
    /// ```rust
    /// use konst::{Parser, result};
    ///
    /// let mut parser = Parser::new("foo--bar,baz--qux");
    ///
    /// result::unwrap!(parser.find_skip("--"));
    /// assert_eq!(parser.remainder(), "bar,baz--qux");
    ///
    /// result::unwrap!(parser.find_skip("bar,"));
    /// assert_eq!(parser.remainder(), "baz--qux");
    ///
    /// result::unwrap!(parser.find_skip("--"));
    /// assert_eq!(parser.remainder(), "qux");
    ///
    /// assert!(parser.find_skip("--").is_err());
    ///
    /// ```
    ///
    /// ### `char` argument
    ///
    /// ```rust
    /// use konst::{Parser, result};
    ///
    /// let mut parser = Parser::new("foo-bar,baz");
    ///
    /// result::unwrap!(parser.find_skip('-'));
    /// assert_eq!(parser.remainder(), "bar,baz");
    ///
    /// result::unwrap!(parser.find_skip(','));
    /// assert_eq!(parser.remainder(), "baz");
    ///
    /// ```
    ///
    pub const fn find_skip<'p, P>(&mut self, needle: P) -> Result<&mut Self, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing_ret_parser! {self, FromStart;
            self.str = match crate::string::find_skip(self.str, needle) {
                Some(x) => x,
                None => throw!(ErrorKind::Find),
            };
        }
    }

    /// Truncates the parsed string to before the last instance of `needle`.
    ///
    /// For calling `rfind_skip` with multiple alternative `needle` string literals,
    /// you can use the [`parser_method`] macro,
    /// [example](self::parser_method#find-example)
    ///
    /// # Example
    ///
    /// ### `&str` argument
    ///
    /// ```rust
    /// use konst::{Parser, result};
    ///
    /// let mut parser = Parser::new("foo--bar,baz--qux");
    ///
    /// result::unwrap!(parser.rfind_skip("--"));
    /// assert_eq!(parser.remainder(), "foo--bar,baz");
    ///
    /// result::unwrap!(parser.rfind_skip(",baz"));
    /// assert_eq!(parser.remainder(), "foo--bar");
    ///
    /// result::unwrap!(parser.rfind_skip("--"));
    /// assert_eq!(parser.remainder(), "foo");
    ///
    /// assert!(parser.rfind_skip("--").is_err());
    ///
    /// ```
    ///
    /// ### `char` argument
    ///
    /// ```rust
    /// use konst::{Parser, result};
    ///
    /// let mut parser = Parser::new("foo,bar-baz");
    ///
    /// result::unwrap!(parser.rfind_skip('-'));
    /// assert_eq!(parser.remainder(), "foo,bar");
    ///
    /// result::unwrap!(parser.rfind_skip(','));
    /// assert_eq!(parser.remainder(), "foo");
    ///
    /// ```
    ///
    pub const fn rfind_skip<'p, P>(&mut self, needle: P) -> Result<&mut Self, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing_ret_parser! {self, FromEnd;
            self.str = match crate::string::rfind_skip(self.str, needle) {
                Some(x) => x,
                None => throw!(ErrorKind::Find),
            };
        }
    }
}
