macro_rules! explain_type_witness {
    () => ("\
        A [type witness](crate::docs::type_witnesses) is \
        an enum whose variants only have [`TypeEq`] fields.
        Each variant requires the enum's type parameter to be a specific type.
    ")
}


/// Gets a [type witness](crate::docs::type_witnesses) for `Self`.
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
/// This example shows how one can make a `const fn` that converts both 
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
///             // `TypeEq::<L, R>::to_right` does an identity conversion from
///             // an `L` to an `R`, which `TypeEq` guarantees are the same type.
///             let string: &str = te.to_right(input);
///             Ok(string)
///         }
///         StrTryFrom::Bytes(te) => {
///             let bytes: &[u8] = te.to_right(input);
///             std::str::from_utf8(bytes)
///         }
///         StrTryFrom::Array(te) => {
///             // this requires care not to infinitely recurse
///             let slice: &[u8] = te.to_right(input);
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
/// // `#[non_exhausitve]` allows adding more supported types.
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

/// Constructs this [type witness](crate::docs::type_witnesses).
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
///         Defaultable::I32(te) => te.to_right(3),
///         Defaultable::Bool(te) => te.to_right(true),
///         Defaultable::Str(te) => te.to_right("empty"),
///         Defaultable::Array(te) => te.to_right([5; L]),
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
///
/// This type can be used to prove that `L` and `R` are the same type,
/// because it can only be safely constructed with 
/// `TypeEq::<L, L>::NEW`(or [`new`](#method.new)),
/// where both type arguments are the same type.
///
/// This type is not too useful by itself, it becomes useful 
/// [when put inside of an enum](crate::docs::type_witnesses#example0).
///
/// # Related
/// 
/// These are related items:
/// 
/// - [`HasTypeWitness`]
/// - [`TypeWitnessTypeArg`]
/// - [`MakeTypeWitness`]
/// 
/// # Soundness
/// 
/// `TypeEq<L, R>` requires both type arguments to be the same type so that 
/// [projecting](#projecting) the type arguments results in the same type for 
/// both arguments.
/// 
/// # Examples
/// 
/// All the [related](#related) items show how `TypeEq` is used`inside of enums
/// (its primary usecase).
/// 
/// The examples below are intended to demonstrate basic properties of `TypeEq`.
/// 
/// ### Projecting
/// 
/// This example demonstrates `TypeEq` projection using [`type_eq_projection_fn`]:
/// 
/// ```rust
/// use konst::polymorphism::{TypeEq, type_eq_projection_fn};
/// 
/// // This macro invocation generates this function:
/// // const fn project_vec<L, R>(teq: TypeEq<L, R>) -> TypeEq<Vec<L>, Vec<R>>
/// type_eq_projection_fn!{
///     // `T` must be both the function parameter, and in the return type.
///     const fn project_vec(T) -> Vec<T>
/// }
/// 
/// fn foo<T>(te: TypeEq<u32, T>) -> Vec<T> {
///     let vec_te: TypeEq<Vec<u32>, Vec<T>> = project_vec(te);
///     vec_te.to_right(vec![3, 5, 8])
/// }
/// 
/// assert_eq!(foo(TypeEq::NEW), vec![3u32, 5, 8]);
/// 
/// ```
/// 
/// 
/// </p>
/// </details>
///
/// (the docs are hidden to work around a rustdoc bug)
#[doc(inline)]
pub use konst_kernel::type_eq::TypeEq;

/// Declares a function for converting a `TypeEq<L, R>`
/// to `TypeEq<Foo<L>, Foo<R>>`.
/// 
/// [**examples below**](#examples)
/// 
/// [**syntax example**](#syntax)
/// 
/// # Limitations
/// 
/// This macro has the following limitations:
/// - It only accepts module paths for a type,
/// followed by the generic parameters of that type,
/// no concrete generic arguments are allowed.
/// 
/// - It can only map one type parameter, the `T` parameter.
/// 
/// - It cannot parse trait bounds in the type parameter list written 
/// the normal way, they must be wrapped in parentheses.
/// 
/// - The `T` type parameter can only be bounded in the parameter list
/// 
/// - The `T` type parameter cannot appear in any trait bounds.
/// 
/// The first two limitations can be worked around by passing a type alias
/// to the macro.
///
/// # Examples
/// 
/// ### Basic
/// 
/// This example shows what the macro does,
/// the [motivating example](#motivating-example) shows why one would use it.
/// 
/// ```rust
/// use konst::polymorphism::{TypeEq,  type_eq_projection_fn};
/// 
/// #[derive(Debug, PartialEq)]
/// struct Foo<T, const N: usize>([T; N]);
/// 
/// // This macro invocation generates:
/// // const fn project_to_foo<L, R, const N: usize>(
/// //     _: TypeEq<L, R>,
/// // ) -> TypeEq<Foo<L, N>, Foo<R, N>>
/// type_eq_projection_fn!{
///     // `T` must be both the function parameter, and in the return type.
///     const fn project_to_foo(T) -> Foo<T, const N: usize>
/// }
/// 
/// // a toy example to demonstrate what projecting a TypeEq does
/// const fn get_foo<'a, R>(te: TypeEq<&'a str, R>) -> Foo<R, 2> {
///     // The type annotation is for the reader
///     let te: TypeEq<Foo<&'a str, 2>, Foo<R, 2>> =
///         project_to_foo::<&'a str, R, 2>(te);
/// 
///     te.to_right(Foo(["foo", "bar"]))
/// }
/// 
/// assert_eq!(get_foo(TypeEq::NEW), Foo(["foo", "bar"]));
/// 
/// ```
/// 
/// ### Motivating example
/// 
/// ```rust
/// use konst::polymorphism::{
///     HasTypeWitness,
///     MakeTypeWitness,
///     TypeEq, 
///     TypeWitnessTypeArg,
///     type_eq_projection_fn,
/// };
/// 
/// fn main() {
///     assert_eq!(Foo(3, false).transform(), Foo(13, false));
///     assert_eq!(Foo("hello", "world").transform(), Foo("mapped", "world"));
/// }
/// 
/// #[derive(Debug, PartialEq)]
/// struct Foo<T, U: Copy>(T, U);
/// 
/// // This macro invocation generates:
/// // const fn project_to_foo<L, R, U>(
/// //     _: TypeEq<L, R>,
/// // ) -> TypeEq<Foo<L, U>, Foo<R, U>>
/// type_eq_projection_fn!{
///     // The `Copy` bound needs to be wrapped in parentheses in `U: (Copy)` to
///     // simplify parsing of trait bounds in the generic parameter list.
///     // 
///     // note: trait bounds are written normally in where clauses,
///     //       they must be unparenthesized.
///     const fn project_to_foo(T) -> Foo<T, U: (Copy)>
/// }
/// 
/// impl<T, U: Copy> Foo<T, U> {
///     const fn transform<'a>(self) -> Foo<T, U>
///     where
///         T: Copy + HasTypeWitness<TheWitness<'a, T>>,
///     {
///         match T::WITNESS {
///             TheWitness::U8(te) => {
///                 // the type annotation is just for the reader
///                 let te: TypeEq<Foo<T, U>, Foo<u8, U>> = project_to_foo(te);
///                 let bar: Foo<u8, U> = te.to_right(self);
///
///                 te.to_left(Foo(bar.0 + 10, bar.1))
///             }
///             TheWitness::Str(te) => {
///                 // the type annotation is just for the reader
///                 let te: TypeEq<Foo<T, U>, Foo<&str, U>> = project_to_foo(te);
///                 te.to_left(Foo("mapped", self.1))
///             }
///         }
///     }
/// }
/// 
/// // A type witmess, a pattern documented in `konst::docs::type_witnesses`
/// 
/// // Simply put, type witnesses emulate matching over a range of types.
/// enum TheWitness<'a, T> {
///     U8(TypeEq<T, u8>),
///     Str(TypeEq<T, &'a str>),
/// }
/// 
/// impl<T> TypeWitnessTypeArg for TheWitness<'_, T> {
///     type Arg = T;
/// }
/// 
/// impl MakeTypeWitness for TheWitness<'_, u8> {
///     const MAKE: Self = Self::U8(TypeEq::NEW);
/// }
/// 
/// impl<'a> MakeTypeWitness for TheWitness<'a, &'a str> {
///     const MAKE: Self = Self::Str(TypeEq::NEW);
/// }
/// 
/// ```
/// 
/// ### Syntax
/// 
/// This example demonstrates all the syntax that this macro supports.
/// 
/// ```rust
/// # use std::fmt::Debug;
/// # use konst::polymorphism::type_eq_projection_fn;
/// #
/// # extern crate self as foo;
/// #
/// # #[derive(Debug, PartialEq, Clone)]
/// # pub struct Ty<'a, 'b: 'a, U: 'a + Debug, const N: usize>(&'a &'b [U; N]);
/// #
/// // This macro invocation generates this function:
/// // 
/// // pub const fn project<'a, 'b, L, R, const N: usize>(
/// //     _: TypeEq<L, R>
/// // ) -> TypeEq<::foo::Ty<'a, 'b, L, N>, ::foo::Ty<'a, 'b, R, N>>
/// // where
/// //     'b: 'a,
/// //     L: 'a + Debug,
/// //     R: 'a + Debug,
/// //     [u32; N]: 'a + core::fmt::Debug
/// type_eq_projection_fn!{
///     /// Documentation for the generated function
///     // 
///     // Without the `const` qualifier, the generated function is non-`const`.
///     // 
///     // `T` must be both the function parameter, and in the return type.
///     pub const fn project(T) -> ::foo::Ty<
///         'a,
///         'b: 'a,
///         // trait bounds in the type parameter list must be parenthesized
///         T: ('a +  Debug), 
///         const N: usize,
///     >
///     where
///         // trait bounds in the where clause are unparenthesized
///         [u32; N]: 'a + core::fmt::Debug,
/// }
/// # fn main(){}
/// ```
/// 
pub use konst_kernel::type_eq_projection_fn;