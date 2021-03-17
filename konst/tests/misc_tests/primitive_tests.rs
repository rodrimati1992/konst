use konst::primitive::cmp::{
    cmp_bool, cmp_char, cmp_option_bool, cmp_option_char, cmp_option_u8, cmp_u8, eq_option_bool,
    eq_option_char, eq_option_u8,
};

use core::cmp::Ordering::{Equal, Greater, Less};

#[test]
fn integer_cmp_test() {
    let eq_u8 = |l: u8, r: u8| l == r;
    assertc_opt_eq_rets! {
        u8, eq_u8, eq_option_u8 =>

        (0, 0, true)
        (0, 1, false)
        (0, 2, false)
        (1, 1, true)
        (1, 2, false)
        (2, 2, true)
    }

    assertc_opt_cmp! {
        u8, cmp_u8, cmp_option_u8 =>

        (0, 0, Equal)
        (0, 1, Less)
        (0, 2, Less)
        (1, 1, Equal)
        (1, 2, Less)
        (2, 2, Equal)
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
