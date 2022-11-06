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
use konst::polymorphism::{HasTypeWitness, TypeWitnessTypeArg, MakeTypeWitness, TypeEq};


assert_eq!(to_le_bytes(3u8), [3u8]);
assert_eq!(to_le_bytes(5i8), [5u8]);
assert_eq!(to_le_bytes(258u16), [2u8, 1]);
assert_eq!(to_le_bytes(259i16), [3u8, 1]);


const fn to_le_bytes<T, const N: usize>(arg: T) -> [u8; N]
where
    T: HasTypeWitness<TheWitness<T, N>>
{
    match T::WITNESS {
        TheWitness::U8(te, ter) => ter.to_left(te.to_right(arg).to_le_bytes()),
        TheWitness::U16(te, ter) => ter.to_left(te.to_right(arg).to_le_bytes()),
        TheWitness::I8(te, ter) => ter.to_left(te.to_right(arg).to_le_bytes()),
        TheWitness::I16(te, ter) => ter.to_left(te.to_right(arg).to_le_bytes()),
    }
}

enum TheWitness<T, const N: usize> {
    U8(TypeEq<T, u8>, TypeEq<[u8; N], [u8; 1]>),
    U16(TypeEq<T, u16>, TypeEq<[u8; N], [u8; 2]>),
    I8(TypeEq<T, i8>, TypeEq<[u8; N], [u8; 1]>),
    I16(TypeEq<T, i16>, TypeEq<[u8; N], [u8; 2]>),
}

impl<T, const N: usize> TypeWitnessTypeArg for TheWitness<T, N> {
    type Arg = T;
}

impl MakeTypeWitness for TheWitness<u8, 1> {
    const MAKE: Self = Self::U8(TypeEq::NEW, TypeEq::NEW);
}

impl MakeTypeWitness for TheWitness<u16, 2> {
    const MAKE: Self = Self::U16(TypeEq::NEW, TypeEq::NEW);
}

impl MakeTypeWitness for TheWitness<i8, 1> {
    const MAKE: Self = Self::I8(TypeEq::NEW, TypeEq::NEW);
}

impl MakeTypeWitness for TheWitness<i16, 2> {
    const MAKE: Self = Self::I16(TypeEq::NEW, TypeEq::NEW);
}
```

[`TypeEq`]: crate::polymorphism::TypeEq

*/
