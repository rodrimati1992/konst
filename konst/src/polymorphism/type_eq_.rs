macro_rules! explain_type_witness {
    () => ("\
        A *type witness* is \
        an enum whose variants only have [`TypeEq`] fields.
        Each variant requires the enum's type parameter to be a specific type.
    ")
}


/// Gets a type witness for `Self`.
/// 
#[doc = explain_type_witness!()]
/// 
/// This trait has a generic implementation and can't be implemented outside of `konst`
/// 
/// # Related
/// 
/// These are related items:
/// 
/// - [`TypeEq`]
/// - [`TypeWitnessTypeArg`]
/// - [`MakeTypeWitness`]
/// 
/// # Example 
/// 
/// This example shows how you can make a `const fn` that converts both 
/// `&str` and `&[u8]` to `&str`
/// 
/// ```rust
/// use konst::polymorphism::{HasTypeWitness, TypeWitnessTypeArg, MakeTypeWitness, TypeEq};
/// 
/// const fn str_try_from<'a, T, const L: usize>(input: T) -> Result<&'a str, std::str::Utf8Error>
/// where
///     T: Copy + HasTypeWitness<StrTryFrom<'a, T, L>>
/// {
///     match T::WITNESS {
///         StrTryFrom::Str(te) => {
///             // `TypeEq::<L, R>::sidecast` does an identity conversion from
///             // an `L` to an `R`, which `TypeEq` guarantees are the same type.
///             let string: &str = te.sidecast(input);
///             Ok(string)
///         }
///         StrTryFrom::Bytes(te) => {
///             let bytes: &[u8] = te.sidecast(input);
///             std::str::from_utf8(bytes)
///         }
///         StrTryFrom::Array(te) => {
///             // this requires care not to infinitely recurse
///             let slice: &[u8] = te.sidecast(input);
///             str_try_from(slice)
///         }
///     }
/// }
/// 
/// assert_eq!(str_try_from("hello"), Ok("hello"));
/// 
/// assert_eq!(str_try_from(&[b'w', b'o', b'r', b'l', b'd']), Ok("world"));
/// 
/// assert_eq!(str_try_from(b"foo bar" as &[_]), Ok("foo bar"));
/// 
/// // this enum is a type witness
/// // `#[non_exhausitve]` allows adding more supported types to the set.
/// #[non_exhaustive]
/// pub enum StrTryFrom<'a, T, const L: usize> {
///     // This variant requires `T == &'a str`
///     Str(TypeEq<T, &'a str>),
///
///     // This variant requires `T == &'a [u8]`
///     Bytes(TypeEq<T, &'a [u8]>),
///
///     // This variant requires `T == &'a [u8; L]`
///     Array(TypeEq<T, &'a [u8; L]>),
/// }
/// 
/// impl<'a, T, const L: usize> TypeWitnessTypeArg for StrTryFrom<'a, T, L> {
///     type Arg = T;
/// }
/// 
/// impl<'a> MakeTypeWitness for StrTryFrom<'a, &'a str, 0> {
///     const MAKE: Self = Self::Str(TypeEq::NEW);
/// }
/// 
/// impl<'a> MakeTypeWitness for StrTryFrom<'a, &'a [u8], 0> {
///     const MAKE: Self = Self::Bytes(TypeEq::NEW);
/// }
/// 
/// impl<'a, const L: usize> MakeTypeWitness for StrTryFrom<'a, &'a [u8; L], L> {
///     const MAKE: Self = Self::Array(TypeEq::NEW);
/// }
/// 
/// ```
pub use konst_kernel::type_eq::HasTypeWitness;

/// Gets the type argument that `Self` witnesses.
/// 
#[doc = explain_type_witness!()]
/// 
/// [**example shared with `MakeTypeWitness`**](MakeTypeWitness#example)
/// 
/// # Related
/// 
/// These are related items:
/// 
/// - [`TypeEq`]
/// - [`MakeTypeWitness`]
/// - [`HasTypeWitness`]
/// 
pub use konst_kernel::type_eq::TypeWitnessTypeArg;

/// Constructs this type witness.
/// 
#[doc = explain_type_witness!()]
/// 
/// # Related
/// 
/// These are related items:
/// 
/// - [`TypeEq`]
/// - [`TypeWitnessTypeArg`]
/// - [`HasTypeWitness`]
/// 
/// # Example
/// 
/// ```rust
/// use konst::polymorphism::{HasTypeWitness, TypeWitnessTypeArg, MakeTypeWitness, TypeEq};
/// 
/// const fn default<T, const L: usize>(ret: Defaultable<'_, T, L>) -> T {
///     match ret {
///         Defaultable::I32(te) => te.sidecast(3),
///         Defaultable::Bool(te) => te.sidecast(true),
///         Defaultable::Str(te) => te.sidecast("empty"),
///         Defaultable::Array(te) => te.sidecast([5; L]),
///     }
/// }
/// 
/// // using `<i32 as HasTypeWitness<_>>::WITNESS`
/// assert_eq!(default(i32::WITNESS), 3);
/// 
/// assert_eq!(default(bool::WITNESS), true);
/// 
/// // using `<Defaultable<..> as MakeTypeWitness>::MAKE`
/// let string: &str = default(MakeTypeWitness::MAKE);
/// assert_eq!(string, "empty");
///
/// let array: [u32; 3] = default(MakeTypeWitness::MAKE);
/// assert_eq!(array, [5, 5, 5]);
/// 
/// // this enum is a type witness
/// #[non_exhaustive]
/// enum Defaultable<'a, Ret, const L: usize> {
///     // This variant requires `Ret == i32`
///     I32(TypeEq<i32, Ret>),
///
///     // This variant requires `Ret == bool`
///     Bool(TypeEq<bool, Ret>),
///
///     // This variant requires `Ret == &'a str`
///     Str(TypeEq<&'a str, Ret>),
///
///     // This variant requires `Ret == [u32; L]`
///     Array(TypeEq<[u32; L], Ret>),
/// }
/// 
/// impl<Ret, const L: usize> TypeWitnessTypeArg for Defaultable<'_, Ret, L> {
///     type Arg = Ret;
/// }
/// 
/// impl MakeTypeWitness for Defaultable<'_, i32, 0> {
///     const MAKE: Self = Self::I32(TypeEq::NEW);
/// }
/// 
/// impl MakeTypeWitness for Defaultable<'_, bool, 0> {
///     const MAKE: Self = Self::Bool(TypeEq::NEW);
/// }
/// 
/// impl<'a> MakeTypeWitness for Defaultable<'a, &'a str, 0> {
///     const MAKE: Self = Self::Str(TypeEq::NEW);
/// }
/// 
/// impl<const L: usize> MakeTypeWitness for Defaultable<'_, [u32; L], L> {
///     const MAKE: Self = Self::Array(TypeEq::NEW);
/// }
/// 
/// ```
/// 
pub use konst_kernel::type_eq::MakeTypeWitness;


/// Value-level proof that `L` is the same type as `R`
///
/// <details>
/// <summary><b>item docs</b></summary>
/// <p>
/// This type can be used to prove that `L` and `R` are the same type,
/// because it can only be constructed with `TypeEq::<L, L>::NEW`,
/// where both type arguments are the same type.
///
/// # Related
/// 
/// These are related items:
/// 
/// - [`HasTypeWitness`]
/// - [`TypeWitnessTypeArg`]
/// - [`MakeTypeWitness`]
/// 
/// </p>
/// </details>
///
/// (the docs are hidden to work around a rustdoc bug)
#[doc(inline)]
pub use konst_kernel::type_eq::TypeEq;