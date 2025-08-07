macro_rules! ltgt_shared_docs {
    ($op:tt) => {
        concat!(
            "Evaluates the const equivalent of `$left ",
            stringify!($op),
            " $right` for [`ConstCmp`](crate::cmp::ConstCmp) implementors.\n",
            "\n",
            "This delegates to the [`const_cmp`](crate::cmp::const_cmp) ",
            "macro to do the comparison itself.\n",
            "\n",
            "# Limitation\n",
            "\n",
            "The arguments must be concrete types, and have a fully inferred type.\n",
            "eg: if you pass an integer literal it must have a suffix to indicate its type.\n",
        )
    };
}
use ltgt_shared_docs;

macro_rules! ltgt_for_shared_docs {
    ($op:tt $method:literal) => {
        concat!(
            "Compares two standard library types for ordering,\n",
            "that can't be compared with the [`",
            $method,
            "`] macro.\n",
            "\n",
            "This macro delegates to the [`const_cmp_for`] macro ",
            "to do the comparison.\n",
            "\n",
            "[`ConstCmp`]: crate::cmp::ConstCmp\n",
            "[`",
            $method,
            "`]: crate::cmp::",
            $method,
            "\n",
            "\n",
            "[`const_eq_for`]: crate::cmp::const_eq_for\n",
            "[`const_cmp_for`]: crate::cmp::const_cmp_for\n",
        )
    };
}
use ltgt_for_shared_docs;

#[doc = self::ltgt_shared_docs!(<)]
///
/// # Example
///
/// ```rust
/// use konst::cmp::const_lt;
///
/// const _: () = {
///     assert!(!const_lt!([3u8, 5], [3u8]));
///     assert!(!const_lt!([3u8, 5], [3u8, 5]));
///     assert!( const_lt!([3u8, 5], [3u8, 5, 8]));
/// };
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_lt as const_lt;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_lt {
    ($left:expr, $right:expr $(,)*) => {
        $crate::__::matches!($crate::cmp::const_cmp!($left, $right), $crate::__::Less,)
    };
}

#[doc = self::ltgt_for_shared_docs!(< "const_lt")]
///
/// # Example
///
/// ```rust
/// use konst::cmp::const_lt_for;
///
/// #[derive(Copy, Clone)]
/// enum Shape {
///     Square,
///     Circle,
///     Line,
/// }
/// use Shape::*;
///
/// const _: () = {
///     assert!(!const_lt_for!(slice; [Square, Circle], [Square, Square], |x| *x as u8));
///     assert!(!const_lt_for!(slice; [Square, Circle], [Square, Circle], |x| *x as u8));
///     assert!( const_lt_for!(slice; [Square, Circle], [Square, Line  ], |x| *x as u8));
/// };
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_lt_for as const_lt_for;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_lt_for {
    ($($args:tt)*) => {
        $crate::__::matches!(
            $crate::cmp::const_cmp_for!($($args)*),
            $crate::__::Less,
        )
    };
}

////////////////////////////////////////////////////////////////////////////////

#[doc = self::ltgt_shared_docs!(<=)]
///
/// # Example
///
/// ```rust
/// use konst::cmp::const_le;
///
/// const _: () = {
///     assert!(!const_le!([3u8, 5], [3u8]));
///     assert!( const_le!([3u8, 5], [3u8, 5]));
///     assert!( const_le!([3u8, 5], [3u8, 5, 8]));
/// };
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_le as const_le;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_le {
    ($left:expr, $right:expr $(,)*) => {
        $crate::__::matches!(
            $crate::cmp::const_cmp!($left, $right),
            $crate::__::Less | $crate::__::Equal,
        )
    };
}

