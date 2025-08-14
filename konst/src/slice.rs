//! `const fn` equivalents of slice methods.
//!
//! # Removed in 0.4.0
//!
//! These functions were removed in 0.4.0 because there is an equivalent
//! const fn in the standard library:
//!
//! - `as_chunks`: [slice::as_chunks]
//! - `as_rchunks`: [slice::as_rchunks]
//! - `as_chunks_mut`: [slice::as_chunks_mut]
//! - `as_rchunks_mut`: [slice::as_rchunks_mut]
//! - `bytes_trim_end`: [slice::trim_ascii_end]
//! - `bytes_trim_start`: [slice::trim_ascii_start]
//! - `bytes_trim`: [slice::trim_ascii]
//! - `first_mut`: [slice::first_mut]
//! - `last_mut`: [slice::last_mut]
//! - `split_first_mut`: [slice::split_first_mut]
//! - `split_last_mut`: [slice::split_last_mut]
//!
//! The `array_chunks*` functions were removed in 0.4.0 because the
//! unstable equivalent function was removed.
//! You can replace instances of `konst::slice::array_chunks(slice)`
//! with `konst::slice::iter(slice.as_chunks().0)`.
//!
//!
//!

use core::fmt::{self, Display};

/// `const fn`s for comparing slices for equality and ordering.
#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

mod bytes_pattern;
mod slice_concatenation;
mod slice_const_methods;
mod slice_filler;

#[cfg(feature = "iter")]
mod slice_iter_methods;

pub use bytes_pattern::BytesPattern;

pub(crate) use bytes_pattern::PatternNorm;

pub use self::slice_concatenation::*;
pub use self::slice_const_methods::*;
pub use self::slice_filler::*;

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

////////////////////////////////////////////////////////////////////////////////

/// The error produced by trying to convert from
/// `&[T]` to `&[T; N]`, or from `&mut [T]` to `&mut [T; N]`.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TryIntoArrayError {
    slice_len: usize,
    array_len: usize,
}

impl Display for TryIntoArrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "could not convert slice of length `{}` to array of length `{}`",
            self.slice_len, self.array_len,
        )
    }
}

const _: () = {
    use const_panic::{
        PanicFmt, PanicVal, flatten_panicvals,
        fmt::{self as cfmt, ComputePvCount, FmtArg, FmtKind},
    };

    impl PanicFmt for TryIntoArrayError {
        type This = Self;
        type Kind = const_panic::IsCustomType;

        const PV_COUNT: usize = ComputePvCount {
            field_amount: 2,
            summed_pv_count: <usize>::PV_COUNT * 2,
            delimiter: cfmt::TypeDelim::Braced,
        }
        .call();
    }

    impl TryIntoArrayError {
        /// Formats a TryIntoArrayError
        pub const fn to_panicvals(
            &self,
            fmtarg: FmtArg,
        ) -> [PanicVal<'static>; TryIntoArrayError::PV_COUNT] {
            match fmtarg.fmt_kind {
                FmtKind::Debug => {
                    flatten_panicvals! {fmtarg;
                        "TryIntoArrayError",
                        open: cfmt::OpenBrace,
                            "slice_len: ", usize => self.slice_len, cfmt::COMMA_SEP,
                            "array_len: ", usize => self.array_len, cfmt::COMMA_TERM,
                        close: cfmt::CloseBrace,
                    }
                }
                _ => const_panic::utils::flatten_panicvals(&[&[
                    PanicVal::write_str("could not convert slice of length `"),
                    PanicVal::from_usize(self.slice_len, FmtArg::DEBUG),
                    PanicVal::write_str("` to array of length `"),
                    PanicVal::from_usize(self.array_len, FmtArg::DEBUG),
                    PanicVal::write_str("`"),
                ]]),
            }
        }
    }
};

////////////////////////////////////////////////////////////////////////////////

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
///     let array = result::unwrap!(try_into_array(slice));
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
#[inline]
pub const fn try_into_array<T, const N: usize>(slice: &[T]) -> Result<&[T; N], TryIntoArrayError> {
    if slice.len() == N {
        let ptr = slice.as_ptr() as *const [T; N];
        unsafe { Ok(&*ptr) }
    } else {
        Err(TryIntoArrayError {
            slice_len: slice.len(),
            array_len: N,
        })
    }
}

/// Tries to convert from `&mut [T]` to `&mut [T; N]`.
///
/// Returns an `Err(TryIntoArrayError{..})` when the slice doesn't match the expected length.
///
/// # Example
///
/// ```rust
/// use konst::{slice, result};
///
/// const fn mut_array_from<const LEN: usize>(slice: &mut [u8], from: usize) -> &mut [u8; LEN] {
///     let sliced = slice::slice_range_mut(slice, from, from + LEN);
///     result::unwrap!(slice::try_into_array_mut(sliced))
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
#[inline]
pub const fn try_into_array_mut<T, const N: usize>(
    slice: &mut [T],
) -> Result<&mut [T; N], TryIntoArrayError> {
    if slice.len() == N {
        unsafe { Ok(&mut *(slice as *mut [T] as *mut [T; N])) }
    } else {
        Err(TryIntoArrayError {
            slice_len: slice.len(),
            array_len: N,
        })
    }
}

#[doc(hidden)]
pub struct __AssertSlice<'a, T> {
    pub x: &'a [T],
}

#[doc(hidden)]
pub struct __AssertSliceMut<'a, T> {
    pub x: &'a mut [T],
}
