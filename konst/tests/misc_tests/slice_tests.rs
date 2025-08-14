use konst::slice::try_into_array;

#[cfg(feature = "cmp")]
use konst::{
    cmp::{const_cmp, const_eq},
    slice::{
        self,
        cmp::{self as slice_cmp, cmp_slice_bytes, eq_slice_bytes},
    },
};

#[cfg(feature = "cmp")]
use konst::slice::cmp::{cmp_option_slice_bytes, eq_option_slice_bytes};

mod bytes_fns_tests;

mod slice_concatenation_tests;

#[cfg(feature = "iter")]
mod slice_iter_copied;

#[cfg(feature = "iter")]
mod non_iter_slice_iterators;

#[test]
#[cfg(feature = "cmp")]
fn eq_slice_test() {
    macro_rules! cases {
        ($(($ty:ident $eq:ident $eq_opt:ident))*) => ($({
            assertc_opt_eq_rets! {
                &[$ty], slice_cmp::$eq, slice_cmp::$eq_opt =>

                (&[], &[], true)
                (&[], &[0], false)
                (&[0], &[], false)
                (&[0], &[0], true)
                (&[0], &[1], false)
                (&[1], &[0], false)
                (&[0], &[0, 1], false)
                (&[0, 1], &[0], false)
                (&[0, 1], &[0, 1], true)
                (&[0, 1], &[0, 2], false)
            }
        })*)
    }

    cases! {
        (u8 eq_slice_u8 eq_option_slice_u8)
        (u16 eq_slice_u16 eq_option_slice_u16)
        (u32 eq_slice_u32 eq_option_slice_u32)
        (u64 eq_slice_u64 eq_option_slice_u64)
        (u128 eq_slice_u128 eq_option_slice_u128)
        (usize eq_slice_usize eq_option_slice_usize)
        (i8 eq_slice_i8 eq_option_slice_i8)
        (i16 eq_slice_i16 eq_option_slice_i16)
        (i32 eq_slice_i32 eq_option_slice_i32)
        (i64 eq_slice_i64 eq_option_slice_i64)
        (i128 eq_slice_i128 eq_option_slice_i128)
        (isize eq_slice_isize eq_option_slice_isize)
    }
}

#[test]
#[cfg(feature = "cmp")]
fn cmp_slice_test() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    macro_rules! cases {
        ($(($ty:ident $eq:ident $eq_opt:ident))*) => ($({
            assertc_opt_cmp! {
                &[$ty], slice_cmp::$eq, slice_cmp::$eq_opt =>
                (&[], &[], Equal)
                (&[], &[0], Less)
                (&[0], &[], Greater)
                (&[0], &[0], Equal)
                (&[0], &[1], Less)
                (&[0], &[0, 1], Less)
                (&[0, 1], &[0, 1], Equal)
                (&[0, 1], &[0, 2], Less)
            }
        })*)
    }

    cases! {
        (u8 cmp_slice_u8 cmp_option_slice_u8)
        (u16 cmp_slice_u16 cmp_option_slice_u16)
        (u32 cmp_slice_u32 cmp_option_slice_u32)
        (u64 cmp_slice_u64 cmp_option_slice_u64)
        (u128 cmp_slice_u128 cmp_option_slice_u128)
        (usize cmp_slice_usize cmp_option_slice_usize)
        (i8 cmp_slice_i8 cmp_option_slice_i8)
        (i16 cmp_slice_i16 cmp_option_slice_i16)
        (i32 cmp_slice_i32 cmp_option_slice_i32)
        (i64 cmp_slice_i64 cmp_option_slice_i64)
        (i128 cmp_slice_i128 cmp_option_slice_i128)
        (isize cmp_slice_isize cmp_option_slice_isize)
    }
}

#[test]
#[cfg(feature = "cmp")]
fn cmp_slice_char_test() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    assertc_opt_eq_rets! {
        &[char], slice_cmp::eq_slice_char, slice_cmp::eq_option_slice_char =>
        (&[], &[], true)
        (&[], &['0'], false)
        (&['0'], &[], false)
        (&['0'], &['0'], true)
        (&['0'], &['1'], false)
        (&['0'], &['0', '1'], false)
        (&['0', '1'], &['0', '1'], true)
        (&['0', '1'], &['0', '2'], false)
    }

    assertc_opt_cmp! {
        &[char], slice_cmp::cmp_slice_char, slice_cmp::cmp_option_slice_char =>
        (&[], &[], Equal)
        (&[], &['0'], Less)
        (&['0'], &[], Greater)
        (&['0'], &['0'], Equal)
        (&['0'], &['1'], Less)
        (&['0'], &['0', '1'], Less)
        (&['0', '1'], &['0', '1'], Equal)
        (&['0', '1'], &['0', '2'], Less)
    }
}