#[doc = self::ltgt_for_shared_docs!(<= "const_le")]
///
/// # Example
///
/// ```rust
/// use konst::cmp::const_le_for;
///
/// #[derive(Copy, Clone)]
/// enum Shape {
///     Square,
///     Circle,
///     Line,
/// }
/// use Shape::*;
///
/// const _: () = {
///     assert!(!const_le_for!(slice; [Square, Circle], [Square, Square], |x| *x as u8));
///     assert!( const_le_for!(slice; [Square, Circle], [Square, Circle], |x| *x as u8));
///     assert!( const_le_for!(slice; [Square, Circle], [Square, Line  ], |x| *x as u8));
/// };
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_le_for as const_le_for;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_le_for {
    ($($args:tt)*) => {
        $crate::__::matches!(
            $crate::cmp::const_cmp_for!($($args)*),
            $crate::__::Less | $crate::__::Equal,
        )
    };
}

////////////////////////////////////////////////////////////////////////////////

#[doc = self::ltgt_shared_docs!(>)]
///
/// # Example
///
/// ```rust
/// use konst::cmp::const_gt;
///
/// const _: () = {
///     assert!( const_gt!([3u8, 5], [3u8]));
///     assert!(!const_gt!([3u8, 5], [3u8, 5]));
///     assert!(!const_gt!([3u8, 5], [3u8, 5, 8]));
/// };
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_gt as const_gt;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_gt {
    ($left:expr, $right:expr $(,)*) => {
        $crate::__::matches!($crate::cmp::const_cmp!($left, $right), $crate::__::Greater,)
    };
}

#[doc = self::ltgt_for_shared_docs!(> "const_gt")]
///
/// # Example
///
/// ```rust
/// use konst::cmp::const_gt_for;
///
/// #[derive(Copy, Clone)]
/// enum Shape {
///     Square,
///     Circle,
///     Line,
/// }
/// use Shape::*;
///
/// const _: () = {
///     assert!( const_gt_for!(slice; [Square, Circle], [Square, Square], |x| *x as u8));
///     assert!(!const_gt_for!(slice; [Square, Circle], [Square, Circle], |x| *x as u8));
///     assert!(!const_gt_for!(slice; [Square, Circle], [Square, Line  ], |x| *x as u8));
/// };
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_gt_for as const_gt_for;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_gt_for {
    ($($args:tt)*) => {
        $crate::__::matches!(
            $crate::cmp::const_cmp_for!($($args)*),
            $crate::__::Greater
        )
    };
}

////////////////////////////////////////////////////////////////////////////////

#[doc = self::ltgt_shared_docs!(>=)]
///
/// # Example
///
/// ```rust
/// use konst::cmp::const_ge;
///
/// const _: () = {
///     assert!( const_ge!([3u8, 5], [3u8]));
///     assert!( const_ge!([3u8, 5], [3u8, 5]));
///     assert!(!const_ge!([3u8, 5], [3u8, 5, 8]));
/// };
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_ge as const_ge;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_ge {
    ($left:expr, $right:expr $(,)*) => {
        $crate::__::matches!(
            $crate::cmp::const_cmp!($left, $right),
            $crate::__::Greater | $crate::__::Equal,
        )
    };
}

#[doc = self::ltgt_for_shared_docs!(>= "const_ge")]
///
/// # Example
///
/// ```rust
/// use konst::cmp::const_ge_for;
///
/// #[derive(Copy, Clone)]
/// enum Shape {
///     Square,
///     Circle,
///     Line,
/// }
/// use Shape::*;
///
/// const _: () = {
///     assert!( const_ge_for!(slice; [Square, Circle], [Square, Square], |x| *x as u8));
///     assert!( const_ge_for!(slice; [Square, Circle], [Square, Circle], |x| *x as u8));
///     assert!(!const_ge_for!(slice; [Square, Circle], [Square, Line  ], |x| *x as u8));
/// };
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__const_ge_for as const_ge_for;

#[doc(hidden)]
#[macro_export]
macro_rules! __const_ge_for {
    ($($args:tt)*) => {
        $crate::__::matches!(
            $crate::cmp::const_cmp_for!($($args)*),
            $crate::__::Greater | $crate::__::Equal
        )
    };
}
