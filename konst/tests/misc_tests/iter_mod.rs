use konst::iter;

mod collect_const_tests;

const fn is_rru8_even(n: &&u8) -> bool {
    is_u8_even(**n)
}

const fn is_ru8_even(n: &u8) -> bool {
    is_u8_even(*n)
}

const fn is_u8_even(n: u8) -> bool {
    n % 2 == 0
}

const fn add_usize(l: usize, r: usize) -> usize {
    l + r
}

#[test]
fn iterator_rev_and_flatten_test() {
    const fn rev_then_flatten(slice: &[[u8; 3]; 3]) -> [u8; 9] {
        let mut arr = [0; 9];

        iter::eval! {
            slice,
                rev(),
                flatten(),
                copied(),
                enumerate(),
                for_each(|(i, v)| arr[i] = v),
        }

        arr
    }
    const fn flatten_then_rev(slice: &[[u8; 3]; 3]) -> [u8; 9] {
        let mut arr = [0; 9];

        iter::eval! {
            slice,
                flatten(),
                rev(),
                copied(),
                enumerate(),
                for_each(|(i, v)| arr[i] = v),
        }

        arr
    }

    let input = &[[1, 3, 5], [9, 11, 13], [15, 17, 19]];
    assert_eq!(rev_then_flatten(input), [15, 17, 19, 9, 11, 13, 1, 3, 5]);
    assert_eq!(flatten_then_rev(input), [19, 17, 15, 13, 11, 9, 5, 3, 1]);
}

#[test]
fn iterator_rev_and_flat_map_test() {
    const fn rev_then_flat_map(slice: &[[u8; 3]; 3]) -> [u8; 9] {
        let mut arr = [0; 9];

        iter::eval! {
            slice,
                rev(),
                flat_map(|&[a, b, c]| &[100 + a, b, c]),
                copied(),
                enumerate(),
                for_each(|(i, v)| arr[i] = v),
        }

        arr
    }
    const fn flat_map_then_rev(slice: &[[u8; 3]; 3]) -> [u8; 9] {
        let mut arr = [0; 9];

        iter::eval! {
            slice,
                flat_map(|&[a, b, c]| &[100 + a, b, c]),
                rev(),
                copied(),
                enumerate(),
                for_each(|(i, v)| arr[i] = v),
        }

        arr
    }

    let input = &[[1, 3, 5], [9, 11, 13], [15, 17, 19]];
    assert_eq!(
        rev_then_flat_map(input),
        [115, 17, 19, 109, 11, 13, 101, 3, 5]
    );
    assert_eq!(
        flat_map_then_rev(input),
        [19, 17, 115, 13, 11, 109, 5, 3, 101]
    );
}

#[test]
fn iterator_all_test() {
    const fn all_fn(slice: &[u8]) -> bool {
        iter::eval!(slice, all(is_ru8_even))
    }

    const fn all_fn_breaking(slice: &[u8]) -> Option<bool> {
        Some(iter::eval!(
            slice,
            all(|&elem| elem % 2 == 0 && return None,)
        ))
    }

    assert!(all_fn(&[]));
    assert!(all_fn(&[0]));
    assert!(!all_fn(&[0, 1, 2]));
    assert!(all_fn(&[0, 2, 4]));

    assert_eq!(all_fn_breaking(&[]), Some(true));
    assert_eq!(all_fn_breaking(&[0]), None);
    assert_eq!(all_fn_breaking(&[1]), Some(false));
}

