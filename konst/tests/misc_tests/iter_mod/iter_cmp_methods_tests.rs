use konst::iter;

use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    ops::Range,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Comparisons<T> {
    normal: T,
    swapped: T,
    rev_lhs: T,
}

impl Comparisons<bool> {
    const fn negate(self) -> Self {
        Comparisons {
            normal: !self.normal,
            swapped: !self.swapped,
            rev_lhs: !self.rev_lhs,
        }
    }
}

#[test]
fn comparisons_megate_test() {
    assert_eq!(
        Comparisons {
            normal: false,
            swapped: true,
            rev_lhs: false,
        }
        .negate(),
        Comparisons {
            normal: true,
            swapped: false,
            rev_lhs: true,
        }
    );

    assert_eq!(
        Comparisons {
            normal: true,
            swapped: false,
            rev_lhs: true,
        }
        .negate(),
        Comparisons {
            normal: false,
            swapped: true,
            rev_lhs: false,
        }
    );
}

#[derive(Debug)]
struct AssertOpposites {
    value: Comparisons<bool>,
    expected_value: Comparisons<bool>,
    opposite: Comparisons<bool>,
}
impl AssertOpposites {
    #[track_caller]
    fn call(self) {
        assert_eq!(self.value, self.expected_value, "value == expected_value");

        assert_eq!(
            self.opposite,
            self.expected_value.negate(),
            "opposite == expected_value.negate()"
        );
    }
}

#[test]
fn cmp_method_test() {
    const fn orderings(left: &Range<u8>, right: &Range<u8>) -> Comparisons<Ordering> {
        Comparisons {
            normal: iter::eval! {left,cmp(right)},
            swapped: iter::eval! {right,cmp(left)},
            rev_lhs: iter::eval! {left,rev(),cmp(right)},
        }
    }

    let ref aa = 0..10;
    let ref bb = 5..8;
    let ref cc = 0..13;
    let ref dd = 11..13;

    assert_eq!(
        orderings(aa, aa),
        Comparisons {
            normal: Equal,
            swapped: Equal,
            rev_lhs: Greater
        }
    );

    assert_eq!(
        orderings(aa, bb),
        Comparisons {
            normal: Less,
            swapped: Greater,
            rev_lhs: Greater
        }
    );

    assert_eq!(
        orderings(aa, cc),
        Comparisons {
            normal: Less,
            swapped: Greater,
            rev_lhs: Greater
        }
    );

    assert_eq!(
        orderings(aa, dd),
        Comparisons {
            normal: Less,
            swapped: Greater,
            rev_lhs: Less
        }
    );
}

