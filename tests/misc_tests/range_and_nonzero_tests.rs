use konst::{
    nonzero::{
        cmp_nonzeroisize, cmp_nonzerou32, cmp_option_nonzeroisize, cmp_option_nonzerou32,
        eq_nonzeroisize, eq_nonzerou32, eq_option_nonzeroisize, eq_option_nonzerou32,
    },
    range::{eq_range_char, eq_range_u8, eq_rangeinc_char, eq_rangeinc_u8},
};

#[test]
#[cfg(feature = "range")]
fn eq_range_test() {
    use std::ops::Range;

    assertc_opt_eq_rets! {
        &Range<u8>, eq_range_u8 =>

        (&(3..5), &(3..5), true)
        (&(100..200), &(100..200), true)
        (&(3..5), &(4..5), false)
        (&(3..5), &(3..6), false)
    }

    assertc_opt_eq_rets! {
        &Range<char>, eq_range_char =>

        (&('3'..'5'), &('3'..'5'), true)
        (&('A'..'E'), &('A'..'E'), true)
        (&('3'..'5'), &('4'..'5'), false)
        (&('3'..'5'), &('3'..'6'), false)
    }
}

#[test]
#[cfg(feature = "range")]
fn eq_rangeinc_test() {
    use std::ops::RangeInclusive;

    assertc_opt_eq_rets! {
        &RangeInclusive<u8>, eq_rangeinc_u8 =>

        (&(3..=5), &(3..=5), true)
        (&(34..=255), &(34..=255), true)
        (&(3..=5), &(4..=5), false)
        (&(3..=5), &(3..=6), false)
    }

    assertc_opt_eq_rets! {
        &RangeInclusive<char>, eq_rangeinc_char =>

        (&('3'..='5'), &('3'..='5'), true)
        (&('A'..='#'), &('A'..='#'), true)
        (&('3'..='5'), &('4'..='5'), false)
        (&('3'..='5'), &('3'..='6'), false)
    }
}

#[test]
#[cfg(feature = "nonzero")]
fn option_nonzerou32_test() {
    use core::{
        cmp::Ordering::{Equal, Greater, Less},
        num::NonZeroU32,
    };

    let make = |x: u32| NonZeroU32::new(x).unwrap();

    assertc_opt_eq_rets! {
        NonZeroU32, eq_nonzerou32, eq_option_nonzerou32 =>

        (make(3), make(3), true)
        (make(100), make(100), true)
        (make(3), make(50), false)
        (make(1), make(200), false)
    }

    assertc_opt_cmp! {
        NonZeroU32, cmp_nonzerou32, cmp_option_nonzerou32 =>

        (make(3), make(3), Equal)
        (make(100), make(100), Equal)
        (make(3), make(50), Less)
        (make(200), make(2), Greater)
    }
}

#[test]
#[cfg(feature = "nonzero")]
fn option_nonzeroisize_test() {
    use core::{
        cmp::Ordering::{Equal, Greater, Less},
        num::NonZeroIsize,
    };

    let make = |x: isize| NonZeroIsize::new(x).unwrap();

    assertc_opt_eq_rets! {
        NonZeroIsize, eq_nonzeroisize, eq_option_nonzeroisize =>

        (make(3), make(3), true)
        (make(100), make(100), true)
        (make(-100), make(-100), true)
        (make(3), make(50), false)
        (make(1), make(200), false)
        (make(200), make(-200), false)
    }

    assertc_opt_cmp! {
        NonZeroIsize, cmp_nonzeroisize, cmp_option_nonzeroisize =>

        (make(3), make(3), Equal)
        (make(100), make(100), Equal)
        (make(-100), make(-100), Equal)
        (make(3), make(50), Less)
        (make(1), make(200), Less)
        (make(200), make(-200), Greater)
    }
}
