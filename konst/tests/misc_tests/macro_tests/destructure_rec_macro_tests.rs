use konst::destructure_rec;

use ::std::cell::Cell;

// for testing that structural macro uses the leading `::` in paths
mod std {
    pub mod ops {
        pub struct Range {
            pub foo: u32,
            pub bar: u32,
        }
    }

    pub mod num {
        pub struct Wrapping(pub String, pub String);
    }
}

type Tuple2<T> = (T, T);

type Tuple3<T> = (T, T, T);

////////////////////////////////////////////////////////////////////////

#[test]
fn test_identity() {
    const fn inner() {
        konst::destructure_rec! {_foo = 3}
        konst::destructure_rec! {_foo: u32 = 3}
    }

    inner()
}

#[test]
fn test_mut_pat() {
    const fn inner() -> (u32, u32) {
        konst::destructure_rec! {mut foo = 3}
        konst::destructure_rec! {mut bar: u32 = 13}

        foo += 5;
        bar += 5;

        (foo, bar)
    }

    assert_eq!(inner(), (8, 18));
}

#[test]
fn test_many_bindings_pat() {
    const fn inner() -> [u32; 3] {
        konst::destructure_rec! {
            (ref foo @ foob, ref mut bar @ mut barb @ mut barc) = (3, 13)
        }

        let _: &u32 = foo;
        let _: u32 = foob;
        let _: &mut u32 = bar;
        let _: u32 = barb;

        *bar = 100;
        *bar = 100;
        barb = 200;

        barc += *foo;
        barc += foob;
        barc += *bar;
        barc += barb;

        [foob, barb, barc]
    }

    assert_eq!(inner(), [3, 200, 319]);
}

#[test]
fn test_underscore() {
    const fn inner() {
        konst::destructure_rec! {_ = 3}
        konst::destructure_rec! {_: u32 = 3}
    }

    inner()
}

#[test]
fn test_ref_pat() {
    const fn inner<T: Copy>(val: (T, T)) {
        konst::destructure_rec! { (ref a, ref b) = val }
        let _: &T = a;
        let _: &T = b;
    }
    inner((3, 5));

    //

    konst::destructure_rec! { (ref a, ref b) = (vec![3, 5], vec![8, 13]) }

    let _: &Vec<u32> = a;
    let _: &Vec<u32> = b;

    assert_eq!(a.as_slice(), &[3, 5][..]);
    assert_eq!(b.as_slice(), &[8, 13][..]);
}

#[test]
fn test_ref_mut_pat() {
    const fn inner<T: Copy>(val: (T, T)) {
        konst::destructure_rec! { (ref mut a, ref mut b) = val }
        let _: &mut T = a;
        let _: &mut T = b;
    }
    inner((3, 5));

    //

    konst::destructure_rec! { (ref mut a, ref mut b) = (vec![3, 5], vec![8, 13]) }

    let _: &mut Vec<u32> = a;
    let _: &mut Vec<u32> = b;

    assert_eq!(a.as_mut_slice(), &mut [3, 5][..]);
    assert_eq!(b.as_mut_slice(), &mut [8, 13][..]);
}

#[test]
fn test_deref_pat() {
    const fn inner<T: Copy>(val: [&(T, T); 2]) -> (T, T, &(T, T)) {
        konst::destructure_rec! { [&(a, b), c] = val }

        (a, b, c)
    }

    _ = inner([&(3, 5), &(8, 13)]);
}

#[test]
fn test_deref_mut_pat() {
    const fn inner<T: Copy>(val: [&mut (T, T); 2]) -> (T, T, &mut (T, T)) {
        konst::destructure_rec! { [&mut (a, b), c] = val }

        (a, b, c)
    }

    _ = inner([&mut (3, 5), &mut (8, 13)]);
}

#[test]
fn test_deref_ref_pat() {
    const fn inner<T: Copy>(val: [&(T, T); 2]) {
        konst::destructure_rec! { [&(ref a, ref b), c] = val }

        let _: &T = a;
        let _: &T = b;
        let _: &(T, T) = c;
    }

    _ = inner([&(3, 5), &(8, 13)]);
}