#[test]
fn eq_ne_method_test() {
    // ensures that items that only define `const_eq` (i.e.: not also `const_cmp`)
    // work with iter::eval's `eq` and `ne` methods
    struct OnlyEq(u8);

    konst::cmp::impl_cmp! {
        impl OnlyEq;

        pub const fn const_eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    const fn call_eq(left: &[OnlyEq], right: &[OnlyEq]) -> Comparisons<bool> {
        Comparisons {
            normal: iter::eval! {left,eq(right)},
            swapped: iter::eval! {right,eq(left)},
            rev_lhs: iter::eval! {left,rev(),eq(right)},
        }
    }

    const fn call_ne(left: &[OnlyEq], right: &[OnlyEq]) -> Comparisons<bool> {
        Comparisons {
            normal: iter::eval! {left,ne(right)},
            swapped: iter::eval! {right,ne(left)},
            rev_lhs: iter::eval! {left,rev(),ne(right)},
        }
    }

    let ref aa = *(0..10).map(OnlyEq).collect::<Vec<_>>();
    let ref bb = *(5..8).map(OnlyEq).collect::<Vec<_>>();

    AssertOpposites {
        value: call_eq(aa, aa),
        expected_value: Comparisons {
            normal: true,
            swapped: true,
            rev_lhs: false,
        },
        opposite: call_ne(aa, aa),
    }
    .call();

    AssertOpposites {
        value: call_eq(aa, bb),
        expected_value: Comparisons {
            normal: false,
            swapped: false,
            rev_lhs: false,
        },
        opposite: call_ne(aa, bb),
    }
    .call();
}

#[test]
fn lt_ge_method_test() {
    const fn call_lt(left: &Range<u8>, right: &Range<u8>) -> Comparisons<bool> {
        Comparisons {
            normal: iter::eval! {left,lt(right)},
            swapped: iter::eval! {right,lt(left)},
            rev_lhs: iter::eval! {left,rev(),lt(right)},
        }
    }

    const fn call_ge(left: &Range<u8>, right: &Range<u8>) -> Comparisons<bool> {
        Comparisons {
            normal: iter::eval! {left,ge(right)},
            swapped: iter::eval! {right,ge(left)},
            rev_lhs: iter::eval! {left,rev(),ge(right)},
        }
    }

    let ref aa = 0..10;
    let ref bb = 0..12;

    AssertOpposites {
        value: call_lt(aa, aa),
        expected_value: Comparisons {
            normal: false,
            swapped: false,
            rev_lhs: false,
        },
        opposite: call_ge(aa, aa),
    }
    .call();

    AssertOpposites {
        value: call_lt(aa, bb),
        expected_value: Comparisons {
            normal: true,
            swapped: false,
            rev_lhs: false,
        },
        opposite: call_ge(aa, bb),
    }
    .call();
}

#[test]
fn le_gt_method_test() {
    const fn call_le(left: &Range<u8>, right: &Range<u8>) -> Comparisons<bool> {
        Comparisons {
            normal: iter::eval! {left,le(right)},
            swapped: iter::eval! {right,le(left)},
            rev_lhs: iter::eval! {left,rev(),le(right)},
        }
    }

    const fn call_gt(left: &Range<u8>, right: &Range<u8>) -> Comparisons<bool> {
        Comparisons {
            normal: iter::eval! {left,gt(right)},
            swapped: iter::eval! {right,gt(left)},
            rev_lhs: iter::eval! {left,rev(),gt(right)},
        }
    }

    let ref aa = 0..10;
    let ref bb = 0..12;

    AssertOpposites {
        value: call_le(aa, aa),
        expected_value: Comparisons {
            normal: true,
            swapped: true,
            rev_lhs: false,
        },
        opposite: call_gt(aa, aa),
    }
    .call();

    AssertOpposites {
        value: call_le(aa, bb),
        expected_value: Comparisons {
            normal: true,
            swapped: false,
            rev_lhs: false,
        },
        opposite: call_gt(aa, bb),
    }
    .call();
}

#[test]
fn is_sorted_test() {
    const fn constness(slice: &[u8]) -> bool {
        iter::eval! {slice, is_sorted()}
    }

    assert!(constness(&[]));
    assert!(constness(&[1]));

    assert!(!constness(&[1, 0]));
    assert!(constness(&[1, 1]));
    assert!(constness(&[1, 2]));

    assert!(!constness(&[1, 0, 0]));
    assert!(!constness(&[1, 1, 0]));
    assert!(!constness(&[1, 2, 0]));
    assert!(constness(&[1, 2, 3]));
}

#[test]
fn is_sorted_by_test() {
    const fn constness(slice: &[u8]) -> bool {
        iter::eval! {slice, is_sorted_by(|l, r| **l >= **r)}
    }

    assert!(constness(&[]));
    assert!(constness(&[1]));

    assert!(constness(&[1, 0]));
    assert!(constness(&[1, 1]));
    assert!(!constness(&[1, 2]));

    assert!(constness(&[1, 0, 0]));
    assert!(constness(&[1, 1, 0]));
    assert!(constness(&[2, 1, 0]));
    assert!(!constness(&[1, 2, 0]));
    assert!(!constness(&[1, 2, 3]));
}

#[test]
fn is_sorted_by_key_test() {
    const fn constness(slice: &[(u8,)]) -> bool {
        iter::eval! {slice, is_sorted_by_key(|x| x.0)}
    }

    assert!(constness(&[].map(|x| (x,))));
    assert!(constness(&[1].map(|x| (x,))));

    assert!(!constness(&[1, 0].map(|x| (x,))));
    assert!(constness(&[1, 1].map(|x| (x,))));
    assert!(constness(&[1, 2].map(|x| (x,))));

    assert!(!constness(&[1, 0, 0].map(|x| (x,))));
    assert!(!constness(&[1, 1, 0].map(|x| (x,))));
    assert!(!constness(&[1, 2, 0].map(|x| (x,))));
    assert!(constness(&[1, 2, 3].map(|x| (x,))));
}
