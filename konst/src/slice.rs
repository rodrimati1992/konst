//! `const fn` equivalents of slice methods.

/// `const fn`s for comparing slices for equality and ordering.
#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

mod slice_const_methods;

pub use slice_const_methods::*;

__declare_slice_cmp_fns! {
    import_path = "konst",

    (
        ///
        ///  # Example
        ///
        ,
        /// ```rust
        /// use konst::slice::eq_bytes;
        ///
        /// const FOO: &[u8] = b"foo";
        /// const BAR: &[u8] = b"fooooo";
        /// const BAZ: &[u8] = b"bar";
        ///
        ///
        /// const FOO_EQ_FOO: bool = eq_bytes(FOO, FOO);
        /// assert!( FOO_EQ_FOO );
        ///
        /// const FOO_EQ_BAR: bool = eq_bytes(FOO, BAR);
        /// assert!( !FOO_EQ_BAR );
        ///
        /// const FOO_EQ_BAZ: bool = eq_bytes(FOO, BAZ);
        /// assert!( !FOO_EQ_BAZ );
        ///
        /// ```
        ///
        ,
        /// ```rust
        /// use konst::slice::cmp_bytes;
        ///
        /// use std::cmp::Ordering;
        ///
        /// const FOO: &[u8] = b"foo";
        /// const BAR: &[u8] = b"fooooo";
        /// const BAZ: &[u8] = b"bar";
        ///
        ///
        /// const FOO_CMP_FOO: Ordering = cmp_bytes(FOO, FOO);
        /// assert_eq!(FOO_CMP_FOO, Ordering::Equal);
        ///
        /// const FOO_CMP_BAR: Ordering = cmp_bytes(FOO, BAR);
        /// assert_eq!(FOO_CMP_BAR, Ordering::Less);
        ///
        /// const FOO_CMP_BAZ: Ordering = cmp_bytes(FOO, BAZ);
        /// assert_eq!(FOO_CMP_BAZ, Ordering::Greater);
        ///
        /// ```
        ///
        ,
        u8,
        eq_bytes,
        cmp_bytes,
    )
}

__declare_fns_with_docs! {
    (Option<&'a [u8]>, (eq_option_bytes, cmp_option_bytes))

    docs(default)

    macro = __impl_option_cmp_fns!(
        for['a,]
        params(l, r)
        eq_comparison = eq_bytes(l, r),
        cmp_comparison = cmp_bytes(l, r),
        parameter_copyability = copy,
    ),
}

/// Fallible conversion from `&[T]` to `&[T; N]`, usable in `const`s, but not in `const fn`s.
///
/// Evaluates to an `Err(TryIntoArrayError{..})` when the slice doesn't match the expected length.
///
/// For an alternative that can be used in `const fn`s, there is the [`try_into_array`] function,
/// but it can only be used with the nightly compiler.
///
/// # Features
///
/// By default you need to pass the length of the returned array.
///
/// To infer the length of the array you need to enable the `"const_generics"` feature,
/// which requires Rust 1.51.0
///
/// # Example
///
/// ### Explicit length
///
/// ```rust
/// use konst::{
///     slice::{TryIntoArrayError, try_into_array},
///     result,
/// };
///
///
/// const ARR_5: Option<&[u64; 5]> = {
///     let slice: &[u64] = &[1, 10, 100, 1000, 10000];
///
///     result::ok!(try_into_array!(slice, 5))
/// };
///
/// assert_eq!(ARR_5, Some(&[1, 10, 100, 1000, 10000]));
///
///
/// const ERR: Result<&[u64; 5], TryIntoArrayError> = {
///     let slice: &[u64] = &[];
///
///     try_into_array!(slice, 5)
/// };
///
/// assert!(ERR.is_err());
///
/// ```
///
/// ### Length inference
///
/// `try_into_array` can infer the length of the array with the
/// `"const_generic"` feature, which requires Rust 1.51.0.
///
#[cfg_attr(feature = "const_generics", doc = "```rust")]
#[cfg_attr(not(feature = "const_generics"), doc = "```ignore")]
/// use konst::{slice::try_into_array, unwrap_ctx};
///
/// const ARR_3: &[u64; 3] = {
///     let slice: &[u64] = &[3, 5, 8];
///
///     // Letting the macro infer the length of the array,
///     let array = unwrap_ctx!(try_into_array!(slice));
///     
///     // You can destructure the array into its elements like this
///     let [a, b, c] = *array;
///     
///     array
/// };
///
/// assert_eq!(ARR_3, &[3, 5, 8]);
///
/// ```
///
/// [`try_into_array`]: ./fn.try_into_array.html
#[doc(inline)]
pub use konst_macro_rules::try_into_array;

/// The error produced by trying to convert from a `&[T]` to a `&[T; N]`
#[doc(inline)]
pub use konst_macro_rules::slice_::TryIntoArrayError;

/// Fallible conversion from `&[T]` to `&[T; N]`, usable in `const fn`s. Requires Rust nightly.
///
/// Returns an `Err(TryIntoArrayError{..})` when the slice doesn't match the expected length.
///
/// For an alternative that work on stable Rust, there is the [`try_into_array`] macro,
/// but it can only be used in `const`s, not in `const fn`s .
///
/// # Features
///
/// This is not enabled by default,
/// you need to enable the `"deref_raw_in_fn"` feature to use it,
/// which requires Rust nightly.
///
/// # Example
///
/// ```rust
/// use konst::{
///     slice::{TryIntoArrayError, try_into_array},
///     result,
///     unwrap_ctx,
/// };
///
///
/// const fn arr_5() -> Option<&'static [u64; 5]> {
///     let slice: &[u64] = &[1, 10, 100, 1000, 10000];
///
///     // Passing the length explicitly to the macro
///     result::ok!(try_into_array::<_, 5>(slice))
/// }
///
/// assert_eq!(arr_5(), Some(&[1, 10, 100, 1000, 10000]));
///
///
/// const fn err() -> Result<&'static [u64; 5], TryIntoArrayError> {
///     let slice: &[u64] = &[];
///
///     // Letting the macro infer the length of the array,
///     try_into_array(slice)
/// }
///
/// assert!(err().is_err());
///
///
/// const fn arr_3() -> &'static [u64; 3] {
///     let slice: &[u64] = &[3, 5, 8];
///
///     let array = unwrap_ctx!(try_into_array(slice));
///     
///     // You can destructure the array into its elements like this
///     let [a, b, c] = *array;
///     
///     array
/// }
///
/// assert_eq!(arr_3(), &[3, 5, 8]);
///
/// ```
///
/// [`try_into_array`]: ./macro.try_into_array.html
#[cfg(feature = "deref_raw_in_fn")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "deref_raw_in_fn")))]
#[doc(inline)]
pub use konst_macro_rules::slice_::try_into_array_func as try_into_array;
