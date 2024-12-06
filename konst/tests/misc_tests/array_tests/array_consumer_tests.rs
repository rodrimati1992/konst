use std::cell::RefCell;
use std::collections::BTreeSet;
use std::mem::ManuallyDrop;

use konst::array::ArrayConsumer;

const fn md<T>(md: T) -> core::mem::ManuallyDrop<T> {
    core::mem::ManuallyDrop::new(md)
}

#[test]
fn new_test() {
    const fn _callable<T, const LEN: usize>(arr: [T; LEN]) -> ArrayConsumer<T, LEN> {
        ArrayConsumer::new(arr)
    }
}

#[test]
fn empty_test() {
    const fn _callable<T, const LEN: usize>() -> ArrayConsumer<T, LEN> {
        ArrayConsumer::empty()
    }
}

#[test]
fn assert_is_empty_test() {
    const fn _callable<T, const LEN: usize>(ac: ArrayConsumer<T, LEN>) {
        ac.assert_is_empty();
    }

    {
        let iter: ArrayConsumer<u8, 0> = ArrayConsumer::new([]);
        iter.assert_is_empty();
    }

    {
        let mut iter = ArrayConsumer::new([3, 5, 8]);
        assert_eq!(iter.next(), Some(md(3)));
        assert_eq!(iter.next(), Some(md(5)));
        assert_eq!(iter.next(), Some(md(8)));
        assert_eq!(iter.next(), None);
        iter.assert_is_empty();
    }

    {
        let mut iter = ArrayConsumer::new([3, 5, 8]);
        assert_eq!(iter.next_back(), Some(md(8)));
        assert_eq!(iter.next_back(), Some(md(5)));
        assert_eq!(iter.next_back(), Some(md(3)));
        assert_eq!(iter.next_back(), None);
        iter.assert_is_empty();
    }

    {
        let mut iter = ArrayConsumer::new([3, 5, 8, 13]);
        assert_eq!(iter.next(), Some(md(3)));
        assert_eq!(iter.next_back(), Some(md(13)));
        assert_eq!(iter.next_back(), Some(md(8)));
        assert_eq!(iter.next(), Some(md(5)));
        assert_eq!(iter.next_back(), None);
        iter.assert_is_empty();
    }
}

#[test]
fn as_slice_test() {
    const fn _callable<T, const LEN: usize>(ac: &ArrayConsumer<T, LEN>) -> &[T] {
        ac.as_slice()
    }
    const fn _callable_mut<T, const LEN: usize>(ac: &mut ArrayConsumer<T, LEN>) -> &mut [T] {
        ac.as_mut_slice()
    }

    {
        let mut iter: ArrayConsumer<u8, 0> = ArrayConsumer::new([]);
        assert_eq!(iter.as_slice(), &[][..]);
        assert_eq!(iter.as_mut_slice(), &[][..]);
    }

    {
        let mut iter = ArrayConsumer::new([3, 5, 8, 13]);
        assert_eq!(iter.as_slice(), &[3, 5, 8, 13][..]);
        assert_eq!(iter.as_mut_slice(), &mut [3, 5, 8, 13][..]);

        assert_eq!(iter.next(), Some(md(3)));
        assert_eq!(iter.as_slice(), &[5, 8, 13][..]);
        assert_eq!(iter.as_mut_slice(), &mut [5, 8, 13][..]);

        assert_eq!(iter.next_back(), Some(md(13)));
        assert_eq!(iter.as_slice(), &[5, 8][..]);
        assert_eq!(iter.as_mut_slice(), &mut [5, 8][..]);

        assert_eq!(iter.next_back(), Some(md(8)));
        assert_eq!(iter.as_slice(), &[5][..]);
        assert_eq!(iter.as_mut_slice(), &mut [5][..]);

        assert_eq!(iter.next(), Some(md(5)));
        assert_eq!(iter.as_slice(), &[][..]);
        assert_eq!(iter.as_mut_slice(), &mut [][..]);

        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.as_slice(), &[][..]);
        assert_eq!(iter.as_mut_slice(), &mut [][..]);

        iter.assert_is_empty();
    }
}

