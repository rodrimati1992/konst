use konst::slice::{bytes_find, bytes_rfind};

macro_rules! slice_splitting_test {
    (
        $slice_up_to:ident,
        $split_at:ident,
        $slice_from:ident,
        [$($mut:tt)*]
    ) => {
        use konst::slice::{$slice_up_to, $split_at, $slice_from};

        let $($mut)* list = (0..=258).collect::<Vec<u32>>();
        let $($mut)* listb = list.clone();

        for &pow in [1usize, 8, 64, 256].iter() {
            let lengths = [pow.saturating_sub(2), pow - 1, pow, pow + 1, pow + 2];
            for &length in lengths.iter() {
                let sub = & $($mut)* list[..length];
                let sub_len = sub.len();
                let sub2 = & $($mut)* listb[..length];
                for i in 0..=sub_len {
                    assert_eq!($slice_up_to(sub, i), & $($mut)* sub2[..i]);

                    assert_eq!($slice_from(sub, i), & $($mut)* sub2[i..]);

                    let (left, right) = sub2.$split_at(i);
                    assert_eq!($split_at(sub, i), (left, right));
                }
                assert_eq!($slice_up_to(sub, sub_len + 1), sub2);
                assert_eq!($slice_up_to(sub, sub_len + 2), sub2);
                assert_eq!($slice_up_to(sub, sub_len + 3), sub2);

                assert_eq!($split_at(sub, sub_len + 1), (& $($mut)* *sub2, & $($mut)* [][..]));
                assert_eq!($split_at(sub, sub_len + 2), (& $($mut)* *sub2, & $($mut)* [][..]));
                assert_eq!($split_at(sub, sub_len + 3), (& $($mut)* *sub2, & $($mut)* [][..]));

                assert_eq!($slice_from(sub, sub_len + 1), & $($mut)* []);
                assert_eq!($slice_from(sub, sub_len + 2), & $($mut)* []);
                assert_eq!($slice_from(sub, sub_len + 3), & $($mut)* []);
            }
        }
    };
}

#[cfg(any(not(miri), feature = "constant_time_slice"))]
#[test]
fn slice_up_to_from_test() {
    slice_splitting_test! {
        slice_up_to,
        split_at,
        slice_from,
        []
    }
}

#[cfg(feature = "mut_refs")]
#[cfg(feature = "constant_time_slice")]
#[test]
fn slice_up_to_from_mut_test() {
    slice_splitting_test! {
        slice_up_to_mut,
        split_at_mut,
        slice_from_mut,
        [mut]
    }
}

macro_rules! range_tests {
    ($slice_range:ident, [$($mut:tt)*]) => {
            use konst::slice::$slice_range;

            let arr = &$($mut)*[3, 5, 8, 13, 21, 34, 55, 89];

            assert_eq!(*$slice_range(arr, 0, 7), [3, 5, 8, 13, 21, 34, 55]);
            assert_eq!(*$slice_range(arr, 0, 8), [3, 5, 8, 13, 21, 34, 55, 89]);
            assert_eq!(*$slice_range(arr, 0, 9), [3, 5, 8, 13, 21, 34, 55, 89]);
            assert_eq!(*$slice_range(arr, 0, 10), [3, 5, 8, 13, 21, 34, 55, 89]);
            assert_eq!(*$slice_range(arr, 0, usize::MAX), [3, 5, 8, 13, 21, 34, 55, 89]);


            assert_eq!(*$slice_range(arr, 3, 5), [13, 21]);
            assert_eq!(*$slice_range(arr, 3, 6), [13, 21, 34]);
            assert_eq!(*$slice_range(arr, 3, 7), [13, 21, 34, 55]);
            assert_eq!(*$slice_range(arr, 5, 3), []);

            assert_eq!(*$slice_range(arr, 1, usize::MAX), [5, 8, 13, 21, 34, 55, 89]);
            assert_eq!(*$slice_range(arr, 2, usize::MAX), [8, 13, 21, 34, 55, 89]);
            assert_eq!(*$slice_range(arr, 3, usize::MAX), [13, 21, 34, 55, 89]);
            assert_eq!(*$slice_range(arr, 4, usize::MAX), [21, 34, 55, 89]);
            assert_eq!(*$slice_range(arr, 5, usize::MAX), [34, 55, 89]);
            assert_eq!(*$slice_range(arr, 6, usize::MAX), [55, 89]);
            assert_eq!(*$slice_range(arr, 7, usize::MAX), [89]);
            assert_eq!(*$slice_range(arr, 8, usize::MAX), []);
            assert_eq!(*$slice_range(arr, 9, usize::MAX), []);
    };
}