#[test]
#[cfg(feature = "cmp")]
fn cmp_slice_bool_test() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    assertc_opt_eq_rets! {
        &[bool], slice_cmp::eq_slice_bool, slice_cmp::eq_option_slice_bool =>
        (&[], &[], true)
        (&[], &[false], false)
        (&[false], &[], false)
        (&[false], &[false], true)
        (&[false], &[true], false)
        (&[false], &[false, true], false)
        (&[false, true], &[false, true], true)
    }

    assertc_opt_cmp! {
        &[bool], slice_cmp::cmp_slice_bool, slice_cmp::cmp_option_slice_bool =>
        (&[], &[], Equal)
        (&[], &[false], Less)
        (&[false], &[], Greater)
        (&[false], &[false], Equal)
        (&[false], &[true], Less)
        (&[false], &[false, true], Less)
        (&[false, true], &[false, true], Equal)
        (&[false, false], &[false, true], Less)
    }
}

#[test]
#[cfg(feature = "cmp")]
fn slice_of_bytes_eq_test() {
    assertc_opt_eq_rets! {
        &[&[u8]], eq_slice_bytes, eq_option_slice_bytes =>
        (&[], &[], true)
        (&[], &[&[0]], false)
        (&[&[0]], &[], false)
        (&[&[0]], &[&[0]], true)
        (&[&[0]], &[&[1]], false)
        (&[&[1]], &[&[0]], false)

        (&[&[0]], &[&[0, 1]], false)
        (&[&[0, 1]], &[&[0]], false)
        (&[&[0, 1]], &[&[0, 1]], true)
        (&[&[0, 1]], &[&[0, 2]], false)

        (&[&[0], &[1]], &[&[0], &[1]], true)
        (&[&[0], &[1]], &[&[0], &[1, 2]], false)
        (&[&[0], &[1, 2]], &[&[0], &[1]], false)
        (&[&[0], &[1, 2]], &[&[0], &[1, 2]], true)
        (&[&[0], &[1, 2]], &[&[0], &[1, 3]], false)
    }
}

#[test]
#[cfg(feature = "cmp")]
fn slice_of_bytes_cmp_test() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    assertc_opt_cmp! {
        &[&[u8]], cmp_slice_bytes, cmp_option_slice_bytes =>

        (&[], &[], Equal)
        (&[], &[&[0]], Less)
        (&[&[0]], &[], Greater)
        (&[&[0]], &[&[0]], Equal)
        (&[&[0]], &[&[1]], Less)
        (&[&[1]], &[&[0]], Greater)

        (&[&[0]], &[&[0, 1]], Less)
        (&[&[0, 1]], &[&[0]], Greater)
        (&[&[0, 1]], &[&[0, 1]], Equal)
        (&[&[0, 1]], &[&[0, 2]], Less)

        (&[&[0], &[1]], &[&[0], &[1]], Equal)
        (&[&[1], &[1]], &[&[0], &[1]], Greater)
        (&[&[0], &[1]], &[&[0], &[1, 2]], Less)
        (&[&[0], &[1, 2]], &[&[0], &[1]], Greater)
        (&[&[0], &[1, 2]], &[&[0], &[1, 2]], Equal)
        (&[&[0], &[1, 2]], &[&[0], &[1, 3]], Less)
    }
}

#[test]
#[cfg(feature = "cmp")]
fn slice_of_str_eq_test() {
    assertc_opt_eq_rets! {
        &[&str], slice_cmp::eq_slice_str, slice_cmp::eq_option_slice_str =>
        (&[], &[], true)
        (&[], &["0"], false)
        (&["0"], &[], false)
        (&["0"], &["0"], true)
        (&["0"], &["1"], false)
        (&["1"], &["0"], false)

        (&["0"], &["01"], false)
        (&["01"], &["0"], false)
        (&["01"], &["01"], true)
        (&["01"], &["02"], false)

        (&["0", "1"], &["0", "1"], true)
        (&["0", "1"], &["0", "12"], false)
        (&["0", "12"], &["0", "1"], false)
        (&["0", "12"], &["0", "12"], true)
        (&["0", "12"], &["0", "13"], false)
    }
}