#[test]
fn test_deref_ref_mut_pat() {
    const fn inner<T: Copy>(val: [&mut (T, T); 2]) {
        konst::destructure_rec! { [&mut (ref mut a, ref mut b), c] = val }

        let _: &mut T = a;
        let _: &mut T = b;
        let _: &mut (T, T) = c;
    }

    _ = inner([&mut (3, 5), &mut (8, 13)]);
}

////////////////////////////////////////////////////////////////////////
//                  Unit Structs
////////////////////////////////////////////////////////////////////////

struct UnitStruct;

#[test]
fn test_unit_struct() {
    const fn inner() {
        konst::destructure_rec! {UnitStruct = UnitStruct}
        konst::destructure_rec! {UnitStruct::<> = UnitStruct}
        konst::destructure_rec! {UnitStruct::< >= UnitStruct}
        konst::destructure_rec! {UnitStruct: UnitStruct = UnitStruct}
        konst::destructure_rec! {UnitStruct::<>: UnitStruct = UnitStruct}
        konst::destructure_rec! {self::UnitStruct::<>: UnitStruct = UnitStruct}
        konst::destructure_rec! {self::UnitStruct: UnitStruct = UnitStruct}

        konst::destructure_rec! {[self::UnitStruct, UnitStruct] = [UnitStruct, UnitStruct]}
    }

    inner()
}

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
        destructure_rec! {BracedStruct {foo, bar, baz} = val}

        (foo, bar, baz)
    }

    const fn no_type_annotation_trailing_comma<T>(val: BracedStruct<T>) -> (String, T, T) {
        destructure_rec! {BracedStruct {foo, bar, baz,} = val}

        (foo, bar, baz)
    }

    const fn no_type_annotation_comma<T>(val: BracedStruct<T>) -> (String, T, T) {
        destructure_rec! {BracedStruct {foo, bar, baz} = val}

        (foo, bar, baz)
    }

    const fn with_type_annotation<T>(val: BracedStruct<T>) -> (String, T, T) {
        destructure_rec! {BracedStruct {foo, bar, baz}: BracedStruct<T> = val}

        (foo, bar, baz)
    }

    const fn with_typed_pattern<T>(val: BracedStruct<T>) -> (String, T, T) {
        destructure_rec! {BracedStruct<T> {foo, bar, baz}: BracedStruct<T> = val}

        (foo, bar, baz)
    }

    const fn renamed_fields<T>(val: BracedStruct<T>) -> (String, T, T) {
        destructure_rec! {BracedStruct {foo: baz, bar, baz: qux} = val}

        (baz, bar, qux)
    }

    for func in [
        no_type_annotation,
        no_type_annotation_trailing_comma,
        no_type_annotation_comma,
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
fn test_braced_struct_destructuring_ref_mut_patterns() {
    #[expect(unused_mut)]
    const fn inner<T: Copy>(val: BracedStruct<T>) -> String {
        destructure_rec! { BracedStruct { mut foo, ref bar, ref mut baz } = val }

        let _: String = foo;
        let _: &T = bar;
        let _: &mut T = baz;

        foo
    }

    let make = || BracedStruct {
        foo: "hello".to_string(),
        bar: 3,
        baz: 5,
    };

    assert_eq!(inner(make()), "hello");

    //

    destructure_rec! { BracedStruct {mut foo, ref bar, ref mut baz} = make() }

    let _: String = foo;
    let _: &i32 = bar;
    let _: &mut i32 = baz;

    assert_eq!(foo, "hello");
    foo.push_str(" world");
    assert_eq!(foo, "hello world");
    assert_eq!(bar, &3);
    assert_eq!(baz, &mut 5);
}

#[test]
fn test_braced_struct_leading_double_colon() {
    const fn with_leading(range: ::std::ops::Range<u8>) -> (u8, u8) {
        konst::destructure_rec! {::std::ops::Range{start, end} = range}
        (start, end)
    }

    assert_eq!(with_leading(0..10), (0, 10));

    const fn no_leading(range: std::ops::Range) -> (u32, u32) {
        konst::destructure_rec! {std::ops::Range{foo, bar} = range}
        (foo, bar)
    }

    assert_eq!(no_leading(std::ops::Range { foo: 3, bar: 5 }), (3, 5));
}

#[test]
fn test_braced_shadowing() {
    struct BracedPair<T, U> {
        val: T,
        other: U,
    }

    const fn inner<T>(val: BracedPair<([T; 4], u128), u8>) -> impl AsRef<[T]> {
        destructure_rec! {BracedPair{val, other: _} = val}
        destructure_rec! {(val, _) = val}
        val
    }

    assert_eq!(
        inner(BracedPair {
            val: ([8, 13, 21, 34], 3),
            other: 5
        })
        .as_ref(),
        &[8, 13, 21, 34][..]
    );
}

#[test]
fn test_empty_braced_struct() {
    #[derive(Copy, Clone)]
    struct Empty<const N: usize> {}

    const fn inner() {
        type Empty1 = Empty<1>;

        let empty1 = Empty::<1> {};
        let empty1b = Empty {};
        let empty2 = Empty {};
        let empty3 = Empty {};
        konst::destructure_rec! { Empty {} = empty1 }
        konst::destructure_rec! { Empty1 {} = empty1b }
        konst::destructure_rec! { Empty {}: Empty<2> = empty2 }
        konst::destructure_rec! { Empty<3> {} = empty3 }
    }

    inner()
}

#[test]
fn test_braced_struct_other_patterns() {
    {
        const fn ignore_last<T: Copy>(val: BracedStruct<T>) -> (String, T) {
            destructure_rec! {BracedStruct<T> {foo, bar, baz: _}: BracedStruct<T> = val}

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
            destructure_rec! {
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
        destructure_rec! {TupleStruct (foo, bar, baz) = val}

        (foo, bar, baz)
    }

    const fn no_type_annotation_trailing_comma<T>(val: TupleStruct<T>) -> (String, T, T) {
        destructure_rec! {TupleStruct (foo, bar, baz,) = val}

        (foo, bar, baz)
    }

    const fn with_type_annotation<T>(val: TupleStruct<T>) -> (String, T, T) {
        destructure_rec! {TupleStruct (foo, bar, baz): TupleStruct<T> = val}

        (foo, bar, baz)
    }

    const fn with_typed_pattern<T>(val: TupleStruct<T>) -> (String, T, T) {
        destructure_rec! {TupleStruct<T> (foo, bar, baz): TupleStruct<T> = val}

        (foo, bar, baz)
    }

    for func in [
        no_type_annotation,
        no_type_annotation_trailing_comma,
        with_type_annotation,
        with_typed_pattern,
    ] {
        let val = TupleStruct("hello".to_string(), 3, 5);

        assert_eq!(func(val), ("hello".to_string(), 3, 5));
    }
}

#[test]
fn test_packed_tuple_struct_ref_mut_pat() {
    #[expect(unused_mut)]
    const fn inner<T: Copy>(val: TupleStruct<T>) -> String {
        // `ref` and `ref mut` patterns don't borrow the `#[repr(packed)]` struct,
        // it borrows the temporary that is moved out of the field.
        destructure_rec! { TupleStruct(mut foo, ref bar, ref mut baz) = val }

        let _: String = foo;
        let _: &T = bar;
        let _: &mut T = baz;

        foo
    }

    let make = || TupleStruct("hello".to_string(), 3, 5);

    assert_eq!(inner(make()), "hello");

    //

    destructure_rec! { TupleStruct(mut foo, ref bar, ref mut baz) = make() }

    let _: String = foo;
    let _: &i32 = bar;
    let _: &mut i32 = baz;

    assert_eq!(foo, "hello");
    foo.push_str(" world");
    assert_eq!(foo, "hello world");
    assert_eq!(bar, &3);
    assert_eq!(baz, &mut 5);
}

#[test]
fn test_tuple_struct_shadowing() {
    struct TuplePair<T, U>(T, U);

    const fn inner<T>(val: TuplePair<([T; 4], u128), u8>) -> impl AsRef<[T]> {
        destructure_rec! {TuplePair(val, _) = val}
        destructure_rec! {(val, _) = val}
        val
    }

    assert_eq!(
        inner(TuplePair(([8, 13, 21, 34], 3), 5)).as_ref(),
        &[8, 13, 21, 34][..]
    );
}

#[test]
fn test_empty_tuple_struct() {
    struct Empty<const N: usize>();

    const fn inner() {
        type Empty1 = Empty<1>;

        let empty1 = Empty::<1>();
        let empty1b = Empty();
        let empty2 = Empty();
        let empty3 = Empty();
        konst::destructure_rec! { Empty () = empty1 }
        konst::destructure_rec! { Empty1 () = empty1b }
        konst::destructure_rec! { Empty (): Empty<2> = empty2 }
        konst::destructure_rec! { Empty<3>() = empty3 }
    }

    inner()
}

#[test]
fn test_tuple_struct_leading_double_colon() {
    const fn with_leading<T>(val: ::std::num::Wrapping<T>) -> T {
        konst::destructure_rec! {::std::num::Wrapping(n) = val}
        n
    }

    assert_eq!(with_leading(::std::num::Wrapping(3)), 3);

    const fn no_leading(val: std::num::Wrapping) -> (String, String) {
        konst::destructure_rec! {std::num::Wrapping(foo, bar) = val}
        (foo, bar)
    }

    let nl = std::num::Wrapping("foo".to_string(), "bar".to_string());
    assert_eq!(no_leading(nl), ("foo".to_string(), "bar".to_string()));
}

#[test]
fn test_ignore_tuple_struct_field() {
    const fn func<T: Copy>(val: TupleStruct<T>) -> (String, T) {
        destructure_rec! {TupleStruct(foo, _, baz): TupleStruct<T> = val}

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
        destructure_rec! {(foo, bar, baz) = val}

        (bar, foo, baz)
    }

    const fn no_type_annotation_trailing_comma<T>(val: Tuple<T>) -> (T, String, T) {
        destructure_rec! {(foo, bar, baz,) = val}

        (bar, foo, baz)
    }

    const fn with_type_annotation<T>(val: Tuple<T>) -> (T, String, T) {
        destructure_rec! {(foo, bar, baz): Tuple<T> = val}

        (bar, foo, baz)
    }

    for func in [
        no_type_annotation,
        no_type_annotation_trailing_comma,
        with_type_annotation,
    ] {
        let val = ("hello".to_string(), 3, 5);

        assert_eq!(func(val), (3, "hello".to_string(), 5));
    }
}

#[test]
fn test_tuple_shadowing() {
    const fn inner<T>(val: (([T; 4], u128), u8)) -> impl AsRef<[T]> {
        destructure_rec! {(val, _) = val}
        destructure_rec! {(val, _) = val}
        val
    }

    assert_eq!(
        inner((([8, 13, 21, 34], 3), 5)).as_ref(),
        &[8, 13, 21, 34][..]
    );
}

#[test]
fn test_unit() {
    const fn inner() {
        konst::destructure_rec! {() = ()}
        konst::destructure_rec! {(): () = ()}
    }

    inner()
}

#[test]
fn test_ignore_tuple_field() {
    const fn func<T: Copy>(val: Tuple<T>) -> (String, T) {
        destructure_rec! {(foo, _, baz): Tuple<T> = val}

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
        destructure_rec! {[foo, bar, baz] = val}

        (bar, foo, baz)
    }

    const fn no_type_annotation_trailing_comma<T>(val: [T; 3]) -> (T, T, T) {
        destructure_rec! {[foo, bar, baz,] = val}

        (bar, foo, baz)
    }

    const fn with_type_annotation<T>(val: [T; 3]) -> (T, T, T) {
        destructure_rec! {[foo, bar, baz]: [T; 3] = val}

        (bar, foo, baz)
    }

    for func in [
        no_type_annotation,
        no_type_annotation_trailing_comma,
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
            destructure_rec! {[foo @ .., bar, baz] = val}

            (foo, bar, baz)
        }

        let (rem, a, b) = rem_at_start(val());

        assert_eq! {rem.as_ref(), &[s(3), s(5), s(8)][..]}
        assert_eq! {a, "13"}
        assert_eq! {b, "21"}
    }

    {
        const fn rem_at_middle<T>(val: [T; 5]) -> (T, T, impl AsRef<[T]>, T) {
            destructure_rec! {[a, b, rem @ .., c] = val}

            (a, b, rem, c)
        }

        let (a, b, rem, c) = rem_at_middle(val());

        assert_eq! {a, "3"}
        assert_eq! {b, "5"}
        assert_eq! {rem.as_ref(), &[s(8), s(13)][..]}
        assert_eq! {c, "21"}
    }

    {}
}

#[test]
fn test_array_destructuring_rem_many_bindings() {
    const fn rem_at_end<T: Copy>(
        val: [T; 5],
    ) -> (T, T, impl AsRef<[T]>, impl AsRef<[T]>, impl AsRef<[T]>) {
        destructure_rec! {[a, b, c @ d @ rem @ ..] = val}

        (a, b, c, d, rem)
    }

    let (a, b, c, d, rem) = rem_at_end([3, 5, 8, 13, 21]);

    assert_eq!(a, 3);
    assert_eq!(b, 5);
    for x in [c.as_ref(), d.as_ref(), rem.as_ref()] {
        assert_eq!(x, [8, 13, 21].as_slice());
    }
}

#[test]
fn test_many_bindings_array() {
    const fn at_middle<T: Copy>(val: [T; 3]) -> (T, T, T, T, T) {
        destructure_rec! {[a, b @ c @ d, e] = val}

        (a, b, c, d, e)
    }

    let (a, b, c, d, e) = at_middle([3, 5, 8]);

    assert_eq!(a, 3);
    assert_eq!(b, 5);
    assert_eq!(c, 5);
    assert_eq!(d, 5);
    assert_eq!(e, 8);
}

#[test]
fn test_many_bindings_tuple() {
    const fn at_middle<T: Copy>(val: (T, T, T)) -> (T, T, T, T, T) {
        destructure_rec! {(a, b @ c @ d, e) = val}

        (a, b, c, d, e)
    }

    let (a, b, c, d, e) = at_middle((3, 5, 8));

    assert_eq!(a, 3);
    assert_eq!(b, 5);
    assert_eq!(c, 5);
    assert_eq!(d, 5);
    assert_eq!(e, 8);
}

#[test]
fn test_many_bindings_struct() {
    const fn repeated<T: Copy>(val: BracedStruct<T>) -> (String, T, T, T) {
        destructure_rec! {BracedStruct { foo, bar, baz: a @ b} = val}

        (foo, bar, a, b)
    }

    let (foo, bar, a, b) = repeated(BracedStruct {
        foo: "foo".into(),
        bar: 3,
        baz: 5,
    });

    assert_eq!(foo, "foo");
    assert_eq!(bar, 3);
    assert_eq!(a, 5);
    assert_eq!(b, 5);
}

#[test]
fn test_array_shadowing() {
    const fn inner<T>(val: [([T; 4], u128); 1]) -> impl AsRef<[T]> {
        destructure_rec! {[val] = val}
        destructure_rec! {(val, _) = val}
        val
    }

    assert_eq!(inner([([8, 13, 21, 34], 3)]).as_ref(), &[8, 13, 21, 34][..]);
}

#[test]
fn test_empty() {
    const fn repeat<const N: usize>() -> [u8; N] {
        [0; N]
    }

    const fn inner() {
        let empty1 = [0u8; 0];
        let empty2 = repeat();
        let empty3 = repeat();

        konst::destructure_rec! {[] = empty1}
        konst::destructure_rec! {[] = empty2}
        konst::destructure_rec! {[..]: [u8; 0] = empty3}
        konst::destructure_rec! {[..]: [u8; 0] = []}
    }

    inner()
}

#[test]
fn test_ignore_array_elem() {
    const fn func<T: Copy>(val: [T; 3]) -> (T, T) {
        destructure_rec! {[foo, _, baz]: [T; 3] = val}

        (foo, baz)
    }

    let val = [3, 5, 8];

    assert_eq!(func(val), (3, 8));
}

#[test]
fn test_array_ignore_rem_pat() {
    macro_rules! dry {($($prefix:tt)*) => ({
        let val = || [3, 5, 8, 13, 21];

        {
            const fn rem_at_start<T: Copy>(val: [T; 5]) -> (T, T) {
                destructure_rec!{[$($prefix)* .., bar, baz] = val}

                (bar, baz)
            }

            assert_eq!{rem_at_start(val()), (13, 21)}

        }

        {
            const fn rem_at_middle<T: Copy>(val: [T; 5]) -> (T, T, T) {
                destructure_rec!{[a, b, $($prefix)* .., c] = val}

                (a, b, c)
            }

            assert_eq!{rem_at_middle(val()), (3, 5, 21)}
        }


        {
            const fn rem_at_end<T: Copy>(val: [T; 5]) -> (T, T) {
                destructure_rec!{[a, b, $($prefix)* ..] = val}

                (a, b)
            }

            assert_eq!{rem_at_end(val()), (3, 5)}
        }
    })}

    dry! {}
    dry! {_ @}
}

#[test]
fn test_array_ignore_rem_pat_no_leak() {
    #[derive(Debug, PartialEq)]
    struct Adder<'a>(u128, &'a Cell<u128>);

    impl Drop for Adder<'_> {
        fn drop(&mut self) {
            self.1.update(|x| x + self.0);
        }
    }

    {
        let cell = &Cell::new(0);
        let val = [Adder(1, cell), Adder(4, cell), Adder(16, cell)];

        destructure_rec! {[.., qux] = val}
        assert_eq!(cell.get(), 5);

        drop(qux);
        assert_eq!(cell.get(), 21);
    }

    {
        let cell = &Cell::new(0);
        let val = [Adder(1, cell), Adder(4, cell), Adder(16, cell)];

        destructure_rec! {[bar, ..] = val}
        assert_eq!(cell.get(), 20);

        drop(bar);
        assert_eq!(cell.get(), 21);
    }
    {
        let cell = &Cell::new(0);
        let val = [Adder(1, cell), Adder(4, cell), Adder(16, cell)];

        destructure_rec! {[bar, .., baz] = val}
        assert_eq!(cell.get(), 4);

        drop(bar);
        assert_eq!(cell.get(), 5);

        drop(baz);
        assert_eq!(cell.get(), 21);
    }
    {
        let cell = &Cell::new(0);
        let val = [Adder(1, cell), Adder(4, cell), Adder(16, cell)];

        destructure_rec! {[..] = val}
        assert_eq!(cell.get(), 21);
    }
}

