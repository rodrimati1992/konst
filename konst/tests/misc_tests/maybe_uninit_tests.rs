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
fn uninit_array_test() {
    use konst::maybe_uninit as mu;

    use std::mem::{self, MaybeUninit};

    let mut reffs: [MaybeUninit<&u8>; 2] = maybe_uninit::uninit_array::<&u8, 2>();

    reffs[0] = MaybeUninit::new(&10u8);
    reffs[1] = MaybeUninit::new(&20);

    unsafe {
        assert_eq!(maybe_uninit::array_assume_init(reffs), [&10u8, &20u8]);
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
#[cfg(feature = "mut_refs")]
fn write_test() {
    let mut reff = MaybeUninit::new(&0u32);
    let mut ordering = MaybeUninit::new(Ordering::Greater);
    let mut string = MaybeUninit::new("wowow");

    unsafe {
        *maybe_uninit::write(&mut reff, &3) = &5;
        *maybe_uninit::write(&mut ordering, Ordering::Equal) = Ordering::Less;
        *maybe_uninit::write(&mut string, "what") = "why";

        assert_eq!(maybe_uninit::assume_init_mut(&mut reff), &mut &5u32);
        assert_eq!(
            maybe_uninit::assume_init_mut(&mut ordering),
            &mut Ordering::Less
        );
        assert_eq!(maybe_uninit::assume_init_mut(&mut string), &mut "why");
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

#[test]
#[cfg(feature = "mut_refs")]
fn as_mut_ptr_test() {
    let mut reff = MaybeUninit::new(&0u32);
    let mut booll = MaybeUninit::new(true);
    let mut ordering = MaybeUninit::new(Ordering::Greater);
    let mut string = MaybeUninit::new("wowow");

    unsafe {
        *maybe_uninit::as_mut_ptr(&mut reff) = &3;
        *maybe_uninit::as_mut_ptr(&mut booll) = false;
        *maybe_uninit::as_mut_ptr(&mut ordering) = Ordering::Equal;
        *maybe_uninit::as_mut_ptr(&mut string) = "what";

        assert_eq!(maybe_uninit::assume_init_mut(&mut reff), &mut &3u32);
        assert_eq!(maybe_uninit::assume_init_mut(&mut booll), &mut false);
        assert_eq!(
            maybe_uninit::assume_init_mut(&mut ordering),
            &mut Ordering::Equal
        );
        assert_eq!(maybe_uninit::assume_init_mut(&mut string), &mut "what");
    }
}
