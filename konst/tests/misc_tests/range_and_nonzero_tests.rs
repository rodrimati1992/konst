use konst::{nonzero::cmp as nz_cmp, range::cmp as range_cmp};

#[cfg(feature = "iter")]
mod range_iter_tests;

#[test]
#[cfg(feature = "cmp")]
fn eq_range_test() {
    use std::ops::Range;

    macro_rules! unsigned_cases {($(($ty:ident $eq:ident))*) => ($({
        assertc_opt_eq_rets! {
            &Range<$ty>, range_cmp::$eq =>

            (&(3..5), &(3..5), true)
            (&(100..127), &(100..127), true)
            (&(3..5), &(4..5), false)
            (&(3..5), &(3..6), false)
        }
    })*)}
    unsigned_cases! {
        (u8 eq_range_u8)
        (u16 eq_range_u16)
        (u32 eq_range_u32)
        (u64 eq_range_u64)
        (u128 eq_range_u128)
        (usize eq_range_usize)
    }

    macro_rules! signed_cases {($(($ty:ident $eq:ident))*) => ($({
        assertc_opt_eq_rets! {
            &Range<$ty>, range_cmp::$eq =>

            (&(3..5), &(3..5), true)
            (&(100..127), &(100..127), true)
            (&(-3..-5), &(3..5), false)
            (&(3..5), &(3..6), false)
        }
    })*)}
    signed_cases! {
        (i8 eq_range_i8)
        (i16 eq_range_i16)
        (i32 eq_range_i32)
        (i64 eq_range_i64)
        (i128 eq_range_i128)
        (isize eq_range_isize)
    }

    assertc_opt_eq_rets! {
        &Range<char>, range_cmp::eq_range_char =>

        (&('3'..'5'), &('3'..'5'), true)
        (&('A'..'E'), &('A'..'E'), true)
        (&('3'..'5'), &('4'..'5'), false)
        (&('3'..'5'), &('3'..'6'), false)
    }
}

#[test]
#[cfg(feature = "cmp")]
fn eq_rangeinc_test() {
    use std::ops::RangeInclusive;

    macro_rules! unsigned_cases {($(($ty:ident $eq:ident))*) => ($({
        assertc_opt_eq_rets! {
            &RangeInclusive<$ty>, range_cmp::$eq =>

            (&(3..=5), &(3..=5), true)
            (&(34..=255), &(34..=255), true)
            (&(3..=5), &(4..=5), false)
            (&(3..=5), &(3..=6), false)
        }
    })*)}
    unsigned_cases! {
        (u8 eq_rangeinc_u8)
        (u16 eq_rangeinc_u16)
        (u32 eq_rangeinc_u32)
        (u64 eq_rangeinc_u64)
        (u128 eq_rangeinc_u128)
        (usize eq_rangeinc_usize)
    }

    macro_rules! signed_cases {($(($ty:ident $eq:ident))*) => ($({
        assertc_opt_eq_rets! {
            &RangeInclusive<$ty>, range_cmp::$eq =>

            (&(3..=5), &(3..=5), true)
            (&(34..=127), &(34..=127), true)
            (&(-3..=-5), &(3..=5), false)
            (&(3..=5), &(3..=6), false)
        }
    })*)}
    signed_cases! {
        (i8 eq_rangeinc_i8)
        (i16 eq_rangeinc_i16)
        (i32 eq_rangeinc_i32)
        (i64 eq_rangeinc_i64)
        (i128 eq_rangeinc_i128)
        (isize eq_rangeinc_isize)
    }

    assertc_opt_eq_rets! {
        &RangeInclusive<char>, range_cmp::eq_rangeinc_char =>

        (&('3'..='5'), &('3'..='5'), true)
        (&('A'..='#'), &('A'..='#'), true)
        (&('3'..='5'), &('4'..='5'), false)
        (&('3'..='5'), &('3'..='6'), false)
    }
}