////////////////////////////////////////////////////////

#[test]
fn test_metavar_whole_pattern() {
    const fn func<T>(val: [[T; 3]; 2]) -> (impl AsRef<[T]>, T, impl AsRef<[T]>) {
        macro_rules! foo {
            ($pat:pat) => {
                destructure_rec! {$pat = val}
            };
        }

        foo! {[a, [b, c @ ..]]}

        (a, b, c)
    }

    let (a, b, c) = func([[3, 5, 8], [13, 21, 34]]);

    assert_eq!(a.as_ref(), [3, 5, 8].as_slice());
    assert_eq!(b, 13);
    assert_eq!(c.as_ref(), [21, 34].as_slice());
}

#[test]
fn test_metavar_subpattern() {
    const fn func<T>(val: [[T; 3]; 2]) -> (impl AsRef<[T]>, T, impl AsRef<[T]>) {
        macro_rules! foo {
            (($a:ident, $a_pat:pat), $b:ident, ($c:ident, $c_pat:pat)) => {
                destructure_rec! {[$a, [$b, $c @ $c_pat]] = val}

                ($a, $b, $c)
            };
        }

        foo! {(a, a), b, (c, ..)}
    }

    let (a, b, c) = func([[3, 5, 8], [13, 21, 34]]);

    assert_eq!(a.as_ref(), [3, 5, 8].as_slice());
    assert_eq!(b, 13);
    assert_eq!(c.as_ref(), [21, 34].as_slice());
}

