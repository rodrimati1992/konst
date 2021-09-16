use konst::manually_drop;

use core::{cmp::Ordering, mem::ManuallyDrop};

#[test]
#[cfg(feature = "rust_1_56")]
fn as_inner_test() {
    let reff = ManuallyDrop::new(&0u32);
    let booll = ManuallyDrop::new(true);
    let ordering = ManuallyDrop::new(Ordering::Greater);
    let string = ManuallyDrop::new("wowow");

    assert_eq!(manually_drop::as_inner(&reff), &&0u32);
    assert_eq!(manually_drop::as_inner(&booll), &true);
    assert_eq!(manually_drop::as_inner(&ordering), &Ordering::Greater);
    assert_eq!(manually_drop::as_inner(&string), &"wowow");
}
