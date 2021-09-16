use konst::maybe_uninit;

use std::{cmp::Ordering, mem::MaybeUninit};

#[test]
#[cfg(feature = "rust_1_56")]
fn assume_init_test() {
    let reff = MaybeUninit::new(&0u32);
    let booll = MaybeUninit::new(true);
    let ordering = MaybeUninit::new(Ordering::Greater);
    let string = MaybeUninit::new("wowow");
    unsafe {
        assert_eq!(maybe_uninit::assume_init(reff), &0u32);
        assert_eq!(maybe_uninit::assume_init(booll), true);
        assert_eq!(maybe_uninit::assume_init(ordering), Ordering::Greater);
        assert_eq!(maybe_uninit::assume_init(string), "wowow");
    }
}

#[test]
#[cfg(feature = "rust_1_56")]
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
#[cfg(feature = "rust_1_56")]
fn assume_init_ref_test() {
    let reff = MaybeUninit::new(&0u32);
    let booll = MaybeUninit::new(true);
    let ordering = MaybeUninit::new(Ordering::Greater);
    let string = MaybeUninit::new("wowow");
    unsafe {
        assert_eq!(maybe_uninit::assume_init_ref(&reff), &&0u32);
        assert_eq!(maybe_uninit::assume_init_ref(&booll), &true);
        assert_eq!(maybe_uninit::assume_init_ref(&ordering), &Ordering::Greater);
        assert_eq!(maybe_uninit::assume_init_ref(&string), &"wowow");
    }
}

#[test]
#[cfg(feature = "mut_refs")]
fn assume_init_mut_test() {
    let mut reff = MaybeUninit::new(&0u32);
    let mut booll = MaybeUninit::new(true);
    let mut ordering = MaybeUninit::new(Ordering::Greater);
    let mut string = MaybeUninit::new("wowow");

    unsafe {
        *maybe_uninit::assume_init_mut(&mut reff) = &3;
        *maybe_uninit::assume_init_mut(&mut booll) = false;
        *maybe_uninit::assume_init_mut(&mut ordering) = Ordering::Equal;
        *maybe_uninit::assume_init_mut(&mut string) = "what";

        assert_eq!(maybe_uninit::assume_init_mut(&mut reff), &mut &3u32);
        assert_eq!(maybe_uninit::assume_init_mut(&mut booll), &mut false);
        assert_eq!(
            maybe_uninit::assume_init_mut(&mut ordering),
            &mut Ordering::Equal
        );
        assert_eq!(maybe_uninit::assume_init_mut(&mut string), &mut "what");
    }
}

#[test]
fn as_ptr_test() {
    let reff = MaybeUninit::new(&0u32);
    let booll = MaybeUninit::new(true);
    let ordering = MaybeUninit::new(Ordering::Greater);
    let string = MaybeUninit::new("wowow");
    unsafe {
        assert_eq!(*maybe_uninit::as_ptr(&reff), &0u32);
        assert_eq!(*maybe_uninit::as_ptr(&booll), true);
        assert_eq!(*maybe_uninit::as_ptr(&ordering), Ordering::Greater);
        assert_eq!(*maybe_uninit::as_ptr(&string), "wowow");
    }
}
