#[doc(hidden)]
#[macro_export]
macro_rules! __declare_string_cmp_fns {
    (
        import_path = $path:expr,

        equality_fn = $str_eq:ident,
        ordering_fn = $str_cmp:ident,
        /// Equivalent to ordering_fn, but returns a U8Ordering
        ordering_fn_inner = $str_cmp_inner:ident,
    ) => {
        $crate::__declare_string_cmp_fns! {
            @inner
            equality_fn = $str_eq,
            ordering_fn = $str_cmp,
            use_str_eq = concat!("use ", $path, "::", stringify!($str_eq), ";"),
            use_str_cmp = concat!("use ", $path, "::", stringify!($str_cmp), ";"),
        }
    };
    (@inner

        equality_fn = $str_eq:ident,
        ordering_fn = $str_cmp:ident,
        use_str_eq = $str_eq_import:expr,
        use_str_cmp = $str_cmp_import:expr,
    ) => {
        $crate::__delegate_const_eq! {
            skip_coerce;
            /// A const equivalent of `&str` equality comparison.
            ///
            /// # Example
            ///
            /// ```rust
            #[doc = $str_eq_import]
            ///
            /// const FOO: &str = "foo";
            /// const BAR: &str = "fooooo";
            /// const BAZ: &str = "bar";
            ///
            ///
            /// const FOO_EQ_FOO: bool = str_eq(FOO, FOO);
            /// assert!( FOO_EQ_FOO );
            ///
            /// const FOO_EQ_BAR: bool = str_eq(FOO, BAR);
            /// assert!( !FOO_EQ_BAR );
            ///
            /// const FOO_EQ_BAZ: bool = str_eq(FOO, BAZ);
            /// assert!( !FOO_EQ_BAZ );
            ///
            /// ```
            ///
            #[inline]
            pub const fn str_eq(ref left: &str, right: &str) -> bool {
                let left = left.as_bytes();
                let right = right.as_bytes();

                if left.len() != right.len() {
                    return false;
                }

                let mut i = 0;
                while i != left.len() {
                    if left[i] != right[i] {
                        return false;
                    }
                    i += 1;
                }

                true
            }
        }

        __delegate_const_ord! {
            skip_coerce;
            /// A const equivalent of `str::cmp`.
            ///
            /// # Example
            ///
            /// ```rust
            #[doc = $str_cmp_import]
            ///
            /// use std::cmp::Ordering;
            ///
            /// const FOO: &str = "foo";
            /// const BAR: &str = "fooooo";
            /// const BAZ: &str = "bar";
            ///
            ///
            /// const FOO_CMP_FOO: Ordering = str_cmp(FOO, FOO);
            /// assert_eq!(FOO_CMP_FOO, Ordering::Equal);
            ///
            /// const FOO_CMP_BAR: Ordering = str_cmp(FOO, BAR);
            /// assert_eq!(FOO_CMP_BAR, Ordering::Less);
            ///
            /// const FOO_CMP_BAZ: Ordering = str_cmp(FOO, BAZ);
            /// assert_eq!(FOO_CMP_BAZ, Ordering::Greater);
            ///
            /// ```
            ///
            #[inline]
            pub const fn str_cmp(ref left: &str, right: &str) -> $crate::__::Ordering {
                str_cmp_inner(left.as_bytes(), right.as_bytes()).to_ordering()
            }
        }

        #[inline]
        const fn str_cmp_inner(left: &[u8], right: &[u8]) -> $crate::__::U8Ordering {
            use $crate::__::U8Ordering;

            let left_len = left.len();
            let right_len = right.len();
            let (min_len, on_ne) = if left_len < right_len {
                (left_len, U8Ordering::LESS)
            } else {
                (right_len, U8Ordering::GREATER)
            };

            let mut i = 0;
            while i < min_len {
                $crate::__priv_ret_if_ne! {left[i], right[i]}
                i += 1;
            }

            if left_len == right_len {
                U8Ordering::EQUAL
            } else {
                on_ne
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules!  __declare_slice_cmp_fns{
    (
        import_path = $path:expr,


        $((
            $(#[$attr_both:meta])*,
            $(#[$attr_eq:meta])*,
            $(#[$attr_ord:meta])*,
            $type:ty,
            $eq_fn_name:ident,
            $cmp_fn_name:ident,
        ))*
    )=>{
        $(
            __declare_slice_cmp_fns!{
                @step_two
                    import_path = $path,

                    $(#[$attr_both])*,
                    $(#[$attr_eq])*,
                    $(#[$attr_ord])*,
                    concat!(
                        "Compares two `&[",
                        stringify!($type),
                        "]` for equality.",
                    ),
                    concat!(
                        "Compares two `&[",
                        stringify!($type),
                        "]`, returning the order of `left` relative to `right`.",
                    ),
                    $type,
                    $eq_fn_name,
                    $cmp_fn_name,
            }
        )*
    };
    (@step_two
        import_path = $path:expr,

        $(#[$attr_both:meta])*,
        $(#[$attr_eq:meta])*,
        $(#[$attr_ord:meta])*,
        $docs_eq:expr,
        $docs_ord:expr,
        $ty:ty,
        $eq_fn_name:ident,
        $cmp_fn_name:ident,
    ) => {
        $crate::__delegate_const_eq!{
            skip_coerce;

            #[doc = $docs_eq]
            $(#[$attr_both])*
            $(#[$attr_eq])*
            #[inline]
            pub const fn $eq_fn_name(ref left: &[$ty], right: &[$ty]) -> bool {
                if left.len() != right.len() {
                    return false;
                }

                let mut i = 0;
                while i != left.len() {
                    if left[i] != right[i] {
                        return false;
                    }
                    i += 1;
                }

                true
            }
        }



        __delegate_const_ord!{
            skip_coerce;
            for['a,]

            #[doc = $docs_ord]
            $(#[$attr_both])*
            $(#[$attr_ord])*
            #[inline]
            pub const fn $cmp_fn_name(ref left: &[$ty], right: &[$ty]) -> $crate::__::Ordering {
                use $crate::__::U8Ordering;

                const fn cmp_inner(left: &[$ty], right: &[$ty]) -> $crate::__::U8Ordering {
                    let left_len = left.len();

                    $crate::__priv_ret_if_ne! {left_len, right.len()}

                    let mut i = 0;
                    while i < left_len {
                        $crate::__priv_ret_if_ne! {left[i], right[i]}
                        i += 1;
                    }

                    U8Ordering::EQUAL
                }

                cmp_inner(left, right).to_ordering()
            }
        }

    };
}

#[doc(hidden)]
#[macro_export]
macro_rules!  __declare_fns_with_docs{
    (
        $(($($args:tt)*))*

        docs $docs:tt

        macro = $macro:ident ! $macro_prefix:tt,
    )=>{
        $(
            $crate::__declare_fns_with_docs!{
                @inner
                ($($args)*)

                docs $docs

                macro = $macro ! $macro_prefix,
            }
        )*
    };
    (@inner
        (
            $type:ty,
            ($($func_name:ident),* $(,)?)
            $($rem:tt)*
        )

        docs(
            $(($before:expr, $after:expr))*
        )

        macro = $macro:ident ! ($($prefix:tt)*),
    ) => {

        $macro!{
            $($prefix)*
            ($type, ($($func_name),*) $($rem)* )

            docs(
                $(concat!($before, stringify!($type), $after)),*
            )
        }


    };
    (@inner
        (
            $type:ty,
            ($($func_name:ident),* $(,)?)
            $($rem:tt)*
        )

        docs(default)

        macro = $macro:ident ! ($($prefix:tt)*),
    ) => {

        $macro!{
            $($prefix)*
            ($type, ($($func_name),*) $($rem)* )

            docs(
                concat!(
                    "Compares two `",
                    stringify!($type),
                    "` for equality.",
                ),
                concat!(
                    "Compares two `",
                    stringify!($type),
                    "`, returning the ordering of `left` relative to `right`."
                ),
            )
        }


    };
}

macro_rules! __impl_option_cmp_fns {
    (
        $(for[$($impl:tt)*])?
        params($l:ident, $r:ident)
        eq_comparison = $eq_comparison:expr,
        cmp_comparison = $cmp_comparison:expr,

        ($type:ty, ($eq_fn_name:ident, $cmp_fn_name:ident))

        docs( $docs_eq:expr, $docs_cmp:expr, )
    ) => (
        __delegate_const_eq!{
            $(for[$($impl)*])?

            #[doc = $docs_eq]
            pub const fn $eq_fn_name(copy left: $type, right: $type) -> bool {
                match (left, right) {
                    (Some($l), Some($r)) => $eq_comparison,
                    (None, None) => true,
                    _ => false,
                }
            }
        }

        __delegate_const_ord!{
            $(for[$($impl)*])?

            #[doc = $docs_cmp]
            pub const fn $cmp_fn_name(copy left: $type, right: $type) -> core::cmp::Ordering {
                use core::cmp::Ordering;

                match (left, right) {
                    (Some($l), Some($r)) => $cmp_comparison,
                    (Some(_), None) => Ordering::Greater,
                    (None, Some(_)) => Ordering::Less,
                    (None, None) => Ordering::Equal,
                }
            }
        }
    )
}
