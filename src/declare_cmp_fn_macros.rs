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
        pub const fn str_eq(left: &str, right: &str) -> bool {
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
        pub const fn str_cmp(left: &str, right: &str) -> $crate::__::Ordering {
            str_cmp_inner(left.as_bytes(), right.as_bytes()).to_ordering()
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
macro_rules!  __declare_cmp_fns{
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
            __declare_cmp_fns!{
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
                        "A const equivalent of `<[",
                        stringify!($type),
                        "]>::cmp`.",
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
        #[doc = $docs_eq]
        $(#[$attr_both])*
        $(#[$attr_eq])*
        #[inline]
        pub const fn $eq_fn_name(left: &[$ty], right: &[$ty]) -> bool {
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


        #[doc = $docs_ord]
        $(#[$attr_both])*
        $(#[$attr_ord])*
        #[inline]
        pub const fn $cmp_fn_name(left: &[$ty], right: &[$ty]) -> $crate::__::Ordering {
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
    };
}
