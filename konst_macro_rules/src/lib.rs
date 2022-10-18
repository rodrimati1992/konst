//! Implementation detail of the `konst` crate.
#![no_std]
#![cfg_attr(feature = "nightly_mut_refs", feature(const_mut_refs))]

#[macro_use]
mod array_macros;

#[macro_use]
mod internal_macros;

pub mod into_iter;
#[doc(hidden)]
pub mod iter;

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
pub mod collect_const;

pub use konst_kernel::__unsafe_utils;

#[doc(hidden)]
pub mod __ {
    pub use core::{assert, panic};

    pub use crate::{
        array_macros::{assert_array, uninit_array, AssumInitCopyArray},
        collect_const::{CollectorCmd, ComputedLength},
    };

    pub use crate::into_iter::{EmptyIter, IntoIterWrapper, IsIntoIterKind};

    pub use core::{
        cmp::Ordering::{self, Equal, Greater, Less},
        compile_error, concat,
        marker::PhantomData,
        matches,
        mem::{transmute, ManuallyDrop, MaybeUninit},
        ops::Range,
        option::Option::{self, None, Some},
        primitive::{bool, str, u8, usize},
        result::Result::{self, Err, Ok},
        stringify,
    };
}
