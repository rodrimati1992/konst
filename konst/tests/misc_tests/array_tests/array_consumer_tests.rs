use std::cell::RefCell;
use std::collections::BTreeSet;

use konst::array::{IntoIter, IntoIterRev};
use konst::iter::into_iter;

#[test]
fn new_test() {
    const fn _callable<T, const LEN: usize>(arr: [T; LEN]) -> IntoIter<T, LEN> {
        IntoIter::new(arr)
    }
}

#[test]
fn empty_test() {
    const fn _callable<T, const LEN: usize>() -> IntoIter<T, LEN> {
        IntoIter::empty()
    }
}

#[should_panic]
#[test]
fn assert_is_empty_panics_test() {
    IntoIter::new([3]).assert_is_empty();
}

#[should_panic]
#[test]
fn assert_is_empty_rev_panics_test() {
    IntoIter::new([3]).rev().assert_is_empty();
}

#[test]
fn assert_is_empty_test() {
    const fn _callable<T, const LEN: usize>(ac: IntoIter<T, LEN>) {
        ac.assert_is_empty();
    }

    {
        let iter: IntoIter<u8, 0> = IntoIter::new([]);
        iter.assert_is_empty();
    }

    {
        let mut iter = IntoIter::new([3, 5, 8]);
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), None);
        iter.assert_is_empty();
    }

    {
        let mut iter = IntoIter::new([3, 5, 8]);
        assert_eq!(iter.next_back(), Some(8));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), None);
        iter.assert_is_empty();
    }

    {
        let mut iter = IntoIter::new([3, 5, 8, 13]);
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(13));
        assert_eq!(iter.next_back(), Some(8));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next_back(), None);
        iter.assert_is_empty();
    }
}

#[test]
fn assert_is_empty_rev_test() {
    const fn _callable<T, const LEN: usize>(ac: IntoIterRev<T, LEN>) {
        ac.assert_is_empty();
    }

    {
        let mut iter: IntoIterRev<_, 3> = IntoIter::new([3, 5, 8]).rev();
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
        iter.assert_is_empty();
    }
}

#[test]
fn as_slice_test() {
    const fn _callable<T, const LEN: usize>(ac: &IntoIter<T, LEN>) -> &[T] {
        ac.as_slice()
    }
    const fn _callable_mut<T, const LEN: usize>(ac: &mut IntoIter<T, LEN>) -> &mut [T] {
        ac.as_mut_slice()
    }

    {
        let mut iter: IntoIter<u8, 0> = IntoIter::new([]);
        assert_eq!(iter.as_slice(), &[0; 0][..]);
        assert_eq!(iter.as_mut_slice(), &[0; 0][..]);
    }

    {
        let mut iter = IntoIter::new([3, 5, 8, 13]);
        assert_eq!(iter.as_slice(), &[3, 5, 8, 13][..]);
        assert_eq!(iter.as_mut_slice(), &mut [3, 5, 8, 13][..]);

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.as_slice(), &[5, 8, 13][..]);
        assert_eq!(iter.as_mut_slice(), &mut [5, 8, 13][..]);

        assert_eq!(iter.next_back(), Some(13));
        assert_eq!(iter.as_slice(), &[5, 8][..]);
        assert_eq!(iter.as_mut_slice(), &mut [5, 8][..]);

        assert_eq!(iter.next_back(), Some(8));
        assert_eq!(iter.as_slice(), &[5][..]);
        assert_eq!(iter.as_mut_slice(), &mut [5][..]);

        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.as_slice(), &[0; 0][..]);
        assert_eq!(iter.as_mut_slice(), &mut [0; 0][..]);

        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.as_slice(), &[0; 0][..]);
        assert_eq!(iter.as_mut_slice(), &mut [0; 0][..]);

        iter.assert_is_empty();
    }
}

