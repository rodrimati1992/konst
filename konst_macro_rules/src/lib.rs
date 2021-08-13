//! Implementation detail of the `konst` crate.
#![no_std]
#![cfg_attr(feature = "deref_raw_in_fn", feature(const_fn_union))]
#![cfg_attr(feature = "nightly_mut_refs", feature(const_mut_refs))]

#[macro_use]
mod option_macros_;

#[macro_use]
mod result_macros_;

#[macro_use]
#[doc(hidden)]
pub mod slice_;

#[doc(hidden)]
pub mod utils;

#[doc(hidden)]
pub mod __ {
    pub use core::{
        cmp::Ordering::{self, Equal, Greater, Less},
        compile_error,
        marker::PhantomData,
        matches,
        mem::transmute,
        ops::Range,
        option::Option::{self, None, Some},
        primitive::u8,
        result::Result::{self, Err, Ok},
    };
}