#[test]
fn slice_range_test() {
    range_tests! {slice_range, []}
}

#[cfg(feature = "mut_refs")]
#[test]
fn slice_range_mut_test() {
    range_tests! {slice_range_mut, [mut]}
}

// This doesn't use unsafe
#[cfg(not(miri))]
#[test]
fn find_test() {
    #[track_caller]
    fn ass(
        left: &[u8],
        right: &[u8],
        finds: &[(usize, Option<usize>)],
        rfinds: &[(usize, Option<usize>)],
    ) {
        for (offset, opt) in finds.iter().copied() {
            assert_eq!(
                bytes_find(left, right, offset),
                opt,
                "in find, offset: {}",
                offset
            );

            if !right.is_empty() {
                assert_eq!(
                    bytes_find(b"", right, offset),
                    None,
                    "in find empty left, offset: {}",
                    offset
                );
            }
            if offset < left.len() {
                assert_eq!(
                    bytes_find(left, b"", offset),
                    Some(offset),
                    "in find empty right, offset: {}",
                    offset
                );
            }
        }
        for (offset, opt) in rfinds.iter().copied() {
            assert_eq!(
                bytes_rfind(left, right, offset),
                opt,
                "in rfind, offset: {}",
                offset
            );

            if !right.is_empty() {
                assert_eq!(
                    bytes_rfind(b"", right, offset),
                    None,
                    "in rfind empty left, offset: {}",
                    offset
                );
            }
            if offset < left.len() {
                assert_eq!(
                    bytes_rfind(left, b"", offset),
                    Some(offset),
                    "in rfind empty right, offset: {}",
                    offset
                );
            }
        }
    }

    ass(
        b"foo-bar-baz-foo---",
        b"foo",
        &[
            (0, Some(0)),
            (1, Some(12)),
            (4, Some(12)),
            (12, Some(12)),
            (13, None),
            (17, None),
            (18, None),
            (!0, None),
        ],
        &[
            (!0, Some(12)),
            (17, Some(12)),
            (15, Some(12)),
            (14, Some(12)),
            (13, Some(0)),
            (12, Some(0)),
            (3, Some(0)),
            (3, Some(0)),
            (2, Some(0)),
            (1, None),
            (0, None),
        ],
    );

    ass(
        b"foo-bar-baz-foo---",
        b"f",
        &[
            (0, Some(0)),
            (1, Some(12)),
            (4, Some(12)),
            (12, Some(12)),
            (13, None),
            (17, None),
            (18, None),
            (!0, None),
        ],
        &[
            (!0, Some(12)),
            (18, Some(12)),
            (17, Some(12)),
            (14, Some(12)),
            (13, Some(12)),
            (12, Some(12)),
            (11, Some(0)),
            (3, Some(0)),
            (3, Some(0)),
            (2, Some(0)),
            (0, Some(0)),
        ],
    );

    // Tests overlapping patterns
    ass(
        b"lawlawnawn--awn-lawlawn",
        b"lawn",
        &[
            (0, Some(3)),
            (1, Some(3)),
            (2, Some(3)),
            (3, Some(3)),
            (4, Some(19)),
            (16, Some(19)),
            (17, Some(19)),
            (18, Some(19)),
            (20, None),
            (22, None),
            (23, None),
            (!0, None),
        ],
        &[
            (!0, Some(19)),
            (23, Some(19)),
            (22, Some(19)),
            (20, Some(3)),
            (18, Some(3)),
            (17, Some(3)),
            (16, Some(3)),
            (8, Some(3)),
            (7, Some(3)),
            (6, Some(3)),
            (5, None),
            (4, None),
            (3, None),
            (2, None),
            (1, None),
            (0, None),
        ],
    );
}