#[test]
fn as_slice_rev_test() {
    const fn _callable<T, const LEN: usize>(ac: &IntoIterRev<T, LEN>) -> &[T] {
        ac.as_slice()
    }
    const fn _callable_mut<T, const LEN: usize>(ac: &mut IntoIterRev<T, LEN>) -> &mut [T] {
        ac.as_mut_slice()
    }

    {
        let mut iter: IntoIterRev<u8, 0> = IntoIter::new([]).rev();
        assert_eq!(iter.as_slice(), &[0; 0][..]);
        assert_eq!(iter.as_mut_slice(), &[0; 0][..]);
    }

    {
        let mut iter = IntoIter::new([3, 5, 8, 13]).rev();
        assert_eq!(iter.as_slice(), &[3, 5, 8, 13][..]);
        assert_eq!(iter.as_mut_slice(), &mut [3, 5, 8, 13][..]);

        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.as_slice(), &[5, 8, 13][..]);
        assert_eq!(iter.as_mut_slice(), &mut [5, 8, 13][..]);

        assert_eq!(iter.next(), Some(13));
        assert_eq!(iter.as_slice(), &[5, 8][..]);
        assert_eq!(iter.as_mut_slice(), &mut [5, 8][..]);

        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.as_slice(), &[5][..]);
        assert_eq!(iter.as_mut_slice(), &mut [5][..]);

        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.as_slice(), &[0; 0][..]);
        assert_eq!(iter.as_mut_slice(), &mut [0; 0][..]);

        assert_eq!(iter.next(), None);
        assert_eq!(iter.as_slice(), &[0; 0][..]);
        assert_eq!(iter.as_mut_slice(), &mut [0; 0][..]);

        iter.assert_is_empty();
    }
}

#[test]
fn next_test() {
    const fn _callable<T, const LEN: usize>(ac: &mut IntoIter<T, LEN>) -> Option<T> {
        ac.next()
    }

    let mut iter = IntoIter::new([3, 5, 8]);
    assert_eq!(iter.as_slice(), &[3, 5, 8][..]);
    assert_eq!(iter.as_mut_slice(), &mut [3, 5, 8][..]);

    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.as_slice(), &[5, 8][..]);
    assert_eq!(iter.as_mut_slice(), &mut [5, 8][..]);

    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.as_slice(), &[8][..]);
    assert_eq!(iter.as_mut_slice(), &mut [8][..]);

    assert_eq!(iter.next(), Some(8));
    assert_eq!(iter.as_slice(), &[0; 0][..]);
    assert_eq!(iter.as_mut_slice(), &mut [0; 0][..]);

    assert_eq!(iter.next(), None);
    assert_eq!(iter.as_slice(), &[0; 0][..]);
    assert_eq!(iter.as_mut_slice(), &mut [0; 0][..]);

    iter.assert_is_empty();
}

#[test]
fn next_back_test() {
    const fn _callable<T, const LEN: usize>(ac: &mut IntoIter<T, LEN>) -> Option<T> {
        ac.next_back()
    }

    let mut iter: IntoIter<_, 3> = into_iter!([3, 5, 8]);
    assert_eq!(iter.as_slice(), &[3, 5, 8][..]);
    assert_eq!(iter.as_mut_slice(), &mut [3, 5, 8][..]);

    assert_eq!(iter.next_back(), Some(8));
    assert_eq!(iter.as_slice(), &[3, 5][..]);
    assert_eq!(iter.as_mut_slice(), &mut [3, 5][..]);

    assert_eq!(iter.next_back(), Some(5));
    assert_eq!(iter.as_slice(), &[3][..]);
    assert_eq!(iter.as_mut_slice(), &mut [3][..]);

    assert_eq!(iter.next_back(), Some(3));
    assert_eq!(iter.as_slice(), &[0; 0][..]);
    assert_eq!(iter.as_mut_slice(), &mut [0; 0][..]);

    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.as_slice(), &[0; 0][..]);
    assert_eq!(iter.as_mut_slice(), &mut [0; 0][..]);

    iter.assert_is_empty();
}

