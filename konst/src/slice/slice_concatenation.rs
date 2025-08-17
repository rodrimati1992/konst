/// Macro equivalent of `<[&[T]]>::concat`, which takes a constant as an argument.
///
/// This acts like a compile-time-evaluated version of this function:
/// ```rust
/// pub const fn slice_concat<T>(slices: &'static [&'static [T]]) -> [T; LEN]
/// where
///     T: Copy
/// # { [] }
/// # const LEN: usize = 0;
/// ```
///
/// Where `LEN` is the summed length of all inner slices.
///
/// # Example
///
/// ```rust
/// use konst::slice::slice_concat;
///
/// const S: &[&[u8]] = &[&[3, 5], &[8, 13, 21, 34]];
/// assert_eq!(slice_concat!(u8, S), [3, 5, 8, 13, 21, 34]);
///
/// assert_eq!(slice_concat!(u8, &[]), []);
///
/// assert_eq!(slice_concat!(u8, &[&[], &[1, 2, 3], &[4, 5]]), [1, 2, 3, 4, 5]);
///
/// ```
#[doc(inline)]
pub use crate::__slice_concat as slice_concat;

#[doc(hidden)]
#[macro_export]
macro_rules! __slice_concat {
    ($elem_ty:ty, $slice:expr $(,)*) => {{
        const __ARGS_81608BFNA5: &[&[$elem_ty]] = $slice;
        {
            const LEN: $crate::__::usize = $crate::slice::__concat_sum_lengths(__ARGS_81608BFNA5);

            const CONC: [$elem_ty; LEN] = $crate::slice::__concat_slices(__ARGS_81608BFNA5);

            CONC
        }
    }};
}

#[doc(hidden)]
pub const fn __concat_sum_lengths<T>(slice: &[&[T]]) -> usize {
    let mut sum = 0usize;
    crate::for_range! {i in 0..slice.len() =>
        sum += slice[i].len();
    }
    sum
}

#[doc(hidden)]
pub const fn __concat_slices<T, const N: usize>(slices: &[&[T]]) -> [T; N]
where
    T: Copy,
{
    if let Ok(x) = crate::slice::try_into_array::<T, N>(&[]) {
        return *x;
    }

    let mut out = [*first_elem(slices); N];
    let mut out_i = 0usize;

    crate::for_range! {si in 0..slices.len() =>
        let slice = slices[si];
        crate::for_range! {i in 0..slice.len() =>
            out[out_i] = slice[i];
            out_i += 1;
        }
    }

    out
}

// returns the first T in a `&[&[T]]`
const fn first_elem<'a, T>(slices: &[&'a [T]]) -> &'a T {
    crate::for_range! {si in 0..slices.len() =>
        if let [first, ..] = slices[si] {
            return first;
        }
    }

    panic!("there was no element in any slice");
}
