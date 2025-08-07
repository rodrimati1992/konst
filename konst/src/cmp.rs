//! Comparisong-related items.
//!
//! The main item here is the [`ConstCmp`] trait.

mod cmp_wrapper;

mod const_cmp;

mod coerce_to_cmp_macros;
pub(crate) mod const_eq_macros;
pub(crate) mod const_ltgt_macros;
pub(crate) mod const_ord_macros;
mod impl_cmp_macro;
mod minmax_macros;

pub use self::{
    cmp_wrapper::CmpWrapper,
    const_cmp::{__AssertConstCmp, ConstCmp, ConstCmpUnref, IsAConstCmp},
};

#[doc(inline)]
pub use self::{
    coerce_to_cmp_macros::coerce_to_cmp,
    const_eq_macros::{const_eq, const_eq_for, const_ne, const_ne_for},
    const_ltgt_macros::{
        const_ge, const_ge_for, const_gt, const_gt_for, const_le, const_le_for, const_lt,
        const_lt_for,
    },
    const_ord_macros::{const_cmp, const_cmp_for, try_equal},
    impl_cmp_macro::impl_cmp,
    minmax_macros::{max, max_by, max_by_key, min, min_by, min_by_key},
};

#[doc(no_inline)]
pub use crate::polymorphism::kinds::{IsNotStdKind, IsRefKind, IsStdKind};
