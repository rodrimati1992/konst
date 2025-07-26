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

#[test]
fn test_repeat_n() {
    const fn isconst() -> [u8; 8] {
        let mut iter: iter::RepeatN<u8> = iter::repeat_n(10, 12);

        let mut iter_copy: iter::RepeatN<u8> = iter.copy();
        let a0 = iter_copy.next().unwrap();
        let a1 = iter.next().unwrap();
        let b = iter.next_back().unwrap();
        let iter: iter::RepeatN<u8> = iter.rev();
        let mut iter: iter::RepeatN<u8> = iter::into_iter!(iter.copy());
        let c0 = iter.next().unwrap();
        let c1 = iter.next().unwrap();
        let d = iter.next_back().unwrap();
        let mut iter: iter::RepeatN<u8> = iter.rev();
        let e = iter.next().unwrap();
        let f = iter.next_back().unwrap();

        [a0, a1, b, c0, c1, d, e, f]
    }

    assert_eq!(isconst(), [10u8; 8]);

    assert_eq!(
        iter::collect_const! {char => iter::repeat_n('a', 0)},
        ['a'; 0]
    );
    assert_eq!(
        iter::collect_const! {char => iter::repeat_n('b', 1)},
        ['b'; 1]
    );
    assert_eq!(
        iter::collect_const! {char => iter::repeat_n('c', 2)},
        ['c'; 2]
    );
    assert_eq!(
        iter::collect_const! {char => iter::repeat_n('d', 3)},
        ['d'; 3]
    );
}
