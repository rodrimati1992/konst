#[doc(hidden)]
#[macro_export]
macro_rules! __cmp_assert_inner {
    ($left:expr, $right:expr, $is_equal:ident, $operator:literal, $($($fmt:tt)+)?) => {
        match (&$left, &$right) {
            (left, right) => {
                if let $is_equal = $crate::cmp::coerce_to_cmp!($left).const_eq(right) {
                    $crate::__::concat_panic!{
                        display: concat!(
                            "\nassertion failed: LEFT ",
                            $operator,
                            " RIGHT\n left: `",
                        ),
                        left,
                        "`\nright: `",
                        right,
                        "`\n",
                        $( ": ", $($fmt)+)?
                    }
                }
            }
        }
    }
}

macro_rules! cmp_assertc_docs {
    () => {
        concat!(
            "[**examples below**](#examples)",
            "\n\n",
            "This macro is only evaluated at compile-time if used in a context that requires it ",
            "(eg: in the expression assigned to a `const _: () = `)",
            "\n\n",
            "# Formatting ",
            "\n\n",
            "This uses the same syntax for formatting arguments as ",
            "[`const_panic::concat_panic`](macro@const_panic::concat_panic).",
            "\n\n",
            "By default, this only supports primitive types as arguments, ",
            "to format arrays or custom types you must enable ",
            r#"`const_panic`'s `"non_basic"` feature."#,
            "\n\n",
            "To pass user-defined types, ",
            "they must implement both of these traits as described in their docs:\n",
            "- [`konst::cmp::ConstCmp`](crate::cmp::ConstCmp)\n",
            "- [`const_panic::fmt::PanicFmt`]\n",
            "\n\n",
        )
    };
}

/// For asserting that two values are equal.
///
#[doc = cmp_assertc_docs!()]
///
/// # Examples
///
/// ### Zipping slices
///
/// This example requires the `"iter"` feature
///
#[cfg_attr(feature = "iter", doc = "```rust")]
#[cfg_attr(not(feature = "iter"), doc = "```ignore")]
/// use konst::{iter, slice};
///
/// const A: &[u8] = &[3, 5, 8, 13];
/// const B: &[u8] = &[0, 1, 2, 3];
///
/// const C: &[(u8, u8)] = &{
///     konst::assertc_eq!(A.len(), B.len());
///
///     iter::collect_const!((u8, u8) =>
///         slice::iter_copied(A),
///             zip(slice::iter_copied(B)),
///     )
/// };
///
/// # fn main () {
/// assert_eq!(C, [(3, 0), (5, 1), (8, 2), (13, 3)]);
/// # }
/// ```
///
/// If either slice was a different length, this would be the compile-time error:
///
/// ```text
/// error[E0080]: evaluation panicked:
///               assertion failed: LEFT == RIGHT
///                left: `3`
///               right: `4`
///   --> konst/src/macros/assert_cmp_macros.rs:84:5
///    |
/// 10 |     konst::assertc_eq!(A.len(), B.len());
///    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `C` failed here
///    |
///    = note: this error originates in the macro `$crate::__::concat_panic` which comes from the expansion of the macro `konst::assertc_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
///
/// ```
///
/// ### User-defined type
///
/// This example demonstrates formatting of user-defined types.
///
/// The `const_panic::PanicFmt` derive that this example uses
/// requres enabling `const_panic`'s `"derive"` feature.
///
#[cfg_attr(feature = "const_panic_derive", doc = "```rust")]
#[cfg_attr(not(feature = "const_panic_derive"), doc = "```ignore")]
/// use konst::assertc_eq;
/// use konst::const_panic::PanicFmt;
///
/// const _: () = assert_same_layout(layout_for!(u32), layout_for!(i32));
///
/// #[track_caller]
/// const fn assert_same_layout(left: Layout, right: Layout) {
///     assertc_eq!{left, right, "layout mismatch"}
/// }
///
/// #[derive(PanicFmt)]
/// struct Layout {
///     type_name: &'static str,
///     size: usize,
///     alignment: usize,
/// }
///
/// konst::cmp::impl_cmp!{
///     impl Layout;
///     
///     pub const fn const_eq(&self, other: &Self) -> bool {
///         konst::cmp::const_eq!(self.size, other.size) &&
///         konst::cmp::const_eq!(self.alignment, other.alignment)
///     }
/// }
///
/// impl Layout {
///     pub const fn new<T>(type_name: &'static str) -> Self {
///         Self {
///             type_name,
///             size: std::mem::size_of::<T>(),
///             alignment: std::mem::align_of::<T>(),
///         }
///     }
/// }
///
/// macro_rules! layout_for {
///     ($ty:ty) => {
///         Layout::new::<$ty>(stringify!($ty))
///     }
/// } use layout_for;
///
/// # fn main(){}
/// ```
///
/// If the types were changed, the example would fail compilation with this error:
/// ```text
/// error[E0080]: evaluation panicked:
///               assertion failed: LEFT == RIGHT
///                left: `Layout { type_name: "u32", size: 4, alignment: 4 }`
///               right: `Layout { type_name: "[u8; 4]", size: 4, alignment: 1 }`
///               : layout mismatch
///  --> konst/src/macros/assert_cmp_macros.rs:124:15
///   |
/// 7 | const _: () = assert_same_layout(layout_for!(u32), layout_for!([u8; 4]));
///   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `_` failed here
/// ```
///
#[macro_export]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
macro_rules! assertc_eq {
    ($left:expr, $right:expr $(, $($fmt:tt)* )? ) => (
        $crate::__cmp_assert_inner!{$left, $right, false, "==", $($($fmt)*)?}
    );
}

/// For asserting that two values are unequal.
///
#[doc = cmp_assertc_docs!()]
///
/// # Examples
///
/// ### Unique strings
///
/// ```rust
/// # fn main() {
/// assert_eq!(NAMES, ["bob", "matt", "rob"]);
///
/// const NAMES: &[&str] = assert_unique(&["bob", "matt", "rob"]);
/// # }
///
/// #[track_caller]
/// const fn assert_unique<'a, 'b>(names: &'a [&'b str]) -> &'a [&'b str] {
///     konst::for_range!{x in 0..names.len() =>
///         konst::for_range!{y in 0..names.len() =>
///             if x == y { continue }
///             konst::assertc_ne!{
///                 names[x],
///                 names[y],
///                 "equal names at index `", x, "` and `", y, "`"
///             }
///         }
///     }
///     names
/// }
///
/// ```
///
/// If the argument had repeated strings, this would be the error:
///
/// ```text
/// error[E0080]: evaluation panicked:
///               assertion failed: LEFT != RIGHT
///                left: `"rob"`
///               right: `"rob"`
///               : equal names at index `1` and `2`
///  --> konst/src/macros/assert_cmp_macros.rs:201:24
///   |
/// 7 | const NAMES: &[&str] = assert_unique(&["bob", "rob", "rob"]);
///   |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ evaluation of `main::NAMES` failed here
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
#[macro_export]
macro_rules! assertc_ne {
    ($left:expr, $right:expr $(, $($fmt:tt)* )? ) => (
        $crate::__cmp_assert_inner!{$left, $right, true, "!=", $($($fmt)*)?}
    );
}
