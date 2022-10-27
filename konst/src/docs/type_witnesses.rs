/*!
This page describes what type witnesses are,
their purpose in `konst`, and how to use them.

Type witnesses are enums that allow coercing between a type parameter and a
range of possible types, depending on the variant.
They are used by `konst` to emulate polymorphism inside of `const fn`s,
because trait methods cannot be called in `const fn`s on stable(as of Rust 1.65.0).

The simplest type witness is [`TypeEq<L, R>`](crate::polymorphism::TypeEq),
which only allows coercing between `L` and `R`.

Other type witnesses are enums with [`TypeEq`] fields,
which can coerce between a type parameter and as many types as they have variants.

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
        TheWitness::U8(te, ter) => ter.coerce(te.coerce(arg).to_le_bytes()),
        TheWitness::U16(te, ter) => ter.coerce(te.coerce(arg).to_le_bytes()),
        TheWitness::I8(te, ter) => ter.coerce(te.coerce(arg).to_le_bytes()),
        TheWitness::I16(te, ter) => ter.coerce(te.coerce(arg).to_le_bytes()),
    }
}

enum TheWitness<T, const N: usize> {
    U8(TypeEq<T, u8>, TypeEq<[u8; 1], [u8; N]>),
    U16(TypeEq<T, u16>, TypeEq<[u8; 2], [u8; N]>),
    I8(TypeEq<T, i8>, TypeEq<[u8; 1], [u8; N]>),
    I16(TypeEq<T, i16>, TypeEq<[u8; 2], [u8; N]>),
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
