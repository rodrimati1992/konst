use konst::iter;

#[test]
fn test_repeat() {
    const fn isconst() -> [u8; 8] {
        let mut iter: iter::Repeat<u8> = iter::repeat(10);

        let mut iter_copy: iter::Repeat<u8> = iter.copy();
        let a0 = iter_copy.next().unwrap();
        let a1 = iter.next().unwrap();
        let b = iter.next_back().unwrap();
        let iter: iter::Repeat<u8> = iter.rev();
        let mut iter: iter::Repeat<u8> = iter::into_iter!(iter.copy());
        let c0 = iter.next().unwrap();
        let c1 = iter.next().unwrap();
        let d = iter.next_back().unwrap();
        let mut iter: iter::Repeat<u8> = iter.rev();
        let e = iter.next().unwrap();
        let f = iter.next_back().unwrap();

        [a0, a1, b, c0, c1, d, e, f]
    }

    assert_eq!(isconst(), [10u8; 8]);
}
