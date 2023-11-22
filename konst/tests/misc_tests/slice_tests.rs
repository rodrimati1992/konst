use konst::{cmp_str, eq_str, slice::try_into_array};

#[cfg(feature = "cmp")]
use konst::{
    const_cmp, const_eq,
    slice::cmp::{cmp_slice_bytes, cmp_slice_u8, eq_slice_bytes, eq_slice_u8},
};

#[cfg(feature = "cmp")]
use konst::{
    cmp_option_str, eq_option_str,
    slice::cmp::{
        cmp_option_slice_bytes, cmp_option_slice_u8, eq_option_slice_bytes, eq_option_slice_u8,
    },
};

mod array_chunks_tests;

mod bytes_fns_tests;

mod slice_concatenation_tests;

#[cfg(feature = "iter")]
mod slice_iter_copied;

#[cfg(feature = "iter")]
mod non_iter_slice_iterators;

#[test]
#[cfg(feature = "cmp")]
fn eq_slice_test() {
    assertc_opt_eq_rets! {
        &[u8], eq_slice_u8, eq_option_slice_u8 =>

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
fn eq_str_test() {
    assertc_opt_eq_rets! {
        &str, eq_str, eq_option_str =>
        ("", "", true)
        ("", "0", false)
        ("0", "", false)
        ("0", "0", true)
        ("0", "1", false)
        ("1", "0", false)
        ("0", "0, 1", false)
        ("0, 1", "0", false)
        ("0, 1", "1", false)
        ("0, 1", "0, 1", true)
        ("0, 1", "0, 2", false)
    }
}

#[test]
#[cfg(feature = "cmp")]
fn cmp_slice_test() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    assertc_opt_cmp! {
        &[u8], cmp_slice_u8, cmp_option_slice_u8 =>
        (&[], &[], Equal)
        (&[], &[0], Less)
        (&[0], &[], Greater)
        (&[0], &[0], Equal)
        (&[0], &[1], Less)
        (&[0], &[0, 1], Less)
        (&[0, 1], &[0, 1], Equal)
        (&[0, 1], &[0, 2], Less)
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
fn cmp_str_test() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    assertc_opt_cmp! {
        &str, cmp_str, cmp_option_str =>
        ("0", "", Greater)
        ("0", "1", Less)
        ("0", "01", Less)
        ("1", "01", Greater)
        ("099999", "12", Less)
        ("111111", "12", Less)
        ("120", "12", Greater)
        ("199999", "12", Greater)
        ("299999", "12", Greater)
        ("01", "02", Less)
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
#[cfg(feature = "mut_refs")]
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

    let iter = konst::iter::into_iter!(slice);
    assert_eq!(iter.as_slice(), [3, 5, 8, 13, 21]);

    let (elem, iter) = iter.next_back().unwrap();
    assert_eq!(*elem, 21);
    assert_eq!(iter.as_slice(), [3, 5, 8, 13]);

    let (elem, iter) = iter.next().unwrap();
    assert_eq!(*elem, 3);
    assert_eq!(iter.as_slice(), [5, 8, 13]);

    let (elem, iter) = iter.next().unwrap();
    assert_eq!(*elem, 5);
    assert_eq!(iter.as_slice(), [8, 13]);

    let (elem, iter) = iter.next().unwrap();
    assert_eq!(*elem, 8);
    assert_eq!(iter.as_slice(), [13]);

    let (elem, iter) = iter.next_back().unwrap();
    assert_eq!(*elem, 13);
    assert_eq!(iter.as_slice(), [0u8; 0]);

    assert!(iter.next().is_none());
}

#[cfg(feature = "iter")]
#[test]
fn slice_iter_rev() {
    let slice: &[u8] = &[3, 5, 8, 13, 21];
    let iter = konst::iter::into_iter!(slice);

    let (elem, iter) = iter.rev().next().unwrap();
    assert_eq!(*elem, 21);

    // making sure to call next_back on the reversed iterator
    let (elem, iter) = iter.next_back().unwrap();
    assert_eq!(*elem, 3);

    let (elem, iter) = iter.rev().next().unwrap();
    assert_eq!(*elem, 5);

    let (elem, iter) = iter.next().unwrap();
    assert_eq!(*elem, 8);

    {
        let (elem, iter) = iter.copy().next().unwrap();
        assert_eq!(*elem, 13);
        assert!(iter.copy().next_back().is_none());
        assert!(iter.next().is_none());
    }
    {
        let (elem, iter) = iter.next_back().unwrap();
        assert_eq!(*elem, 13);
        assert!(iter.copy().next().is_none());
        assert!(iter.next_back().is_none());
    }
}