#[test]
fn copy_test() {
    const fn _callable<T: Copy, const LEN: usize>(ac: &IntoIter<T, LEN>) -> IntoIter<T, LEN> {
        ac.copy()
    }

    let mut consumer: IntoIter<_, 6> = into_iter!([3, 5, 8, 13, 21, 34]);
    _ = consumer.next();
    _ = consumer.next_back();
    _ = consumer.next_back();

    assert_eq!(consumer.as_slice(), &[5, 8, 13][..]);
    assert_eq!(consumer.copy().as_slice(), &[5, 8, 13][..]);
}

#[test]
fn copy_rev_test() {
    const fn _callable<T: Copy, const LEN: usize>(ac: &IntoIterRev<T, LEN>) -> IntoIterRev<T, LEN> {
        ac.copy()
    }

    let mut consumer: IntoIterRev<_, 6> = IntoIter::new([3, 5, 8, 13, 21, 34]).rev();
    _ = consumer.next_back();
    _ = consumer.next();
    _ = consumer.next();

    assert_eq!(consumer.as_slice(), &[5, 8, 13][..]);
    assert_eq!(consumer.copy().as_slice(), &[5, 8, 13][..]);
}

#[test]
fn clone_test() {
    fn _callable<T: Clone, const LEN: usize>(ac: &IntoIter<T, LEN>) -> IntoIter<T, LEN> {
        ac.clone()
    }

    let ts = |x: i32| x.to_string();

    let mut consumer = IntoIter::new([3, 5, 8, 13, 21, 34].map(ts));
    _ = consumer.next();
    _ = consumer.next_back();
    _ = consumer.next_back();

    assert_eq!(consumer.as_slice(), &[5, 8, 13].map(ts)[..]);
    assert_eq!(consumer.clone().as_slice(), &[5, 8, 13].map(ts)[..]);
}

#[test]
fn clone_rev_test() {
    fn _callable<T: Clone, const LEN: usize>(ac: &IntoIterRev<T, LEN>) -> IntoIterRev<T, LEN> {
        ac.clone()
    }

    let ts = |x: i32| x.to_string();

    let mut consumer = IntoIter::new([3, 5, 8, 13, 21, 34].map(ts)).rev();
    _ = consumer.next_back();
    _ = consumer.next();
    _ = consumer.next();

    assert_eq!(consumer.as_slice(), &[5, 8, 13].map(ts)[..]);
    assert_eq!(consumer.clone().as_slice(), &[5, 8, 13].map(ts)[..]);
}

#[derive(Debug, PartialEq)]
struct ToSet<'a>(u128, &'a RefCell<BTreeSet<u128>>);

impl Drop for ToSet<'_> {
    fn drop(&mut self) {
        self.1.borrow_mut().insert(self.0);
    }
}

#[test]
fn drop_test() {
    let set = RefCell::new(BTreeSet::from([]));

    let mut iter = IntoIter::new([3, 5, 8, 13, 21, 34, 55].map(|x| ToSet(x, &set)));

    assert!(set.borrow().is_empty());

    let _ = iter.next();
    assert!(set.borrow().iter().copied().eq([3u128]), "{set:?}");

    let _ = iter.next_back();
    assert!(set.borrow().iter().copied().eq([3u128, 55]), "{set:?}");

    let _ = iter.next_back();
    assert!(set.borrow().iter().copied().eq([3u128, 34, 55]), "{set:?}");

    let _ = iter.next();
    assert!(
        set.borrow().iter().copied().eq([3u128, 5, 34, 55]),
        "{set:?}"
    );

    drop(iter);

    assert!(
        set.borrow()
            .iter()
            .copied()
            .eq([3u128, 5, 8, 13, 21, 34, 55]),
        "{set:?}"
    );
}
