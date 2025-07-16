use konst::maybe_uninit;

use std::{cmp::Ordering, mem::MaybeUninit};

#[test]
fn array_assume_init_test() {
    let reff = [MaybeUninit::new(&3u32); 5];
    let booll = [MaybeUninit::new(true); 5];
    let ordering = [MaybeUninit::new(Ordering::Greater); 5];
    let string = [MaybeUninit::new("wowow"); 5];
    unsafe {
        assert_eq!(maybe_uninit::array_assume_init(reff), [&3u32; 5]);
        assert_eq!(maybe_uninit::array_assume_init(booll), [true; 5]);
        assert_eq!(
            maybe_uninit::array_assume_init(ordering),
            [Ordering::Greater; 5]
        );
        assert_eq!(maybe_uninit::array_assume_init(string), ["wowow"; 5]);
    }
}

#[test]
fn uninit_array_test() {
    use std::mem::MaybeUninit;

    let mut reffs: [MaybeUninit<&u8>; 2] = maybe_uninit::uninit_array::<&u8, 2>();

    reffs[0] = MaybeUninit::new(&10u8);
    reffs[1] = MaybeUninit::new(&20);

    unsafe {
        assert_eq!(maybe_uninit::array_assume_init(reffs), [&10u8, &20u8]);
    }
}
