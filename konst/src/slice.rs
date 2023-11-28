//! `const fn` equivalents of slice methods.
//!
//! # Removed in 0.3.0
//!
//! These functions were removed in 0.3.0 because there is an equivalent
//! const fn in the standard library:
//!
//! - `first`
//! - `last`
//! - `split_first`
//! - `split_last`
//!
//!
//!

/// `const fn`s for comparing slices for equality and ordering.
#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

mod bytes_pattern;
mod slice_as_chunks;
mod slice_concatenation;
mod slice_const_methods;

#[cfg(feature = "iter")]
mod slice_iter_methods;

pub use bytes_pattern::BytesPattern;

pub(crate) use bytes_pattern::PatternNorm;

pub use self::slice_as_chunks::*;
pub use self::slice_concatenation::*;
pub use self::slice_const_methods::*;

#[cfg(feature = "iter")]
pub use slice_iter_methods::*;

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

/// The error produced by trying to convert from
/// `&[T]` to `&[T; N]`, or from `&mut [T]` to `&mut [T; N]`.
#[doc(inline)]
pub use konst_kernel::slice::slice_for_konst::TryIntoArrayError;

/// Tries to convert from `&[T]` to `&[T; N]`.
///
/// Returns an `Err(TryIntoArrayError{..})` when the slice doesn't match the expected length.
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
///     // Passing the length explicitly to the function
///     result::ok!(try_into_array::<_, 5>(slice))
/// }
///
/// assert_eq!(arr_5(), Some(&[1, 10, 100, 1000, 10000]));
///
///
/// const fn err() -> Result<&'static [u64; 5], TryIntoArrayError> {
///     let slice: &[u64] = &[];
///
///     // Letting the function infer the length of the array,
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
#[doc(inline)]
pub use konst_kernel::slice::slice_for_konst::try_into_array_func as try_into_array;

/// Tries to convert from `&mut [T]` to `&mut [T; N]`.
///
/// Returns an `Err(TryIntoArrayError{..})` when the slice doesn't match the expected length.
///
/// # Example
///
/// ```rust
/// # #![feature(const_mut_refs)]
/// use konst::{slice, unwrap_ctx};
///
/// const fn mut_array_from<const LEN: usize>(slice: &mut [u8], from: usize) -> &mut [u8; LEN] {
///     let sliced = slice::slice_range_mut(slice, from, from + LEN);
///     unwrap_ctx!(slice::try_into_array_mut(sliced))
/// }
///
/// # fn main() {
///
/// let slice = &mut [3, 5, 8, 13, 21, 34, 55, 89, 144, 233];
///
/// let foo: &mut [u8; 2] = mut_array_from(slice, 0);
/// assert_eq!(foo, &mut [3, 5]);
///
/// let bar: &mut [u8; 3] = mut_array_from(slice, 2);
/// assert_eq!(bar, &mut [8, 13, 21]);
///
/// let baz: &mut [u8; 4] = mut_array_from(slice, 4);
/// assert_eq!(baz, &mut [21, 34, 55, 89]);
///
/// # }
/// ```
///
#[cfg(feature = "mut_refs")]
#[cfg_attr(
    feature = "docsrs",
    doc(cfg(any(feature = "mut_refs", feature = "nightly_mut_refs")))
)]
#[doc(inline)]
pub use konst_kernel::slice::slice_for_konst::try_into_array_mut_func as try_into_array_mut;
