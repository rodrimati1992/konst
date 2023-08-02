/// Declares a function for converting a `TypeEq<L, R>`
/// to `TypeEq<Foo<L>, Foo<R>>`.
/// 
/// As an alternative to this macro, you can look at [`TypeEq::project`] and [`TypeEq::map`]
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
/// // A type witness, a pattern documented in `konst::docs::type_witnesses`
/// //
/// // Simply put, type witnesses emulate matching over a range of types
/// // (not values of those types, the types themselves).
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
/// //     L: 'b + Debug,
/// //     R: 'b + Debug,
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
///         T: ('b +  Debug), 
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