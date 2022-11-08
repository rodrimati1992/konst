//! Comparisong-related items.

mod cmp_wrapper;

mod const_cmp;

pub use self::{
    cmp_wrapper::CmpWrapper,
    const_cmp::{ConstCmp, ConstCmpUnref, IsAConstCmp},
};

#[doc(no_inline)]
pub use crate::polymorphism::kinds::{IsNotStdKind, IsRefKind, IsStdKind};
