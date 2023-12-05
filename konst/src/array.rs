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

/// Const equivalent of
/// [`array::map`](https://doc.rust-lang.org/std/primitive.array.html#method.map).
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

/// Const equivalent of [`array::from_fn`](core::array::from_fn).
///
#[doc = leak_warning!()]
///
/// # Limitations
///
/// When the array type is annotated, the array type must be one of:
/// - Square brackets (e.g: `from_fn!([usize; 10] => |i| i)`)
/// - A parenthesized type (e.g: `from_fn!((foo::ArrayAlias) => |i| i * 2)`)
/// - A single identifier (e.g: `from_fn!(ArrayAlias => |i| func(i))`)
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
