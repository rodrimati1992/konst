pub mod macros;

pub mod slice;
mod utils;

#[doc(hidden)]
pub mod __ {
    pub use core::{
        ops::Range,
        option::Option::{self, None, Some},
        result::Result::{self, Err, Ok},
    };
}
