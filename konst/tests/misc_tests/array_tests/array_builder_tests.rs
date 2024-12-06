use std::cell::RefCell;
use std::collections::BTreeSet;

use konst::array::{ArrayBuilder, ArrayConsumer};

#[test]
fn new_test() {
    const fn _callable<T, const N: usize>() -> ArrayBuilder<T, N> {
        ArrayBuilder::new()
    }
}

#[test]
fn len_and_is_full_test() {
    const fn _callable<T, const N: usize>(this: &ArrayBuilder<T, N>) -> (usize, bool) {
        (this.len(), this.is_full())
    }

    let mut this: ArrayBuilder<_, 3> = ArrayBuilder::new();

    assert_eq!(this.len(), 0);
    assert!(!this.is_full());

    this.push(3);
    assert_eq!(this.len(), 1);
    assert!(!this.is_full());

    this.push(5);
    assert_eq!(this.len(), 2);
    assert!(!this.is_full());

    this.push(8);
    assert_eq!(this.len(), 3);
    assert!(this.is_full());
}

#[test]
fn as_slice_test() {
    const fn _callable<'a, T, const N: usize>(this: &'a ArrayBuilder<T, N>) {
        let _: &'a [T] = this.as_slice();
    }
    const fn _callable_mut<'a, T, const N: usize>(this: &'a mut ArrayBuilder<T, N>) {
        let _: &'a mut [T] = this.as_mut_slice();
    }

    let mut this: ArrayBuilder<_, 3> = ArrayBuilder::new();

    assert_eq!((&this).as_slice(), &[][..]);
    assert_eq!(this.as_mut_slice(), &mut [][..]);

    this.push(3);
    assert_eq!((&this).as_slice(), &[3][..]);
    assert_eq!(this.as_mut_slice(), &mut [3][..]);

    this.push(5);
    assert_eq!((&this).as_slice(), &[3, 5][..]);
    assert_eq!(this.as_mut_slice(), &mut [3, 5][..]);

    this.push(8);
    assert_eq!((&this).as_slice(), &[3, 5, 8][..]);
    assert_eq!(this.as_mut_slice(), &mut [3, 5, 8][..]);
}

#[test]
fn push_build_test() {
    let mut this: ArrayBuilder<_, 3> = ArrayBuilder::new();

    this.push(3);
    this.push(5);
    this.push(8);

    assert_eq!(this.build(), [3, 5, 8]);
}

#[test]
#[should_panic]
fn push_panics_test() {
    let mut this: ArrayBuilder<_, 3> = ArrayBuilder::new();

    this.push(3);
    this.push(5);
    this.push(8);
    this.push(13);
}

#[test]
#[should_panic]
fn build_panics_test() {
    let this: ArrayBuilder<u32, 5> = ArrayBuilder::new();

    let _ = this.build();
}


#[test]
fn infer_length_from_consumer_test() {
    const fn _callable<T, const LEN: usize>(ab: &ArrayBuilder<T, LEN>, ac: &ArrayConsumer<T, LEN>) {
        ab.infer_length_from_consumer(ac);
    }

    let mut this = ArrayBuilder::new();
    this.infer_length_from_consumer(&ArrayConsumer::new([0, 0, 0]));

    this.push(3);
    this.push(5);
    this.push(8);

    assert_eq!(this.build(), [3, 5, 8]);
}

#[test]
fn copy_test() {
    const fn _callable<T: Copy, const LEN: usize>(ac: &ArrayBuilder<T, LEN>) -> ArrayBuilder<T, LEN> {
        ac.copy()
    }

    let mut builder = ArrayBuilder::<i32, 6>::new();
    builder.push(5);
    builder.push(8);
    builder.push(13);

    assert_eq!(builder.as_slice(), &[5, 8, 13][..]);
    assert_eq!(builder.copy().as_slice(), &[5, 8, 13][..]);
}

#[test]
fn clone_test() {
    fn _callable<T: Clone, const LEN: usize>(ac: &ArrayBuilder<T, LEN>) -> ArrayBuilder<T, LEN> {
        ac.clone()
    }


    let mut builder = ArrayBuilder::<String, 6>::new();
    builder.push(5.to_string());
    builder.push(8.to_string());
    builder.push(13.to_string());

    let ts = |x: i32| x.to_string();
    assert_eq!(builder.as_slice(), &[5, 8, 13].map(ts)[..]);
    assert_eq!(builder.clone().as_slice(), &[5, 8, 13].map(ts)[..]);
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

    let mut builder: ArrayBuilder<_, 5> = ArrayBuilder::new();

    builder.push(ToSet(3, &set));
    builder.push(ToSet(5, &set));
    builder.push(ToSet(8, &set));

    assert!(set.borrow().is_empty());

    drop(builder);

    assert!(set.borrow().iter().copied().eq([3u128, 5, 8]), "{set:?}");
}



