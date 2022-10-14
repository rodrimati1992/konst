use std::ops::{Range, RangeFrom, RangeInclusive};

use konst::{iter, range};

macro_rules! compare_with_std {
    ($($range_op:tt)*) => {
        use rand::rngs::SmallRng;
        use rand::{Rng, SeedableRng};

        let mut rng = SmallRng::seed_from_u64(6249204433781597762);

        let mut history = Vec::new();
        for start in 0..=6 {
            for end in 0..=6 {
                history.clear();
                let mut std_iter = start $($range_op)* end;
                let mut iter = iter::into_iter!(start $($range_op)* end);

                for _ in 0..=20 {
                    let pair = if rng.gen() {
                        history.push("next");
                        (iter.copy().next(), std_iter.next())
                    } else {
                        history.push("next_back");
                        (iter.copy().next_back(), std_iter.next_back())
                    };

                    let extra_info = || format!(
                        "start: {}  end: {} history: {:?}",
                        start,
                        end,
                        history,
                    );

                    match pair {
                        (Some((elem, next_iter)), Some(elem_std)) => {
                            iter = next_iter;

                            assert_eq!(elem, elem_std, "{}", extra_info());
                        }
                        (Some((elem, _)), None) => {
                            panic!(
                                "konst iter had {:?} when std iter was exhausted {}",
                                elem,
                                extra_info(),
                            )
                        }
                        (None, Some(elem)) => {
                            panic!(
                                "std iter had {:?} when konst iter was exhausted {}",
                                elem,
                                extra_info(),
                            )
                        }
                        (None, None) => {}
                    }

                }
            }
        }

    }
}

macro_rules! reversible_const_callable_test {
    ($Range:ty, $NormalIter:ty, $Reversed:ty,) => {
        const fn __(range: $Range) {
            let iter = konst::iter::into_iter!(range);
            iter.copy().next();
            iter.copy().next_back();

            let rev: $Reversed = iter.copy().rev();

            let _: $NormalIter = rev.copy().rev();
            rev.copy().next();
            rev.copy().next_back();
        }
    };
}

#[test]
fn range_iter_const_callable_test() {
    reversible_const_callable_test! {
        Range<usize>,
        range::RangeIter<usize>,
        range::RangeIterRev<usize>,
    }
}

#[test]
fn range_iter_test() {
    assert_eq!(collect_const_iter!(100..0), [0usize; 0]);
    assert_eq!(collect_const_iter!(1..0), [0usize; 0]);
    assert_eq!(collect_const_iter!(0..0), [0usize; 0]);
    assert_eq!(collect_const_iter!(0..1), [0]);
    assert_eq!(collect_const_iter!(0..2), [0, 1]);
    assert_eq!(collect_const_iter!(0..3), [0, 1, 2]);
}

#[test]
fn range_iter_rev_test() {
    assert_eq!(
        collect_const_iter!(iter::into_iter!(100..0).rev()),
        [0usize; 0]
    );
    assert_eq!(
        collect_const_iter!(iter::into_iter!(1..0).rev()),
        [0usize; 0]
    );
    assert_eq!(
        collect_const_iter!(iter::into_iter!(0..0).rev()),
        [0usize; 0]
    );
    assert_eq!(collect_const_iter!(iter::into_iter!(0..1).rev()), [0]);
    assert_eq!(collect_const_iter!(iter::into_iter!(0..2).rev()), [1, 0]);
    assert_eq!(collect_const_iter!(iter::into_iter!(0..3).rev()), [2, 1, 0]);
}

#[test]
fn range_iter_mixed_test() {
    compare_with_std! {..}
}

#[test]
fn range_inclusive_iter_const_callable_test() {
    reversible_const_callable_test! {
        RangeInclusive<usize>,
        range::RangeInclusiveIter<usize>,
        range::RangeInclusiveIterRev<usize>,
    }
}

#[test]
fn range_inclusive_iter_test() {
    assert_eq!(collect_const_iter!(100..=0), [0usize; 0]);
    assert_eq!(collect_const_iter!(1..=0), [0usize; 0]);
    assert_eq!(collect_const_iter!(0..=0), [0]);
    assert_eq!(collect_const_iter!(0..=1), [0, 1]);
    assert_eq!(collect_const_iter!(0..=2), [0, 1, 2]);
    assert_eq!(collect_const_iter!(0..=3), [0, 1, 2, 3]);
}

#[test]
fn range_inclusive_iter_rev_test() {
    assert_eq!(
        collect_const_iter!(iter::into_iter!(100..=0).rev()),
        [0usize; 0]
    );
    assert_eq!(
        collect_const_iter!(iter::into_iter!(1..=0).rev()),
        [0usize; 0]
    );
    assert_eq!(collect_const_iter!(iter::into_iter!(0..=0).rev()), [0]);
    assert_eq!(collect_const_iter!(iter::into_iter!(0..=1).rev()), [1, 0]);
    assert_eq!(
        collect_const_iter!(iter::into_iter!(0..=2).rev()),
        [2, 1, 0]
    );
    assert_eq!(
        collect_const_iter!(iter::into_iter!(0..=3).rev()),
        [3, 2, 1, 0]
    );
}

#[test]
fn range_iter_inclusive_mixed_test() {
    compare_with_std! {..=}
}

#[test]
fn range_from_const_callable_test() {
    const fn __(range: RangeFrom<usize>) {
        let iter = iter::into_iter!(range);
        iter.copy().next();
    }
}

#[test]
fn range_from_iter_test() {
    let iter = iter::into_iter!(0..);

    let (next, iter) = iter.next().unwrap();
    assert_eq!(next, 0);

    let (next, iter) = iter.next().unwrap();
    assert_eq!(next, 1);

    let (next, _) = iter.next().unwrap();
    assert_eq!(next, 2);
}