////////////////////////////////////////////////////////

#[test]
fn test_braced_forget_fields() {
    struct BracedPair<T, U> {
        val: T,
        #[expect(dead_code)]
        other: U,
    }

    const fn inner<T>(val: BracedPair<[T; 2], u8>) -> impl AsRef<[T]> {
        destructure_rec! {
            #[forget_ignored_fields]
            BracedPair { val, .. } = val
        }
        val
    }

    assert_eq!(
        inner(BracedPair {
            val: [8, 13],
            other: 5
        })
        .as_ref(),
        &[8, 13][..]
    );
}

#[test]
fn test_tuple_forget_fields() {
    const fn inner<T>(val: ([T; 2], u8)) -> impl AsRef<[T]> {
        destructure_rec! {
            #[forget_ignored_fields]
            (val, ..) = val
        }
        val
    }

    assert_eq!(inner(([8, 13], 5)).as_ref(), &[8, 13][..]);
}

////////////////////////////////////////////////////////

#[test]
fn test_parenthesized_pattern() {
    const fn func<T>(val: [T; 3]) -> (T, T, T) {
        destructure_rec! {[(a), ((b)), (((c)))] = val}

        (a, b, c)
    }

    let (a, b, c) = func([3, 5, 8]);

    assert_eq!(a, 3);
    assert_eq!(b, 5);
    assert_eq!(c, 8);
}

