#![no_std]

#[macro_use]
mod option_macros_;

#[macro_use]
mod result_macros_;

#[doc(hidden)]
pub mod __ {
    pub use core::{
        cmp::Ordering::{self, Equal, Greater, Less},
        compile_error, matches,
        ops::Range,
        option::Option::{self, None, Some},
        result::Result::{self, Err, Ok},
    };
}
