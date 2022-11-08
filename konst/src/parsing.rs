//! Parsing using `const fn` methods.
//!
//! You can use the [`Parser`] type to parse from string,
//! more information in its documentation.
//!
//! If you're looking for functions to parse some type from an entire string
//! (instead of only part of it),
//! then you want to look in the module for that type, eg: [`primitive::parse_bool`].
//!
//! If you do want to parse a type fron only part of a string, then you can use
//! [`Parser`]'s `parse_*` methods, or the [`parse_with`] macro.
//!
//! [`Parser`]: ./struct.Parser.html
//! [`primitive::parse_bool`]: ../primitive/fn.parse_bool.html
//! [`parse_with`]: ../macro.parse_with.html
//!

mod get_parser;
mod non_parsing_methods;
mod parse_errors;
mod primitive_parsing;

/////////////////////////////////////////////////////////////////////////////////

pub use self::{
    get_parser::{HasParser, StdParser},
    parse_errors::{ErrorKind, ParseDirection, ParseError, ParseValueResult, ParserResult},
};

use crate::string::{self, Pattern};

/// For parsing and traversing over strings in const contexts.
///
/// If you're looking for functions to parse some type from an entire string
/// (instead of only part of it),
/// then you want to look in the module for that type, eg: [`primitive::parse_u64`].
///
/// [`primitive::parse_u64`]: ../primitive/fn.parse_u64.html
///
/// # Mutation
///
/// Because `konst` only requires Rust 1.65.0,
/// in order to mutate a parser you must reassign the parser returned by its methods.
/// <br>eg: `parser = parser.trim_start();`
///
/// To help make this more ergonomic for `Result`-returning methods, you can use these macros:
///
/// - [`try_rebind`]:
/// Like the `?` operator,
/// but also reassigns variables and declares new ones with the value in the `Ok` variant.
///
/// - [`rebind_if_ok`]:
/// Like an `if let Ok`,
/// but also reassigns variables and declares new ones with the value in the `Ok` variant.
///
/// - [`parser_method`]:
/// Parses any of the string literal patterns using a supported `Parser` method.
///
/// [`try_rebind`]: ../macro.try_rebind.html
/// [`rebind_if_ok`]: ../macro.rebind_if_ok.html
/// [`parser_method`]: crate::parser_method
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
///     parsing::{Parser, ParseValueResult},
///     for_range, parser_method, try_, unwrap_ctx,
/// };
///
/// // We need to parse the length into a separate const to use it as the length of the array.
/// const LEN_AND_PARSER: (usize, Parser<'_>) = {
///     let input = "\
///         6;
///         up, 0, 90, down, left, right,
///     ";
///     
///     let parser = Parser::new(input);
///     let (len, parser) = unwrap_ctx!(parser.parse_usize());
///     (len, unwrap_ctx!(parser.strip_prefix(';')))
/// };
///
/// const ANGLES: [Angle; LEN_AND_PARSER.0] =
///     unwrap_ctx!(Angle::parse_array(LEN_AND_PARSER.1)).0;
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
///     const fn parse_array<const LEN: usize>(
///         mut parser: Parser<'_>
///     ) -> ParseValueResult<'_, [Angle; LEN]> {
///         let mut ret = [Angle::UP; LEN];
///         
///         for_range!{i in 0..LEN =>
///             (ret[i], parser) = try_!(Angle::parse(parser.trim_start()));
///             
///             parser = parser.trim_start();
///             if !parser.is_empty() {
///                 parser = try_!(parser.strip_prefix(','));
///             }
///         }
///         Ok((ret, parser))
///     }
///
///     pub const fn parse(mut parser: Parser<'_>) -> ParseValueResult<'_, Angle> {
///         // this doesn't use the `rebind_if_ok` macro because it returns early.
///         if let Ok((angle, parser)) = parser.parse_u64() {
///             return Ok((Self::new(angle), parser))
///         }
///         
///         let angle = parser_method!{parser, strip_prefix;
///             "up" => Self::UP,
///             "right" => Self::RIGHT,
///             "down" => Self::DOWN,
///             "left" => Self::LEFT,
///             _ => return Err(parser.into_other_error(&"could not parse Direction"))
///         };
///         Ok((angle, parser))
///     }
/// }
///
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
    /// use konst::{
    ///     result::unwrap_ctx,
    ///     Parser,
    /// };
    ///
    /// assert_eq!(VARS, ["foo", "bar", "baz"]);
    ///
    /// const VARS: [&str; 3] = {
    ///     let parser = Parser::new("foo,bar,baz");
    ///     
    ///     let (foo, parser) = unwrap_ctx!(parser.split_terminator(','));
    ///     let (bar, parser) = unwrap_ctx!(parser.split_terminator(','));
    ///     
    ///     // `.split_terminator(',')` errors here
    ///     // because there's no `,` in the remainder of the string,
    ///     assert!(parser.split_terminator(',').is_err());
    ///     
    ///     [foo, bar, parser.remainder()]
    /// };
    ///
    /// ```
    pub const fn split_terminator<'p, P>(
        mut self,
        delimiter: P,
    ) -> Result<(&'a str, Self), ParseError<'a>>
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
    /// use konst::{
    ///     result::unwrap_ctx,
    ///     Parser,
    /// };
    ///
    /// assert_eq!(VARS, ["baz", "bar", "foo"]);
    ///
    /// const VARS: [&str; 3] = {
    ///     let parser = Parser::new("foo,bar,baz");
    ///     
    ///     let (baz, parser) = unwrap_ctx!(parser.rsplit_terminator(','));
    ///     let (bar, parser) = unwrap_ctx!(parser.rsplit_terminator(','));
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
        mut self,
        delimiter: P,
    ) -> Result<(&'a str, Self), ParseError<'a>>
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
    /// use konst::{
    ///     result::unwrap_ctx,
    ///     Parser,
    /// };
    ///
    /// assert_eq!(VARS, ["foo", "bar", ""]);
    ///
    /// const VARS: [&str; 3] = {
    ///     let parser = Parser::new("foo,bar,");
    ///     
    ///     let (foo, parser) = unwrap_ctx!(parser.split(','));
    ///     let (bar, parser) = unwrap_ctx!(parser.split(','));
    ///     let (empty, parser) = unwrap_ctx!(parser.split(','));
    ///     
    ///     assert!(parser.split(',').is_err());
    ///     assert!(parser.remainder().is_empty());
    ///     
    ///     [foo, bar, empty]
    /// };
    ///
    /// ```
    pub const fn split<'p, P>(mut self, delimiter: P) -> Result<(&'a str, Self), ParseError<'a>>
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
    /// use konst::{
    ///     result::unwrap_ctx,
    ///     Parser,
    /// };
    ///
    /// assert_eq!(VARS, ["baz", "bar", ""]);
    ///
    /// const VARS: [&str; 3] = {
    ///     let parser = Parser::new(",bar,baz");
    ///     
    ///     let (baz, parser) = unwrap_ctx!(parser.rsplit(','));
    ///     let (bar, parser) = unwrap_ctx!(parser.rsplit(','));
    ///     let (empty, parser) = unwrap_ctx!(parser.rsplit(','));
    ///     
    ///     assert!(parser.rsplit(',').is_err());
    ///     assert!(parser.remainder().is_empty());
    ///     
    ///     [baz, bar, empty]
    /// };
    ///
    /// ```
    pub const fn rsplit<'p, P>(mut self, delimiter: P) -> Result<(&'a str, Self), ParseError<'a>>
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
    ///     parsing::{Parser, ParseValueResult},
    ///     eq_str,
    ///     for_range, parser_method, try_rebind, unwrap_ctx,
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
    ///     let mut parser = Parser::new("shello,i3,i5,sworld");
    ///     
    ///     for_range!{i in 0..arr.len() =>
    ///         (arr[i], parser) = unwrap_ctx!(parse_value(parser));
    ///         if !parser.is_empty() {
    ///             parser = unwrap_ctx!(parser.strip_prefix(','))
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
    /// pub const fn parse_value(mut parser: Parser<'_>) -> ParseValueResult<'_, Value<'_>> {
    ///     let val = parser_method!{parser, strip_prefix;
    ///         "s" => {
    ///             try_rebind!{(let string, parser) = parser.split_keep(',')}
    ///             Value::Str(string)
    ///         }
    ///         "i" => {
    ///             try_rebind!{(let integer, parser) = parser.parse_u64()}
    ///             Value::U64(integer)
    ///         }
    ///         _ => return Err(parser.into_other_error(&"expected either `s` or `ì`"))
    ///     };
    ///     Ok((val, parser))
    /// }
    /// ```
    ///
    pub const fn split_keep<'p, P>(
        mut self,
        delimiter: P,
    ) -> Result<(&'a str, Self), ParseError<'a>>
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
    /// [example](crate::parser_method#parsing-enum-example)
    ///
    /// # Examples
    ///
    /// ### Basic
    ///
    /// ```
    /// use konst::{Parser, rebind_if_ok};
    ///
    /// let mut parser = Parser::new("foo;bar;baz;");
    ///
    /// assert!(parser.strip_prefix("aaa").is_err());
    ///
    /// rebind_if_ok!{parser = parser.strip_prefix("foo;")}
    /// assert_eq!(parser.remainder(), "bar;baz;");
    ///
    /// rebind_if_ok!{parser = parser.strip_prefix("bar;")}
    /// assert_eq!(parser.remainder(), "baz;");
    ///
    /// rebind_if_ok!{parser = parser.strip_prefix("baz;")}
    /// assert_eq!(parser.remainder(), "");
    ///
    ///
    /// ```
    ///
    /// ### `char` argument
    ///
    /// ```rust
    /// use konst::{Parser, rebind_if_ok};
    ///
    /// let mut parser = Parser::new("abcde");
    ///
    /// rebind_if_ok!{parser = parser.strip_prefix('a')}
    /// assert_eq!(parser.remainder(), "bcde");
    ///
    /// rebind_if_ok!{parser = parser.strip_prefix('b')}
    /// assert_eq!(parser.remainder(), "cde");
    ///
    /// rebind_if_ok!{parser = parser.strip_prefix('c')}
    /// assert_eq!(parser.remainder(), "de");
    ///
    /// ```
    ///
    #[inline]
    pub const fn strip_prefix<'p, P>(mut self, matched: P) -> Result<Self, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing! {self, FromStart;
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
    /// use konst::{Parser, rebind_if_ok};
    ///
    /// let mut parser = Parser::new("foo;bar;baz;");
    ///
    /// assert!(parser.strip_suffix("aaa").is_err());
    ///
    /// rebind_if_ok!{parser = parser.strip_suffix("baz;")}
    /// assert_eq!(parser.remainder(), "foo;bar;");
    ///
    /// rebind_if_ok!{parser = parser.strip_suffix("bar;")}
    /// assert_eq!(parser.remainder(), "foo;");
    ///
    /// rebind_if_ok!{parser = parser.strip_suffix("foo;")}
    /// assert_eq!(parser.remainder(), "");
    ///
    /// ```
    ///
    /// ### `char` argument
    ///
    /// ```rust
    /// use konst::{Parser, rebind_if_ok};
    ///
    /// let mut parser = Parser::new("edcba");
    ///
    /// rebind_if_ok!{parser = parser.strip_suffix('a')}
    /// assert_eq!(parser.remainder(), "edcb");
    ///
    /// rebind_if_ok!{parser = parser.strip_suffix('b')}
    /// assert_eq!(parser.remainder(), "edc");
    ///
    /// rebind_if_ok!{parser = parser.strip_suffix('c')}
    /// assert_eq!(parser.remainder(), "ed");
    ///
    /// ```
    ///
    #[inline]
    pub const fn strip_suffix<'p, P>(mut self, matched: P) -> Result<Self, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing! {self, FromEnd;
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
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::new("    foo\n\t bar    ");
    ///
    /// parser = parser.trim();
    /// assert_eq!(parser.remainder(), "foo\n\t bar");
    ///
    /// ```
    pub const fn trim(mut self) -> Self {
        parsing! {self, FromBoth;
            self.str = crate::string::trim(self.str);
        }
    }

    /// Removes whitespace from the start of the parsed string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::new("    foo\n\t bar");
    ///
    /// parser = parser.trim_start();
    /// assert_eq!(parser.remainder(), "foo\n\t bar");
    ///
    /// parser = unwrap_ctx!(parser.strip_prefix("foo")).trim_start();
    /// assert_eq!(parser.remainder(), "bar");
    ///
    /// ```
    pub const fn trim_start(mut self) -> Self {
        parsing! {self, FromStart;
            self.str = crate::string::trim_start(self.str);
        }
    }

    /// Removes whitespace from the end of the parsed string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::new("foo,\n    bar,\n    ");
    ///
    /// parser = parser.trim_end();
    /// assert_eq!(parser.remainder(), "foo,\n    bar,");
    ///
    /// parser = unwrap_ctx!(parser.strip_suffix("bar,")).trim_end();
    /// assert_eq!(parser.remainder(), "foo,");
    ///
    /// ```
    pub const fn trim_end(mut self) -> Self {
        parsing! {self, FromEnd;
            self.str = crate::string::trim_end(self.str);
        }
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
    /// parser = parser.trim_matches("<>");
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
    /// parser = parser.trim_matches(' ');
    /// assert_eq!(parser.remainder(), "world");
    /// ```
    ///
    pub const fn trim_matches<'p, P>(mut self, needle: P) -> Self
    where
        P: Pattern<'p>,
    {
        parsing! {self, FromBoth;
            self.str = crate::string::trim_matches(self.str, needle);
        }
    }

    /// Repeatedly removes all instances of `needle` from the start of the parsed string.
    ///
    /// For trimming with multiple `needle`s, you can use the [`parser_method`] macro,
    /// [example](crate::parser_method#trimming-example)
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
    ///     parser = parser.trim_start_matches("Hello");
    ///     assert_eq!(parser.remainder(), " world!");
    /// }
    /// {
    ///     let mut parser = Parser::new("        Hi!");
    ///     parser = parser.trim_start_matches("    ");
    ///     assert_eq!(parser.remainder(), "Hi!");
    /// }
    /// {
    ///     let mut parser = Parser::new("------Bye!");
    ///     parser = parser.trim_start_matches("----");
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
    /// parser = parser.trim_start_matches(' ');
    /// assert_eq!(parser.remainder(), "----world");
    ///
    /// parser = parser.trim_start_matches('-');
    /// assert_eq!(parser.remainder(), "world");
    ///
    /// parser = parser.trim_start_matches('-');
    /// assert_eq!(parser.remainder(), "world");
    ///
    /// ```
    ///
    pub const fn trim_start_matches<'p, P>(mut self, needle: P) -> Self
    where
        P: Pattern<'p>,
    {
        parsing! {self, FromStart;
            self.str = crate::string::trim_start_matches(self.str, needle);
        }
    }

    /// Repeatedly removes all instances of `needle` from the start of the parsed string.
    ///
    /// For trimming with multiple `needle`s, you can use the [`parser_method`] macro,
    /// [example](crate::parser_method#trimming-example)
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
    ///     parser = parser.trim_end_matches("world!");
    ///     assert_eq!(parser.remainder(), "Hello ");
    /// }
    /// {
    ///     let mut parser = Parser::new("Hi!        ");
    ///     parser = parser.trim_end_matches("    ");
    ///     assert_eq!(parser.remainder(), "Hi!");
    /// }
    /// {
    ///     let mut parser = Parser::new("Bye!------");
    ///     parser = parser.trim_end_matches("----");
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
    /// parser = parser.trim_end_matches(' ');
    /// assert_eq!(parser.remainder(), "world----");
    ///
    /// parser = parser.trim_end_matches('-');
    /// assert_eq!(parser.remainder(), "world");
    ///
    /// parser = parser.trim_end_matches('-');
    /// assert_eq!(parser.remainder(), "world");
    ///
    /// ```
    ///
    pub const fn trim_end_matches<'p, P>(mut self, needle: P) -> Self
    where
        P: Pattern<'p>,
    {
        parsing! {self, FromEnd;
            self.str = crate::string::trim_end_matches(self.str, needle);
        }
    }

    /// Skips the parser after the first instance of `needle`.
    ///
    /// For calling `find_skip` with multiple alternative `needle` string literals,
    /// you can use the [`parser_method`] macro,
    /// [example](crate::parser_method#find-example)
    ///
    /// # Example
    ///
    /// ### `&str` argument
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::new("foo--bar,baz--qux");
    ///
    /// parser = unwrap_ctx!(parser.find_skip("--"));
    /// assert_eq!(parser.remainder(), "bar,baz--qux");
    ///
    /// parser = unwrap_ctx!(parser.find_skip("bar,"));
    /// assert_eq!(parser.remainder(), "baz--qux");
    ///
    /// parser = unwrap_ctx!(parser.find_skip("--"));
    /// assert_eq!(parser.remainder(), "qux");
    ///
    /// assert!(parser.find_skip("--").is_err());
    ///
    /// ```
    ///
    /// ### `char` argument
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::new("foo-bar,baz");
    ///
    /// parser = unwrap_ctx!(parser.find_skip('-'));
    /// assert_eq!(parser.remainder(), "bar,baz");
    ///
    /// parser = unwrap_ctx!(parser.find_skip(','));
    /// assert_eq!(parser.remainder(), "baz");
    ///
    /// ```
    ///
    pub const fn find_skip<'p, P>(mut self, needle: P) -> Result<Self, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing! {self, FromStart;
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
    /// [example](crate::parser_method#find-example)
    ///
    /// # Example
    ///
    /// ### `&str` argument
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::new("foo--bar,baz--qux");
    ///
    /// parser = unwrap_ctx!(parser.rfind_skip("--"));
    /// assert_eq!(parser.remainder(), "foo--bar,baz");
    ///
    /// parser = unwrap_ctx!(parser.rfind_skip(",baz"));
    /// assert_eq!(parser.remainder(), "foo--bar");
    ///
    /// parser = unwrap_ctx!(parser.rfind_skip("--"));
    /// assert_eq!(parser.remainder(), "foo");
    ///
    /// assert!(parser.rfind_skip("--").is_err());
    ///
    /// ```
    ///
    /// ### `char` argument
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_ctx};
    ///
    /// let mut parser = Parser::new("foo,bar-baz");
    ///
    /// parser = unwrap_ctx!(parser.rfind_skip('-'));
    /// assert_eq!(parser.remainder(), "foo,bar");
    ///
    /// parser = unwrap_ctx!(parser.rfind_skip(','));
    /// assert_eq!(parser.remainder(), "foo");
    ///
    /// ```
    ///
    pub const fn rfind_skip<'p, P>(mut self, needle: P) -> Result<Self, ParseError<'a>>
    where
        P: Pattern<'p>,
    {
        try_parsing! {self, FromEnd;
            self.str = match crate::string::rfind_skip(self.str, needle) {
                Some(x) => x,
                None => throw!(ErrorKind::Find),
            };
        }
    }
}
