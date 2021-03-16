use konst::{
    cmp_str, const_cmp, const_eq, eq_str,
    slice::cmp::{cmp_slice_bytes, cmp_slice_u8, eq_slice_bytes, eq_slice_u8},
};

#[cfg(feature = "option")]
use konst::{
    cmp_option_str, eq_option_str,
    slice::cmp::{
        cmp_option_slice_bytes, cmp_option_slice_u8, eq_option_slice_bytes, eq_option_slice_u8,
    },
};

#[test]
#[cfg(feature = "slice")]
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
#[cfg(feature = "slice")]
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
#[cfg(feature = "str")]
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
#[cfg(feature = "slice")]
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
#[cfg(feature = "slice")]
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
#[cfg(feature = "str")]
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
