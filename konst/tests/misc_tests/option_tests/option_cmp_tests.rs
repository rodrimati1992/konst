// sanity test to ensure that Option comparison is sane
// there's more tests for Option<T> comparison in other modules

use std::cmp::Ordering::{Equal, Greater, Less};

#[test]
fn option_eq_is_same_as_std_test() {
    for (l, r, expected) in [
        (None, None::<u8>, true),
        (None::<u8>, Some(0u8), false),
        (Some(0u8), None::<u8>, false),
        (Some(0u8), Some(0u8), true),
        (Some(0u8), Some(1u8), false),
        (Some(1u8), Some(0u8), false),
    ] {
        assert_eq!(konst::cmp::const_eq!(l, r), expected, "l: {l:?}\nr: {r:?}");
        assert_eq!(
            konst::cmp::const_eq_for!(option; l, r),
            expected,
            "l: {l:?}\nr: {r:?}"
        );
        assert_eq!(l == r, expected, "l: {l:?}\nr: {r:?}");
    }
}

#[test]
fn option_cmp_is_same_as_std_test() {
    for (l, r, expected) in [
        (None, None::<u8>, Equal),
        (None::<u8>, Some(0u8), Less),
        (Some(0u8), None::<u8>, Greater),
        (Some(0u8), Some(0u8), Equal),
        (Some(0u8), Some(1u8), Less),
        (Some(1u8), Some(0u8), Greater),
    ] {
        assert_eq!(konst::cmp::const_cmp!(l, r), expected, "l: {l:?}\nr: {r:?}");
        assert_eq!(
            konst::cmp::const_cmp_for!(option; l, r),
            expected,
            "l: {l:?}\nr: {r:?}"
        );
        assert_eq!(std::cmp::Ord::cmp(&l, &r), expected, "l: {l:?}\nr: {r:?}");
    }
}
