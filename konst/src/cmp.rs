//! Comparisong-related items.
//!
//! The main item here is the [`ConstCmp`] trait.

mod cmp_wrapper;

mod const_cmp;

pub(crate) mod const_eq_macros;
pub(crate) mod const_ord_macros;
mod impl_cmp_macro;

pub use self::{
    cmp_wrapper::CmpWrapper,
    const_cmp::{ConstCmp, ConstCmpUnref, IsAConstCmp},
};

#[doc(inline)]
pub use self::{
    const_eq_macros::{const_eq, const_eq_for},
    const_ord_macros::{const_cmp, const_cmp_for, try_equal},
    impl_cmp_macro::impl_cmp,
};

#[doc(no_inline)]
pub use crate::polymorphism::kinds::{IsNotStdKind, IsRefKind, IsStdKind};

#[doc(no_inline)]
pub use crate::{coerce_to_cmp, max, max_by, max_by_key, min, min_by, min_by_key};
