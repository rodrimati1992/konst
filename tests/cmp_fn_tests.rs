use const_cmp::{
    const_cmp, const_eq,
    slice::{slice_cmp_bytes, slice_cmp_u8, slice_eq_bytes, slice_eq_u8},
    str_cmp, str_eq,
};

#[test]
#[cfg(feature = "slice_cmp")]
fn slice_eq_test() {
    macro_rules! assert_s_cmp {
        ($left:expr, $right:expr, $expected:expr) => {
            let left: &[u8] = $left;
            assert_eq!(slice_eq_u8(left, $right), $expected);
            assert_eq!(const_eq!(left, $right), $expected);
        };
    }

    assert_s_cmp!(&[], &[], true);
    assert_s_cmp!(&[], &[0], false);
    assert_s_cmp!(&[0], &[], false);
    assert_s_cmp!(&[0], &[0], true);
    assert_s_cmp!(&[0], &[1], false);
    assert_s_cmp!(&[1], &[0], false);
    assert_s_cmp!(&[0], &[0, 1], false);
    assert_s_cmp!(&[0, 1], &[0], false);
    assert_s_cmp!(&[0, 1], &[0, 1], true);
    assert_s_cmp!(&[0, 1], &[0, 2], false);
}

#[test]
#[cfg(feature = "slice_cmp")]
fn slice_of_bytes_eq_test() {
    macro_rules! assert_s_cmp {
        ($left:expr, $right:expr, $expected:expr) => {
            let left: &[&[u8]] = $left;
            let right: &[&[u8]] = $right;
            assert_eq!(slice_eq_bytes(left, right), $expected);
            assert_eq!(const_eq!(left, right), $expected);
        };
    }

    assert_s_cmp!(&[], &[], true);
    assert_s_cmp!(&[], &[&[0]], false);
    assert_s_cmp!(&[&[0]], &[], false);
    assert_s_cmp!(&[&[0]], &[&[0]], true);
    assert_s_cmp!(&[&[0]], &[&[1]], false);
    assert_s_cmp!(&[&[1]], &[&[0]], false);

    assert_s_cmp!(&[&[0]], &[&[0, 1]], false);
    assert_s_cmp!(&[&[0, 1]], &[&[0]], false);
    assert_s_cmp!(&[&[0, 1]], &[&[0, 1]], true);
    assert_s_cmp!(&[&[0, 1]], &[&[0, 2]], false);

    assert_s_cmp!(&[&[0], &[1]], &[&[0], &[1]], true);
    assert_s_cmp!(&[&[0], &[1]], &[&[0], &[1, 2]], false);
    assert_s_cmp!(&[&[0], &[1, 2]], &[&[0], &[1]], false);
    assert_s_cmp!(&[&[0], &[1, 2]], &[&[0], &[1, 2]], true);
    assert_s_cmp!(&[&[0], &[1, 2]], &[&[0], &[1, 3]], false);
}

#[test]
#[cfg(feature = "str_cmp")]
fn str_eq_test() {
    macro_rules! assert_s_cmp {
        ($left:expr, $right:expr, $expected:expr) => {
            assert_eq!(str_eq($left, $right), $expected);
            assert_eq!(const_eq!($left, $right), $expected);
        };
    }

    assert_s_cmp!("", "", true);
    assert_s_cmp!("", "0", false);
    assert_s_cmp!("0", "", false);
    assert_s_cmp!("0", "0", true);
    assert_s_cmp!("0", "1", false);
    assert_s_cmp!("1", "0", false);
    assert_s_cmp!("0", "0, 1", false);
    assert_s_cmp!("0, 1", "0", false);
    assert_s_cmp!("0, 1", "1", false);
    assert_s_cmp!("0, 1", "0, 1", true);
    assert_s_cmp!("0, 1", "0, 2", false);
}

macro_rules! assert_slices_ord {
    ($ty:ty, $function:path => $left:expr, $right:expr, $expected:expr) => {
        let left: $ty = $left;
        let right: $ty = $right;

        assert_eq!(const_cmp!(left, right), $expected);
        assert_eq!($function(left, right), $expected);
        assert_eq!(Ord::cmp(left, right), $expected);

        assert_eq!(const_cmp!(right, left), $expected.reverse());
        assert_eq!($function(right, left), $expected.reverse());
        assert_eq!(Ord::cmp(right, left), $expected.reverse());
    };
}

#[test]
#[cfg(feature = "slice_cmp")]
fn slice_cmp_test() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    macro_rules! assert_s_cmp {
        ($($anything:tt)*) => {
            assert_slices_ord!{ &[u8], slice_cmp_u8 => $($anything)* }
        }
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
#[cfg(feature = "slice_cmp")]
fn slice_of_bytes_cmp_test() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    macro_rules! assert_s_cmp {
        ($($anything:tt)*) => {
            assert_slices_ord!{ &[&[u8]], slice_cmp_bytes => $($anything)* }
        }
    }

    assert_s_cmp!(&[], &[], Equal);
    assert_s_cmp!(&[], &[&[0]], Less);
    assert_s_cmp!(&[&[0]], &[], Greater);
    assert_s_cmp!(&[&[0]], &[&[0]], Equal);
    assert_s_cmp!(&[&[0]], &[&[1]], Less);
    assert_s_cmp!(&[&[1]], &[&[0]], Greater);

    assert_s_cmp!(&[&[0]], &[&[0, 1]], Less);
    assert_s_cmp!(&[&[0, 1]], &[&[0]], Greater);
    assert_s_cmp!(&[&[0, 1]], &[&[0, 1]], Equal);
    assert_s_cmp!(&[&[0, 1]], &[&[0, 2]], Less);

    assert_s_cmp!(&[&[0], &[1]], &[&[0], &[1]], Equal);
    assert_s_cmp!(&[&[1], &[1]], &[&[0], &[1]], Greater);
    assert_s_cmp!(&[&[0], &[1]], &[&[0], &[1, 2]], Less);
    assert_s_cmp!(&[&[0], &[1, 2]], &[&[0], &[1]], Greater);
    assert_s_cmp!(&[&[0], &[1, 2]], &[&[0], &[1, 2]], Equal);
    assert_s_cmp!(&[&[0], &[1, 2]], &[&[0], &[1, 3]], Less);
}

#[test]
#[cfg(feature = "str_cmp")]
fn str_cmp_test() {
    use core::cmp::Ordering::{Equal, Greater, Less};

    macro_rules! assert_s_cmp {
        ($left:expr, $right:expr, $expected:expr) => {
            assert_eq!(const_cmp!($left, $right), $expected, "C");
            assert_eq!(str_cmp($left, $right), $expected, "A");
            assert_eq!($left.cmp($right), $expected, "B");

            assert_eq!(const_cmp!($left, $left), Equal);
            assert_eq!(str_cmp($left, $left), Equal);
            assert_eq!(str_cmp($right, $right), Equal);

            assert_eq!(const_cmp!($right, $left), $expected.reverse(), "cmp");
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
