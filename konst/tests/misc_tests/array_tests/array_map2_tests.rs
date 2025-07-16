use crate::misc_tests::test_utils::assert_type;

use konst::array;

use std::cell::RefCell;
use std::collections::BTreeSet;

#[repr(transparent)]
#[derive(Debug, PartialEq)]
struct Dropp<T>(T);

impl<T> Drop for Dropp<T> {
    fn drop(&mut self) {}
}

impl<T> Dropp<T> {
    const fn into_inner(self) -> T {
        let this = core::mem::ManuallyDrop::new(self);
        // SAFETY: #[repr(transparent)] guarantees the same representation
        unsafe { std::mem::transmute_copy(&this) }
    }
}

#[derive(Debug, PartialEq)]
struct NonCopy<T>(T);

#[test]
#[should_panic]
#[allow(unreachable_code)]
fn array_map_break() {
    konst::array::map!([(); 3], |_| -> () { break });
}

#[test]
fn array_map_non_copy() {
    const fn map_foos<const N: usize>(input: [NonCopy<u8>; N]) -> [NonCopy<i8>; N] {
        array::map!(input, |nc| NonCopy(nc.0 as i8))
    }

    assert_eq!(
        map_foos([0, 1, 255u8].map(NonCopy)),
        [0, 1, -1i8].map(NonCopy)
    );
}

#[test]
fn array_map_drop() {
    const fn map_foos<const N: usize>(input: [Dropp<u8>; N]) -> [Dropp<i8>; N] {
        array::map!(input, |nc| Dropp(nc.into_inner() as i8))
    }

    assert_eq!(map_foos([0, 1, 255u8].map(Dropp)), [0, 1, -1i8].map(Dropp));
}

#[test]
fn array_map_non_copy_ref_pat() {
    const fn map_foos<const N: usize>(input: [NonCopy<u8>; N]) -> [NonCopy<i8>; N] {
        array::map!(input, |ref x| NonCopy(x.0 as i8))
    }

    assert_eq!(
        map_foos([0, 1, 255u8].map(NonCopy)),
        [0, 1, -1i8].map(NonCopy)
    );
}

#[derive(Debug, PartialEq)]
struct ToSet<'a>(u128, &'a RefCell<BTreeSet<u128>>);

impl Drop for ToSet<'_> {
    fn drop(&mut self) {
        self.1.borrow_mut().insert(self.0);
    }
}