#[test]
fn iterator_any_test() {
    const fn any_fn(slice: &[u8]) -> bool {
        iter::eval!(slice, any(is_ru8_even))
    }

    const fn any_fn_breaking(slice: &[u8]) -> Option<bool> {
        Some(iter::eval!(
            slice,
            any(|&elem| match elem % 3 {
                0 => true,
                1 => false,
                _ => return None,
            })
        ))
    }

    assert!(!any_fn(&[]));
    assert!(!any_fn(&[1]));
    assert!(!any_fn(&[1, 3, 5]));
    assert!(any_fn(&[0]));
    assert!(any_fn(&[1, 2]));
    assert!(any_fn(&[0, 2, 4]));

    assert_eq!(any_fn_breaking(&[]), Some(false));
    assert_eq!(any_fn_breaking(&[1]), Some(false));
    assert_eq!(any_fn_breaking(&[1, 4]), Some(false));
    assert_eq!(any_fn_breaking(&[0]), Some(true));
    assert_eq!(any_fn_breaking(&[1, 0]), Some(true));
    assert_eq!(any_fn_breaking(&[0, 1, 2]), Some(true));
    assert_eq!(any_fn_breaking(&[2, 1, 0]), None);
    assert_eq!(any_fn_breaking(&[2, 1]), None);
    assert_eq!(any_fn_breaking(&[2]), None);
}

#[test]
fn count_tests() {
    // ensure that this macro is const-evaluable
    const COUNT: usize = iter::eval!(0..10, count());
    assert_eq!(COUNT, 10);

    for (range, count) in vec![(0..0, 0), (0..1, 1), (0..2, 2), (0..3, 3), (0..4, 4)] {
        assert_eq!(iter::eval!(range, count()), count);
    }

    for (slice, count) in vec![
        (&[0u8; 0] as &[_], 0),
        (&[0u8; 1], 1),
        (&[0u8; 2], 2),
        (&[0u8; 3], 3),
        (&[0u8; 4], 4),
    ] {
        assert_eq!(iter::eval!(slice, count()), count);
    }
}

#[test]
fn flat_map_count_test() {
    const fn range_f(n: &usize) -> std::ops::Range<usize> {
        let x10 = *n * 10;
        x10..x10 + 4
    }

    for (slice, count) in vec![(&[3usize] as &[_], 4), (&[3, 5], 8), (&[3, 5, 8], 12)] {
        assert_eq!(iter::eval!(slice, flat_map(range_f), count()), count);
    }
}

#[test]
fn flat_map_nth_test() {
    const fn range_f(n: &usize) -> std::ops::Range<usize> {
        let x10 = *n * 10;
        x10..x10 + 2
    }

    for &(i, v) in &[
        (0, Some(30)),
        (1, Some(31)),
        (2, Some(50)),
        (3, Some(51)),
        (4, None),
    ] {
        assert_eq!(iter::eval!(&[3, 5], flat_map(range_f), nth(i)), v);
    }
}

#[test]
fn flat_map_rposition_test() {
    const fn range_f(n: &usize) -> std::ops::Range<usize> {
        let x10 = *n * 10;
        x10..x10 + 2
    }

    for &(eq, v) in &[
        (51, Some(0)),
        (50, Some(1)),
        (31, Some(2)),
        (30, Some(3)),
        (0, None),
    ] {
        assert_eq!(
            iter::eval!(&[3, 5], flat_map(range_f), rposition(|e| e == eq)),
            v
        );

        assert_eq!(
            iter::eval!(&[3, 5], flat_map(range_f), rev(), position(|e| e == eq)),
            v
        );
    }
}

#[test]
fn find_tests() {
    const fn find_even(slice: &[u8]) -> Option<&u8> {
        iter::eval!(
            slice,
            find(|&&elem| match elem % 4 {
                1 => false,
                3 => return Some(&255),
                _ => true,
            })
        )
    }

    assert_eq!(find_even(&[]), None);
    assert_eq!(find_even(&[1]), None);
    assert_eq!(find_even(&[1, 5]), None);
    assert_eq!(find_even(&[1, 0]), Some(&0));
    assert_eq!(find_even(&[5, 1, 2]), Some(&2));
    assert_eq!(find_even(&[1, 3]), Some(&255));
    assert_eq!(find_even(&[1, 3, 2]), Some(&255));

    {
        const fn calls_const_fn(slice: &[u8]) -> Option<&u8> {
            iter::eval!(slice, find(is_rru8_even))
        }

        assert_eq!(calls_const_fn(&[]), None);
        assert_eq!(calls_const_fn(&[1]), None);
        assert_eq!(calls_const_fn(&[2]), Some(&2));
        assert_eq!(calls_const_fn(&[1, 2]), Some(&2));
        assert_eq!(calls_const_fn(&[4]), Some(&4));
        assert_eq!(calls_const_fn(&[1, 4, 2]), Some(&4));
    }
}

