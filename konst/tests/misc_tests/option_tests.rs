use core::mem::{ManuallyDrop, forget};

use konst::{manually_drop as md, option};

#[cfg(feature = "iter")]
mod option_iter_tests;

struct ImplsDrop<T>(T);

impl<T> ImplsDrop<T> {
    const fn new(val: T) -> Self {
        Self(val)
    }

    const fn into_inner(self) -> T {
        unsafe {
            let this = ManuallyDrop::new(self);

            core::mem::transmute_copy(&*md::as_inner(&this))
        }
    }
}

impl<T> Drop for ImplsDrop<T> {
    fn drop(&mut self) {}
}

#[track_caller]
const fn unreach<T>() -> T {
    unreachable!()
}

#[test]
fn unwrap_or_else_droppable_test() {
    const fn constness() -> [ImplsDrop<u32>; 2] {
        [
            option::unwrap_or_else!(None::<ImplsDrop<u32>>, || ImplsDrop::new(3)),
            option::unwrap_or_else!(Some(ImplsDrop::new(5)), || unreach()),
        ]
    }

    assert_eq!(constness().map(ImplsDrop::into_inner), [3u32, 5]);
}

#[test]
fn ok_or_else_droppable_test() {
    const fn constness() -> [Result<ImplsDrop<u32>, ImplsDrop<&'static str>>; 2] {
        [
            option::ok_or_else!(None::<ImplsDrop<u32>>, || ImplsDrop::new("hello")),
            option::ok_or_else!(Some(ImplsDrop::new(5)), || unreach()),
        ]
    }

    assert_eq!(
        constness().map(|opt| opt
            .map(ImplsDrop::into_inner)
            .map_err(ImplsDrop::into_inner)),
        [Err("hello"), Ok(5u32)],
    );
}

#[test]
fn map_droppable_test() {
    const fn constness() -> [Option<ImplsDrop<u32>>; 2] {
        [
            option::map!(None::<ImplsDrop<u32>>, |x| {
                forget(x);
                unreach()
            }),
            option::map!(Some(ImplsDrop::new(5)), |x| ImplsDrop::new(
                x.into_inner() * 7
            )),
        ]
    }

    assert_eq!(
        constness().map(|opt| opt.map(ImplsDrop::into_inner)),
        [None, Some(35u32)],
    );
}

#[test]
fn and_then_droppable_test() {
    const fn constness() -> [Option<ImplsDrop<u32>>; 3] {
        [
            option::and_then!(None::<ImplsDrop<u32>>, |x| {
                forget(x);
                unreach()
            }),
            option::and_then!(Some(ImplsDrop::new(5)), |x| Some(ImplsDrop::new(
                x.into_inner() * 7
            ))),
            option::and_then!(Some(ImplsDrop::new(8)), |x| {
                forget(x);
                None
            }),
        ]
    }

    assert_eq!(
        constness().map(|opt| opt.map(ImplsDrop::into_inner)),
        [None, Some(35u32), None],
    );
}

#[test]
fn or_else_droppable_test() {
    const fn constness() -> [Option<ImplsDrop<u32>>; 3] {
        [
            option::or_else!(None::<ImplsDrop<u32>>, || None),
            option::or_else!(None::<ImplsDrop<u32>>, || Some(ImplsDrop::new(5))),
            option::or_else!(Some(ImplsDrop::new(8)), || unreach()),
        ]
    }

    assert_eq!(
        constness().map(|opt| opt.map(ImplsDrop::into_inner)),
        [None, Some(5u32), Some(8u32)],
    );
}
