use crate::misc_tests::test_utils::assert_type;

use std::cell::RefCell;
use std::collections::BTreeSet;

use konst::array::{ArrayBuilder, IntoIter};
use konst::drop_flavor::{DropFlavor, MayDrop, NonDrop};

#[test]
fn constructors_const_and_type_test() {
    const fn _callable0<T, const N: usize>() -> ArrayBuilder<T, N, MayDrop> {
        ArrayBuilder::of_drop()
    }
    const fn _callable1<T: Copy, const N: usize>() -> ArrayBuilder<T, N, NonDrop> {
        ArrayBuilder::of_copy()
    }

    assert_type::<_, ArrayBuilder<u8, 0, MayDrop>>(&ArrayBuilder::<u8, 0, _>::of_drop());

    assert_type::<_, ArrayBuilder<u8, 0, NonDrop>>(&ArrayBuilder::<u8, 0, _>::of_copy());
}

#[test]
fn len_and_is_full_test() {
    const fn _callable<T, D, const N: usize>(this: &ArrayBuilder<T, N, D>) -> (usize, bool)
    where
        D: DropFlavor,
    {
        (this.len(), this.is_full())
    }

    macro_rules! case {
        ($ctor:ident) => {{
            let mut this = ArrayBuilder::$ctor::<3>();

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
        }};
    }

    case! {of_drop}
    case! {of_copy}
}

#[test]
fn as_slice_test() {
    const fn _callable<'a, T, D, const N: usize>(this: &'a ArrayBuilder<T, N, D>)
    where
        D: DropFlavor,
    {
        let _: &'a [T] = this.as_slice();
    }
    const fn _callable_mut<'a, T, D, const N: usize>(this: &'a mut ArrayBuilder<T, N, D>)
    where
        D: DropFlavor,
    {
        let _: &'a mut [T] = this.as_mut_slice();
    }

    macro_rules! case {
        ($ctor:ident) => {{
            let mut this = ArrayBuilder::$ctor::<3>();

            assert_eq!((&this).as_slice(), &[0; 0][..]);
            assert_eq!(this.as_mut_slice(), &mut [0; 0][..]);

            this.push(3);
            assert_eq!((&this).as_slice(), &[3][..]);
            assert_eq!(this.as_mut_slice(), &mut [3][..]);

            this.push(5);
            assert_eq!((&this).as_slice(), &[3, 5][..]);
            assert_eq!(this.as_mut_slice(), &mut [3, 5][..]);

            this.push(8);
            assert_eq!((&this).as_slice(), &[3, 5, 8][..]);
            assert_eq!(this.as_mut_slice(), &mut [3, 5, 8][..]);
        }};
    }

    case! {of_drop}
    case! {of_copy}
}

#[test]
fn push_build_test() {
    macro_rules! case {
        ($ctor:ident) => {{
            let mut this = ArrayBuilder::$ctor::<3>();

            this.push(3);
            this.push(5);
            this.push(8);

            assert_eq!(this.build(), [3, 5, 8]);
        }};
    }

    case! {of_drop}
    case! {of_copy}
}

#[test]
#[should_panic]
fn push_panics_of_drop_test() {
    let mut this = ArrayBuilder::of_drop::<3>();

    this.push(3);
    this.push(5);
    this.push(8);
    this.push(13);
}

#[test]
#[should_panic]
fn push_panics_copy_test() {
    let mut this = ArrayBuilder::of_copy::<3>();

    this.push(3);
    this.push(5);
    this.push(8);
    this.push(13);
}

#[test]
#[should_panic]
fn build_panics_test() {
    let this: ArrayBuilder<u32, 5, MayDrop> = ArrayBuilder::of_drop();

    let _ = this.build();
}

#[test]
fn infer_length_from_consumer_test() {
    const fn _callable<T, D: DropFlavor, D2: DropFlavor, const LEN: usize>(
        ab: &ArrayBuilder<T, LEN, D>,
        ac: &IntoIter<T, LEN, D2>,
    ) {
        ab.infer_length_from_consumer(ac);
    }

    let mut this = ArrayBuilder::of_drop();
    this.infer_length_from_consumer(&IntoIter::of_copy([0, 0, 0]));

    this.push(3);
    this.push(5);
    this.push(8);

    assert_eq!(this.build(), [3, 5, 8]);
}

#[test]
fn copy_test() {
    const fn _callable<T: Copy, D: DropFlavor, const LEN: usize>(
        ac: &ArrayBuilder<T, LEN, D>,
    ) -> ArrayBuilder<T, LEN, D> {
        ac.copy()
    }

    macro_rules! case {
        ($ctor:ident) => {{
            let mut builder = ArrayBuilder::<i32, 6, _>::$ctor();
            builder.push(5);
            builder.push(8);
            builder.push(13);

            assert_eq!(builder.as_slice(), &[5, 8, 13][..]);
            assert_eq!(builder.copy().as_slice(), &[5, 8, 13][..]);
        }};
    }

    case! {of_drop}
    case! {of_copy}
}

#[test]
fn clone_test() {
    fn _callable<T, D, const LEN: usize>(ac: &ArrayBuilder<T, LEN, D>) -> ArrayBuilder<T, LEN, D>
    where
        T: Clone,
        D: DropFlavor,
    {
        ac.clone()
    }

    let mut builder = ArrayBuilder::<String, 6, _>::of_drop();
    builder.push(5.to_string());
    builder.push(8.to_string());
    builder.push(13.to_string());

    let ts = |x: i32| x.to_string();
    assert_eq!(builder.as_slice(), &[5, 8, 13].map(ts)[..]);
    assert_eq!(builder.clone().as_slice(), &[5, 8, 13].map(ts)[..]);
}

#[test]
fn default_test() {
    let builder_copy: ArrayBuilder<u8, 4, NonDrop> = Default::default();
    let builder_drop: ArrayBuilder<String, 4, MayDrop> = Default::default();

    assert!(builder_copy.is_empty());
    assert!(builder_drop.is_empty());
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

    let mut builder = ArrayBuilder::of_drop::<5>();

    builder.push(ToSet(3, &set));
    builder.push(ToSet(5, &set));
    builder.push(ToSet(8, &set));

    assert!(set.borrow().is_empty());

    drop(builder);

    assert!(set.borrow().iter().copied().eq([3u128, 5, 8]), "{set:?}");
}

#[test]
fn into_drop_and_copy_test() {
    macro_rules! case {
        ($ctor:ident $from_flavor:ident $conv_method:ident $into_flavor:ident) => {
            let mut builder = ArrayBuilder::$ctor::<3>();
            assert_type::<_, ArrayBuilder<&u8, 3, $from_flavor>>(&builder);

            builder.push(&3u8);
            builder.push(&5u8);

            let mut builder = builder.$conv_method();
            assert_type::<_, ArrayBuilder<&u8, 3, $into_flavor>>(&builder);

            builder.push(&8u8);

            assert_eq!(builder.build(), [&3u8, &5, &8]);
        };
    }

    case! {of_copy NonDrop into_drop MayDrop}
    case! {of_drop MayDrop into_copy NonDrop}
}
