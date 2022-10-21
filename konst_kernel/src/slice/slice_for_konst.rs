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
    /// For erroring with an error message.
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