#[test]
fn test_tuple1_pattern() {
    const fn func<T>(val: [(T,); 3]) -> (T, T, T) {
        destructure_rec! {[(a,), ((b,)), (((c,)))] = val}

        (a, b, c)
    }

    let (a, b, c) = func([(3,), (5,), (8,)]);

    assert_eq!(a, 3);
    assert_eq!(b, 5);
    assert_eq!(c, 8);
}

////////////////////////////////////////////////////////

#[repr(packed)]
struct PackTupStruct<T>(T, T);

#[test]
fn test_nested_array_tuple() {
    const fn func<T>(val: [Tuple3<T>; 2]) -> (T, T, T, T, T, T) {
        destructure_rec! {[(a, b, c), (d, e, f)] = val}

        (a, b, c, d, e, f)
    }

    let (a, b, c, d, e, f) = func([(3, 5, 8), (13, 21, 34)]);

    assert_eq!(a, 3);
    assert_eq!(b, 5);
    assert_eq!(c, 8);
    assert_eq!(d, 13);
    assert_eq!(e, 21);
    assert_eq!(f, 34);
}

#[test]
fn test_nested_tuple_array() {
    const fn func<T>(val: Tuple2<[T; 3]>) -> (impl AsRef<[T]>, T, T, impl AsRef<[T]>, T) {
        destructure_rec! {([a @ .., b], [c, d @ .., e]) = val}

        (a, b, c, d, e)
    }

    let (a, b, c, d, e) = func(([3, 5, 8], [13, 21, 34]));

    assert_eq!(a.as_ref(), [3, 5].as_slice());
    assert_eq!(b, 8);
    assert_eq!(c, 13);
    assert_eq!(d.as_ref(), [21].as_slice());
    assert_eq!(e, 34);
}