#[test]
#[cfg(feature = "cmp")]
fn slice_of_str_cmp_test() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    assertc_opt_cmp! {
        &[&str], slice_cmp::cmp_slice_str, slice_cmp::cmp_option_slice_str =>

        (&[], &[], Equal)
        (&[], &["0"], Less)
        (&["0"], &[], Greater)
        (&["0"], &["0"], Equal)
        (&["0"], &["1"], Less)
        (&["1"], &["0"], Greater)

        (&["0"], &["01"], Less)
        (&["01"], &["0"], Greater)
        (&["01"], &["01"], Equal)
        (&["01"], &["02"], Less)

        (&["0", "1"], &["0", "1"], Equal)
        (&["1", "1"], &["0", "1"], Greater)
        (&["0", "1"], &["0", "12"], Less)
        (&["0", "12"], &["0", "1"], Greater)
        (&["0", "12"], &["0", "12"], Equal)
        (&["0", "12"], &["0", "13"], Less)
    }
}

#[test]
fn try_into_array_fn_test() {
    let slice = &[0, 2, 3, 4][..];

    {
        assert!(try_into_array::<_, 0>(slice).is_err());
        assert!(try_into_array::<_, 1>(slice).is_err());
        assert!(try_into_array::<_, 2>(slice).is_err());
        assert!(try_into_array::<_, 3>(slice).is_err());
        assert_eq!(try_into_array::<_, 4>(slice), Ok(&[0, 2, 3, 4]));
        assert!(try_into_array::<_, 5>(slice).is_err());
        assert!(try_into_array::<_, 6>(slice).is_err());
    }

    macro_rules! err_len {
        ($len:expr) => {{
            let res: Result<&[_; $len], _> = try_into_array(slice);
            assert!(res.is_err());
        }};
    }

    err_len! {0}
    err_len! {1}
    err_len! {2}
    err_len! {3}
    {
        let res: Result<&[_; 4], _> = try_into_array(slice);
        assert_eq!(res, Ok(&[0, 2, 3, 4]));
    }
    err_len! {5}
}

#[test]
fn try_into_array_mut_test() {
    use konst::slice::try_into_array_mut;

    let mut slice = [0, 2, 3, 4];

    assert!(try_into_array_mut::<_, 0>(&mut slice).is_err());
    assert!(try_into_array_mut::<_, 1>(&mut slice).is_err());
    assert!(try_into_array_mut::<_, 2>(&mut slice).is_err());
    assert!(try_into_array_mut::<_, 3>(&mut slice).is_err());

    assert_eq!(
        try_into_array_mut::<_, 0>(&mut slice[..0]),
        Ok(&mut [0i32; 0])
    );

    macro_rules! assert_around {
        ($prev:expr, $len:expr, $after:expr, $expected:expr) => {
            assert!(try_into_array_mut::<_, $prev>(&mut slice[..$len]).is_err());
            assert_eq!(
                try_into_array_mut::<_, $len>(&mut slice[..$len]),
                Ok(&mut $expected)
            );
            assert!(try_into_array_mut::<_, $after>(&mut slice[..$len]).is_err());
        };
    }

    assert_around! {0, 1, 2, [0]}
    assert_around! {1, 2, 3, [0, 2]}
    assert_around! {2, 3, 4, [0, 2, 3]}
    assert_around! {3, 4, 5, [0, 2, 3, 4]}

    assert!(try_into_array_mut::<_, 5>(&mut slice).is_err());
    assert!(try_into_array_mut::<_, 6>(&mut slice).is_err());
}

#[test]
fn try_into_array_fmt_test() {
    type Buff = const_panic::ArrayString<256>;

    let err = try_into_array::<_, 0>(&[1]).unwrap_err();

    macro_rules! fmt_case {
        ($fmtarg:ident, $fmtstring:literal) => {{
            assert_eq!(
                Buff::from_panicvals(&err.to_panicvals(const_panic::FmtArg::$fmtarg)).unwrap(),
                *format!(concat!("{:", $fmtstring, "}"), err),
            );
        }};
    }

    fmt_case! {DEBUG, "?"}
    fmt_case! {ALT_DEBUG, "#?"}
    fmt_case! {DISPLAY, ""}
    fmt_case! {ALT_DISPLAY, "#"}
}