#[test]
fn array_map_nonlocal_return() {
    fn inner<const N: usize>(input: [ToSet<'_>; N], break_at: usize) -> Option<[ToSet<'_>; N]> {
        let mut i = 0;

        Some(array::map!(input, |mut sb| if i < break_at {
            sb.0 += 100;
            i += 1;
            sb
        } else {
            return None;
        }))
    }

    const LEN: usize = 4;
    for break_at in 0..LEN {
        let set = RefCell::new(BTreeSet::from([]));
        let input_values = [3, 5, 8, 13];
        let array: [_; LEN] = input_values.map(|x| ToSet(x, &set));

        assert_eq!(inner(array, break_at), None);

        let mut expected: [_; LEN] = std::array::from_fn(|i| {
            if i < break_at {
                input_values[i] + 100
            } else {
                input_values[i]
            }
        });
        expected.sort();

        assert!(set.borrow().iter().eq(&expected), "{set:?}\n{expected:?}");
    }

    {
        let set = RefCell::new(BTreeSet::from([]));
        let array = [ToSet(3, &set), ToSet(4, &set), ToSet(5, &set)];

        let ret = inner(array, usize::MAX);
        assert!(ret.is_some());

        assert!(set.borrow().iter().eq(&[]), "{set:?}");

        drop(ret);

        assert!(set.borrow().iter().eq(&[103, 104, 105]), "{set:?}");
    }
}

#[test]
fn array_map_parameter_type_annotation() {
    macro_rules! with_comma {
        (($($p:tt)*) ($($e:tt)*)) => ({
            // ensuring that the type annotation is used
            const SQUARED: [u32; 3] =
                array::map!([3, 5, 8], |x: u32 $($p)*| x.pow(2) $($e)*);

            assert_eq!(SQUARED, [9, 25, 64]);
        })
    }

    with_comma! {() ()}
    with_comma! {(,) ()}
    with_comma! {() (,)}
    with_comma! {(,) (,)}
}

#[test]
fn array_map_pattern_param() {
    struct Foo(u32, u32);

    macro_rules! with_comma {
        (($($p:tt)*) ($($e:tt)*)) => ({
            // ensuring that the type annotation is used
            const FIBB: [u32; 3] = array::map!(
                [Foo(3, 5), Foo(8, 13), Foo(21, 34)],
                |Foo(l, r) $($p)*| l + r $($e)*
            );

            assert_eq!(FIBB, [8, 21, 55]);
        })
    }

    with_comma! {() ()}
    with_comma! {(,) ()}
    with_comma! {() (,)}
    with_comma! {(,) (,)}
}

#[test]
fn array_map_pass_function_as_arg() {
    const X: [Option<u32>; 2] = array::map!([10, 20], Some);
    const Y: [u32; 3] = array::map!([0b1, 0b11, 0b111], <u32>::count_ones);

    assert_eq!(X, [Some(10), Some(20)]);
    assert_eq!(Y, [1, 2, 3]);
}

#[test]
fn array_map_with_return_type_annotation() {
    use konst::array::map;

    let mapped = map!([(); 3], |_| -> u32 { Default::default() });
    assert_type::<_, [u32; 3]>(&mapped);
}

#[test]
fn array_map_infer_returned_length() {
    let mapped: &[_] = &konst::array::map!([3, 5, 8], |x| x * 2);
    assert_eq!(mapped, &[6, 10, 16][..]);
}

/////////////////////////////////////////////////
// from_fn tests

#[test]
fn array_from_fn_tests() {
    use konst::array::from_fn;

    {
        const fn evens<const N: usize>() -> [usize; N] {
            from_fn!(|i| i * 2)
        }

        assert_eq!(evens::<0>(), [0usize; 0]);
        assert_eq!(evens::<1>(), [0usize]);
        assert_eq!(evens::<2>(), [0usize, 2]);
        assert_eq!(evens::<3>(), [0usize, 2, 4]);
    }

    // closure with explicit parameter type
    {
        const XS: [usize; 3] = from_fn!(|x: usize| x * 2);

        assert_eq!(XS, [0, 2, 4]);
    }

    // closure with explicit return type
    {
        let xs: [_; 3] = from_fn!(|_| -> &str { Default::default() });
        assert_type::<_, [&str; 3]>(&xs);
    }

    // explicit array type, infer elem type
    {
        let xs = from_fn!([_; 3] => |x| (x as u32).pow(2));
        assert_eq!(xs, [0, 1, 4]);
    }

    // explicit array type
    {
        let xs = from_fn!([u32; 3] => |x| x as _);
        assert_eq!(xs, [0, 1, 2]);
    }

    // explicit array type, parenthesized
    {
        let xs = from_fn!((Array<u32, 3>) => |x| x as _);
        assert_eq!(xs, [0, 1, 2]);
    }

    // explicit array type, unparenthesized
    {
        let xs = from_fn!(Array<u32, 3> => |x| x as _);
        assert_eq!(xs, [0, 1, 2]);
    }

    // explicit array type, parenthesized, infer elem type
    {
        let xs = from_fn!((Array<_, 3>) => |x| x);
        assert_eq!(xs, [0, 1, 2]);
    }

    // explicit array type, single ident
    {
        type Arr = [u32; 3];

        let xs = from_fn!(Arr => |x| x as _);
        assert_eq!(xs, [0, 1, 2]);
    }

    // ensuring that functions can be used
    {
        let xs: [_; 3] = from_fn!(usize_to_str);
        assert_eq!(xs, ["zero", "one", "two"]);
    }
    // ensuring that functions can be used, and also explicit array type
    {
        assert_eq!(
            from_fn!([_; 4] => usize_to_str),
            ["zero", "one", "two", "three"]
        );
    }
}

const fn usize_to_str(i: usize) -> &'static str {
    ["zero", "one", "two", "three", "four"][i]
}

type Array<T, const N: usize> = [T; N];

#[test]
fn array_from_fn_non_copy() {
    assert_eq!(
        konst::array::from_fn!([NonCopy<usize>; 3] => NonCopy),
        [0usize, 1, 2].map(NonCopy),
    );
}

#[test]
fn array_from_fn_nonlocal_return() {
    fn inner<const N: usize>(
        set: &RefCell<BTreeSet<u128>>,
        break_at: usize,
    ) -> Option<[ToSet<'_>; N]> {
        Some(konst::array::from_fn!(|i| if i < break_at {
            ToSet(i as u128, &set)
        } else {
            return None;
        }))
    }

    const LEN: usize = 4;
    for break_at in 0..LEN {
        let set = RefCell::new(BTreeSet::from([]));

        assert_eq!(inner::<LEN>(&set, break_at), None);

        assert!(
            set.borrow().iter().copied().eq(0..break_at as u128),
            "{set:?}\n{break_at}"
        );
    }

    {
        let set = RefCell::new(BTreeSet::from([]));

        let ret = inner::<LEN>(&set, usize::MAX);

        assert!(ret.is_some());
        assert!(set.borrow().iter().eq(&[]), "{set:?}");

        drop(ret);

        assert!(set.borrow().iter().copied().eq(0..LEN as u128), "{set:?}");
    }
}