#[test]
fn test_nested_packed_tuplestruct_array() {
    const fn func<T>(val: PackTupStruct<[T; 3]>) -> (impl AsRef<[T]>, T, T, impl AsRef<[T]>, T) {
        destructure_rec! {PackTupStruct([a @ .., b], [c, d @ .., e]) = val}

        (a, b, c, d, e)
    }

    let (a, b, c, d, e) = func(PackTupStruct([3u128, 5, 8], [13, 21, 34]));

    assert_eq!(a.as_ref(), [3, 5].as_slice());
    assert_eq!(b, 8);
    assert_eq!(c, 13);
    assert_eq!(d.as_ref(), [21].as_slice());
    assert_eq!(e, 34);
}

#[test]
fn test_nested_tuple_tuple() {
    const fn func<T>(val: Tuple2<Tuple2<T>>) -> (T, T, T, T) {
        destructure_rec! {((a, b), (c, d)) = val}

        (a, b, c, d)
    }

    let (a, b, c, d) = func(((3, 5), (8, 13)));

    assert_eq!(a, 3);
    assert_eq!(b, 5);
    assert_eq!(c, 8);
    assert_eq!(d, 13);
}

#[test]
fn test_nested_tuple_tuplestruct() {
    const fn func<T>(val: Tuple2<TupleStruct<T>>) -> (String, T, T, String, T, T) {
        destructure_rec! {(TupleStruct(a, b, c), TupleStruct(d, e, f)) = val}

        (a, b, c, d, e, f)
    }

    let (a, b, c, d, e, f) = func((
        TupleStruct("foo".into(), 3, 5),
        TupleStruct("bar".into(), 8, 13),
    ));

    assert_eq!(a, "foo".to_string());
    assert_eq!(b, 3);
    assert_eq!(c, 5);
    assert_eq!(d, "bar".to_string());
    assert_eq!(e, 8);
    assert_eq!(f, 13);
}

