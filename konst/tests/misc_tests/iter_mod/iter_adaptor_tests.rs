use konst::{iter, option};

#[test]
fn test_repeat() {
    const fn isconst() -> [u8; 8] {
        let mut iter: iter::Repeat<u8> = iter::repeat(10);

        let mut iter_copy: iter::Repeat<u8> = iter.copy();
        let a0 = option::unwrap!(iter_copy.next());
        let a1 = option::unwrap!(iter.next());
        let b = option::unwrap!(iter.next_back());
        let iter: iter::Repeat<u8> = iter.rev();
        let mut iter: iter::Repeat<u8> = iter::into_iter!(iter.copy());
        let c0 = option::unwrap!(iter.next());
        let c1 = option::unwrap!(iter.next());
        let d = option::unwrap!(iter.next_back());
        let mut iter: iter::Repeat<u8> = iter.rev();
        let e = option::unwrap!(iter.next());
        let f = option::unwrap!(iter.next_back());

        [a0, a1, b, c0, c1, d, e, f]
    }

    assert_eq!(isconst(), [10u8; 8]);
}
