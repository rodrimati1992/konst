//! this crate has a few items reexported by `konst`, and used by `const_panic`.
//!
//! None of the items in this crate are documented,
//! because they are documented in the `konst` crate.
//!
//!
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]

mod macros;

#[cfg(feature = "rust_1_64")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
pub mod slice;

#[cfg(feature = "rust_1_64")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
pub mod string;

#[cfg(feature = "rust_1_64")]
mod utils;

#[doc(hidden)]
pub mod __ {
    pub use core::{
        ops::Range,
        option::Option::{self, None, Some},
        result::Result::{self, Err, Ok},
    };
}