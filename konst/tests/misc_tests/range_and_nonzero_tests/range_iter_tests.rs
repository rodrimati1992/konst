use std::ops::{Range, RangeFrom, RangeInclusive};

use konst::{iter, range};

macro_rules! compare_with_std {
    ($($range_op:tt)*) => {
        use rand::rngs::SmallRng;
        use rand::{Rng, SeedableRng};

        let mut rng = SmallRng::seed_from_u64(6249204433781597762);

        let mut history = Vec::new();
        for start in 0..=6usize {
            for end in 0..=6usize {
                history.clear();
                let mut std_iter = start $($range_op)* end;
                let mut iter = iter::into_iter!(start $($range_op)* end);

                for _ in 0..=20 {
                    let pair = if rng.gen() {
                        history.push("next");
                        (iter.next(), std_iter.next())
                    } else {
                        history.push("next_back");
                        (iter.next_back(), std_iter.next_back())
                    };

                    let extra_info = || format!(
                        "start: {}  end: {} history: {:?}",
                        start,
                        end,
                        history,
                    );

                    match pair {
                        (Some(elem), Some(elem_std)) => {
                            assert_eq!(elem, elem_std, "{}", extra_info());
                        }
                        (Some(elem), None) => {
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
    assert_eq!(collect_const_iter!(0..1), [0usize]);
    assert_eq!(collect_const_iter!(0..2), [0usize, 1]);
    assert_eq!(collect_const_iter!(0..3), [0usize, 1, 2]);
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
    assert_eq!(collect_const_iter!(iter::into_iter!(0..1).rev()), [0usize]);
    assert_eq!(
        collect_const_iter!(iter::into_iter!(0..2).rev()),
        [1usize, 0]
    );
    assert_eq!(
        collect_const_iter!(iter::into_iter!(0..3).rev()),
        [2usize, 1, 0]
    );
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
    assert_eq!(collect_const_iter!(0..=0), [0usize]);
    assert_eq!(collect_const_iter!(0..=1), [0usize, 1]);
    assert_eq!(collect_const_iter!(0..=2), [0usize, 1, 2]);
    assert_eq!(collect_const_iter!(0..=3), [0usize, 1, 2, 3]);
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
    assert_eq!(collect_const_iter!(iter::into_iter!(0..=0).rev()), [0usize]);
    assert_eq!(
        collect_const_iter!(iter::into_iter!(0..=1).rev()),
        [1usize, 0]
    );
    assert_eq!(
        collect_const_iter!(iter::into_iter!(0..=2).rev()),
        [2usize, 1, 0]
    );
    assert_eq!(
        collect_const_iter!(iter::into_iter!(0..=3).rev()),
        [3usize, 2, 1, 0]
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
    let mut iter = iter::into_iter!(0..);

    assert_eq!(iter.next().unwrap(), 0usize);
    assert_eq!(iter.next().unwrap(), 1);
    assert_eq!(iter.next().unwrap(), 2);
}

#[test]
fn test_non_usize_integer_iters() {
    use collect_const_iter as cci;

    macro_rules! test_with_ranges {
        ($($ranges_array:expr,)*) => {$(

            for range in $ranges_array {
                let range = ||range.clone();
                assert_eq!(cci!(range()), range().collect::<Vec<_>>());
                assert_eq!(cci!(&range()), range().collect::<Vec<_>>());

                let rrev = || konst::iter::into_iter!(range()).rev();
                assert_eq!(cci!(rrev()), range().rev().collect::<Vec<_>>());
            }
        )*};
    }

    macro_rules! test_case {
        ($min:expr, $max:expr) => {{
            test_with_ranges! {
                [$min..$min + 5, $max - 5..$max],
                [$min..=$min + 5, $max - 5..=$max],
            }

            {
                let start = $max - 6;
                let mut iter = konst::iter::into_iter!(start..);

                for expected in start..$max {
                    let item = iter.next().unwrap();
                    assert_eq!(item, expected);
                }
            }
        }};
    }

    test_case!(u8::MIN, u8::MAX);
    test_case!(i8::MIN, i8::MAX);

    test_case!(u16::MIN, u16::MAX);
    test_case!(i16::MIN, i16::MAX);

    test_case!(u32::MIN, u32::MAX);
    test_case!(i32::MIN, i32::MAX);

    test_case!(u64::MIN, u64::MAX);
    test_case!(i64::MIN, i64::MAX);

    test_case!(u128::MIN, u128::MAX);
    test_case!(i128::MIN, i128::MAX);

    test_case!(usize::MIN, usize::MAX);
    test_case!(isize::MIN, isize::MAX);
}

macro_rules! test_char_range_inner {
    ($range:expr, $next_fn:ident) => {{
        let mut kiter = konst::iter::into_iter!($range);
        let mut iter = $range;

        while let Some(elem) = iter.$next_fn() {
            let kelem = kiter.$next_fn().unwrap();
            assert_eq!(kelem, elem);
        }
        assert!(kiter.$next_fn().is_none());
    }};
}
macro_rules! test_char_range {
    ($range:expr) => {{
        test_char_range_inner! {$range, next}
        test_char_range_inner! {$range, next_back}
    }};
}

#[cfg(not(miri))]
#[test]
fn test_char_ranges() {
    test_char_range! {char::MAX..=char::MAX}
    test_char_range! {char::MAX..char::MAX}
    test_char_range! {char::MAX..'\0'}
    test_char_range! {'\0'..char::MAX}
    test_char_range! {'\0'..=char::MAX}

    test_char_range! {'\0'..'\0'}
    test_char_range! {'\0'..='\0'}
    test_char_range! {'\u{10FFFF}'..='\u{10FFFE}'}
}
