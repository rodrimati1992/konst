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

/// An alias for `Result<bool, konst::primitive::ParseBoolError>`
#[cfg(feature = "parsing")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
pub type ParseBoolResult = Result<bool, ParseBoolError>;

////////////////////////////////////////////////////////////////////////////////

use core::fmt::{self, Display};

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
