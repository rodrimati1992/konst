//! This crate has a few items reexported by `konst`, and used by `const_panic`.
//!
//! None of the items in this crate are documented,
//! because they are documented in the 0.3 `konst` crate.
//!
//!
#![no_std]
#![cfg_attr(feature = "nightly_mut_refs", feature(const_mut_refs))]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]

#[macro_use]
mod macros;

#[doc(hidden)]
pub mod collect_const;

#[doc(hidden)]
#[cfg(feature = "__for_konst")]
pub mod into_iter;

#[doc(hidden)]
#[cfg(feature = "__for_konst")]
pub mod iter;

pub mod type_eq;

#[cfg(feature = "rust_1_64")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
pub mod slice;

#[cfg(feature = "rust_1_64")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
pub mod string;

#[doc(hidden)]
pub mod __unsafe_utils;

#[cfg(feature = "rust_1_64")]
#[doc(hidden)]
pub mod utils;

#[doc(hidden)]
pub mod __ {
    #[cfg(feature = "__for_konst")]
    pub use crate::{
        collect_const::{CollectorCmd, ComputedLength},
        into_iter::{EmptyIter, IntoIterWrapper, IsIntoIterKind},
        macros::array_macros::{assert_array, uninit_array, AssumInitCopyArray},
    };

    pub use core::{
        assert,
        mem::{ManuallyDrop, MaybeUninit},
        ops::Range,
        option::Option::{self, None, Some},
        panic,
        primitive::{bool, usize},
        result::Result::{self, Err, Ok},
    };
}
