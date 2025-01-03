use konst::slice;

macro_rules! slice_splitting_test {
    (
        $slice_up_to:ident,
        $split_at:ident,
        $slice_from:ident,
        [$($mut:tt)*]
    ) => {
        use konst::slice::{$slice_up_to, $split_at, $slice_from};

        let $($mut)* list = (0..=258).collect::<Vec<u16>>();
        let $($mut)* listb = list.clone();

        for &pow in [1usize, 8, 64, 128].iter() {
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

                assert_eq!($split_at(sub, sub_len + 1), (& $($mut)* *sub2, & $($mut)* [0u16; 0][..]));
                assert_eq!($split_at(sub, sub_len + 2), (& $($mut)* *sub2, & $($mut)* [0u16; 0][..]));
                assert_eq!($split_at(sub, sub_len + 3), (& $($mut)* *sub2, & $($mut)* [0u16; 0][..]));

                assert_eq!($slice_from(sub, sub_len + 1), & $($mut)* [0u16; 0]);
                assert_eq!($slice_from(sub, sub_len + 2), & $($mut)* [0u16; 0]);
                assert_eq!($slice_from(sub, sub_len + 3), & $($mut)* [0u16; 0]);
            }
        }
    };
}

#[test]
fn slice_up_to_from_test() {
    slice_splitting_test! {
        slice_up_to,
        split_at,
        slice_from,
        []
    }
}

#[cfg(feature = "rust_1_83")]
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
            assert_eq!(*$slice_range(arr, 5, 3), [0usize; 0]);

            assert_eq!(*$slice_range(arr, 1, usize::MAX), [5, 8, 13, 21, 34, 55, 89]);
            assert_eq!(*$slice_range(arr, 2, usize::MAX), [8, 13, 21, 34, 55, 89]);
            assert_eq!(*$slice_range(arr, 3, usize::MAX), [13, 21, 34, 55, 89]);
            assert_eq!(*$slice_range(arr, 4, usize::MAX), [21, 34, 55, 89]);
            assert_eq!(*$slice_range(arr, 5, usize::MAX), [34, 55, 89]);
            assert_eq!(*$slice_range(arr, 6, usize::MAX), [55, 89]);
            assert_eq!(*$slice_range(arr, 7, usize::MAX), [89]);
            assert_eq!(*$slice_range(arr, 8, usize::MAX), [0usize; 0]);
            assert_eq!(*$slice_range(arr, 9, usize::MAX), [0usize; 0]);
    };
}

#[test]
fn slice_range_test() {
    range_tests! {slice_range, []}
}

#[cfg(feature = "rust_1_83")]
#[test]
fn slice_range_mut_test() {
    range_tests! {slice_range_mut, [mut]}
}

#[test]
fn slice_single_elem_get_test() {
    #[allow(unused_mut)]
    let mut arr = [3, 5, 8, 13, 21, 34, 55, 89];

    for i in (0..10).chain([!0 - 1, !0].iter().copied()) {
        assert_eq!(slice::get(&arr, i), arr.get(i));

        #[cfg(feature = "rust_1_83")]
        {
            let mut clone = arr;
            assert_eq!(slice::get_mut(&mut arr, i), clone.get_mut(i));
        }
    }
}

#[test]
fn slice_ranged_get_test() {
    #[allow(unused_mut)]
    let mut arr = [3, 5, 8, 13, 21, 34, 55, 89];
    #[allow(unused_mut, unused_variables)]
    let mut clone = arr;
    let len = arr.len();

    let indices = [0, 1, 2, 3, 4, 5, 6, 7, 8, !0 - 1, !0];

    for x in indices.iter().copied() {
        assert_eq!(slice::slice_from(&arr, x), &arr[x.min(len)..]);
        assert_eq!(slice::slice_up_to(&arr, x), &arr[..x.min(len)]);
        assert_eq!(slice::get_from(&arr, x), arr.get(x..));
        assert_eq!(slice::get_up_to(&arr, x), arr.get(..x));

        #[cfg(feature = "rust_1_83")]
        {
            assert_eq!(slice::slice_from_mut(&mut arr, x), &mut clone[x.min(len)..]);
            assert_eq!(
                slice::slice_up_to_mut(&mut arr, x),
                &mut clone[..x.min(len)]
            );
            assert_eq!(slice::get_from_mut(&mut arr, x), clone.get_mut(x..));
            assert_eq!(slice::get_up_to_mut(&mut arr, x), clone.get_mut(..x));
        }

        for end in indices.iter().copied() {
            {
                let tmp = slice::slice_range(&arr, x, end);
                if x <= end {
                    assert_eq!(tmp, &arr[x.min(len)..end.min(len)]);
                } else {
                    assert_eq!(tmp, &[0usize; 0][..]);
                }
            }
            assert_eq!(slice::get_range(&arr, x, end), arr.get(x..end));

            #[cfg(feature = "rust_1_83")]
            {
                let tmp = slice::slice_range_mut(&mut arr, x, end);
                if x <= end {
                    assert_eq!(tmp, &mut clone[x.min(len)..end.min(len)]);
                } else {
                    assert_eq!(tmp, &mut [0usize; 0][..]);
                }
                assert_eq!(
                    slice::get_range_mut(&mut arr, x, end),
                    clone.get_mut(x..end)
                );
            }
        }
    }
}