#[test]
fn test_nested_tuplestruct_tuple() {
    const fn func<T>(val: TupleStruct<Tuple2<T>>) -> (String, T, T, T, T) {
        destructure_rec! {TupleStruct(a, (b, c), (d, e)) = val}

        (a, b, c, d, e)
    }

    let (a, b, c, d, e) = func(TupleStruct("foo".into(), (3, 5), (8, 13)));

    assert_eq!(a, "foo".to_string());
    assert_eq!(b, 3);
    assert_eq!(c, 5);
    assert_eq!(d, 8);
    assert_eq!(e, 13);
}

#[test]
fn test_nested_packed_tuplestruct_tuple() {
    const fn func<T>(val: PackTupStruct<Tuple2<T>>) -> (T, T, T, T) {
        destructure_rec! {PackTupStruct((b, c), (d, e)) = val}

        (b, c, d, e)
    }

    let (b, c, d, e) = func(PackTupStruct((3u128, 5), (8, 13)));

    assert_eq!(b, 3);
    assert_eq!(c, 5);
    assert_eq!(d, 8);
    assert_eq!(e, 13);
}

#[test]
fn test_nested_tuple_bracedstruct() {
    const fn func<T>(val: Tuple2<BracedStruct<T>>) -> (String, T, T, String, T, T) {
        destructure_rec! {
            (BracedStruct{foo, bar, baz}, BracedStruct { foo: d, bar: e, baz: f }) = val
        }

        (foo, bar, baz, d, e, f)
    }

    let (a, b, c, d, e, f) = func((
        BracedStruct {
            foo: "foo".into(),
            bar: 3,
            baz: 5,
        },
        BracedStruct {
            foo: "bar".into(),
            bar: 8,
            baz: 13,
        },
    ));

    assert_eq!(a, "foo".to_string());
    assert_eq!(b, 3);
    assert_eq!(c, 5);
    assert_eq!(d, "bar".to_string());
    assert_eq!(e, 8);
    assert_eq!(f, 13);
}

