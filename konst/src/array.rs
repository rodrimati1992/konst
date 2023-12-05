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
/// # Example
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
