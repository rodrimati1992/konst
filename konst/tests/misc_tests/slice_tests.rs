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
fn try_into_array_macro_explicit_test() {
    let slice = &[0, 2, 3, 4][..];

    assert!(try_into_array!(slice, 0).is_err());
    assert!(try_into_array!(slice, 1).is_err());
    assert!(try_into_array!(slice, 2).is_err());
    assert!(try_into_array!(slice, 3).is_err());
    assert_eq!(try_into_array!(slice, 4), Ok(&[0, 2, 3, 4]));
    assert!(try_into_array!(slice, 5).is_err());
    assert!(try_into_array!(slice, 6).is_err());
}

#[test]
#[cfg(feature = "const_generics")]
fn try_into_array_macro_implicit_test() {
    let slice = &[0, 2, 3, 4][..];

    macro_rules! try_into_infer_err {
        ($slice:expr, $len:expr) => {{
            let arr: Result<&[_; $len], _> = try_into_array!(slice);
            assert!(arr.is_err());
        }};
    }

    try_into_infer_err! {slice, 0}
    try_into_infer_err! {slice, 1}
    try_into_infer_err! {slice, 2}
    try_into_infer_err! {slice, 3}

    let arr_4: Result<&[_; 4], _> = try_into_array!(slice,);
    assert_eq!(arr_4, Ok(&[0, 2, 3, 4]));

    try_into_infer_err! {slice, 5}
    try_into_infer_err! {slice, 6}
}

#[test]
#[cfg(feature = "deref_raw_in_fn")]
fn try_into_array_fn_test() {
    let slice = &[0, 2, 3, 4][..];

    assert!(try_into_array::<_, 0>(slice).is_err());
    assert!(try_into_array::<_, 1>(slice).is_err());
    assert!(try_into_array::<_, 2>(slice).is_err());
    assert!(try_into_array::<_, 3>(slice).is_err());
    assert_eq!(try_into_array::<_, 4>(slice), Ok(&[0, 2, 3, 4]));
    assert!(try_into_array::<_, 5>(slice).is_err());
    assert!(try_into_array::<_, 6>(slice).is_err());
}
