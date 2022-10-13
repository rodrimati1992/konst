use crate::Parser;

macro_rules! define_parse_methods {
    (
        $((
            $(#[$attr:meta])*
            fn $fn_name:ident,
            $parsing:ty,
            $err:ident
            $(,)?
        ))*
    ) => (
        $(
            define_parse_methods_inner!{
                concat!(
                    "Parses a `", stringify!($parsing), "` from a `&str`.\n\n",
                    "This returns an `Err` if the string would not successfully `.parse()` into a `",
                    stringify!($parsing),
                    "`.\n\n",
                    "To parse a `",
                    stringify!($parsing),
                    "` from only part of a string, you can use [`Parser::parse_",
                    stringify!($parsing),
                    "`](../parsing/struct.Parser.html#method.parse_",
                    stringify!($parsing),
                    ")",
                    ".\n\n",
                ),
                concat!(
                    "Like [`", stringify!($fn_name), "`](./fn.", stringify!($fn_name),".html)",
                    "but takes a `&[u8]` argument."
                ),
                $(#[$attr])*,
                $fn_name,
                $parsing,
                $err,
            }
        )*
    );
}
macro_rules! define_parse_methods_inner{
    (
        $s_docs:expr,
        $b_docs:expr,
        $(#[$attr:meta])*,
        $fn_name:ident,
        $parsing:ty,
        $err:ident,
    ) => {
        #[doc = $s_docs]
        $(#[$attr])*
        #[inline]
        #[cfg(feature = "parsing")]
        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
        pub const fn $fn_name(s: &str) -> Result<$parsing, $err> {
            match Parser::new(s).$fn_name() {
                Ok((num, parser)) if parser.is_empty() => Ok(num),
                _ => Err($err {
                    _priv: (),
                }),
            }
        }
    }
}

define_parse_methods! {
    (
        /// # Example
        ///
        /// ```rust
        /// use konst::{
        ///     primitive::{ParseIntResult, parse_u128},
        ///     unwrap_ctx,
        /// };
        ///
        /// const I: ParseIntResult<u128> = parse_u128("1000");
        ///
        /// assert_eq!(I, Ok(1000));
        /// assert_eq!(parse_u128("123"), Ok(123));
        /// assert_eq!(parse_u128("0"), Ok(0));
        ///
        /// // This is how you can unwrap integers parsed from strings, at compile-time.
        /// const I2: u128 = unwrap_ctx!(parse_u128("1000"));
        /// assert_eq!(I2, 1000);
        ///
        /// assert!(parse_u128("-1").is_err());
        /// assert!(parse_u128("100A").is_err());
        /// assert!(parse_u128("-").is_err());
        ///
        /// ```
        ///
        fn parse_u128, u128, ParseIntError
    )
    (
        /// # Example
        ///
        /// ```rust
        /// use konst::{
        ///     primitive::{ParseIntResult, parse_i128},
        ///     unwrap_ctx,
        /// };
        ///
        /// const I: ParseIntResult<i128> = parse_i128("1234");
        ///
        /// assert_eq!(I, Ok(1234));
        /// assert_eq!(parse_i128("123"), Ok(123));
        /// assert_eq!(parse_i128("0"), Ok(0));
        /// assert_eq!(parse_i128("-1"), Ok(-1));
        ///
        /// // This is how you can unwrap integers parsed from strings, at compile-time.
        /// const I2: i128 = unwrap_ctx!(parse_i128("1234"));
        /// assert_eq!(I2, 1234);
        ///
        /// assert!(parse_i128("100A").is_err());
        /// assert!(parse_i128("-A").is_err());
        /// assert!(parse_i128("-").is_err());
        ///
        /// ```
        ///
        fn parse_i128, i128, ParseIntError
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `u128`](./fn.parse_u128.html).
        fn parse_u64, u64, ParseIntError
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `i128`](./fn.parse_i128.html).
        fn parse_i64, i64, ParseIntError
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `u128`](./fn.parse_u128.html).
        fn parse_u32, u32, ParseIntError
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `i128`](./fn.parse_i128.html).
        fn parse_i32, i32, ParseIntError
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `u128`](./fn.parse_u128.html).
        fn parse_u16, u16, ParseIntError
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `i128`](./fn.parse_i128.html).
        fn parse_i16, i16, ParseIntError
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `u128`](./fn.parse_u128.html).
        fn parse_u8, u8, ParseIntError
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `i128`](./fn.parse_i128.html).
        fn parse_i8, i8, ParseIntError
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `u128`](./fn.parse_u128.html).
        fn parse_usize, usize, ParseIntError
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `i128`](./fn.parse_i128.html).
        fn parse_isize, isize, ParseIntError
    )
    (
        /// # Example
        ///
        /// ```rust
        /// use konst::{
        ///     primitive::{ParseBoolResult, parse_bool},
        ///     unwrap_ctx,
        /// };
        ///
        /// const T: ParseBoolResult = parse_bool("true");
        /// const F: ParseBoolResult = parse_bool("false");
        ///
        /// assert_eq!(T, Ok(true));
        /// assert_eq!(F, Ok(false));
        ///
        /// // This is how you can unwrap bools parsed from strings, at compile-time.
        /// const T2: bool = unwrap_ctx!(parse_bool("true"));
        /// const F2: bool = unwrap_ctx!(parse_bool("false"));
        ///
        /// assert_eq!(T2, true);
        /// assert_eq!(F2, false);
        ///
        /// assert!(parse_bool("0").is_err());
        /// assert!(parse_bool("FALSE").is_err());
        ///
        ///
        /// ```
        ///
        fn parse_bool, bool, ParseBoolError
    )
}

////////////////////////////////////////////////////////////////////////////////

/// An alias for `Result<T, konst::primitive::ParseIntError>`
#[cfg(feature = "parsing")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
pub type ParseIntResult<T> = Result<T, ParseIntError>;

/// An alias for `Result<bool, konst::primitive::ParseBoolError>`
#[cfg(feature = "parsing")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
pub type ParseBoolResult = Result<bool, ParseBoolError>;

////////////////////////////////////////////////////////////////////////////////

use core::fmt::{self, Display};

/// The error returned by integer-parsing methods.
#[cfg(feature = "parsing")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ParseIntError {
    _priv: (),
}

impl Display for ParseIntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("could not parse an integer")
    }
}

impl ParseIntError {
    /// Panics with this error as the message
    pub const fn panic(&self) -> ! {
        panic!("could not parse an integer")
    }
}

////////////////////////////////////////////////////////////////////////////////

/// The error returned by bool-parsing methods.
#[cfg(feature = "parsing")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ParseBoolError {
    _priv: (),
}

impl Display for ParseBoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("could not parse a bool")
    }
}

impl ParseBoolError {
    /// Panics with this error as the message
    pub const fn panic(&self) -> ! {
        panic!("could not parse a bool");
    }
}