#[cfg(feature = "iter")]
#[test]
fn slice_iter_const_callable() {
    const fn __<'a>(slice: &'a [u8]) {
        let _: konst::slice::Iter<'a, u8> = konst::slice::iter(slice);
        konst::slice::iter(slice).next();
        konst::slice::iter(slice).next_back();
        konst::slice::iter(slice).copy();

        let rev: konst::slice::IterRev<'a, u8> = konst::slice::iter(slice).rev();

        rev.copy();
        let _: konst::slice::Iter<'a, u8> = rev.copy().rev();
        rev.copy().next();
        rev.copy().next_back();
    }
}

#[cfg(feature = "iter")]
#[test]
fn slice_iter_both_directions() {
    let slice: &[u8] = &[3, 5, 8, 13, 21];
    let slice_refs: Vec<&u8> = slice.iter().collect();

    assert_eq!(collect_const_iter!(slice), slice_refs);
    for iter in vec![
        konst::slice::iter(slice),
        konst::slice::iter(slice).copy(),
        konst::slice::iter(slice).rev().rev(),
    ] {
        assert_eq!(collect_const_iter!(iter), slice_refs);
    }

    for iter in vec![
        konst::slice::iter(slice).rev(),
        konst::slice::iter(slice).copy().rev(),
    ] {
        assert_eq!(
            collect_const_iter!(iter),
            slice.iter().rev().collect::<Vec<&u8>>(),
        );
    }
}

#[cfg(feature = "iter")]
#[test]
fn slice_iter_mixed_directions() {
    let slice: &[u8] = &[3, 5, 8, 13, 21];

    let mut iter = konst::iter::into_iter!(slice);
    assert_eq!(iter.as_slice(), [3, 5, 8, 13, 21]);

    assert_eq!(*iter.next_back().unwrap(), 21);
    assert_eq!(iter.as_slice(), [3, 5, 8, 13]);

    assert_eq!(*iter.next().unwrap(), 3);
    assert_eq!(iter.as_slice(), [5, 8, 13]);

    assert_eq!(*iter.next().unwrap(), 5);
    assert_eq!(iter.as_slice(), [8, 13]);

    assert_eq!(*iter.next().unwrap(), 8);
    assert_eq!(iter.as_slice(), [13]);

    assert_eq!(*iter.next_back().unwrap(), 13);
    assert_eq!(iter.as_slice(), [0u8; 0]);

    assert!(iter.next().is_none());
}

#[cfg(feature = "iter")]
#[test]
fn slice_iter_rev() {
    let slice: &[u8] = &[3, 5, 8, 13, 21];
    let iter = konst::iter::into_iter!(slice);

    let mut iter = iter.rev();
    assert_eq!(*iter.next().unwrap(), 21);

    // making sure to call next_back on the reversed iterator
    assert_eq!(*iter.next_back().unwrap(), 3);

    let mut iter = iter.rev();
    assert_eq!(*iter.next().unwrap(), 5);

    assert_eq!(*iter.next().unwrap(), 8);

    {
        let mut iter = iter.copy();
        let elem = iter.next().unwrap();
        assert_eq!(*elem, 13);
        assert!(iter.copy().next_back().is_none());
        assert!(iter.next().is_none());
    }
    {
        let elem = iter.next_back().unwrap();
        assert_eq!(*elem, 13);
        assert!(iter.copy().next().is_none());
        assert!(iter.next_back().is_none());
    }
}

#[cfg(feature = "iter")]
#[test]
fn slice_iter_mut_const_callable() {
    const fn __<'a>(slice: &'a mut [u8]) {
        let _: konst::slice::IterMut<'_, u8> = konst::slice::iter_mut(slice);
        konst::slice::iter_mut(slice).next();
        konst::slice::iter_mut(slice).next_back();

        let _: konst::slice::IterMut<'_, u8> = konst::slice::iter_mut(slice).rev().rev();

        let mut rev: konst::slice::IterMutRev<'_, u8> = konst::slice::iter_mut(slice).rev();
        rev.next();
        rev.next_back();
        rev.as_slice();
        rev.as_mut_slice();

        // ensure that the lifetime is the same when the slice isn't reborrowed
        let _: konst::slice::IterMut<'a, u8> = konst::slice::iter_mut(slice);
    }
}