#[test]
fn rfind_tests() {
    const fn rfind_even(slice: &[u8]) -> Option<&u8> {
        iter::eval!(
            slice,
            rfind(|&&elem| match elem % 4 {
                1 => false,
                3 => return Some(&255),
                _ => true,
            })
        )
    }

    assert_eq!(rfind_even(&[]), None);
    assert_eq!(rfind_even(&[1]), None);
    assert_eq!(rfind_even(&[5, 1]), None);
    assert_eq!(rfind_even(&[0, 1]), Some(&0));
    assert_eq!(rfind_even(&[2, 1, 5]), Some(&2));
    assert_eq!(rfind_even(&[3, 1]), Some(&255));
    assert_eq!(rfind_even(&[2, 3, 1]), Some(&255));

    {
        const fn calls_const_fn(slice: &[u8]) -> Option<&u8> {
            iter::eval!(slice, rfind(is_rru8_even,))
        }

        assert_eq!(calls_const_fn(&[]), None);
        assert_eq!(calls_const_fn(&[1]), None);
        assert_eq!(calls_const_fn(&[2]), Some(&2));
        assert_eq!(calls_const_fn(&[2, 1]), Some(&2));
        assert_eq!(calls_const_fn(&[4]), Some(&4));
        assert_eq!(calls_const_fn(&[2, 4, 1]), Some(&4));
    }
}

#[test]
fn find_map_test() {
    const fn find_even(slice: &[u16]) -> Option<u16> {
        iter::eval!(
            slice,
            find_map(|&elem| match elem % 4 {
                1 => None,
                3 => return Some(u16::MAX),
                _ => Some(elem * 10),
            })
        )
    }

    assert_eq!(find_even(&[]), None);
    assert_eq!(find_even(&[1]), None);
    assert_eq!(find_even(&[1, 5]), None);
    assert_eq!(find_even(&[1, 0]), Some(0));
    assert_eq!(find_even(&[5, 1, 2]), Some(20));
    assert_eq!(find_even(&[1, 3]), Some(u16::MAX));
    assert_eq!(find_even(&[1, 3, 2]), Some(u16::MAX));

    {
        const fn calls_const_fn(slice: &[u16]) -> Option<u16> {
            const fn func(n: &u16) -> Option<u16> {
                if *n % 2 == 0 {
                    Some(*n * 10)
                } else {
                    None
                }
            }

            iter::eval!(slice, find_map(func))
        }

        assert_eq!(calls_const_fn(&[]), None);
        assert_eq!(calls_const_fn(&[1]), None);
        assert_eq!(calls_const_fn(&[2]), Some(20));
        assert_eq!(calls_const_fn(&[1, 2]), Some(20));
    }
}

#[test]
fn fold_test() {
    const fn shifter(range: &[u8]) -> u128 {
        iter::eval!(range, fold(0, |accum, &elem| (accum << 8) | (elem as u128)))
    }

    const fn sum_range(range: std::ops::Range<usize>) -> usize {
        iter::eval!(range, fold(0, add_usize))
    }

    const fn ret_on_0(slice: &[u8]) -> Option<u8> {
        Some(iter::eval!(
            slice,
            fold(0, |accum, &elem| if elem == 0 {
                return None;
            } else {
                accum + elem
            })
        ))
    }

    assert_eq!(shifter(&[]), 0);
    assert_eq!(shifter(&[1]), 0x01);
    assert_eq!(shifter(&[1, 2]), 0x0102);
    assert_eq!(shifter(&[1, 2, 3]), 0x010203);

    assert_eq!(sum_range(5..0), 0);
    assert_eq!(sum_range(0..0), 0);
    assert_eq!(sum_range(0..1), 0);
    assert_eq!(sum_range(0..2), 1);
    assert_eq!(sum_range(0..3), 3);
    assert_eq!(sum_range(0..4), 6);
    assert_eq!(sum_range(0..5), 10);

    assert_eq!(ret_on_0(&[]), Some(0));
    assert_eq!(ret_on_0(&[0]), None);
    assert_eq!(ret_on_0(&[2, 0]), None);
    assert_eq!(ret_on_0(&[1, 1]), Some(2));
    assert_eq!(ret_on_0(&[1, 2]), Some(3));
}

