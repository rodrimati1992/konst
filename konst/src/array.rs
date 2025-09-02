//! Const equivalents of array functions.

#[doc(hidden)]
pub mod __array_macros_2;

#[cfg(feature = "iter")]
mod array_builder;

#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use self::array_builder::ArrayBuilder;

#[cfg(feature = "iter")]
mod array_into_iter;

#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use self::array_into_iter::{IntoIter, IntoIterRev};

macro_rules! drop_warning {
    () => {
        concat!(
            "# Note",
            "\n\n",
            "`return` inside the closure passed to this macro ",
            "attempts to return from the function where this macro is called, ",
            "which drops the array elements, ",
            "and dropping isn't allowed in const as of Rust 1.89.0.",
            "\n\n",
            "The same applies to `?`, ",
            "and labelled `break`/`continue` into labels from outside the closure.",
        )
    };
}

use drop_warning;

/// Const equivalent of
/// [`array::map`](https://doc.rust-lang.org/std/primitive.array.html#method.map)
///
/// Consider using [`konst::array::map_nd`](crate::array::map_nd)
/// if you're mapping from and into an
/// array of non-drop types in const and you need early returns.
///
#[doc = drop_warning!()]
///
#[doc = crate::docs::closure_arg_annotated_params_limitations_docs!("")]
///
/// # Example
///
/// ```rust
/// assert_eq!(PAIRS, [(3, "hello"), (5, "world"), (8, "foo")]);
///
/// const PAIRS: [(u8, &str); 3] =
///     swap_pairs([("hello", 3), ("world", 5), ("foo", 8)]);
///
/// const fn swap_pairs<T, U, const N: usize>(pairs: [(T, U); N]) -> [(U, T); N] {
///     konst::array::map!(pairs, |pair: (T, U)| {
///         // need to use `destructure` to destructure types that may contain Drop fields
///         konst::destructure!{(a, b) = pair}
///         (b, a)
///     })
/// }
/// ```
///
#[doc(inline)]
#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use crate::__array_map_by_val as map;

/// Const equivalent of [`array::from_fn`](core::array::from_fn).
///
/// Consider using [`konst::array::from_fn_nd`](crate::array::from_fn_nd)
/// if you're creating an array of non-drop types in const and you need early returns.
///
#[doc = drop_warning!()]
///
#[doc = crate::docs::closure_arg_annotated_params_limitations_docs!("")]
///
/// # Example
///
/// ```rust
/// use konst::array;
///
/// {
///     const POWERS: [u64; 5] = array::from_fn!(|i| 2u64.pow(i as u32));
///
///     assert_eq!(POWERS, [1, 2, 4, 8, 16]);
/// }
///
/// // Annotating the array type
/// assert_eq!(
///     array::from_fn!([&str; 6] => |i| konst::string::str_up_to("hello", i)),
///     ["", "h", "he", "hel", "hell", "hello"],
/// );
/// ```
///
#[doc(inline)]
#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use crate::__array_from_fn2 as from_fn;

/// Const equivalent of
/// [`array::map`](https://doc.rust-lang.org/std/primitive.array.html#method.map),
/// which allows `return`ing from the closure in const.
///
/// Consider using [`konst::array::map`](crate::array::map)
/// if the closure does not contain any early returns.
///
/// # Note
///
/// This macro requires the input and output elements to not need dropping.
/// In exchange, there can be `return`s inside the closure passed to this macro,
/// which will return from the function that this macro is invoked inside of.
///
#[doc = crate::docs::closure_arg_annotated_params_limitations_docs!("")]
///
/// # Example
///
/// ```rust
/// use konst::{array, result};
///
/// use std::num::ParseIntError;
///
/// const OK: [u8; 4] = result::unwrap!(parse_u8s(["3", "5", "8", "13"]));
/// assert_eq!(OK, [3, 5, 8, 13]);
///
/// const ERR: ParseIntError = result::unwrap_err!(parse_u8s(["3", "AAA", "5"]));
///
/// const fn parse_u8s<const N: usize>(strs: [&str; N]) -> Result<[u8; N], ParseIntError> {
///     Ok(array::map_nd!(strs, |s| konst::try_!(u8::from_str_radix(s, 10))))
/// }
/// ```
///
#[doc(inline)]
#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use crate::__array_map_by_val_nd as map_nd;

/// Const equivalent of [`array::from_fn`](core::array::from_fn),
/// which allows `return`ing from the closure in const.
///
/// Consider using [`konst::array::from_fn`](crate::array::from_fn)
/// if the closure does not contain any early returns.
///
/// # Note
///
/// This macro requires the output elements to not need dropping.
/// In exchange, there can be `return`s inside the closure passed to this macro,
/// which will return from the function that this macro is invoked inside of.
///
#[doc = crate::docs::closure_arg_annotated_params_limitations_docs!("")]
///
/// # Example
///
/// ### No array type annotation
///
/// ```rust
/// use konst::{array, string};
///
/// const NON3: Option<[&str; 4]> = split_comma("3, 5, 8");
/// assert_eq!(NON3, None);
///
/// const SOM: [&str; 4] = split_comma("3, 5, 8, 13").unwrap();
/// assert_eq!(SOM, ["3", "5", "8", "13"]);
///
/// // drops the excess comma-separated items
/// const SOM5: [&str; 4] = split_comma("3, 5, 8, 13, 21").unwrap();
/// assert_eq!(SOM5, ["3", "5", "8", "13"]);
///
/// const fn split_comma<const N: usize>(string: &str) -> Option<[&str; N]> {
///     let mut iter = string::split(string, ",");
///
///     Some(array::from_fn_nd!(|_| konst::try_opt!(iter.next()).trim_ascii()))
/// }
/// ```
///
/// ### Array type annotation
///
/// ```rust
/// use konst::{array, string};
///
/// // Annotating the array type
/// let arr = array::from_fn_nd!([u8; 4] => |x| x.pow(2) as _);
/// assert_eq!(arr, [0, 1, 4, 9])
/// ```
///
#[doc(inline)]
#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use crate::__array_from_fn_nd as from_fn_nd;
