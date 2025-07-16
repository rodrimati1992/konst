/// Parses a type that impls [`HasParser`] with the passed in [`Parser`].
///
/// # Example
///
/// This example demonstrates how you can use this macro to parse both
/// standard library and user-defined types.
///
/// ```rust
/// use konst::{parse_with, try_, unwrap_ctx};
///
/// use konst::parsing::{HasParser, Parser, ParseValueResult};
///
/// const PAIR: (u32, Foo) = unwrap_ctx!(parse_pair(&mut Parser::new("100,Baz")));
///
/// assert_eq!(PAIR.0, 100);
/// assert_eq!(PAIR.1, Foo::Baz);
///
/// const fn parse_pair<'p>(parser: &mut Parser<'p>) -> ParseValueResult<'p, (u32, Foo)> {
///     let left = try_!(parse_with!(parser, u32));
///     _ = parser.strip_prefix(',');
///     let right = try_!(parse_with!(parser, Foo));
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
///     const fn parse_with<'p>(parser: &mut Parser<'p>) -> ParseValueResult<'p, Self> {
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
/// [`Parser`]: ./parsing/struct.Parser.html
/// [`HasParser`]: ./parsing/trait.HasParser.html
#[macro_export]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
macro_rules! parse_with {
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