#[test]
fn rfold_test() {
    const fn shifter(range: &[u8]) -> u128 {
        const fn func(accum: u128, elem: &u8) -> u128 {
            (accum << 8) | (*elem as u128)
        }

        iter::eval!(range, rfold(0, func,))
    }

    const fn ret_on_0(slice: &[u8]) -> Option<u8> {
        Some(iter::eval!(
            slice,
            rfold(0, |accum, &elem| if elem == 0 {
                return None;
            } else {
                accum + elem
            })
        ))
    }

    assert_eq!(shifter(&[]), 0);
    assert_eq!(shifter(&[1]), 0x01);
    assert_eq!(shifter(&[1, 2]), 0x0201);
    assert_eq!(shifter(&[1, 2, 3]), 0x030201);

    assert_eq!(ret_on_0(&[]), Some(0));
    assert_eq!(ret_on_0(&[0]), None);
    assert_eq!(ret_on_0(&[0, 2]), None);
    assert_eq!(ret_on_0(&[1, 1]), Some(2));
    assert_eq!(ret_on_0(&[2, 1]), Some(3));
}

#[test]
fn for_each_test() {
    const fn sum_positives(slice: &[u64]) -> Option<u64> {
        let mut sum = 0u64;
        iter::for_each! {&elem in slice =>
            if elem == 0 {
                return None;
            } else {
                sum += elem;
            }
        }
        Some(sum)
    }

    assert_eq!(sum_positives(&[]), Some(0));
    assert_eq!(sum_positives(&[0]), None);
    assert_eq!(sum_positives(&[1, 0]), None);
    assert_eq!(sum_positives(&[1, 0, 2]), None);
    assert_eq!(sum_positives(&[1]), Some(1));
    assert_eq!(sum_positives(&[1, 2]), Some(3));
    assert_eq!(sum_positives(&[1, 2, 3]), Some(6));
    assert_eq!(sum_positives(&[1, 2, 3, 4]), Some(10));
}

#[test]
fn for_each_zip_test() {
    const fn enum_sum_positives(slice: &[u64]) -> Result<u64, usize> {
        let mut sum = 0u64;
        iter::for_each! {(i, &elem) in 0usize..,zip(slice) =>
            if elem == 0 {
                return Err(i);
            } else {
                sum += elem;
            }
        }
        Ok(sum)
    }

    fn trunc_iters(slice: &[char]) -> Vec<((usize, usize), &char)> {
        let mut vect = Vec::new();
        iter::for_each! {tup in 0..=3,zip(10..=16),zip(slice) =>
            vect.push(tup);
        }
        vect
    }

    assert_eq!(enum_sum_positives(&[]), Ok(0));
    assert_eq!(enum_sum_positives(&[0]), Err(0));
    assert_eq!(enum_sum_positives(&[1, 0]), Err(1));
    assert_eq!(enum_sum_positives(&[1, 0, 2]), Err(1));
    assert_eq!(enum_sum_positives(&[1, 2, 0]), Err(2));
    assert_eq!(enum_sum_positives(&[1]), Ok(1));
    assert_eq!(enum_sum_positives(&[1, 2]), Ok(3));
    assert_eq!(enum_sum_positives(&[1, 2, 3]), Ok(6));
    assert_eq!(enum_sum_positives(&[1, 2, 3, 4]), Ok(10));

    assert_eq!(trunc_iters(&[]), []);
    assert_eq!(trunc_iters(&['a']), [((0, 10), &'a')]);
    assert_eq!(trunc_iters(&['a', 'b']), [((0, 10), &'a'), ((1, 11), &'b')]);
    assert_eq!(
        trunc_iters(&['a', 'b', 'c']),
        [((0, 10), &'a'), ((1, 11), &'b'), ((2, 12), &'c')],
    );

    for slice in vec![
        &['a', 'b', 'c', 'd'][..],
        &['a', 'b', 'c', 'd', 'e'],
        &['a', 'b', 'c', 'd', 'e', 'f'],
    ] {
        assert_eq!(
            trunc_iters(slice),
            [
                ((0, 10), &'a'),
                ((1, 11), &'b'),
                ((2, 12), &'c'),
                ((3, 13), &'d')
            ],
        );
    }
}

