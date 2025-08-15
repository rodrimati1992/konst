use core::mem::{ManuallyDrop, forget};

use konst::{manually_drop as md, option};

#[cfg(feature = "cmp")]
mod option_cmp_tests;

#[cfg(feature = "iter")]
mod option_iter_tests;

#[derive(Debug, PartialEq, Eq)]
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
fn filter_test() {
    const fn is_odd(x: &u32) -> bool {
        *x % 2 == 1
    }

    const ARR: &[Option<u32>] = &[
        option::filter!(Some(0), |&x| x == 0),
        option::filter!(Some(1), |x| *x == 0),
        option::filter!(None, |_| loop {}),
        option::filter!(Some(3), is_odd),
        option::filter!(Some(4), is_odd),
        option::filter!(None, is_odd),
    ];

    assert_eq!(ARR, &[Some(0), None, None, Some(3), None, None]);
}

#[test]
fn unwrap_or_test() {
    const ARR: &[u32] = &[
        option::unwrap_or!(Some(3), 10000),
        option::unwrap_or!(None, 5),
    ];

    assert_eq!(ARR, &[3, 5]);
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
fn ok_or_test() {
    const ARR: &[Result<u32, &str>] =
        &[option::ok_or!(Some(3), "ggg"), option::ok_or!(None, "hhh")];
    assert_eq!(ARR, &[Ok(3), Err("hhh")]);
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

#[test]
fn insert_test() {
    const fn constness() -> [Option<u32>; 2] {
        let mut arr = [None, Some(21)];

        *option::insert!(&mut arr[0], 3) += 100;
        *option::insert!(&mut arr[1], 3) += 200;

        arr
    }

    assert_eq!(constness(), [103, 203].map(Some));
}

#[test]
fn get_or_insert_test() {
    const fn constness() -> [Option<u32>; 2] {
        let mut arr = [None, Some(21)];

        *option::get_or_insert!(&mut arr[0], 3) += 100;
        *option::get_or_insert!(&mut arr[1], 3) += 200;

        arr
    }

    assert_eq!(constness(), [103, 221].map(Some));
}

#[test]
fn get_or_insert_with_droppable_test() {
    const fn defaulter() -> ImplsDrop<u32> {
        ImplsDrop::new(8)
    }

    const fn constness() -> [Option<ImplsDrop<u32>>; 3] {
        let mut arr = [None, None, Some(ImplsDrop::new(21))];

        option::get_or_insert_with!(&mut arr[0], || ImplsDrop::new(3)).0 += 100;
        option::get_or_insert_with!(&mut arr[1], defaulter).0 += 100;
        option::get_or_insert_with!(&mut arr[2], || unreach()).0 += 100;

        arr
    }

    assert_eq!(constness(), [103, 108, 121].map(ImplsDrop::new).map(Some));
}

#[test]
fn is_some_and_test() {
    const AA: bool = option::is_some_and!(None::<u8>, |_| unreachable!());
    assert_eq!(AA, false);

    const BB: bool = option::is_some_and!(Some(3), |x| *x == 3);
    assert_eq!(BB, true);

    const CC: bool = option::is_some_and!(&Some(5), is_ten);
    assert_eq!(CC, false);

    const fn is_ten(n: &u8) -> bool {
        *n == 10
    }
}

#[test]
fn is_none_or_test() {
    const AA: bool = option::is_none_or!(None::<u8>, |_| unreachable!());
    assert_eq!(AA, true);

    const BB: bool = option::is_none_or!(Some(3), |x| *x == 3);
    assert_eq!(BB, true);

    const CC: bool = option::is_none_or!(&Some(5), is_ten);
    assert_eq!(CC, false);

    const fn is_ten(n: &u8) -> bool {
        *n == 10
    }
}

#[test]
fn zip_test() {
    const VALS: [Option<(u8, char)>; 4] = [
        option::zip!(None, None),
        option::zip!(None, Some('a')),
        option::zip!(Some(3), None),
        option::zip!(Some(3), Some('a')),
    ];

    assert_eq!(VALS, [None, None, None, Some((3, 'a'))]);
}

#[test]
fn unzip_test() {
    const VALS: [(Option<u8>, Option<char>); 2] =
        [option::unzip(None), option::unzip(Some((3, 'a')))];

    assert_eq!(VALS, [(None, None), (Some(3), Some('a'))]);
}
