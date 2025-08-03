use konst::iter;

use std::{
    cmp::Ordering::{self, Equal, Greater, Less},
    ops::Range,
};

#[derive(Debug, PartialEq, Eq)]
struct Orderings {
    normal: Ordering,
    swapped: Ordering,
    rev: Ordering,
}

#[test]
fn cmp_method_test() {
    const fn orderings(left: &Range<u8>, right: &Range<u8>) -> Orderings {
        Orderings {
            normal: iter::eval! {left,cmp(right)},
            swapped: iter::eval! {right,cmp(left)},
            rev: iter::eval! {left,rev(),cmp(right)},
        }
    }

    let ref aa = 0..10;
    let ref bb = 5..8;
    let ref cc = 0..13;
    let ref dd = 11..13;

    assert_eq!(
        orderings(aa, aa),
        Orderings {
            normal: Equal,
            swapped: Equal,
            rev: Greater
        }
    );

    assert_eq!(
        orderings(aa, bb),
        Orderings {
            normal: Less,
            swapped: Greater,
            rev: Greater
        }
    );

    assert_eq!(
        orderings(aa, cc),
        Orderings {
            normal: Less,
            swapped: Greater,
            rev: Greater
        }
    );

    assert_eq!(
        orderings(aa, dd),
        Orderings {
            normal: Less,
            swapped: Greater,
            rev: Less
        }
    );
}
