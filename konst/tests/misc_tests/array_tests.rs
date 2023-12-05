use crate::misc_tests::test_utils::assert_type;

use konst::array;

#[derive(Debug, PartialEq)]
struct NonCopy<T>(T);

#[test]
#[should_panic]
#[allow(unreachable_code)]
fn array_map_break() {
    konst::array::map!([(); 3], |_| break);
}

#[test]
fn array_map_non_copy() {
    const fn map_foos<const N: usize>(input: [NonCopy<u8>; N]) -> [NonCopy<i8>; N] {
        array::map!(input, |(NonCopy(x))| NonCopy(x as i8))
    }

    assert_eq!(
        map_foos([0, 1, 255u8].map(NonCopy)),
        [0, 1, -1i8].map(NonCopy)
    );
}

#[test]
fn array_map_non_copy_ref_pat() {
    const fn map_foos<const N: usize>(input: [NonCopy<u8>; N]) -> [NonCopy<i8>; N] {
        array::map!(input, |(ref x)| NonCopy(x.0 as i8))
    }

    assert_eq!(
        map_foos([0, 1, 255u8].map(NonCopy)),
        [0, 1, -1i8].map(NonCopy)
    );
}

#[test]
fn array_map_nonlocal_return() {
    const fn map_evens<const N: usize>(input: [u8; N]) -> Option<[u8; N]> {
        Some(array::map!(input, |x| if x % 2 == 0 {
            x * 2
        } else {
            return None;
        }))
    }

    assert_eq!(map_evens([2, 4, 6]), Some([4, 8, 12]));
    assert_eq!(map_evens([2, 3, 6]), None);
}

#[test]
fn array_map_type_annotation() {
    // ensuring that the type annotation is used
    const SQUARED: [u32; 3] = array::map!([3, 5, 8], |x: u32| x.pow(2));

    assert_eq!(SQUARED, [9, 25, 64]);
}

#[test]
fn array_map_pass_function_as_arg() {
    const X: [Option<u32>; 2] = array::map!([10, 20], Some);
    const Y: [u32; 3] = array::map!([0b1, 0b11, 0b111], <u32>::count_ones);

    assert_eq!(X, [Some(10), Some(20)]);
    assert_eq!(Y, [1, 2, 3]);
}

#[test]
fn array_map_more_tests() {
    use konst::array::map;

    {
        let mapped = map!([(); 3], |_| -> u32 { Default::default() });
        assert_type::<_, [u32; 3]>(&mapped);
    }
}

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

#[test]
fn array_from_fn_non_copy() {
    assert_eq!(
        konst::array::from_fn!([NonCopy<usize>; 3] => NonCopy),
        [0usize, 1, 2].map(NonCopy),
    );
}

#[test]
#[should_panic]
#[allow(unreachable_code)]
fn array_from_fn_break() {
    konst::array::from_fn!([(); 3] => |_| break);
}

const fn usize_to_str(i: usize) -> &'static str {
    ["zero", "one", "two", "three", "four"][i]
}

type Array<T, const N: usize> = [T; N];
