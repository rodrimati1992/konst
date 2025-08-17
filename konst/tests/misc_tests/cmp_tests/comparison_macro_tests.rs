use konst::assertc_eq;
use konst::cmp;

use std::cmp::Ordering::{self, Equal, Greater, Less};

struct OnlyEq(u8);
use OnlyEq as OE;

konst::cmp::impl_cmp! {
    impl OnlyEq;

    pub const fn const_eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[allow(dead_code)]
const fn oe_array<const N: usize>(array: [u8; N]) -> [OnlyEq; N] {
    let mut out = [const { OnlyEq(0) }; N];

    ::konst::for_range! {i in 0..N =>
        out[i] = OnlyEq(array[i]);
    }

    out
}

#[allow(dead_code)]
const fn u8_eq(l: &u8, r: &u8) -> bool {
    *l / 2 == *r / 2
}

#[allow(dead_code)]
const fn opt_u8_eq(l_opt: &Option<u8>, r_opt: &Option<u8>) -> bool {
    cmp::const_eq!(l_opt, r_opt)
}

#[allow(dead_code)]
const fn u8_cmp(l: &u8, r: &u8) -> Ordering {
    cmp::const_cmp!(*l / 2, *r / 2)
}

#[allow(dead_code)]
const fn opt_u8_cmp(l_opt: &Option<u8>, r_opt: &Option<u8>) -> Ordering {
    cmp::const_cmp!(l_opt, r_opt)
}

#[test]
fn const_eq_test() {
    const fn constness(l: u8, r: u8) -> bool {
        cmp::const_eq!(OnlyEq(l), OnlyEq(r))
    }

    assert_eq!(constness(10, 9), false);
    assert_eq!(constness(10, 10), true);
    assert_eq!(constness(10, 11), false);
}

#[test]
fn const_eq_for_types_test() {
    const _: () = {
        use konst::cmp::const_eq_for as cef;

        assertc_eq!(cef!(slice; oe_array([0u8; 0]), oe_array([0u8; 0])), true);
        assertc_eq!(cef!(slice; oe_array([0u8; 0]), oe_array([3])), false);
        assertc_eq!(cef!(slice; oe_array([3u8]), oe_array([0u8; 0])), false);
        assertc_eq!(cef!(slice; oe_array([3u8]), oe_array([3])), true);
        assertc_eq!(cef!(slice; oe_array([3u8]), oe_array([5])), false);
        assertc_eq!(cef!(slice; oe_array([3u8]), oe_array([3, 5])), false);
        assertc_eq!(cef!(slice; oe_array([3u8, 5]), oe_array([3])), false);
        assertc_eq!(cef!(slice; oe_array([3u8, 5]), oe_array([3, 5])), true);
        assertc_eq!(cef!(slice; oe_array([3u8, 5]), oe_array([3, 8])), false);

        assertc_eq!(cef!(option; None::<OE>, None::<OE>), true);
        assertc_eq!(cef!(option; Some(OE(0u8)), None::<OE>), false);
        assertc_eq!(cef!(option; None::<OE>, Some(OE(0u8))), false);
        assertc_eq!(cef!(option; Some(OE(0u8)), Some(OE(0u8))), true);
        assertc_eq!(cef!(option; Some(OE(0u8)), Some(OE(1u8))), false);

        assertc_eq!(cef!(range; OE(0u8)..OE(0u8), OE(0u8)..OE(0u8)), true);
        assertc_eq!(cef!(range; OE(0u8)..OE(0u8), OE(0u8)..OE(1u8)), false);
        assertc_eq!(cef!(range; OE(0u8)..OE(0u8), OE(1u8)..OE(0u8)), false);
        assertc_eq!(cef!(range; OE(0u8)..OE(0u8), OE(1u8)..OE(1u8)), false);

        assertc_eq!(
            cef!(range_inclusive; OE(0u8)..=OE(0u8), OE(0u8)..=OE(0u8)),
            true
        );
        assertc_eq!(
            cef!(range_inclusive; OE(0u8)..=OE(0u8), OE(0u8)..=OE(1u8)),
            false
        );
        assertc_eq!(
            cef!(range_inclusive; OE(0u8)..=OE(0u8), OE(1u8)..=OE(0u8)),
            false
        );
        assertc_eq!(
            cef!(range_inclusive; OE(0u8)..=OE(0u8), OE(1u8)..=OE(1u8)),
            false
        );
    };
}

#[test]
fn const_eq_for_nonempty_comparator_test() {
    const _: () = {
        use konst::cmp::const_eq_for as cef;

        assertc_eq!(cef!(option; None::<u8>, None::<u8>, |x| *x / 2), true);
        assertc_eq!(cef!(option; Some(0u8), None::<u8>, |_| 0u8), false);
        assertc_eq!(cef!(option; None::<u8>, Some(0u8), |_| 0u8), false);
        assertc_eq!(cef!(option; Some(0u8), Some(0u8), |x| *x / 2), true);
        assertc_eq!(cef!(option; Some(0u8), Some(2u8), |x| *x / 2), false);
        assertc_eq!(cef!(option; Some(0u8), Some(1u8), |&x| x / 2), true);
        assertc_eq!(cef!(option; Some(0u8), Some(2u8), |&x| x / 2), false);

        assertc_eq!(
            cef!(option; None::<u8>, None::<u8>, |l, r| *l / 2 == *r / 2),
            true
        );
        assertc_eq!(cef!(option; Some(0u8), None::<u8>, |_, _| true), false);
        assertc_eq!(cef!(option; None::<u8>, Some(0u8), |_, _| true), false);
        assertc_eq!(
            cef!(option; Some(0u8), Some(0u8), |l, r| *l / 2 == *r / 2),
            true
        );
        assertc_eq!(
            cef!(option; Some(0u8), Some(2u8), |&l, r| l / 2 == *r / 2),
            false
        );
        assertc_eq!(
            cef!(option; Some(1u8), Some(2u8), |l, &r| *l / 2 == r / 2),
            false
        );
        assertc_eq!(
            cef!(option; Some(1u8), Some(0u8), |&l, &r| l / 2 == r / 2),
            true
        );

        let nested_vals = [
            (None, None, true),
            (None, Some(None), false),
            (None, Some(Some(5u8)), false),
            (Some(None), None, false),
            (Some(None), Some(None), true),
            (Some(Some(3u8)), None, false),
            (Some(Some(3u8)), Some(None), false),
            (Some(Some(3u8)), Some(Some(3u8)), true),
            (Some(Some(3u8)), Some(Some(5u8)), false),
        ];
        ::konst::for_range! {i in 0..nested_vals.len() =>
            let (l, r, expected) = nested_vals[i];

            assertc_eq!(
                cef!(
                    option; l, r,
                    ::tests::misc_tests::cmp_tests::comparison_macro_tests::opt_u8_eq
                ),
                expected,
                "\ni = ", i,
            );
        }
        assertc_eq!(cef!(option; None::<u8>, None::<u8>, self::u8_eq), true);
        assertc_eq!(cef!(option; Some(0u8), None::<u8>, u8_eq), false);
        assertc_eq!(cef!(option; None::<u8>, Some(0u8), u8_eq), false);
        assertc_eq!(cef!(option; Some(0u8), Some(0u8), u8_eq), true);
        assertc_eq!(cef!(option; Some(0u8), Some(2u8), u8_eq), false);
        assertc_eq!(cef!(option; Some(1u8), Some(2u8), u8_eq), false);
        assertc_eq!(cef!(option; Some(1u8), Some(0u8), u8_eq), true);
    };
}

#[test]
fn const_ne_test() {
    const fn constness(l: u8, r: u8) -> bool {
        cmp::const_ne!(OnlyEq(l), OnlyEq(r))
    }

    assert_eq!(constness(10, 9), true);
    assert_eq!(constness(10, 10), false);
    assert_eq!(constness(10, 11), true);
}

#[test]
fn const_ne_for_types_test() {
    const _: () = {
        use konst::cmp::const_ne_for as cnef;

        assertc_eq!(cnef!(option; None::<OE>, None::<OE>), false);
        assertc_eq!(cnef!(option; Some(OE(0u8)), None::<OE>), true);
        assertc_eq!(cnef!(option; None::<OE>, Some(OE(0u8))), true);
        assertc_eq!(cnef!(option; Some(OE(0u8)), Some(OE(0u8))), false);
        assertc_eq!(cnef!(option; Some(OE(0u8)), Some(OE(1u8))), true);
    };
}

#[test]
fn const_cmp_test() {
    const fn constness(l: u8, r: u8) -> Ordering {
        cmp::const_cmp!(l, r)
    }

    assert_eq!(constness(15, 14), Ordering::Greater);
    assert_eq!(constness(15, 15), Ordering::Equal);
    assert_eq!(constness(15, 16), Ordering::Less);
}

#[test]
fn const_cmp_for_test() {
    const _: () = {
        use konst::cmp::const_cmp_for as ccf;

        assertc_eq!(ccf!(slice; [0u8; 0], [0u8; 0]), Equal);
        assertc_eq!(ccf!(slice; [0u8; 0], [3]), Less);
        assertc_eq!(ccf!(slice; [3u8], [0u8; 0]), Greater);
        assertc_eq!(ccf!(slice; [3u8], [3]), Equal);
        assertc_eq!(ccf!(slice; [3u8], [5]), Less);
        assertc_eq!(ccf!(slice; [6u8], [5]), Greater);
        assertc_eq!(ccf!(slice; [3u8], [3, 5]), Less);
        assertc_eq!(ccf!(slice; [4u8], [3, 5]), Greater);
        assertc_eq!(ccf!(slice; [3u8, 5], [3]), Greater);
        assertc_eq!(ccf!(slice; [3u8, 5], [3, 5]), Equal);
        assertc_eq!(ccf!(slice; [3u8, 5], [3, 8]), Less);
        assertc_eq!(ccf!(slice; [4u8, 5], [3, 8]), Greater);

        assertc_eq!(ccf!(option; None::<u8>, None::<u8>), Equal);
        assertc_eq!(ccf!(option; Some(0u8), None::<u8>), Greater);
        assertc_eq!(ccf!(option; None::<u8>, Some(0u8)), Less);
        assertc_eq!(ccf!(option; Some(0u8), Some(1u8)), Less);
        assertc_eq!(ccf!(option; Some(0u8), Some(0u8)), Equal);
        assertc_eq!(ccf!(option; Some(1u8), Some(0u8)), Greater);
    };
}

#[test]
fn const_cmp_for_comparator_test() {
    const _: () = {
        use konst::cmp::const_cmp_for as ccf;

        assertc_eq!(ccf!(option; None::<u8>, None::<u8>, |x| *x / 2), Equal);
        assertc_eq!(ccf!(option; Some(0u8), None::<u8>, |_| 0u8), Greater);
        assertc_eq!(ccf!(option; None::<u8>, Some(0u8), |_| 0u8), Less);
        assertc_eq!(ccf!(option; Some(2u8), Some(0u8), |x| *x / 2), Greater);
        assertc_eq!(ccf!(option; Some(0u8), Some(0u8), |x| *x / 2), Equal);
        assertc_eq!(ccf!(option; Some(0u8), Some(2u8), |x| *x / 2), Less);
        assertc_eq!(ccf!(option; Some(2u8), Some(1u8), |&x| x / 2), Greater);
        assertc_eq!(ccf!(option; Some(0u8), Some(1u8), |&x| x / 2), Equal);
        assertc_eq!(ccf!(option; Some(0u8), Some(2u8), |&x| x / 2), Less);

        // TODO
        assertc_eq!(
            ccf!(option; None::<u8>, None::<u8>, |l, r| u8_cmp(l, r)),
            Equal
        );
        assertc_eq!(ccf!(option; Some(0u8), None::<u8>, |_, _| Equal), Greater);
        assertc_eq!(ccf!(option; None::<u8>, Some(0u8), |_, _| Equal), Less);
        assertc_eq!(
            ccf!(option; Some(0u8), Some(0u8), |l, r| u8_cmp(l, r)),
            Equal
        );
        assertc_eq!(
            ccf!(option; Some(0u8), Some(2u8), |&l, r| u8_cmp(&l, r)),
            Less
        );
        assertc_eq!(
            ccf!(option; Some(1u8), Some(2u8), |l, &r| u8_cmp(l, &r)),
            Less
        );
        assertc_eq!(
            ccf!(option; Some(2u8), Some(0u8), |&l, &r| u8_cmp(&l, &r)),
            Greater
        );

        let nested_vals = [
            (None, None, Equal),
            (None, Some(None), Less),
            (None, Some(Some(5u8)), Less),
            (Some(None), None, Greater),
            (Some(None), Some(None), Equal),
            (Some(Some(3u8)), None, Greater),
            (Some(Some(3u8)), Some(None), Greater),
            (Some(Some(3u8)), Some(Some(5u8)), Less),
            (Some(Some(3u8)), Some(Some(3u8)), Equal),
            (Some(Some(5u8)), Some(Some(3u8)), Greater),
        ];
        ::konst::for_range! {i in 0..nested_vals.len() =>
            let (l, r, expected) = nested_vals[i];

            assertc_eq!(
                ccf!(
                    option; l, r,
                    ::tests::misc_tests::cmp_tests::comparison_macro_tests::opt_u8_cmp
                ),
                expected,
                "\ni = ", i,
            );
        }
        assertc_eq!(ccf!(option; None::<u8>, None::<u8>, self::u8_cmp), Equal);
        assertc_eq!(ccf!(option; Some(0u8), None::<u8>, u8_cmp), Greater);
        assertc_eq!(ccf!(option; None::<u8>, Some(0u8), u8_cmp), Less);
        assertc_eq!(ccf!(option; Some(0u8), Some(0u8), u8_cmp), Equal);
        assertc_eq!(ccf!(option; Some(0u8), Some(2u8), u8_cmp), Less);
        assertc_eq!(ccf!(option; Some(1u8), Some(2u8), u8_cmp), Less);
        assertc_eq!(ccf!(option; Some(1u8), Some(0u8), u8_cmp), Equal);
        assertc_eq!(ccf!(option; Some(2u8), Some(1u8), u8_cmp), Greater);
    };
}

#[test]
fn const_lt_test() {
    const fn constness(l: u8, r: u8) -> bool {
        cmp::const_lt!(l, r)
    }

    assert_eq!(constness(15, 14), false);
    assert_eq!(constness(15, 15), false);
    assert_eq!(constness(15, 16), true);
}

#[test]
fn const_lt_for_test() {
    const _: () = {
        use konst::cmp::const_lt_for as cltf;

        assertc_eq!(cltf!(option; None::<u8>, None::<u8>), false);
        assertc_eq!(cltf!(option; Some(0u8), None::<u8>), false);
        assertc_eq!(cltf!(option; None::<u8>, Some(0u8)), true);
        assertc_eq!(cltf!(option; Some(0u8), Some(0u8)), false);
        assertc_eq!(cltf!(option; Some(0u8), Some(1u8)), true);
        assertc_eq!(cltf!(option; Some(1u8), Some(0u8)), false);
    };
}

#[test]
fn const_le_test() {
    const fn constness(l: u8, r: u8) -> bool {
        cmp::const_le!(l, r)
    }

    assert_eq!(constness(15, 14), false);
    assert_eq!(constness(15, 15), true);
    assert_eq!(constness(15, 16), true);
}

#[test]
fn const_le_for_test() {
    const _: () = {
        use konst::cmp::const_le_for as clef;

        assertc_eq!(clef!(option; None::<u8>, None::<u8>), true);
        assertc_eq!(clef!(option; Some(0u8), None::<u8>), false);
        assertc_eq!(clef!(option; None::<u8>, Some(0u8)), true);
        assertc_eq!(clef!(option; Some(0u8), Some(0u8)), true);
        assertc_eq!(clef!(option; Some(0u8), Some(1u8)), true);
        assertc_eq!(clef!(option; Some(1u8), Some(0u8)), false);
    };
}

#[test]
fn const_gt_test() {
    const fn constness(l: u8, r: u8) -> bool {
        cmp::const_gt!(l, r)
    }

    assert_eq!(constness(15, 14), true);
    assert_eq!(constness(15, 15), false);
    assert_eq!(constness(15, 16), false);
}

#[test]
fn const_gt_for_test() {
    const _: () = {
        use konst::cmp::const_gt_for as cgtf;

        assertc_eq!(cgtf!(option; None::<u8>, None::<u8>), false);
        assertc_eq!(cgtf!(option; Some(0u8), None::<u8>), true);
        assertc_eq!(cgtf!(option; None::<u8>, Some(0u8)), false);
        assertc_eq!(cgtf!(option; Some(0u8), Some(0u8)), false);
        assertc_eq!(cgtf!(option; Some(0u8), Some(1u8)), false);
        assertc_eq!(cgtf!(option; Some(1u8), Some(0u8)), true);
    };
}

#[test]
fn const_ge_test() {
    const fn constness(l: u8, r: u8) -> bool {
        cmp::const_ge!(l, r)
    }

    assert_eq!(constness(15, 14), true);
    assert_eq!(constness(15, 15), true);
    assert_eq!(constness(15, 16), false);
}

#[test]
fn const_ge_for_test() {
    const _: () = {
        use konst::cmp::const_ge_for as cgef;

        assertc_eq!(cgef!(option; None::<u8>, None::<u8>), true);
        assertc_eq!(cgef!(option; Some(0u8), None::<u8>), true);
        assertc_eq!(cgef!(option; None::<u8>, Some(0u8)), false);
        assertc_eq!(cgef!(option; Some(0u8), Some(0u8)), true);
        assertc_eq!(cgef!(option; Some(0u8), Some(1u8)), false);
        assertc_eq!(cgef!(option; Some(1u8), Some(0u8)), true);
    };
}
