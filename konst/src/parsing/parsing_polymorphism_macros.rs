/// Parses a type that impls [`HasParser`] with the passed in [`Parser`].
///
/// # Example
///
/// This example demonstrates how you can use this macro to parse both
/// standard library and user-defined types.
///
/// ```rust
/// use konst::{result, try_};
///
/// use konst::parsing::{HasParser, Parser, ParseError, parse_type};
///
/// const PAIR: (u32, Foo) = result::unwrap!(parse_pair(&mut Parser::new("100,Baz")));
///
/// assert_eq!(PAIR.0, 100);
/// assert_eq!(PAIR.1, Foo::Baz);
///
/// const fn parse_pair<'p>(parser: &mut Parser<'p>) -> Result<(u32, Foo), ParseError<'p>> {
///     let left = try_!(parse_type!(parser, u32));
///     _ = parser.strip_prefix(',');
///     let right = try_!(parse_type!(parser, Foo));
///     
///     Ok((left, right))
/// }
///
///
/// #[derive(Debug, PartialEq)]
/// enum Foo {
///     Bar,
///     Baz,
///     Qux,
/// }
///
/// impl HasParser for Foo {
///     type Parser = Self;
/// }
///
/// impl Foo {
///     const fn parse_with<'p>(parser: &mut Parser<'p>) -> Result<Self, ParseError<'p>> {
///         // You can use the `parser_method` macro instead of this chain of if elses
///         if parser.strip_prefix("Bar").is_ok() {
///             Ok(Foo::Bar)
///         } else if parser.strip_prefix("Baz").is_ok() {
///             Ok(Foo::Baz)
///         } else if parser.strip_prefix("Qux").is_ok() {
///             Ok(Foo::Qux)
///         } else {
///             Err(parser.to_other_error(&"expected one of `Bar`, `Baz`, or `Qux`"))
///         }
///     }
/// }
/// ```
///
/// [`Parser`]: crate::parsing::Parser
/// [`HasParser`]: crate::parsing::HasParser
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
pub use crate::__parse_type as parse_type;

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_type {
    ($parser:expr, $type:ty $(,)*) => {
        match $parser.__borrow_mut() {
            parser @ $crate::Parser { .. } => {
                let res: $crate::__::Result<_, _> =
                    <<$type as $crate::parsing::HasParser>::Parser>::parse_with(parser);
                res
            }
        }
    };
}
