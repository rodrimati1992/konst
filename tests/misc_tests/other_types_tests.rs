use konst::other::{
    cmp_option_ordering, cmp_ordering, cmp_phantomdata, cmp_phantompinned, eq_option_ordering,
    eq_ordering, eq_phantomdata, eq_phantompinned,
};

use core::{
    cmp::Ordering::{self, Equal, Greater, Less},
    marker::{PhantomData, PhantomPinned},
};

#[test]
fn ordering_test() {
    assertc_opt_eq_rets! {
        Ordering, eq_ordering, eq_option_ordering =>

        (Equal, Less, false)
        (Equal, Greater, false)
        (Less, Greater, false)
        (Equal, Equal, true)
        (Less, Less, true)
        (Greater, Greater, true)
    }

    assertc_opt_cmp! {
        Ordering, cmp_ordering, cmp_option_ordering =>

        (Equal, Less, Greater)
        (Equal, Greater, Less)
        (Less, Greater, Less)
        (Equal, Equal, Equal)
        (Less, Less, Equal)
        (Greater, Greater, Equal)
    }
}

#[test]
fn phantomdata_test() {
    assertc_opt_eq_rets! {
        PhantomData<u8>, eq_phantomdata =>
        (PhantomData, PhantomData, true)
    }
    assertc_opt_cmp! {
        PhantomData<u8>, cmp_phantomdata =>
        (PhantomData, PhantomData, Equal)
    }
    /////
    assertc_opt_eq_rets! {
        PhantomData<&'static str>, eq_phantomdata =>
        (PhantomData, PhantomData, true)
    }
    assertc_opt_cmp! {
        PhantomData<&'static str>, cmp_phantomdata =>
        (PhantomData, PhantomData, Equal)
    }
}

#[test]
fn phantompinned_test() {
    assertc_opt_eq_rets! {
        PhantomPinned, eq_phantompinned =>
        (PhantomPinned, PhantomPinned, true)
    }
    assertc_opt_cmp! {
        PhantomPinned, cmp_phantompinned =>
        (PhantomPinned, PhantomPinned, Equal)
    }
}
