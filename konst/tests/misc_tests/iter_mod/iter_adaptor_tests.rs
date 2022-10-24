use konst::{iter, option};

#[test]
fn test_repeat() {
    const fn isconst() -> [u8; 8] {
        let iter: iter::Repeat<u8> = iter::repeat(10);

        let iter_copy: iter::Repeat<u8> = iter.copy();
        let (a0, iter) = option::unwrap!(iter_copy.next());
        let (a1, iter) = option::unwrap!(iter.next());
        let (b, iter) = option::unwrap!(iter.next_back());
        let iter: iter::Repeat<u8> = iter.rev();
        let iter: iter::Repeat<u8> = iter::into_iter!(iter.copy());
        let (c0, iter) = option::unwrap!(iter.next());
        let (c1, iter) = option::unwrap!(iter.next());
        let (d, iter) = option::unwrap!(iter.next_back());
        let iter: iter::Repeat<u8> = iter.rev();
        let (e, iter) = option::unwrap!(iter.next());
        let (f, _) = option::unwrap!(iter.next_back());

        [a0, a1, b, c0, c1, d, e, f]
    }

    assert_eq!(isconst(), [10u8; 8]);
}
