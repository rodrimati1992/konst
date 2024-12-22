use core::{
    mem::ManuallyDrop,
    ops::{Range, RangeFrom, RangeInclusive},
};

use crate::iter::{ConstIntoIter, IntoIterWrapper, IsIteratorKind, IsStdKind};
use crate::iter::step::{self, decrement, increment, Step, __StepRet};

macro_rules! impl_std_kinds {
    ($($ty:ident => $iter:ident,)*) => (
        $(
            impl<T: Step> ConstIntoIter for $ty<T> {
                type Kind = IsStdKind;
                type IntoIter = $iter<T>;
                type Item = T;
            }
            impl<T: Step> ConstIntoIter for &$ty<T> {
                type Kind = IsStdKind;
                type IntoIter = $iter<T>;
                type Item = T;
            }
        )*
    )
}
impl_std_kinds! {
    Range => RangeIter,
    RangeInclusive => RangeInclusiveIter,
    RangeFrom => RangeFromIter,
}

/// Const-iterator for [`Range`](core::ops::Range)
///
/// This is constructed like this:
/// ```rust
/// # let _ =
/// konst::iter::into_iter!(0..10)
/// # ;
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct RangeIter<T> {
    start: T,
    end: T,
}
impl<T: Step> ConstIntoIter for RangeIter<T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = T;
}

/// Reversed const-iterator for [`Range`](core::ops::Range)
///
///
/// This is constructed like this:
/// ```rust
/// # let _ =
/// konst::iter::into_iter!(0..10).rev()
/// # ;
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct RangeIterRev<T> {
    start: T,
    end: T,
}
impl<T: Step> ConstIntoIter for RangeIterRev<T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = T;
}

/// Const-iterator for [`RangeInclusive`](core::ops::RangeInclusive)
///
/// This is constructed like this:
/// ```rust
/// # let _ =
/// konst::iter::into_iter!(0..=10)
/// # ;
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct RangeInclusiveIter<T> {
    start: T,
    end: T,
}
impl<T: Step> ConstIntoIter for RangeInclusiveIter<T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = T;
}

/// Reversed const-iterator for [`RangeInclusive`](core::ops::RangeInclusive)
///
/// This is constructed like this:
/// ```rust
/// # let _ =
/// konst::iter::into_iter!(0..=10).rev()
/// # ;
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct RangeInclusiveIterRev<T> {
    start: T,
    end: T,
}
impl<T: Step> ConstIntoIter for RangeInclusiveIterRev<T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = T;
}

/// Const-iterator for [`RangeFrom`](core::ops::RangeFrom)
///
/// This is constructed like this:
/// ```rust
/// # let _ =
/// konst::iter::into_iter!(0..)
/// # ;
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct RangeFromIter<T> {
    start: T,
}
impl<T: Step> ConstIntoIter for RangeFromIter<T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = T;
}

macro_rules! int_range_shared {
    (is_forward = $is_forward:ident, ty = $Int:ty) => {
        iterator_shared! {
            is_forward = $is_forward,
            item = $Int,
            iter_forward = RangeIter<$Int>,
            iter_reversed = RangeIterRev<$Int>,
            next(self){
                let __StepRet{finished_exclusive, next, ..} =
                    increment(self.start, self.end);

                if finished_exclusive {
                    None
                } else {
                    // this assert can never fail,
                    // because start >= end goes to the other branch
                    // debug_assert!(!overflowed);

                    let ret = self.start;
                    self.start = next;
                    Some(ret)
                }
            },
            next_back {
                let __StepRet{finished_exclusive, next, overflowed, ..} =
                    decrement(self.start, self.end);

                if finished_exclusive {
                    None
                } else {
                    debug_assert!(!overflowed);

                    self.end = next;
                    Some(self.end)
                }
            },
            fields = {start, end},
        }
    };
}

impl<T: Step> RangeIter<T> {
    int_range_shared! {is_forward = true, ty = T}
}

impl<T: Step> RangeIterRev<T> {
    int_range_shared! {is_forward = false, ty = T}
}

//////////////////////////////////////////////////

macro_rules! int_range_inc_shared {
    (is_forward = $is_forward:ident, ty = $Int:ty) => {
        iterator_shared! {
            is_forward = $is_forward,
            item = $Int,
            iter_forward = RangeInclusiveIter<$Int>,
            iter_reversed = RangeInclusiveIterRev<$Int>,
            next(self){
                let __StepRet{finished_inclusive, next, overflowed, ..} =
                    increment(self.start, self.end);

                if finished_inclusive {
                    None
                } else {
                    let ret = self.start;

                    if overflowed {
                        self.start = T::MAX_VAL;
                        self.end = T::MIN_VAL;
                    } else {
                        self.start = next;
                    }

                    Some(ret)
                }
            },
            next_back {
                let __StepRet{finished_inclusive, next, overflowed, ..} =
                    decrement(self.start, self.end);

                if finished_inclusive {
                    None
                } else {
                    let ret = self.end;
                    if overflowed {
                        self.start = T::MAX_VAL;
                        self.end = T::MIN_VAL;
                    } else {
                        self.end = next;
                    }
                    Some(ret)
                }
            },
            fields = {start, end},
        }
    };
}

impl<T: Step> RangeInclusiveIter<T> {
    int_range_inc_shared! {is_forward = true, ty = T}
}

impl<T: Step> RangeInclusiveIterRev<T> {
    int_range_inc_shared! {is_forward = false, ty = T}
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl<T: Step> RangeFromIter<T> {
    iterator_shared! {
        is_forward = true,
        item = T,
        iter_forward = RangeFromIter<T>,
        next(self){
            let __StepRet{next, overflowed, ..} = increment(self.start, T::MAX_VAL);

            debug_assert!(!overflowed);

            let ret = self.start;
            self.start = next;
            Some(ret)
        },
        fields = {start},
    }
}

//////////////////////////////////////////////////

macro_rules! ii_wrapper_range_impls {
    ($range_inc_ii:expr, $($reff:tt)?) => {
        impl<T: Step> IntoIterWrapper<$($reff)? Range<T>, IsStdKind> {
            #[doc = concat!(
                "Converts `",
                stringify!($($reff)? Range<T>),
                "` into an iterator.",
            )]
            pub const fn const_into_iter(self) -> RangeIter<T> {
                let range = ManuallyDrop::into_inner(self.iter);
                RangeIter {
                    start: range.start,
                    end: range.end,
                }
            }
        }

        impl<T: Step> IntoIterWrapper<$($reff)? RangeInclusive<T>, IsStdKind> {
            #[doc = concat!(
                "Converts `",
                stringify!($($reff)? RangeInclusive<T>),
                "` into an iterator.",
            )]
            pub const fn const_into_iter(self) -> RangeInclusiveIter<T> {
                let range = ManuallyDrop::into_inner(self.iter);
                let (start, end) = $range_inc_ii(range);
                RangeInclusiveIter {start, end}
            }
        }

        impl<T: Step> IntoIterWrapper<$($reff)? RangeFrom<T>, IsStdKind> {
            #[doc = concat!(
                "Converts `",
                stringify!($($reff)? RangeFrom<T>),
                "` into an iterator.",
            )]
            pub const fn const_into_iter(self) -> RangeFromIter<T> {
                let range = ManuallyDrop::into_inner(self.iter);
                RangeFromIter {
                    start: range.start,
                }
            }
        }

    }
}

ii_wrapper_range_impls! {step::range_inclusive_into_inner, }
ii_wrapper_range_impls! {step::range_inclusive_ref_into_inner, &}
