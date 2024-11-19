use konst::destructure;


////////////////////////////////////////////////////////////////////////
//                  Braced Structs
////////////////////////////////////////////////////////////////////////

struct BracedStruct<T> {
    foo: String,
    bar: T,
    baz: T,
}

#[test]
fn test_braced_struct_destructuring() {
    const fn no_type_annotation<T>(val: BracedStruct<T>) -> (String, T, T) {
        destructure!{BracedStruct {foo, bar, baz} = val}

        (foo, bar, baz)
    }

    const fn with_type_annotation<T>(val: BracedStruct<T>) -> (String, T, T) {
        destructure!{BracedStruct {foo, bar, baz}: BracedStruct<T> = val}

        (foo, bar, baz)
    }

    const fn with_typed_pattern<T>(val: BracedStruct<T>) -> (String, T, T) {
        destructure!{BracedStruct<T> {foo, bar, baz}: BracedStruct<T> = val}

        (foo, bar, baz)
    }

    const fn renamed_fields<T>(val: BracedStruct<T>) -> (String, T, T) {
        destructure!{BracedStruct {foo: baz, bar, baz: qux} = val}

        (baz, bar, qux)
    }


    for func in [
        no_type_annotation,
        with_type_annotation,
        with_typed_pattern,
        renamed_fields,
    ] {
        let val = BracedStruct {
            foo: "hello".to_string(),
            bar: 3,
            baz: 5,
        };

        assert_eq!(func(val), ("hello".to_string(), 3, 5));
    }
}


#[test]
fn test_braced_struct_other_patterns() {
    {        
        const fn ignore_last<T: Copy>(val: BracedStruct<T>) -> (String, T) {
            destructure!{BracedStruct<T> {foo, bar, baz: _}: BracedStruct<T> = val}

            (foo, bar)
        }

        let val = BracedStruct {
            foo: "hello".to_string(),
            bar: 3,
            baz: 5,
        };

        assert_eq!(ignore_last(val), ("hello".to_string(), 3));
    }

    {        
        const fn destructure_inner<T: Copy>(val: BracedStruct<(T, T)>) -> (String, T, T) {
            destructure!{
                BracedStruct {foo: a, bar: (_, b), baz: (c, _)} = val
            }

            (a, b, c)
        }


        let val = BracedStruct {
            foo: "hello".to_string(),
            bar: (3, 5),
            baz: (8, 13),
        };

        assert_eq!(destructure_inner(val), ("hello".to_string(), 5, 8));
    }
}


////////////////////////////////////////////////////////////////////////
//                  Tuple Structs
////////////////////////////////////////////////////////////////////////

#[repr(packed)]
struct TupleStruct<T>(String, T, T);


#[test]
fn test_packed_tuple_struct_destructuring() {
    const fn no_type_annotation<T>(val: TupleStruct<T>) -> (String, T, T) {
        destructure!{TupleStruct (foo, bar, baz) = val}

        (foo, bar, baz)
    }

    const fn with_type_annotation<T>(val: TupleStruct<T>) -> (String, T, T) {
        destructure!{TupleStruct (foo, bar, baz): TupleStruct<T> = val}

        (foo, bar, baz)
    }

    const fn with_typed_pattern<T>(val: TupleStruct<T>) -> (String, T, T) {
        destructure!{TupleStruct<T>, (foo, bar, baz): TupleStruct<T> = val}

        (foo, bar, baz)
    }

    for func in [
        no_type_annotation,
        with_type_annotation,
        with_typed_pattern,
    ] {
        let val = TupleStruct("hello".to_string(), 3, 5);

        assert_eq!(func(val), ("hello".to_string(), 3, 5));
    }
}


#[test]
fn test_ignore_tuple_struct_field() {
    const fn func<T: Copy>(val: TupleStruct<T>) -> (String, T) {
        destructure!{TupleStruct(foo, _, baz): TupleStruct<T> = val}

        (foo, baz)
    }

    let val = TupleStruct("hello".to_string(), 3, 5);

    assert_eq!(func(val), ("hello".to_string(), 5));
}


////////////////////////////////////////////////////////////////////////
//                      Tuples
////////////////////////////////////////////////////////////////////////


type Tuple<T> = (String, T, T);

