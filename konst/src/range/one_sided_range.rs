use typewit::TypeEq;

use core::ops::{RangeFrom, RangeTo, RangeToInclusive};

/// Trait for ranges that are unbounded on one side,
/// and bounded on the other side.
///
/// This trait is sealed and cannot be implemented outside of `konst`
pub trait OneSidedRange: Sized {
    /// The `T` in `RangeFrom<T>`/`RangeTo<T>`/`RangeToInclusive<T>`
    type Item;

    #[doc(hidden)]
    const __ONE_SIDED_RANGE_WITNESS: __OneSidedRangeWitness<Self>;
}

pub(crate) enum OneSidedRangeBound {
    FromInclusive,
    ToExclusive,
}

pub(crate) const fn to_bound<R>(range: R) -> Option<(OneSidedRangeBound, usize)>
where
    R: OneSidedRange<Item = usize>,
{
    match R::__ONE_SIDED_RANGE_WITNESS {
        __OneSidedRangeWitness::RangeFrom(te) => {
            Some((OneSidedRangeBound::FromInclusive, te.to_right(range).start))
        }
        __OneSidedRangeWitness::RangeTo(te) => {
            Some((OneSidedRangeBound::ToExclusive, te.to_right(range).end))
        }
        __OneSidedRangeWitness::RangeToInclusive(te) => {
            let end = te.to_right(range).end;

            match end.checked_add(1) {
                Some(i) => Some((OneSidedRangeBound::ToExclusive, i)),
                None => None,
            }
        }
    }
}

impl<T> OneSidedRange for RangeFrom<T> {
    type Item = T;

    #[doc(hidden)]
    const __ONE_SIDED_RANGE_WITNESS: __OneSidedRangeWitness<Self> =
        __OneSidedRangeWitness::RangeFrom(TypeEq::NEW);
}

impl<T> OneSidedRange for RangeTo<T> {
    type Item = T;

    #[doc(hidden)]
    const __ONE_SIDED_RANGE_WITNESS: __OneSidedRangeWitness<Self> =
        __OneSidedRangeWitness::RangeTo(TypeEq::NEW);
}

impl<T> OneSidedRange for RangeToInclusive<T> {
    type Item = T;

    #[doc(hidden)]
    const __ONE_SIDED_RANGE_WITNESS: __OneSidedRangeWitness<Self> =
        __OneSidedRangeWitness::RangeToInclusive(TypeEq::NEW);
}

#[doc(hidden)]
#[non_exhaustive]
pub enum __OneSidedRangeWitness<This: OneSidedRange> {
    RangeFrom(TypeEq<This, RangeFrom<This::Item>>),
    RangeTo(TypeEq<This, RangeTo<This::Item>>),
    RangeToInclusive(TypeEq<This, RangeToInclusive<This::Item>>),
}
