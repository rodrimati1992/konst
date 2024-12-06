//! Const equivalents of array functions.

macro_rules! leak_warning {
    () => {
        concat!(
            "# Warning\n",
            "\n",
            "This macro leaks the initialized part of the array\n",
            "if the closure passed to this macro panics or returns early.\n",
            "\n",
            "note: this warning is not relevant if the elements don't need dropping",
            "(e.g: by implementing `Copy`).\n"
        )
    };
}
use leak_warning;

/// Superceeded by [`map_`],
/// const version of [`array::map`](https://doc.rust-lang.org/std/primitive.array.html#method.map).
///
#[doc = leak_warning!()]
///
/// # Limitations
///
/// This macro supports mapping from non-Copy arrays if any of these
/// conditions are met about the parameter of the passed-in closure:
/// 1. it's a pattern that only copies Copy fields of each array element
/// 2. it's a `ref` pattern
///
/// [examples of both of the above conditions below](#map-noncopy-example)
///
/// # Example
///
/// ### Basic
///
/// ```rust
/// use konst::array;
///
/// const TRIMMED: [&str; 3] = array::map!(["  foo", "bar  ", "  baz  "], konst::string::trim);
/// assert_eq!(TRIMMED, ["foo", "bar", "baz"]);
///
/// const LENGTHS: [usize; 3] = array::map!(["foo", "hello", "bar baz"], |s| s.len());
/// assert_eq!(LENGTHS, [3, 5, 7]);
///
/// const SQUARED: [u32; 6] = array::map!([1, 2, 3, 4, 5, 6], |x: u32| x.pow(2));
/// assert_eq!(SQUARED, [1, 4, 9, 16, 25, 36]);
///
/// {
///     let input = [3, 5, 8];
///     let output = array::map!(input, |x| -> u64 { x + 2 });
///     assert_eq!(output, [5, 7, 10]);
/// }
///
/// ```
///
/// <span id="map-noncopy-example"> </span>
/// ### Map from non-Copy array
///
/// Demonstrates both ways to map from a non-Copy array.
///
/// ```rust
/// use konst::array;
///
/// struct NonCopy(u32, u32);
///
/// const PRIME_SUMS: [u32; 3] = {
///     let input = [NonCopy(2, 3), NonCopy(5, 7), NonCopy(11, 13)];
///     
///     // demonstrates the first way to map from non-Copy elements
///     array::map!(input, |NonCopy(l, r)| l + r)
/// };
/// assert_eq!(PRIME_SUMS, [5, 12, 24]);
///
/// const FIBB_SUMS: [u32; 3] = {
///     let input = [NonCopy(2, 3), NonCopy(5, 8), NonCopy(13, 21)];
///     
///     // demonstrates the second way to map from non-Copy elements
///     array::map!(input, |ref nc| nc.0 + nc.1)
/// };
/// assert_eq!(FIBB_SUMS, [5, 13, 34]);
///
/// ```
///
pub use konst_kernel::array_map as map;

#[cfg(feature = "rust_1_83")]
#[doc(hidden)]
pub mod __array_macros_2;

#[cfg(feature = "rust_1_83")]
mod array_builder;

#[cfg(feature = "rust_1_83")]
mod array_consumer;

#[cfg(feature = "rust_1_83")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_83")))]
pub use self::{
    array_builder::ArrayBuilder,
    array_consumer::ArrayConsumer,
};




#[cfg(feature = "rust_1_83")]
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

#[cfg(feature = "rust_1_83")]
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
///     konst::array::map_!(pairs, |pair: (T, U)| {
///         // need to use `destructure` to destructure types that may contain Drop fields
///         konst::destructure!{(a, b) = pair}
///         (b, a)
///     })
/// }
/// ```
///
#[doc(inline)]
#[cfg(feature = "rust_1_83")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_83")))]
pub use crate::__array_map_by_val as map_;

/// Superceeded by [`from_fn_`], const version of
/// [`array::from_fn`](core::array::from_fn).
///
#[doc = leak_warning!()]
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
pub use konst_kernel::array_from_fn as from_fn;

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
///     const POWERS: [u64; 5] = array::from_fn_!(|i| 2u64.pow(i as u32));
///
///     assert_eq!(POWERS, [1, 2, 4, 8, 16]);
/// }
///
/// // Annotating the array type
/// assert_eq!(
///     array::from_fn_!([&str; 6] => |i| konst::string::str_up_to("hello", i)),
///     ["", "h", "he", "hel", "hell", "hello"],
/// );
/// ```
///
#[doc(inline)]
#[cfg(feature = "rust_1_83")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_83")))]
pub use crate::__array_from_fn2 as from_fn_;
