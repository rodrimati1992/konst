use crate::type_eq::{HasTypeWitness, MakeTypeWitness, TypeEq, TypeWitnessTypeArg};

use core::{marker::PhantomData, ops::RangeInclusive};

/// Marker trait for all the types that can be iterated over with ranges.
///
/// This trait is sealed and can only be implemented by `konst`
pub trait Step: HasTypeWitness<StepWitness<Self>> + Copy {
    /// The minimum value of the type.
    const MIN_VAL: Self;

    /// The maximum value of the type.
    const MAX_VAL: Self;

    #[doc(hidden)]
    const __PRIV_KO9Y329U2U: __Priv<Self>;
}

#[doc(hidden)]
pub struct __Priv<T>(PhantomData<fn() -> T>);

pub(crate) struct StepRet<T> {
    pub(crate) finished_inclusive: bool,
    pub(crate) finished_exclusive: bool,
    pub(crate) overflowed: bool,
    pub(crate) next: T,
}

type Pair<T> = (T, T);
crate::type_eq_projection_fn! {
    const fn teq_pair => Pair<from T>
}

crate::type_eq_projection_fn! {
    const fn teq_range_inclusive => RangeInclusive<from T>
}

type RefRangeInclusive<'a, T> = &'a RangeInclusive<T>;
crate::type_eq_projection_fn! {
    const fn teq_ref_range_inclusive => RefRangeInclusive<'a, from T>
}

macro_rules! declare_step_witness {
    (
        $(($variant:ident, $type:ty, $kind:ident))*
    ) => {
        #[non_exhaustive]
        pub enum StepWitness<T: Step> {
            $(
                #[non_exhaustive]
                $variant {
                    teq: TypeEq<T, $type>,
                },
            )*
        }

        impl<T: Step> TypeWitnessTypeArg for StepWitness<T> {
            type Arg = T;
        }

        $(
            impl Step for $type {
                const MIN_VAL: Self = get_min!($kind, $type);
                const MAX_VAL: Self = <Self>::MAX;

                #[doc(hidden)]
                const __PRIV_KO9Y329U2U: __Priv<Self> = __Priv(PhantomData);
            }

            impl MakeTypeWitness for StepWitness<$type> {
                const MAKE: Self = Self::$variant {
                    teq: TypeEq::NEW,
                };
            }
        )*

        pub(crate) const fn increment<T: Step>(start: T, end: T) -> StepRet<T> {
            match HasTypeWitness::WITNESS {
                $(
                    StepWitness::$variant{teq, ..} => {
                        let start = teq.to_right(start);
                        let end = teq.to_right(end);
                        code_for_step!($kind, increment, start, end, teq, to_left)
                    }
                )*
            }
        }

        pub(crate) const fn decrement<T: Step>(start: T, end: T) -> StepRet<T> {
            match HasTypeWitness::WITNESS {
                $(
                    StepWitness::$variant{teq, ..} => {
                        let start = teq.to_right(start);
                        let end = teq.to_right(end);
                        code_for_step!($kind, decrement, start, end, teq, to_left)
                    }
                )*
            }
        }

        pub(crate) const fn range_inclusive_into_inner<T: Step>(
            range: RangeInclusive<T>,
        ) -> (T, T) {
            match HasTypeWitness::WITNESS {
                $(
                    StepWitness::$variant{teq, ..} => {
                        let range = teq_range_inclusive(teq).to_right(range);
                        teq_pair(teq).to_left((*range.start(),*range.end()))
                    }
                )*
            }
        }
        pub(crate) const fn range_inclusive_ref_into_inner<T: Step>(
            range: &RangeInclusive<T>,
        ) -> (T, T) {
            match HasTypeWitness::WITNESS {
                $(
                    StepWitness::$variant{teq, ..} => {
                        let range = teq_ref_range_inclusive(teq).to_right(range);
                        teq_pair(teq).to_left((*range.start(),*range.end()))
                    }
                )*
            }
        }
    };
}

declare_step_witness! {
    (Usize, usize, int)
    (I32, i32, int)
    (U8, u8, int)
    (U32, u32, int)
    (I8, i8, int)
    (U16, u16, int)
    (U64, u64, int)
    (U128, u128, int)
    (I16, i16, int)
    (I64, i64, int)
    (I128, i128, int)
    (Isize, isize, int)
}

macro_rules! code_for_step {
    (int, increment, $start:ident, $end:ident, $teq:expr, $to_dir:ident) => {{
        let (next, overflowed) = $start.overflowing_add(1);
        StepRet {
            finished_inclusive: $start > $end,
            finished_exclusive: $start >= $end,
            overflowed,
            next: $teq.$to_dir(next),
        }
    }};
    (int, decrement, $start:ident, $end:ident, $teq:expr, $to_dir:ident) => {{
        let (next, overflowed) = $end.overflowing_sub(1);
        StepRet {
            finished_inclusive: $end < $start,
            finished_exclusive: $end <= $start,
            overflowed,
            next: $teq.$to_dir(next),
        }
    }};
}
use code_for_step;

macro_rules! get_min {
    (int, $ty:ty) => {
        <$ty>::MIN
    };
}
use get_min;
