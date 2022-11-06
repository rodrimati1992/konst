#![allow(non_camel_case_types)]

use core::fmt::{self, Display};

#[inline]
pub const fn try_into_array_func<T, const N: usize>(
    slice: &[T],
) -> Result<&[T; N], TryIntoArrayError> {
    if slice.len() == N {
        let ptr = slice.as_ptr() as *const [T; N];
        unsafe { Ok(crate::utils::Dereference { ptr }.reff) }
    } else {
        Err(TryIntoArrayError {
            slice_len: slice.len(),
            array_len: N,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "mut_refs")]
#[inline]
pub const fn try_into_array_mut_func<T, const N: usize>(
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

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TryIntoArrayError {
    slice_len: usize,
    array_len: usize,
}

impl TryIntoArrayError {
    /// For panicking with an error message.
    pub const fn panic(&self) -> ! {
        use crate::utils::PanikVal;

        crate::utils::basic_panic(&[
            PanikVal::Str("could not convert slice of length `"),
            PanikVal::Usize(self.slice_len),
            PanikVal::Str("` to array of length`"),
            PanikVal::Usize(self.array_len),
            PanikVal::Str("`"),
        ])
    }
}

impl Display for TryIntoArrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Could not cast slice to array reference")
    }
}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! slice_concat {
    ($elem_ty:ty, $slice:expr $(,)*) => {{
        const __ARGS_81608BFNA5: &[&[$elem_ty]] = $slice;
        {
            const LEN: $crate::__::usize = $crate::slice::concat_sum_lengths(__ARGS_81608BFNA5);

            const CONC: [$elem_ty; LEN] = $crate::slice::concat_slices(__ARGS_81608BFNA5);

            CONC
        }
    }};
}

pub const fn concat_sum_lengths<T>(slice: &[&[T]]) -> usize {
    let mut sum = 0usize;
    crate::for_range! {i in 0..slice.len() =>
        sum += slice[i].len();
    }
    sum
}

pub const fn concat_slices<T, const N: usize>(slices: &[&[T]]) -> [T; N]
where
    T: Copy,
{
    if let Ok(x) = try_into_array_func::<T, N>(&[]) {
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