#[test]
fn next_test() {
    const fn _callable<T, const LEN: usize>(ac: &mut ArrayConsumer<T, LEN>) -> Option<ManuallyDrop<T>> {
        ac.next()
    }

    let mut iter = ArrayConsumer::new([3, 5, 8]);
    assert_eq!(iter.as_slice(), &[3, 5, 8][..]);
    assert_eq!(iter.as_mut_slice(), &mut [3, 5, 8][..]);

    assert_eq!(iter.next(), Some(md(3)));
    assert_eq!(iter.as_slice(), &[5, 8][..]);
    assert_eq!(iter.as_mut_slice(), &mut [5, 8][..]);

    assert_eq!(iter.next(), Some(md(5)));
    assert_eq!(iter.as_slice(), &[8][..]);
    assert_eq!(iter.as_mut_slice(), &mut [8][..]);

    assert_eq!(iter.next(), Some(md(8)));
    assert_eq!(iter.as_slice(), &[][..]);
    assert_eq!(iter.as_mut_slice(), &mut [][..]);

    assert_eq!(iter.next(), None);
    assert_eq!(iter.as_slice(), &[][..]);
    assert_eq!(iter.as_mut_slice(), &mut [][..]);
    
    iter.assert_is_empty();
}

#[test]
fn next_back_test() {
    const fn _callable<T, const LEN: usize>(ac: &mut ArrayConsumer<T, LEN>) -> Option<ManuallyDrop<T>> {
        ac.next_back()
    }

    let mut iter = ArrayConsumer::new([3, 5, 8]);
    assert_eq!(iter.as_slice(), &[3, 5, 8][..]);
    assert_eq!(iter.as_mut_slice(), &mut [3, 5, 8][..]);

    assert_eq!(iter.next_back(), Some(md(8)));
    assert_eq!(iter.as_slice(), &[3, 5][..]);
    assert_eq!(iter.as_mut_slice(), &mut [3, 5][..]);

    assert_eq!(iter.next_back(), Some(md(5)));
    assert_eq!(iter.as_slice(), &[3][..]);
    assert_eq!(iter.as_mut_slice(), &mut [3][..]);

    assert_eq!(iter.next_back(), Some(md(3)));
    assert_eq!(iter.as_slice(), &[][..]);
    assert_eq!(iter.as_mut_slice(), &mut [][..]);

    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.as_slice(), &[][..]);
    assert_eq!(iter.as_mut_slice(), &mut [][..]);

    iter.assert_is_empty();
}

#[test]
fn copy_test() {
    const fn _callable<T: Copy, const LEN: usize>(ac: &ArrayConsumer<T, LEN>) -> ArrayConsumer<T, LEN> {
        ac.copy()
    }

    let mut consumer = ArrayConsumer::new([3, 5, 8, 13, 21, 34]);
    _ = consumer.next().map(ManuallyDrop::into_inner);
    _ = consumer.next_back().map(ManuallyDrop::into_inner);
    _ = consumer.next_back().map(ManuallyDrop::into_inner);

    assert_eq!(consumer.as_slice(), &[5, 8, 13][..]);
    assert_eq!(consumer.copy().as_slice(), &[5, 8, 13][..]);
}

#[test]
fn clone_test() {
    fn _callable<T: Clone, const LEN: usize>(ac: &ArrayConsumer<T, LEN>) -> ArrayConsumer<T, LEN> {
        ac.clone()
    }

    let ts = |x: i32| x.to_string();

    let mut consumer = ArrayConsumer::new([3, 5, 8, 13, 21, 34].map(ts));
    _ = consumer.next().map(ManuallyDrop::into_inner);
    _ = consumer.next_back().map(ManuallyDrop::into_inner);
    _ = consumer.next_back().map(ManuallyDrop::into_inner);

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

    let mut iter = ArrayConsumer::new([3, 5, 8, 13, 21, 34, 55].map(|x| ToSet(x, &set)));

    assert!(set.borrow().is_empty());

    let _ = iter.next().map(ManuallyDrop::into_inner);
    assert!(set.borrow().iter().copied().eq([3u128]), "{set:?}");

    let _ = iter.next_back().map(ManuallyDrop::into_inner);
    assert!(set.borrow().iter().copied().eq([3u128, 55]), "{set:?}");

    let _ = iter.next_back().map(ManuallyDrop::into_inner);
    assert!(set.borrow().iter().copied().eq([3u128, 34, 55]), "{set:?}");

    let _ = iter.next().map(ManuallyDrop::into_inner);
    assert!(set.borrow().iter().copied().eq([3u128, 5, 34, 55]), "{set:?}");

    drop(iter);

    assert!(set.borrow().iter().copied().eq([3u128, 5, 8, 13, 21, 34, 55]), "{set:?}");
}







