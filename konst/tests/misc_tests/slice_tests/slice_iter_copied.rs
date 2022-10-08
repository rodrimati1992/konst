use konst::slice;

#[test]
fn slice_iter_const_callable() {
    const fn __<'a, T: Copy>(slice: &'a [T]) {
        let _: konst::slice::IterCopied<'a, T> = konst::slice::iter_copied(slice);
        konst::slice::iter_copied(slice).next();
        konst::slice::iter_copied(slice).next_back();
        konst::slice::iter_copied(slice).copy();

        let rev: konst::slice::IterCopiedRev<'a, T> = konst::slice::iter_copied(slice).rev();

        rev.copy();
        let _: konst::slice::IterCopied<'a, T> = rev.copy().rev();
        rev.copy().next();
        rev.copy().next_back();
    }
}

#[test]
fn slice_iter_both_directions() {
    let slice: &[u8] = &[3, 5, 8, 13, 21];

    for iter in vec![
        konst::slice::iter_copied(slice),
        konst::slice::iter_copied(slice).copy(),
        konst::slice::iter_copied(slice).rev().rev(),
    ] {
        assert_eq!(collect_const_iter!(iter), slice);
    }

    for iter in vec![
        konst::slice::iter_copied(slice).rev(),
        konst::slice::iter_copied(slice).copy().rev(),
    ] {
        assert_eq!(
            collect_const_iter!(iter),
            slice.iter().copied().rev().collect::<Vec<u8>>(),
        );
    }
}

#[test]
fn slice_iter_mixed_directions() {
    let slice: &[u8] = &[3, 5, 8, 13, 21];

    let mut iter = konst::slice::iter_copied(slice);
    let mut elem;
    assert_eq!(iter.as_slice(), [3, 5, 8, 13, 21]);

    (elem, iter) = iter.next_back().unwrap();
    assert_eq!(elem, 21);
    assert_eq!(iter.as_slice(), [3, 5, 8, 13]);

    (elem, iter) = iter.next().unwrap();
    assert_eq!(elem, 3);
    assert_eq!(iter.as_slice(), [5, 8, 13]);

    (elem, iter) = iter.next().unwrap();
    assert_eq!(elem, 5);
    assert_eq!(iter.as_slice(), [8, 13]);

    (elem, iter) = iter.next().unwrap();
    assert_eq!(elem, 8);
    assert_eq!(iter.as_slice(), [13]);

    (elem, iter) = iter.next_back().unwrap();
    assert_eq!(elem, 13);
    assert_eq!(iter.as_slice(), []);

    assert!(iter.next().is_none());
}

#[test]
fn slice_iter_rev() {
    let slice: &[u8] = &[3, 5, 8, 13, 21];
    let iter = konst::slice::iter_copied(slice);

    let (elem, iter) = iter.rev().next().unwrap();
    assert_eq!(elem, 21);

    // making sure to call next_back on the reversed iterator
    let (elem, iter) = iter.next_back().unwrap();
    assert_eq!(elem, 3);

    let (elem, iter) = iter.rev().next().unwrap();
    assert_eq!(elem, 5);

    let (elem, iter) = iter.next().unwrap();
    assert_eq!(elem, 8);

    {
        let (elem, iter) = iter.copy().next().unwrap();
        assert_eq!(elem, 13);
        assert!(iter.copy().next_back().is_none());
        assert!(iter.next().is_none());
    }
    {
        let (elem, iter) = iter.next_back().unwrap();
        assert_eq!(elem, 13);
        assert!(iter.copy().next().is_none());
        assert!(iter.next_back().is_none());
    }
}
