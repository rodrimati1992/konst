use crate::Parser;

macro_rules! define_parse_methods {
    (
        $((
            $(#[$attr:meta])*
            fn $fn_name:ident,
            fn $fn_name_bytes:ident,
            $parsing:ty $(,)?
        ))*
    ) => (
        $(
            define_parse_methods_inner!{
                concat!(
                    "Parses `", stringify!($parsing), "` from a `&str`.\n\n",
                    "This returns `None` if the string is not a decimal encoding of a `",
                    stringify!($parsing),
                    "`.\n\n",
                ),
                concat!(
                    "Like [`", stringify!($fn_name), "`](./fn.", stringify!($fn_name),".html)",
                    "but takes a `&[u8]` argument."
                ),
                $(#[$attr])*,
                $fn_name,
                $fn_name_bytes,
                $parsing,
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
        $fn_name_bytes:ident,
        $parsing:ty,
    ) => {
        #[doc = $s_docs]
        $(#[$attr])*
        #[inline]
        pub const fn $fn_name(s: &str) -> Option<$parsing> {
            $fn_name_bytes(s.as_bytes())
        }

        #[doc = $b_docs]
        pub const fn $fn_name_bytes(bytes: &[u8]) -> Option<$parsing> {
            match Parser::from_bytes(bytes).$fn_name() {
                Ok((num, parser)) if parser.is_empty() => Some(num),
                _ => None,
            }
        }
    }
}

define_parse_methods! {
    (
        /// # Example
        ///
        /// ```rust
        /// use konst::primitive::parse_u128;
        ///
        /// const I: Option<u128> = parse_u128("1000");
        ///
        /// assert_eq!(I, Some(1000));
        /// assert_eq!(parse_u128("123"), Some(123));
        /// assert_eq!(parse_u128("0"), Some(0));
        ///
        /// assert_eq!(parse_u128("-1"), None);
        /// assert_eq!(parse_u128("100A"), None);
        /// assert_eq!(parse_u128("-"), None);
        ///
        /// ```
        ///
        fn parse_u128, fn parse_u128_b, u128
    )
    (
        /// # Example
        ///
        /// ```rust
        /// use konst::primitive::parse_i128;
        ///
        /// const I: Option<i128> = parse_i128("1234");
        ///
        /// assert_eq!(I, Some(1234));
        /// assert_eq!(parse_i128("123"), Some(123));
        /// assert_eq!(parse_i128("0"), Some(0));
        /// assert_eq!(parse_i128("-1"), Some(-1));
        ///
        /// assert_eq!(parse_i128("100A"), None);
        /// assert_eq!(parse_i128("-A"), None);
        /// assert_eq!(parse_i128("-"), None);
        ///
        /// ```
        ///
        fn parse_i128, fn parse_i128_b, i128
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `u128`](./fn.parse_u128.html).
        fn parse_u64, fn parse_u64_b, u64
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `i128`](./fn.parse_i128.html).
        fn parse_i64, fn parse_i64_b, i64
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `u128`](./fn.parse_u128.html).
        fn parse_u32, fn parse_u32_b, u32
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `i128`](./fn.parse_i128.html).
        fn parse_i32, fn parse_i32_b, i32
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `u128`](./fn.parse_u128.html).
        fn parse_u16, fn parse_u16_b, u16
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `i128`](./fn.parse_i128.html).
        fn parse_i16, fn parse_i16_b, i16
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `u128`](./fn.parse_u128.html).
        fn parse_u8, fn parse_u8_b, u8
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `i128`](./fn.parse_i128.html).
        fn parse_i8, fn parse_i8_b, i8
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `u128`](./fn.parse_u128.html).
        fn parse_usize, fn parse_usize_b, usize
    )
    (
        ///
        /// For an example of how to use this function, you can look at
        /// [the one for `i128`](./fn.parse_i128.html).
        fn parse_isize, fn parse_isize_b, isize
    )
    (
        /// # Example
        ///
        /// ```rust
        /// use konst::primitive::parse_bool;
        ///
        /// const T: Option<bool> = parse_bool("true");
        /// const F: Option<bool> = parse_bool("false");
        ///
        /// assert_eq!(T, Some(true));
        /// assert_eq!(F, Some(false));
        ///
        /// assert_eq!(parse_bool("0"), None);
        /// assert_eq!(parse_bool("FALSE"), None);
        ///
        ///
        /// ```
        ///
        fn parse_bool, fn parse_bool_b, bool
    )
}
