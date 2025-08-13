use core::cmp::Ordering::{self, Equal, Greater, Less};

use konst::{
    cmp::{CmpWrapper as CW, const_cmp, const_eq},
    other::cmp as other_cmp,
};

#[test]
fn ordering_cmp_test() {
    assertc_opt_eq_rets! {
        Ordering, other_cmp::eq_ordering, other_cmp::eq_option_ordering =>

        (Equal, Less, false)
        (Equal, Equal, true)
        (Equal, Greater, false)
    }

    assertc_opt_cmp! {
        Ordering, other_cmp::cmp_ordering, other_cmp::cmp_option_ordering =>

        (Equal, Less, Greater)
        (Equal, Equal, Equal)
        (Equal, Greater, Less)
    }
}

#[test]
fn phantomdata_cmp_test() {
    use core::marker::PhantomData as PD;

    const _: () = {
        assert!(CW::from_ref(&PD::<u32>).const_eq(&PD::<u32>));
        assert!(const_eq!(PD::<u32>, PD::<u32>));
        assert!(other_cmp::eq_phantomdata(PD::<u32>, PD::<u32>));

        assert!(matches!(
            CW::from_ref(&PD::<u32>).const_cmp(&PD::<u32>),
            Equal
        ));
        assert!(matches!(const_cmp!(PD::<u32>, PD::<u32>), Equal));
        assert!(matches!(
            other_cmp::cmp_phantomdata(PD::<u32>, PD::<u32>),
            Equal
        ));
    };
}

#[test]
fn phantompinned_cmp_test() {
    use core::marker::PhantomPinned as PhanPi;

    const _: () = {
        assert!(CW::from_ref(&PhanPi).const_eq(&PhanPi));
        assert!(const_eq!(PhanPi, PhanPi));
        assert!(other_cmp::eq_phantompinned(PhanPi, PhanPi));

        assert!(matches!(CW::from_ref(&PhanPi).const_cmp(&PhanPi), Equal));
        assert!(matches!(const_cmp!(PhanPi, PhanPi), Equal));
        assert!(matches!(
            other_cmp::cmp_phantompinned(PhanPi, PhanPi),
            Equal
        ));
    };
}
