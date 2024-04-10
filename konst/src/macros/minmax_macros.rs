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