#[test]
fn test_tuple_destructuring() {
    const fn no_type_annotation<T>(val: Tuple<T>) -> (T, String, T) {
        destructure!{(foo, bar, baz) = val}

        (bar, foo, baz)
    }

    const fn with_type_annotation<T>(val: Tuple<T>) -> (T, String, T) {
        destructure!{(foo, bar, baz): Tuple<T> = val}

        (bar, foo, baz)
    }

    for func in [
        no_type_annotation,
        with_type_annotation,
    ] {
        let val = ("hello".to_string(), 3, 5);

        assert_eq!(func(val), (3, "hello".to_string(), 5));
    }
}


#[test]
fn test_ignore_tuple_field() {
    const fn func<T: Copy>(val: Tuple<T>) -> (String, T) {
        destructure!{(foo, _, baz): Tuple<T> = val}

        (foo, baz)
    }

    let val = ("hello".to_string(), 3, 5);

    assert_eq!(func(val), ("hello".to_string(), 5));
}



////////////////////////////////////////////////////////////////////////
//                      Arrays
////////////////////////////////////////////////////////////////////////


#[test]
fn test_array_destructuring_individuals() {
    let s = <String as From<&str>>::from;

    const fn no_type_annotation<T>(val: [T; 3]) -> (T, T, T) {
        destructure!{[foo, bar, baz] = val}

        (bar, foo, baz)
    }

    const fn with_type_annotation<T>(val: [T; 3]) -> (T, T, T) {
        destructure!{[foo, bar, baz]: [T; 3] = val}

        (bar, foo, baz)
    }

    for func in [
        no_type_annotation,
        with_type_annotation,
    ] {
        let val = [s("hello"), s("3"), s("5")];

        assert_eq!(func(val), (s("3"), s("hello"), s("5")));
    }
}

#[test]
fn test_array_destructuring_rem_pat() {
    let s = |x: u32| x.to_string();
    let val = || [3, 5, 8, 13, 21].map(s);

    {
        const fn rem_at_start<T>(val: [T; 5]) -> (impl AsRef<[T]>, T, T) {
            destructure!{[foo @ .., bar, baz] = val}

            (foo, bar, baz)
        }

        let (rem, a, b) = rem_at_start(val());

        assert_eq!{rem.as_ref(), &[s(3), s(5), s(8)][..]}
        assert_eq!{a, "13"}
        assert_eq!{b, "21"}

    }

    {
        const fn rem_at_middle<T>(val: [T; 5]) -> (T, T, impl AsRef<[T]>, T) {
            destructure!{[a, b, rem @ .., c] = val}

            (a, b, rem, c)
        }

        let (a, b, rem, c) = rem_at_middle(val());

        assert_eq!{a, "3"}
        assert_eq!{b, "5"}
        assert_eq!{rem.as_ref(), &[s(8), s(13)][..]}
        assert_eq!{c, "21"}
    }


    {
        const fn rem_at_end<T>(val: [T; 5]) -> (T, T, impl AsRef<[T]>) {
            destructure!{[a, b, rem @ ..] = val}

            (a, b, rem)
        }

        let (a, b, rem) = rem_at_end(val());

        assert_eq!{a, "3"}
        assert_eq!{b, "5"}
        assert_eq!{rem.as_ref(), [8, 13, 21].map(s).as_slice()}
    }


}


#[test]
fn test_ignore_array_elem() {
    let s = |x: u32| x.to_string();

    const fn func<T: Copy>(val: [T; 3]) -> (T, T) {
        destructure!{[foo, _, baz]: [T; 3] = val}

        (foo, baz)
    }

    let val = [3, 5, 8];

    assert_eq!(func(val), (3, 8));
}


#[test]
fn test_array_ignore_rem_pat() {
    let val = || [3, 5, 8, 13, 21];

    {
        const fn rem_at_start<T: Copy>(val: [T; 5]) -> (T, T) {
            destructure!{[_ @ .., bar, baz] = val}

            (bar, baz)
        }

        let (a, b) = rem_at_start(val());

        assert_eq!{rem_at_start(val()), (13, 21)}

    }

    {
        const fn rem_at_middle<T: Copy>(val: [T; 5]) -> (T, T, T) {
            destructure!{[a, b, _ @ .., c] = val}

            (a, b, c)
        }

        assert_eq!{rem_at_middle(val()), (3, 5, 21)}
    }


    {
        const fn rem_at_end<T: Copy>(val: [T; 5]) -> (T, T) {
            destructure!{[a, b, _ @ ..] = val}

            (a, b)
        }

        assert_eq!{rem_at_end(val()), (3, 5)}
    }


}




