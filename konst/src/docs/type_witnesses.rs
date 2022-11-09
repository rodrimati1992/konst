/*!
This page describes what type witnesses are,
their purpose in `konst`, and how to use them.

# What are Type Witnesses

Type witnesses are enums that allow coercing between a type parameter and a
range of possible types (one per variant).

The simplest type witness is [`TypeEq<L, R>`](crate::polymorphism::TypeEq),
which only allows coercing between `L` and `R`.

Most type witnesses are enums with [`TypeEq`] fields,
which can coerce between a type parameter and as many types as they have variants.

# Why does `konst` need these

Type witnesses are used by `konst` to emulate polymorphism inside of `const fn`s,
because trait methods cannot be called in `const fn`s on stable(as of Rust 1.65.0).

### Alternative: Inherent method dispatch

Another approach for emulating polymorphism is
constructing a generic struct, then calling an inherent method on that struct.

This inherent method approach has these disadvantages:
- it requires an unusual way of calling functions (`Struct(arg0, arg1).call()`)
- the type it is dispatched over must be concrete
(whereas the type witness approach can be used with bounded type parameters)
- it has worse inference, since the argument type can't be inferred from the return type

<details>
<summary> Example of inherent method dispatch  </summary>

```rust

assert_eq!(Negate(3i8).call(), -3i8);
assert_eq!(Negate(-3i8).call(), 3i8);

assert_eq!(Negate(5i16).call(), -5i16);
assert_eq!(Negate(-5i16).call(), 5i16);

assert_eq!(Negate(8i32).call(), -8i32);
assert_eq!(Negate(-8i32).call(), 8i32);

assert_eq!(Negate('c').call(), 'c');


struct Negate<T>(T);

impl Negate<i8> {
    const fn call(self) -> i8 {
        -self.0
    }
}
impl Negate<i16> {
    const fn call(self) -> i16 {
        -self.0
    }
}
impl Negate<i32> {
    const fn call(self) -> i32 {
        -self.0
    }
}

```

</details>

# Limitations

A limitation of the type witness approach to emulating trait methods
is that the supported types must be known in advance,
and can't be extended in other crates.

# Items

These are the main items for type witnesses:

- [`TypeEq`](crate::polymorphism::TypeEq)
- [`HasTypeWitness`](crate::polymorphism::HasTypeWitness)
- [`TypeWitnessTypeArg`](crate::polymorphism::TypeWitnessTypeArg)
- [`MakeTypeWitness`](crate::polymorphism::MakeTypeWitness)

# Examples

<span id="example0"></span>
### Indexing polymorphism

This example demonstrates how `konst` does polymorphic functions,
which would otherwise require that traits methods are callable in `const fn`s.

```rust
use std::ops::Range;

use konst::{
    polymorphism::{HasTypeWitness, MakeTypeWitness, TypeEq, TypeWitnessTypeArg},
    slice::slice_range,
};

fn main() {
    let array = [3, 5, 8, 13, 21, 34, 55, 89];

    assert_eq!(index(&array, 0), &3);
    assert_eq!(index(&array, 3), &13);
    assert_eq!(index(&array, 0..4), [3, 5, 8, 13]);
    assert_eq!(index(&array, 3..5), [13, 21]);
}

const fn index<T, I>(slice: &[T], index: I) -> &I::Returns
where
    I: SliceIndex<T>,
{
    match I::WITNESS {
        IndexWitness::Usize { arg_te, ret_te } => {
            // `arg_te` (a `TypeEq<I, usize>`) allows coercing between `I` and `usize`,
            // because `TypeEq` is a value-level proof that both types are the same.
            let index: usize = arg_te.to_right(index);

            let ret: &T = &slice[index];

            // `.in_ref()` converts `TypeEq<L, R>` into `TypeEq<&L, &R>`
            let ret_te: TypeEq<&I::Returns, &T> = ret_te.in_ref();

            ret_te.to_left(ret)
        }
        IndexWitness::Range { arg_te, ret_te } => {
            let range: Range<usize> = arg_te.to_right(index);

            let ret: &[T] = slice_range(slice, range.start, range.end);

            ret_te.in_ref().to_left(ret)
        }
    }
}


trait SliceIndex<T>: HasTypeWitness<IndexWitness<Self, T>> + Sized {
    type Returns: ?Sized;
}


enum IndexWitness<I: SliceIndex<T>, T> {
    Usize {
        arg_te: TypeEq<I, usize>,
        ret_te: TypeEq<I::Returns, T>,
    },
    Range {
        arg_te: TypeEq<I, Range<usize>>,
        ret_te: TypeEq<I::Returns, [T]>,
    },
}

impl<I, T> TypeWitnessTypeArg for IndexWitness<I, T>
where
    I: SliceIndex<T>,
{
    type Arg = I;
}


impl<T> SliceIndex<T> for usize {
    type Returns = T;
}

impl<T> MakeTypeWitness for IndexWitness<usize, T> {
    const MAKE: Self = Self::Usize {
        arg_te: TypeEq::NEW,
        ret_te: TypeEq::NEW,
    };
}


impl<T> SliceIndex<T> for Range<usize> {
    type Returns = [T];
}

impl<T> MakeTypeWitness for IndexWitness<Range<usize>, T> {
    const MAKE: Self = Self::Range {
        arg_te: TypeEq::NEW,
        ret_te: TypeEq::NEW,
    };
}


```

When the wrong type is passed for the index,
the compile-time error is the same as with normal generic functions:
```text
error[E0277]: the trait bound `RangeFull: SliceIndex<{integer}>` is not satisfied
  --> src/docs/type_witnesses.rs:66:30
   |
17 |     assert_eq!(index(&array, ..), [13, 21]);
   |                -----         ^^ the trait `SliceIndex<{integer}>` is not implemented for `RangeFull`
   |                |
   |                required by a bound introduced by this call
   |
   = help: the following other types implement trait `SliceIndex<T>`:
             std::ops::Range<usize>
             usize
note: required by a bound in `index`
  --> src/docs/type_witnesses.rs:71:8
   |
20 | const fn index<T, I>(slice: &[T], index: I) -> &I::Returns
   |          ----- required by a bound in this
21 | where
22 |     I: SliceIndex<T>,
   |        ^^^^^^^^^^^^^ required by this bound in `index`
```


[`TypeEq`]: crate::polymorphism::TypeEq

*/
