/// Const equivalent of [`std::cmp::min`]
///
/// The arguments must implement the [`ConstCmp`] trait.
/// Non-standard library types must define a `const_eq` method taking a reference.
///
/// # Example
///
/// ```rust
/// const M: u32 = konst::min!(3u32, 5);
/// assert_eq!(M, 3);
/// ```
///
/// [`ConstCmp`]: crate::cmp::ConstCmp
#[macro_export]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
macro_rules! min {
    ($left:expr, $right:expr) => {
        match ($left, $right) {
            (left, right) => {
                if let $crate::__::Less = $crate::const_cmp!(left, right) {
                    left
                } else {
                    right
                }
            }
        }
    };
}

/// Const equivalent of [`std::cmp::max`]
///
/// The arguments must implement the [`ConstCmp`] trait.
/// Non-standard library types must define a `const_eq` method taking a reference.
///
/// # Example
///
/// ```rust
/// const M: &str = konst::max!("world", "hello");
/// assert_eq!(M, "world");
/// ```
///
/// [`ConstCmp`]: crate::cmp::ConstCmp
#[macro_export]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
macro_rules! max {
    ($left:expr, $right:expr) => {
        match ($left, $right) {
            (left, right) => {
                if let $crate::__::Greater = $crate::const_cmp!(left, right) {
                    left
                } else {
                    right
                }
            }
        }
    };
}

////////////////////////////////////////////////////////////////////////////////

/// Const equivalent of [`std::cmp::min_by`]
///
/// # Example
///
/// ```rust
/// // passing a pseudo-closure as the comparator
/// const AAA: u32 = konst::min_by!(3u32, 10, |&l, &r| konst::const_cmp!(l, r / 4));
/// assert_eq!(AAA, 10);
///
///
/// const fn cmp_len(l: &str, r: &str) -> std::cmp::Ordering {
///     konst::const_cmp!(l.len(), r.len())
/// }
///
/// // passing a function as the comparator
/// const BBB: &str = konst::min_by!("he", "bar", cmp_len);
/// assert_eq!(BBB, "he");
/// ```
#[macro_export]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
macro_rules! min_by {
    ($left:expr, $right:expr, $($closure:tt)*) => {
        $crate::__::__parse_closure_2!{
            ($crate::__minmax_by)
            ($left, $right, Less,)
            (min_by),

            $($closure)*
        }
    };
}

/// Const equivalent of [`std::cmp::max_by`]
///
/// # Example
///
/// ```rust
/// // passing a pseudo-closure as the comparator
/// const AAA: u32 = konst::max_by!(3u32, 10, |&l, &r| konst::const_cmp!(l, r / 4));
/// assert_eq!(AAA, 3);
///
///
/// const fn cmp_len(l: &str, r: &str) -> std::cmp::Ordering {
///     konst::const_cmp!(l.len(), r.len())
/// }
///
/// // passing a function as the comparator
/// const BBB: &str = konst::max_by!("he", "bar", cmp_len);
/// assert_eq!(BBB, "bar");
/// ```
#[macro_export]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
macro_rules! max_by {
    ($left:expr, $right:expr, $($closure:tt)*) => {
        $crate::__::__parse_closure_2!{
            ($crate::__minmax_by)
            ($left, $right, Greater,)
            (max_by),

            $($closure)*
        }
    };
}

#[macro_export]
#[doc(hidden)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
macro_rules! __minmax_by {
    (
        $left:expr, $right:expr, $ord:ident,
        |$left_p:pat_param, $right_p:pat_param| $(-> $ret_ty:ty)? $ret_val:block
    ) => {
        match ($left, $right) {
            (left, right) => {
                let $left_p = &left;
                let $right_p = &right;
                if let $crate::__::$ord = $ret_val {
                    left
                } else {
                    right
                }
            }
        }
    };
}
