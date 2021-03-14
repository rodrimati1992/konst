//! For comparing strings and slices at compile-time.

#![no_std]

#[macro_use]
mod macros;

#[doc(hidden)]
pub mod __for_cmp_impls;

// pub mod other;

pub mod polymorphism;

pub mod primitive;

#[cfg(feature = "str")]
__declare_string_cmp_fns! {
    import_path = "const_cmp",
    equality_fn = eq_str,
    ordering_fn = cmp_str,
    ordering_fn_inner = cmp_str_inner,
}

#[cfg(all(feature = "str", feature = "option"))]
__declare_fns_with_docs! {
    (Option<&'a str>, (eq_option_str, cmp_option_str))

    docs(default)

    macro = __impl_option_cmp_fns!(
        for['a,]

        params(l, r)
        eq_comparison = crate::polymorphism::CmpWrapper(l).const_eq(r),
        cmp_comparison = crate::polymorphism::CmpWrapper(l).const_cmp(r),
    ),
}

/// Functions for comparing slices
#[cfg(feature = "slice")]
pub mod slice;

#[doc(hidden)]
pub mod __ {
    pub use core::cmp::Ordering;
    pub use core::matches;

    pub use crate::__for_cmp_impls::U8Ordering;

    pub use crate::polymorphism::{
        CmpWrapper, ConstCmpMarker, IsAConstCmpMarker, IsNotStdKind, IsStdKind,
    };
}
