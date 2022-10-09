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
        Err(TryIntoArrayError { _priv: () })
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
        Err(TryIntoArrayError { _priv: () })
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct TryIntoArrayError {
    _priv: (),
}

impl TryIntoArrayError {
    /// For erroring with an error message.
    pub const fn panic(&self) -> ! {
        panic!("Could not cast &[T] to &[T; N]")
    }
}

impl Display for TryIntoArrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Could not cast slice to array reference")
    }
}
