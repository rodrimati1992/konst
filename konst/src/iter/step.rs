use typewit::{HasTypeWitness, MakeTypeWitness, TypeEq, TypeWitnessTypeArg};

use core::{marker::PhantomData, ops::RangeInclusive};

/// Trait for all the types that ranges can iterate over.
///
/// This trait is sealed and can only be implemented by `konst`
pub trait Step: HasTypeWitness<__StepWitness<Self>> + Copy {
    /// The minimum value of the type.
    const MIN_VAL: Self;

    /// The maximum value of the type.
    const MAX_VAL: Self;

    // hack to emulate sealed traits
    #[doc(hidden)]
    const __PRIV_KO9Y329U2U: __Priv<Self>;
}

#[doc(hidden)]
pub struct __Priv<T>(PhantomData<fn() -> T>);

#[doc(hidden)]
pub struct __StepRet<T> {
    pub finished_inclusive: bool,
    pub finished_exclusive: bool,
    pub overflowed: bool,
    pub next: T,
}

type Pair<T> = (T, T);

typewit::type_fn! {
    struct PairFn;
    impl<T> T => Pair<T>
}
typewit::type_fn! {
    struct RangeInclusiveFn;
    impl<T> T => RangeInclusive<T>
}

macro_rules! declare_step_witness {
    (
        $(($variant:ident, $type:ty, $kind:ident))*
    ) => {
        #[non_exhaustive]
        #[doc(hidden)]
        pub enum __StepWitness<T: Step> {
            $(
                #[non_exhaustive]
                $variant {
                    teq: TypeEq<T, $type>,
                },
            )*
        }

        impl<T: Step> TypeWitnessTypeArg for __StepWitness<T> {
            type Arg = T;
        }

        $(
            impl Step for $type {
                const MIN_VAL: Self = get_min!($kind, $type);
                const MAX_VAL: Self = <Self>::MAX;

                #[doc(hidden)]
                const __PRIV_KO9Y329U2U: __Priv<Self> = __Priv(PhantomData);
            }

            impl MakeTypeWitness for __StepWitness<$type> {
                const MAKE: Self = Self::$variant {
                    teq: TypeEq::NEW,
                };
            }
        )*

        pub(crate) const fn increment<T: Step>(start: T, end: T) -> __StepRet<T> {
            match HasTypeWitness::WITNESS {
                $(
                    __StepWitness::$variant{teq, ..} => {
                        let start = teq.to_right(start);
                        let end = teq.to_right(end);
                        code_for_step!($kind, increment, start, end, teq, to_left)
                    }
                )*
            }
        }

        pub(crate) const fn decrement<T: Step>(start: T, end: T) -> __StepRet<T> {
            match HasTypeWitness::WITNESS {
                $(
                    __StepWitness::$variant{teq, ..} => {
                        let start = teq.to_right(start);
                        let end = teq.to_right(end);
                        code_for_step!($kind, decrement, start, end, teq, to_left)
                    }
                )*
            }
        }

        // needed to work around the fact that copying RangeInclusive's fields
        // requires that it's passed by reference,
        // it's not possible to call `.start()` or `.end()` on a `RangeInclusive` value.
        //
        //
        // To do it would require  the `Frozen` trait (the "no internal mutability" trait)
        // is stabilized, then this function can be removed.
        pub(crate) const fn range_inclusive_into_inner<T: Step>(
            range: RangeInclusive<T>,
        ) -> (T, T) {
            match HasTypeWitness::WITNESS {
                $(
                    __StepWitness::$variant{teq, ..} => {
                        let range = teq.map(RangeInclusiveFn).to_right(range);
                        teq.map(PairFn).to_left((*range.start(),*range.end()))
                    }
                )*
            }
        }

        pub(crate) const fn range_inclusive_ref_into_inner<T: Step>(
            range: &RangeInclusive<T>,
        ) -> (T, T) {
            (*range.start(), *range.end())
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
    (Char, char, char)
}

macro_rules! code_for_step {
    (char, increment, $start:ident, $end:ident, $teq:expr, $to_dir:ident) => {{
        let (next_num, overflowed) = match $start as u32 {
            0xD7FF => (0xE000, false),
            0x10FFFF => (0, true),
            num => (num + 1, false),
        };
        let next = char::from_u32(next_num).unwrap();

        __StepRet {
            finished_inclusive: $start > $end,
            finished_exclusive: $start >= $end,
            overflowed,
            next: $teq.$to_dir(next),
        }
    }};
    (char, decrement, $start:ident, $end:ident, $teq:expr, $to_dir:ident) => {{
        let (next_num, overflowed) = match $end as u32 {
            0 => (0x10FFFF, true),
            0xE000 => (0xD7FF, false),
            num => (num - 1, false),
        };
        let next = char::from_u32(next_num).unwrap();

        __StepRet {
            finished_inclusive: $end < $start,
            finished_exclusive: $end <= $start,
            overflowed,
            next: $teq.$to_dir(next),
        }
    }};
    (int, increment, $start:ident, $end:ident, $teq:expr, $to_dir:ident) => {{
        let (next, overflowed) = $start.overflowing_add(1);
        __StepRet {
            finished_inclusive: $start > $end,
            finished_exclusive: $start >= $end,
            overflowed,
            next: $teq.$to_dir(next),
        }
    }};
    (int, decrement, $start:ident, $end:ident, $teq:expr, $to_dir:ident) => {{
        let (next, overflowed) = $end.overflowing_sub(1);
        __StepRet {
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
    (char, $ty:ty) => {
        '\0'
    };
}
use get_min;
