use const_cmp::{
    slice::{slice_cmp_u8, slice_eq_u8},
    str_cmp, str_eq,
};

#[test]
#[cfg(feature = "slice_cmp")]
fn slice_eq_test() {
    assert!(slice_eq_u8(&[], &[]));
    assert!(!slice_eq_u8(&[], &[0]));
    assert!(!slice_eq_u8(&[0], &[]));
    assert!(slice_eq_u8(&[0], &[0]));
    assert!(!slice_eq_u8(&[0], &[1]));
    assert!(!slice_eq_u8(&[1], &[0]));
    assert!(!slice_eq_u8(&[0], &[0, 1]));
    assert!(!slice_eq_u8(&[0, 1], &[0]));
    assert!(slice_eq_u8(&[0, 1], &[0, 1]));
    assert!(!slice_eq_u8(&[0, 1], &[0, 2]));
}

#[test]
#[cfg(feature = "str_cmp")]
fn str_eq_test() {
    assert!(str_eq("", ""));
    assert!(!str_eq("", "0"));
    assert!(!str_eq("0", ""));
    assert!(str_eq("0", "0"));
    assert!(!str_eq("0", "1"));
    assert!(!str_eq("1", "0"));
    assert!(!str_eq("0", "0, 1"));
    assert!(!str_eq("0, 1", "0"));
    assert!(!str_eq("0, 1", "1"));
    assert!(str_eq("0, 1", "0, 1"));
    assert!(!str_eq("0, 1", "0, 2"));
}

#[test]
#[cfg(feature = "slice_cmp")]
fn slice_cmp_test() {
    use core::cmp::{
        Ord,
        Ordering::{Equal, Greater, Less},
    };

    macro_rules! assert_s_cmp {
        ($left:expr, $right:expr, $expected:expr) => {
            assert_eq!(slice_cmp_u8($left, $right), $expected);
            assert_eq!(<[u8]>::cmp($left, $right), $expected);

            assert_eq!(slice_cmp_u8($right, $left), $expected.reverse());
            assert_eq!(<[u8]>::cmp($right, $left), $expected.reverse());
        };
    }

    assert_s_cmp!(&[], &[], Equal);
    assert_s_cmp!(&[], &[0], Less);
    assert_s_cmp!(&[0], &[], Greater);
    assert_s_cmp!(&[0], &[0], Equal);
    assert_s_cmp!(&[0], &[1], Less);
    assert_s_cmp!(&[0], &[0, 1], Less);
    assert_s_cmp!(&[0, 1], &[0, 1], Equal);
    assert_s_cmp!(&[0, 1], &[0, 2], Less);
}

#[test]
#[cfg(feature = "str_cmp")]
fn str_cmp_test() {
    use core::cmp::{
        Ord,
        Ordering::{Equal, Greater, Less},
    };

    macro_rules! assert_s_cmp {
        ($left:expr, $right:expr, $expected:expr) => {
            assert_eq!(str_cmp($left, $right), $expected, "A");
            assert_eq!($left.cmp($right), $expected, "B");

            assert_eq!(str_cmp($left, $left), Equal);
            assert_eq!(str_cmp($right, $right), Equal);

            assert_eq!(str_cmp($right, $left), $expected.reverse(), "cmp");
            assert_eq!($right.cmp($left), $expected.reverse(), "cmp");
        };
    }

    assert_s_cmp!("0", "", Greater);
    assert_s_cmp!("0", "1", Less);
    assert_s_cmp!("0", "01", Less);
    assert_s_cmp!("1", "01", Greater);
    assert_s_cmp!("099999", "12", Less);
    assert_s_cmp!("111111", "12", Less);
    assert_s_cmp!("120", "12", Greater);
    assert_s_cmp!("199999", "12", Greater);
    assert_s_cmp!("299999", "12", Greater);
    assert_s_cmp!("01", "02", Less);
}
