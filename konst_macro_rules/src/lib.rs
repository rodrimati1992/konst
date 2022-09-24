//! Implementation detail of the `konst` crate.
#![no_std]
#![cfg_attr(feature = "nightly_mut_refs", feature(const_mut_refs))]

#[cfg(feature = "rust_1_56")]
#[macro_use]
mod array_macros;

#[doc(hidden)]
pub mod iter;

#[macro_use]
mod option_macros_;

#[macro_use]
mod result_macros_;

#[macro_use]
#[doc(hidden)]
pub mod slice_;

#[macro_use]
#[doc(hidden)]
pub mod string;

#[doc(hidden)]
pub mod utils;

#[doc(hidden)]
pub mod __ {
    pub use core::panic;

    #[cfg(feature = "rust_1_56")]
    pub use crate::array_macros::{assert_array, uninit_array, AssumInitCopyArray};

    pub use core::{
        cmp::Ordering::{self, Equal, Greater, Less},
        compile_error,
        marker::PhantomData,
        matches,
        mem::{transmute, MaybeUninit},
        ops::Range,
        option::Option::{self, None, Some},
        primitive::{str, u8},
        result::Result::{self, Err, Ok},
        str::from_utf8_unchecked,
    };
}
