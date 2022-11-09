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

The only other approach for emulating polymorphism is:
constructing a generic struct, then calling an inherent method on that struct.
<br>
This inherent method approach is not as good as the other approach from a user's perspective,
since it has worse compile-time errors
and requires an unusual way of calling functions
(`Function(arg0, arg1).call()`).

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
### Integer polymorphism

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

[`TypeEq`]: crate::polymorphism::TypeEq

*/