#[test]
fn nth_test() {
    // ensure that this macro is const-evaluable
    const ELEM: Option<usize> = iter::eval!(0..4, nth(0));
    assert_eq!(ELEM, Some(0));

    for (range, nth, v) in vec![
        (0..4, 0, Some(0)),
        (0..4, 1, Some(1)),
        (0..4, 2, Some(2)),
        (0..4, 3, Some(3)),
        (0..4, 4, None),
        (0..4, 5, None),
    ] {
        assert_eq!(iter::eval!(range, nth(nth)), v);
    }

    for (slice, nth, v) in vec![
        (&[0, 1, 2, 3] as &[_], 0, Some(&0)),
        (&[0, 1, 2, 3], 1, Some(&1)),
        (&[0, 1, 2, 3], 2, Some(&2)),
        (&[0, 1, 2, 3], 3, Some(&3)),
        (&[0, 1, 2, 3], 4, None),
        (&[0, 1, 2, 3], 5, None),
    ] {
        assert_eq!(iter::eval!(slice, nth(nth,)), v);
    }
}

#[test]
fn position_tests() {
    const fn position_even(slice: &[u8]) -> Option<usize> {
        iter::eval!(
            slice,
            position(|&elem| match elem % 4 {
                1 => false,
                3 => return Some(usize::MAX),
                _ => true,
            })
        )
    }

    assert_eq!(position_even(&[]), None);
    assert_eq!(position_even(&[1]), None);
    assert_eq!(position_even(&[1, 5]), None);
    assert_eq!(position_even(&[0, 1]), Some(0));
    assert_eq!(position_even(&[1, 0]), Some(1));
    assert_eq!(position_even(&[5, 1, 10]), Some(2));
    assert_eq!(position_even(&[1, 3]), Some(usize::MAX));
    assert_eq!(position_even(&[1, 3, 2]), Some(usize::MAX));

    {
        const fn calls_const_fn(slice: &[u8]) -> Option<usize> {
            iter::eval!(slice, position(is_ru8_even))
        }

        assert_eq!(calls_const_fn(&[]), None);
        assert_eq!(calls_const_fn(&[1]), None);
        assert_eq!(calls_const_fn(&[2]), Some(0));
        assert_eq!(calls_const_fn(&[1, 2]), Some(1));
        assert_eq!(calls_const_fn(&[1, 3, 4]), Some(2));
    }
}

#[test]
fn rposition_tests() {
    const fn rposition_even(slice: &[u8]) -> Option<usize> {
        iter::eval!(
            slice,
            rposition(|&elem| match elem % 4 {
                1 => false,
                3 => return Some(usize::MAX),
                _ => true,
            })
        )
    }

    assert_eq!(rposition_even(&[]), None);
    assert_eq!(rposition_even(&[1]), None);
    assert_eq!(rposition_even(&[5, 1]), None);
    assert_eq!(rposition_even(&[1, 0]), Some(0));
    assert_eq!(rposition_even(&[0, 1]), Some(1));
    assert_eq!(rposition_even(&[10, 1, 5]), Some(2));
    assert_eq!(rposition_even(&[3, 1]), Some(usize::MAX));
    assert_eq!(rposition_even(&[2, 3, 1]), Some(usize::MAX));

    {
        const fn calls_const_fn(slice: &[u8]) -> Option<usize> {
            iter::eval!(slice, rposition(is_ru8_even,))
        }

        assert_eq!(calls_const_fn(&[]), None);
        assert_eq!(calls_const_fn(&[1]), None);
        assert_eq!(calls_const_fn(&[2]), Some(0));
        assert_eq!(calls_const_fn(&[2, 1]), Some(1));
        assert_eq!(calls_const_fn(&[4, 3, 1]), Some(2));
    }
}
