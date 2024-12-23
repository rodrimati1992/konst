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

#[test]
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
