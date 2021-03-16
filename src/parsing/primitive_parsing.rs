use super::Parser;

use core::fmt::{self, Display};

impl<'a> Parser<'a> {
    /// Parses a `u128` until a non-digit is reached.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_res};
    ///
    /// {
    ///     let parser = Parser::from_str("12345");
    ///     let (num, parser) = unwrap_res!(parser.parse_u128());
    ///     assert_eq!(num, 12345);
    ///     assert!(parser.bytes().is_empty());
    /// }
    /// {
    ///     let parser = Parser::from_str("1365;6789");
    ///     let (num, parser) = unwrap_res!(parser.parse_u128());
    ///     assert_eq!(num, 1365);
    ///     assert_eq!(parser.bytes(), b";6789");
    /// }
    ///
    /// ```
    ///
    pub const fn parse_u128(mut self) -> Result<(u128, Self), ParseIntError> {
        parse_integer! {unsigned, (u128, u128), self}
    }
    /// Parses a `i128` until a non-digit is reached.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{Parser, unwrap_res};
    ///
    /// {
    ///     let parser = Parser::from_str("12345");
    ///     let (num, parser) = unwrap_res!(parser.parse_i128());
    ///     assert_eq!(num, 12345);
    ///     assert!(parser.bytes().is_empty());
    /// }
    /// {
    ///     let parser = Parser::from_str("-54321;6789");
    ///     let (num, parser) = unwrap_res!(parser.parse_i128());
    ///     assert_eq!(num, -54321);
    ///     assert_eq!(parser.bytes(), b";6789");
    /// }
    ///
    /// ```
    ///
    pub const fn parse_i128(mut self) -> Result<(i128, Self), ParseIntError> {
        parse_integer! {signed, (i128, u128), self}
    }
    /// Parses a `u64` until a non-digit is reached.
    pub const fn parse_u64(mut self) -> Result<(u64, Self), ParseIntError> {
        parse_integer! {unsigned, (u64, u64), self}
    }
    /// Parses a `i64` until a non-digit is reached.
    pub const fn parse_i64(mut self) -> Result<(i64, Self), ParseIntError> {
        parse_integer! {signed, (i64, u64), self}
    }
    /// Parses a `usize` until a non-digit is reached.
    pub const fn parse_usize(mut self) -> Result<(usize, Self), ParseIntError> {
        parse_integer! {unsigned, (usize, usize), self}
    }
    /// Parses a `isize` until a non-digit is reached.
    pub const fn parse_isize(mut self) -> Result<(isize, Self), ParseIntError> {
        parse_integer! {signed, (isize, usize), self}
    }
}

macro_rules! parse_integer {
    ($signedness:ident, ($type:ty, $uns:ty), $parser:ident) => {{
        let mut num: $uns;

        parse_integer! {@parse_signed $signedness, ($type, $uns), $parser, num, sign}

        while let [byte @ b'0'..=b'9', rem @ ..] = $parser.bytes {
            $parser.bytes = rem;

            let (next_mul, overflowed_mul) = num.overflowing_mul(10);
            let (next_add, overflowed_add) = next_mul.overflowing_add((*byte - b'0') as $uns);

            if overflowed_mul | overflowed_add {
                return Err(ParseIntError(()));
            }

            num = next_add;
        }

        parse_integer! {@apply_sign $signedness, ($type, $uns), num, sign}

        Ok((num, $parser))
    }};
    (@parse_signed signed, ($type:ty, $uns:ty), $parser:ident, $num:ident, $isneg:ident) => {
        let $isneg = if let [b'-', rem @ ..] = $parser.bytes {
            $parser.bytes = rem;
            true
        } else {
            false
        };

        parse_integer!(@parse_signed unsigned, ($type, $uns), $parser, $num, $isneg)
    };
    (@parse_signed unsigned, ($type:ty, $uns:ty), $parser:ident, $num:ident, $isneg:ident) => {
        $num = if let [byte @ b'0'..=b'9', rem @ ..] = $parser.bytes {
            $parser.bytes = rem;
            (*byte - b'0') as $uns
        } else {
            return Err(ParseIntError(()));
        };
    };
    (@apply_sign signed, ($type:ty, $uns:ty), $num:ident, $isneg:ident) => {
        const MAX_POS: $uns = <$type>::MAX as $uns;
        const MAX_NEG: $uns = <$type>::MIN as $uns;

        let $num = if $isneg {
            if $num <= MAX_NEG {
                ($num as $type).wrapping_neg()
            } else {
                return Err(ParseIntError(()));
            }
        } else {
            if $num <= MAX_POS {
                $num as $type
            } else {
                return Err(ParseIntError(()));
            }
        };
    };
    (@apply_sign unsigned, ($type:ty, $uns:ty), $num:ident, $isneg:ident) => {};
}
use parse_integer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseIntError(());

impl Display for ParseIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("could not parse an integer")
    }
}
