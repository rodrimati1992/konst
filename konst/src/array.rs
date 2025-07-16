//! Const equivalents of array functions.

#[doc(hidden)]
pub mod __array_macros_2;

mod array_builder;

pub use self::array_builder::ArrayBuilder;

#[cfg(feature = "iter")]
mod array_into_iter;

#[cfg(feature = "iter")]
pub use self::array_into_iter::{IntoIter, IntoIterRev};

macro_rules! drop_warning {
    () => {
        concat!(
            "# Note",
            "\n\n",
            "`return` inside the closure passed to this macro ",
            "attempts to return from the function where this macro is called, ",
            "which drops the array elements, ",
            "and dropping isn't allowed in const as of Rust 1.83.0.",
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
#[doc = drop_warning!()]
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
pub use crate::__array_map_by_val as map;

/// Const equivalent of [`array::from_fn`](core::array::from_fn).
///
#[doc = drop_warning!()]
///
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
pub use crate::__array_from_fn2 as from_fn;
