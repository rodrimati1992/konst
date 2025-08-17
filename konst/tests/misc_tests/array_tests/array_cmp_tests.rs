use konst::cmp::{CmpWrapper, const_cmp, const_eq};

use core::cmp::Ordering::{self, Equal, Greater, Less};

macro_rules! define_asserter {
    (
        fn $asserter:ident, $is_same_val:ident: $exp_ty:ty, $cmp_macro:ident;
        $ty:ty, $function:expr, [$($reverse_method:tt)*]
    ) => {
        fn $asserter<const L: usize, const R: usize>(
            left: [$ty; L],
            right: [$ty; R],
            expected: $exp_ty,
        ) {
            assert_eq!(
                $function(left, right),
                expected,
                "{{A}}\n{:?}\n{:?}",
                left,
                right
            );

            assert_eq!(
                $cmp_macro!(left, right),
                expected,
                "{{B}}\n{:?}\n{:?}",
                left,
                right
            );
            assert_eq!(
                $cmp_macro!(left, &right[..]),
                expected,
                "{{B}}\n{:?}\n{:?}",
                left,
                right
            );
            assert_eq!(
                $cmp_macro!(&left[..], right),
                expected,
                "{{B}}\n{:?}\n{:?}",
                left,
                right
            );

            assert_eq!(
                $function(right, left),
                expected $($reverse_method)*,
                "{{D}}\n{:?}\n{:?}",
                left,
                right
            );

            assert_eq!(
                $cmp_macro!(right, left),
                expected $($reverse_method)*,
                "{{E}}\n{:?}\n{:?}",
                left,
                right
            );
            assert_eq!(
                $cmp_macro!(right, &left[..]),
                expected $($reverse_method)*,
                "{{E}}\n{:?}\n{:?}",
                left,
                right
            );
            assert_eq!(
                $cmp_macro!(&right[..], left),
                expected $($reverse_method)*,
                "{{E}}\n{:?}\n{:?}",
                left,
                right
            );

            assert_eq!(
                $function(left, left),
                $is_same_val,
                "{{G}}\n{:?}\n{:?}",
                left,
                right
            );

            assert_eq!(
                $cmp_macro!(left, left),
                $is_same_val,
                "{{H}}\n{:?}\n{:?}",
                left,
                right
            );
            assert_eq!(
                $cmp_macro!(left, &left[..]),
                $is_same_val,
                "{{H}}\n{:?}\n{:?}",
                left,
                right
            );
            assert_eq!(
                $cmp_macro!(&left[..], left),
                $is_same_val,
                "{{H}}\n{:?}\n{:?}",
                left,
                right
            );

            assert_eq!(
                $function(right, right),
                $is_same_val,
                "{{J}}\n{:?}\n{:?}",
                left,
                right
            );

            assert_eq!(
                $cmp_macro!(right, right),
                $is_same_val,
                "{{K}}\n{:?}\n{:?}",
                left,
                right
            );
            assert_eq!(
                $cmp_macro!(right, &right[..]),
                $is_same_val,
                "{{K}}\n{:?}\n{:?}",
                left,
                right
            );
            assert_eq!(
                $cmp_macro!(&right[..], right),
                $is_same_val,
                "{{K}}\n{:?}\n{:?}",
                left,
                right
            );
        }
    }
}

macro_rules! ass_cmp {
    (
        $ty:ty, $eq_fn:expr, $cmp_fn:expr =>

        $(($left:expr, $right:expr, $eq_expected:expr, $cmp_expected:expr))*
    ) => ({
        const fn ceq<const LN: usize, const RN: usize>(
            left: [$ty; LN],
            right: [$ty; RN],
        ) -> bool {
            CmpWrapper::from_ref(&left).const_eq(&right)
        }
        const fn ccmp<const LN: usize, const RN: usize>(
            left: [$ty; LN],
            right: [$ty; RN],
        ) -> Ordering {
            CmpWrapper::from_ref(&left).const_cmp(&right)
        }

        define_asserter!{
            fn eq_asserts, true: bool, const_eq; $ty, ceq, []
        }
        define_asserter!{
            fn cmp_asserts, Equal: Ordering, const_cmp; $ty, ccmp, [.reverse()]
        }

        $(eq_asserts($left, $right, $eq_expected);)*
        $(cmp_asserts($left, $right, $cmp_expected);)*
    });
}

#[test]
#[should_panic]
fn ensure_asserter_isnt_noop_test() {
    ass_cmp! {u8, ceq, ccmp => ([], [], false, Less)}
}

#[test]
fn cmp_array_test() {
    macro_rules! cases {
        ($($ty:ident)*) => ($({
            ass_cmp! {
                $ty, ceq, ccmp =>
                ([], [], true, Equal)
                ([], [0], false, Less)
                ([0], [], false, Greater)
                ([0], [0], true, Equal)
                ([0], [1], false, Less)
                ([1], [0], false, Greater)
                ([0], [0, 1], false, Less)
                ([0, 1], [0], false, Greater)
                ([0, 1], [0, 1], true, Equal)
                ([0, 1], [0, 2], false, Less)
            }
        })*)
    }

    cases! {u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize}
}

#[test]
fn cmp_arrays_char_test() {
    ass_cmp! {
        char, ceq, ccmp =>
        ([], [], true, Equal)
        ([], ['0'], false, Less)
        (['0'], [], false, Greater)
        (['0'], ['0'], true, Equal)
        (['0'], ['1'], false, Less)
        (['0'], ['0', '1'], false, Less)
        (['0', '1'], ['0', '1'], true, Equal)
        (['0', '1'], ['0', '2'], false, Less)
    }
}

#[test]
fn cmp_slice_bool_test() {
    ass_cmp! {
        bool, ceq, ccmp =>
        ([], [], true, Equal)
        ([], [false], false, Less)
        ([false], [], false, Greater)
        ([false], [false], true, Equal)
        ([false], [true], false, Less)
        ([false], [false, true], false, Less)
        ([false, true], [false, true], true, Equal)
        ([false, false], [false, true], false, Less)
    }
}
