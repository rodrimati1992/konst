//! Comparisong-related items.

mod cmp_wrapper;

mod const_cmp;

pub use self::{
    cmp_wrapper::CmpWrapper,
    const_cmp::{ConstCmp, ConstCmpUnref, IsAConstCmp},
};

#[doc(no_inline)]
pub use crate::polymorphism::kinds::{IsNotStdKind, IsRefKind, IsStdKind};

#[doc(no_inline)]
pub use crate::{
    coerce_to_cmp, const_cmp, const_cmp_for, const_eq, const_eq_for, impl_cmp, max, max_by,
    max_by_key, min, min_by, min_by_key,
};
