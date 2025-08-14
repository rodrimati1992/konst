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
                    "`](crate::parsing::Parser#method.parse_",
                    stringify!($parsing),
                    ")",
                    ".\n\n",
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
            let mut parser = Parser::new(s);

            match parser.$fn_name() {
                Ok(num) if parser.is_empty() => Ok(num),
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
        ///     result::unwrap,
        /// };
        ///
        /// const T: ParseBoolResult = parse_bool("true");
        /// const F: ParseBoolResult = parse_bool("false");
        ///
        /// assert_eq!(T, Ok(true));
        /// assert_eq!(F, Ok(false));
        ///
        /// // This is how you can unwrap bools parsed from strings, at compile-time.
        /// const T2: bool = unwrap!(parse_bool("true"));
        /// const F2: bool = unwrap!(parse_bool("false"));
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
pub type ParseBoolResult = Result<bool, ParseBoolError>;

////////////////////////////////////////////////////////////////////////////////

use core::fmt::{self, Display};

/// The error returned by bool-parsing methods.
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "parsing")))]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct ParseBoolError {
    _priv: (),
}

impl fmt::Debug for ParseBoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ParseBoolError").finish()
    }
}

impl Display for ParseBoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(Self::DISPLAY_ERR)
    }
}

const _: () = {
    use const_panic::{
        PanicFmt, PanicVal,
        fmt::{FmtArg, FmtKind},
    };

    impl PanicFmt for ParseBoolError {
        type This = Self;
        type Kind = const_panic::IsCustomType;

        const PV_COUNT: usize = 1;
    }

    impl ParseBoolError {
        const DISPLAY_ERR: &'static str = "could not parse a bool";

        /// Formats a ParseBoolError
        pub const fn to_panicval(&self, fmtarg: FmtArg) -> PanicVal<'static> {
            match fmtarg.fmt_kind {
                FmtKind::Display => PanicVal::write_str(Self::DISPLAY_ERR),
                _ => PanicVal::write_str("ParseBoolError"),
            }
        }

        /// Formats a ParseBoolError
        pub const fn to_panicvals(
            &self,
            fmtarg: FmtArg,
        ) -> [PanicVal<'static>; ParseBoolError::PV_COUNT] {
            [self.to_panicval(fmtarg)]
        }
    }
};