#[test]
#[cfg(feature = "cmp")]
fn option_nonzero_unsigned_test() {
    use core::{
        cmp::Ordering::{Equal, Greater, Less},
        num,
    };

    macro_rules! unsigned_cases {
        ($(($ty:ident $eq:ident $eq_opt:ident $cmp:ident $cmp_opt:ident))*) => ($({
            let make = |x| num::$ty::new(x).unwrap();

            assertc_opt_eq_rets! {
                num::$ty, nz_cmp::$eq, nz_cmp::$eq_opt =>

                (make(3), make(3), true)
                (make(100), make(100), true)
                (make(3), make(50), false)
                (make(1), make(100), false)
            }

            assertc_opt_cmp! {
                num::$ty, nz_cmp::$cmp, nz_cmp::$cmp_opt =>

                (make(3), make(3), Equal)
                (make(100), make(100), Equal)
                (make(3), make(50), Less)
                (make(100), make(2), Greater)
            }
        })*)
    }
    unsigned_cases! {
        (
            NonZeroU8
            eq_nonzerou8 eq_option_nonzerou8
            cmp_nonzerou8 cmp_option_nonzerou8
        )
        (
            NonZeroU16
            eq_nonzerou16 eq_option_nonzerou16
            cmp_nonzerou16 cmp_option_nonzerou16
        )
        (
            NonZeroU32
            eq_nonzerou32 eq_option_nonzerou32
            cmp_nonzerou32 cmp_option_nonzerou32
        )
        (
            NonZeroU64
            eq_nonzerou64 eq_option_nonzerou64
            cmp_nonzerou64 cmp_option_nonzerou64
        )
        (
            NonZeroU128
            eq_nonzerou128 eq_option_nonzerou128
            cmp_nonzerou128 cmp_option_nonzerou128
        )
        (
            NonZeroUsize
            eq_nonzerousize eq_option_nonzerousize
            cmp_nonzerousize cmp_option_nonzerousize
        )
    }
}

#[test]
#[cfg(feature = "cmp")]
fn option_nonzero_signed_test() {
    use core::{
        cmp::Ordering::{Equal, Greater, Less},
        num,
    };

    macro_rules! signed_cases {
        ($(($ty:ident $eq:ident $eq_opt:ident $cmp:ident $cmp_opt:ident))*) => ($({
            let make = |x| num::$ty::new(x).unwrap();

            assertc_opt_eq_rets! {
                num::$ty, nz_cmp::$eq, nz_cmp::$eq_opt =>

                (make(3), make(3), true)
                (make(100), make(100), true)
                (make(-100), make(-100), true)
                (make(3), make(50), false)
                (make(1), make(120), false)
                (make(120), make(-120), false)
            }

            assertc_opt_cmp! {
                num::$ty, nz_cmp::$cmp, nz_cmp::$cmp_opt =>

                (make(3), make(3), Equal)
                (make(100), make(100), Equal)
                (make(-100), make(-100), Equal)
                (make(3), make(50), Less)
                (make(1), make(120), Less)
                (make(120), make(-120), Greater)
            }
        })*)
    }
    signed_cases! {
        (
            NonZeroI8
            eq_nonzeroi8 eq_option_nonzeroi8
            cmp_nonzeroi8 cmp_option_nonzeroi8
        )
        (
            NonZeroI16
            eq_nonzeroi16 eq_option_nonzeroi16
            cmp_nonzeroi16 cmp_option_nonzeroi16
        )
        (
            NonZeroI32
            eq_nonzeroi32 eq_option_nonzeroi32
            cmp_nonzeroi32 cmp_option_nonzeroi32
        )
        (
            NonZeroI64
            eq_nonzeroi64 eq_option_nonzeroi64
            cmp_nonzeroi64 cmp_option_nonzeroi64
        )
        (
            NonZeroI128
            eq_nonzeroi128 eq_option_nonzeroi128
            cmp_nonzeroi128 cmp_option_nonzeroi128
        )
        (
            NonZeroIsize
            eq_nonzeroisize eq_option_nonzeroisize
            cmp_nonzeroisize cmp_option_nonzeroisize
        )
    }
}
