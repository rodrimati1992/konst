use crate::parsing::{ParseValueResult, Parser};

use core::marker::PhantomData;

/// Gets a type that parses `Self` with a `parse_with` method.
///
/// Implementing this trait allows parsing a type with the [`parse_type`] macro.
///
/// # Implementing this trait
///
/// You can implement this trait like this:
/// ```rust
/// # struct SomeType;
/// # struct SomeParser;
/// # use konst::parsing::HasParser;
/// impl HasParser for SomeType {
///     // This is usually `Self` for user-defined types.
///     type Parser = SomeParser;
/// }
/// ```
/// Then `SomeParser` is expected to have a `parse_with` associated function with this signature:
/// ```rust
/// # struct SomeParser;
/// # struct This;
/// # struct SomeErrorType;
/// impl SomeParser {
///     const fn parse_with<'p>(
///         _: &mut konst::Parser<'p>
///     ) -> Result<This, SomeErrorType>
/// # { unimplemented!() }
/// }
/// ```
///
/// # Example
///
/// ```rust
/// use konst::{result, try_};
///
/// use konst::parsing::{HasParser, Parser, ParseValueResult, parse_type};
///
/// const PAIR: Pair = {
///     let mut parser = Parser::new("100,200");
///     result::unwrap!(parse_type!(parser, Pair))
/// };
///
/// assert_eq!(PAIR, Pair(100, 200));
///
///
/// #[derive(Debug, PartialEq)]
/// struct Pair(u32, u64);
///
/// impl HasParser for Pair {
///     type Parser = Self;
/// }
///
/// impl Pair {
///     const fn parse_with<'p>(parser: &mut Parser<'p>) -> ParseValueResult<'p, Self> {
///         let left = try_!(parse_type!(parser, u32));
///         try_!(parser.strip_prefix(','));
///         let right = try_!(parse_type!(parser, u64));
///
///         Ok(Pair(left, right))
///     }
/// }
/// ```
///
/// [`parse_type`]: crate::parsing::parse_type
/// [`HasParser::Parser`]: #associatedtype.Parser
///
pub trait HasParser: Sized {
    /// The type that parses `Self` with its `parse_with` associated function.
    ///
    /// This is usually `Self` for user-defined types.
    type Parser;
}

////////////////////////////////////////////////////////////////////////////////

/// Parses a standard library type out of a [`Parser`],
/// determined by the `StdType` type parameter.
///
/// Note: since this uses [`Parser`], it doesn't parse the entire string,
/// it parses the starting bytes that successfully parse as the target type.
///
pub struct StdParser<StdType>(PhantomData<StdType>);

macro_rules! impl_std_parser_one {
    ($method:ident, $type:ty, $parse_with_docs:expr) => {
        impl HasParser for $type {
            type Parser = StdParser<$type>;
        }

        impl StdParser<$type> {
            #[doc = $parse_with_docs]
            pub const fn parse_with<'a>(parser: &mut Parser<'a>) -> ParseValueResult<'a, $type> {
                parser.$method()
            }
        }
    };
}
macro_rules! impl_std_parser {
    ($($method:ident -> $type:ty;)*) => (
        $(
            impl_std_parser_one!{
                $method,
                $type,
                concat!("Atempts to parse `", stringify!($type), "`")
            }
        )*
    )
}

impl_std_parser! {
    parse_u128 -> u128;
    parse_i128 -> i128;
    parse_u64 -> u64;
    parse_i64 -> i64;
    parse_u32 -> u32;
    parse_i32 -> i32;
    parse_u16 -> u16;
    parse_i16 -> i16;
    parse_u8 -> u8;
    parse_i8 -> i8;
    parse_usize -> usize;
    parse_isize -> isize;
    parse_bool -> bool;
}