#[test]
fn test_nested_packed_tuplestruct_bracedstruct() {
    const fn func<T>(val: PackTupStruct<BracedStruct<T>>) -> (String, T, T, String, T, T) {
        destructure_rec! {
            PackTupStruct(BracedStruct{foo, bar, baz}, BracedStruct { foo: d, bar: e, baz: f }) =
                val
        }

        (foo, bar, baz, d, e, f)
    }

    let (a, b, c, d, e, f) = func(PackTupStruct(
        BracedStruct {
            foo: "foo".into(),
            bar: 3u128,
            baz: 5,
        },
        BracedStruct {
            foo: "bar".into(),
            bar: 8,
            baz: 13,
        },
    ));

    assert_eq!(a, "foo".to_string());
    assert_eq!(b, 3);
    assert_eq!(c, 5);
    assert_eq!(d, "bar".to_string());
    assert_eq!(e, 8);
    assert_eq!(f, 13);
}

#[test]
fn test_nested_bracedstruct_tuple() {
    const fn func<T>(val: BracedStruct<Tuple2<T>>) -> (String, T, T, T, T) {
        destructure_rec! {
            BracedStruct { foo, bar: (b, c), baz: (d, e) } = val
        }

        (foo, b, c, d, e)
    }

    let (a, b, c, d, e) = func(BracedStruct {
        foo: "foo".into(),
        bar: (3, 5),
        baz: (8, 13),
    });

    assert_eq!(a, "foo".to_string());
    assert_eq!(b, 3);
    assert_eq!(c, 5);
    assert_eq!(d, 8);
    assert_eq!(e, 13);
}