#[cfg(feature = "iter")]
#[test]
fn slice_iter_mut_both_directions() {
    let slice: &mut [u8] = &mut [3, 5, 8, 13, 21];
    let slice_b: &mut [u8] = &mut [3, 5, 8, 13, 21];
    let slice_c: &mut [u8] = &mut [3, 5, 8, 13, 21];

    let slice_refs: Vec<&mut u8> = slice_b.iter_mut().collect();

    assert_eq!(collect_const_iter!(&mut *slice), slice_refs);

    assert_eq!(
        collect_const_iter!(konst::slice::iter_mut(slice)),
        slice_refs
    );

    assert_eq!(
        collect_const_iter!(konst::slice::iter_mut(slice).rev().rev()),
        slice_refs
    );

    assert_eq!(
        collect_const_iter!(konst::slice::iter_mut(slice).rev()),
        slice_c.iter_mut().rev().collect::<Vec<&mut u8>>(),
    );
}

#[cfg(feature = "iter")]
#[test]
fn slice_iter_mut_mixed_directions() {
    let slice: &mut [u8] = &mut [3, 5, 8, 13, 21];

    let mut iter = konst::iter::into_iter!(slice);
    assert_eq!(iter.as_slice(), [3, 5, 8, 13, 21]);

    assert_eq!(*iter.next_back().unwrap(), 21);
    assert_eq!(iter.as_slice(), [3, 5, 8, 13]);

    assert_eq!(*iter.next().unwrap(), 3);
    assert_eq!(iter.as_slice(), [5, 8, 13]);

    assert_eq!(*iter.next().unwrap(), 5);
    assert_eq!(iter.as_slice(), [8, 13]);

    assert_eq!(*iter.next().unwrap(), 8);
    assert_eq!(iter.as_slice(), [13]);

    assert_eq!(*iter.next_back().unwrap(), 13);
    assert_eq!(iter.as_slice(), [0u8; 0]);

    assert!(iter.next().is_none());
}

#[cfg(feature = "iter")]
#[test]
fn slice_iter_mut_rev() {
    let slice: &mut [u8] = &mut [3, 5, 8, 13, 21];
    let iter = konst::iter::into_iter!(slice);

    let mut iter = iter.rev();
    assert_eq!(*iter.next().unwrap(), 21);

    // making sure to call next_back on the reversed iterator
    assert_eq!(*iter.next_back().unwrap(), 3);

    let mut iter = iter.rev();
    assert_eq!(*iter.next().unwrap(), 5);

    assert_eq!(*iter.next().unwrap(), 8);

    assert_eq!(*iter.next_back().unwrap(), 13);
    assert!(iter.next_back().is_none());
}

#[test]
fn fill_test() {
    const fn filler<const N: usize>(val: u8) -> [u8; N] {
        let mut array = [0; N];
        slice::fill(&mut array, val);
        array
    }

    assert_eq!(filler::<0>(5).as_slice(), [5u8; 0].as_slice());
    assert_eq!(filler::<1>(8).as_slice(), [8u8; 1].as_slice());
    assert_eq!(filler::<2>(13).as_slice(), [13u8; 2].as_slice());
    assert_eq!(filler::<3>(21).as_slice(), [21u8; 3].as_slice());
}

#[test]
fn fill_with_closure_test() {
    const fn filler<const N: usize>() -> [u8; N] {
        let mut array = [0; N];
        let mut i = 0u8;

        slice::fill_with!(&mut array, || {
            i += 1;
            i.pow(2)
        });

        array
    }

    assert_eq!(filler::<0>().as_slice(), [0u8; 0].as_slice());
    assert_eq!(filler::<1>().as_slice(), [1].as_slice());
    assert_eq!(filler::<2>().as_slice(), [1, 4].as_slice());
    assert_eq!(filler::<3>().as_slice(), [1, 4, 9].as_slice());
}

#[test]
fn fill_with_func_test() {
    #[derive(Debug, PartialEq)]
    struct NonCopy(u8);

    const fn filler<const N: usize>() -> [NonCopy; N] {
        const fn returner<const N: usize>() -> NonCopy {
            NonCopy(N as u8 + 5)
        }

        let mut array = [const { NonCopy(0) }; N];
        slice::fill_with!(&mut array, returner::<N>);
        array
    }

    assert_eq!(filler::<0>().as_slice(), [].map(NonCopy).as_slice());
    assert_eq!(filler::<1>().as_slice(), [6].map(NonCopy).as_slice());
    assert_eq!(filler::<2>().as_slice(), [7, 7].map(NonCopy).as_slice());
    assert_eq!(filler::<3>().as_slice(), [8, 8, 8].map(NonCopy).as_slice());
}
