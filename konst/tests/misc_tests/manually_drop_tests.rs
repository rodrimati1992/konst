use konst::manually_drop;

use core::{cmp::Ordering, mem::ManuallyDrop};

#[test]
fn as_inner_test() {
    const fn _callable<T>(md: &ManuallyDrop<T>) -> &T {
        manually_drop::as_inner(md)
    }

    let reff = ManuallyDrop::new(&0u32);
    let booll = ManuallyDrop::new(true);
    let ordering = ManuallyDrop::new(Ordering::Greater);
    let string = ManuallyDrop::new("wowow");

    assert_eq!(manually_drop::as_inner(&reff), &&0u32);
    assert_eq!(manually_drop::as_inner(&booll), &true);
    assert_eq!(manually_drop::as_inner(&ordering), &Ordering::Greater);
    assert_eq!(manually_drop::as_inner(&string), &"wowow");
}

#[test]
#[cfg(feature = "rust_1_83")]
fn as_inner_mut_test() {
    const fn _callable<T>(md: &mut ManuallyDrop<T>) -> &mut T {
        manually_drop::as_inner_mut(md)
    }

    let mut reff = ManuallyDrop::new(&0u32);
    let mut booll = ManuallyDrop::new(true);
    let mut ordering = ManuallyDrop::new(Ordering::Greater);
    let mut string = ManuallyDrop::new("wowow");

    *manually_drop::as_inner_mut(&mut reff) = &3;
    *manually_drop::as_inner_mut(&mut booll) = false;
    *manually_drop::as_inner_mut(&mut ordering) = Ordering::Equal;
    *manually_drop::as_inner_mut(&mut string) = "what";

    assert_eq!(manually_drop::as_inner_mut(&mut reff), &mut &3u32);
    assert_eq!(manually_drop::as_inner_mut(&mut booll), &mut false);
    assert_eq!(
        manually_drop::as_inner_mut(&mut ordering),
        &mut Ordering::Equal
    );
    assert_eq!(manually_drop::as_inner_mut(&mut string), &mut "what");
}

#[test]
#[cfg(feature = "rust_1_83")]
fn take_test() {
    const unsafe fn _callable<T>(md: &mut ManuallyDrop<T>) -> T {
        manually_drop::take(md)
    }

    let mut reff = ManuallyDrop::new(&0u32);
    let mut booll = ManuallyDrop::new(true);
    let mut ordering = ManuallyDrop::new(Ordering::Greater);
    let mut string = ManuallyDrop::new(String::from("wowow"));

    unsafe {
        assert_eq!(manually_drop::take(&mut reff), &0u32);
        assert_eq!(manually_drop::take(&mut booll), true);
        assert_eq!(manually_drop::take(&mut ordering), Ordering::Greater);
        assert_eq!(manually_drop::take(&mut string), "wowow");
    }
}
