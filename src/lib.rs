//! For comparing strings and slices at compile-time.

#![no_std]

#[macro_use]
mod declare_cmp_fn_macros;

mod cmp_macros;

#[doc(hidden)]
pub mod __for_cmp_impls;

#[cfg(feature = "str_cmp")]
__declare_string_cmp_fns! {
    import_path = "const_cmp",
    equality_fn = str_eq,
    ordering_fn = str_cmp,
    ordering_fn_inner = str_cmp_inner,
}

/// Functions for comparing slices
#[cfg(feature = "slice_cmp")]
pub mod slice;

#[doc(hidden)]
pub mod __ {
    pub use core::cmp::Ordering;
    pub use core::matches;

    pub use crate::__for_cmp_impls::U8Ordering;
}
