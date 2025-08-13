use konst::primitive::cmp::{
    self as prim_cmp, cmp_bool, cmp_char, cmp_option_bool, cmp_option_char, eq_option_bool,
    eq_option_char,
};

use core::cmp::Ordering::{Equal, Greater, Less};

#[test]
fn integer_cmp_test() {
    macro_rules! cases {
        ($(($ty:ident $eq_opt:ident $cmp:ident $cmp_opt:ident))*) => ($({
            let eq_closure = |l: $ty, r: $ty| l == r;
            assertc_opt_eq_rets! {
                $ty, eq_closure, prim_cmp::$eq_opt =>

                (0, 0, true)
                (0, 1, false)
                (0, 2, false)
                (1, 1, true)
                (1, 2, false)
                (2, 2, true)
            }

            assertc_opt_cmp! {
                $ty, prim_cmp::$cmp, prim_cmp::$cmp_opt =>

                (0, 0, Equal)
                (0, 1, Less)
                (0, 2, Less)
                (1, 1, Equal)
                (1, 2, Less)
                (2, 2, Equal)
            }
        })*)
    }

    cases! {
        (u8 eq_option_u8 cmp_u8 cmp_option_u8)
        (u16 eq_option_u16 cmp_u16 cmp_option_u16)
        (u32 eq_option_u32 cmp_u32 cmp_option_u32)
        (u64 eq_option_u64 cmp_u64 cmp_option_u64)
        (u128 eq_option_u128 cmp_u128 cmp_option_u128)
        (usize eq_option_usize cmp_usize cmp_option_usize)
        (i8 eq_option_i8 cmp_i8 cmp_option_i8)
        (i16 eq_option_i16 cmp_i16 cmp_option_i16)
        (i32 eq_option_i32 cmp_i32 cmp_option_i32)
        (i64 eq_option_i64 cmp_i64 cmp_option_i64)
        (i128 eq_option_i128 cmp_i128 cmp_option_i128)
        (isize eq_option_isize cmp_isize cmp_option_isize)
    }
}

#[test]
fn bool_cmp_test() {
    let eq_bool = |l: bool, r: bool| l == r;
    assertc_opt_eq_rets! {
        bool, eq_bool, eq_option_bool =>

        (true, true, true)
        (true, false, false)
        (false, false, true)
    }

    assertc_opt_cmp! {
        bool, cmp_bool , cmp_option_bool =>

        (true, true, Equal)
        (true, false, Greater)
        (false, false, Equal)
    }
}

#[test]
fn charl_cmp_test() {
    let eq_char = |l: char, r: char| l == r;
    assertc_opt_eq_rets! {
        char, eq_char, eq_option_char =>

        ('a', 'a', true)
        ('a', 'b', false)
        ('b', 'b', true)
    }

    assertc_opt_cmp! {
        char, cmp_char , cmp_option_char =>

        ('a', 'a', Equal)
        ('b', 'a', Greater)
        ('b', 'b', Equal)
    }
}
