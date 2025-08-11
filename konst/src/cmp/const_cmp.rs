use crate::cmp::{CmpWrapper, IsNotStdKind, IsStdKind};

use core::marker::PhantomData;

use typewit::TypeEq;

////////////////////////////////////////////////////////////////////////////////

/// Marker trait for types that implement the const comparison methods.
///
/// # Implementors
///
/// Types that implement this trait are also expected to implement at least one of
/// these inherent methods:
///
/// ```rust
/// use std::cmp::Ordering;
///
/// # struct Foo;
/// # impl Foo {
/// const fn const_eq(&self, other: &Self) -> bool
/// # { true }
///
/// const fn const_cmp(&self, other: &Self) -> Ordering
/// # { Ordering::Equal }
/// # }
///
/// ```
///
/// # Coercions
///
/// The [`Kind`](#associatedtype.Kind) associated type
/// is used in the [`IsAConstCmp`] marker type
/// to automatically wrap types in [`CmpWrapper`] if they're from the standard library,
/// otherwise leaving them unwrapped.
///
///
/// # Example
///
/// ### Manual Implementation
///
/// ```
/// use konst::{
///     cmp::{ConstCmp, IsNotStdKind, const_cmp, const_eq, try_equal},
///     assertc_eq, assertc_ne
/// };
///
/// use std::cmp::Ordering;
///
///
/// struct MyType {
///     x: &'static str,
///     y: &'static [u16],
/// }
///
/// impl ConstCmp for MyType {
///     type Kind = IsNotStdKind;
///     type This = Self;
/// }
///
/// impl MyType {
///     pub const fn const_eq(&self, other: &Self) -> bool {
///         const_eq!(self.x, other.x) &&
///         const_eq!(self.y, other.y)
///     }
///
///     pub const fn const_cmp(&self, other: &Self) -> Ordering {
///         try_equal!(const_cmp!(self.x, other.x));
///         try_equal!(const_cmp!(self.y, other.y))
///     }
/// }
///
/// const _: () = {
///     let foo = MyType{x: "hello", y: &[3, 5, 8, 13]};
///     let bar = MyType{x: "world", y: &[3, 5, 8, 13]};
///
///     assertc_eq!(const_cmp!(foo, foo), Ordering::Equal);
///     assertc_eq!(const_cmp!(foo, bar), Ordering::Less);
///     assertc_eq!(const_cmp!(bar, foo), Ordering::Greater);
///     assert!(const_eq!(foo, foo));
///     assert!(!const_eq!(foo, bar));
/// };
/// ```
///
///
/// ### `ìmpl_cmp`-based Implementation
///
/// You can use [`impl_cmp`] to implement this trait,
/// as well as define the same methods for
/// multiple implementations with different type arguments.
///
/// ```
/// use konst::cmp::{const_cmp, const_eq, impl_cmp, try_equal};
/// use konst::{assertc_eq, assertc_ne};
///
/// use std::cmp::Ordering;
///
///
/// struct MyType<'a, T> {
///     x: &'a str,
///     y: &'a [T],
/// }
///
/// impl_cmp!{
///     // The comparison functions are only implemented for these types.
///     impl['a] MyType<'a, bool>;
///     impl['a] MyType<'a, u16>;
///     impl['a] MyType<'a, &'static str>;
///
///     pub const fn const_eq(&self, other: &Self) -> bool {
///         const_eq!(self.x, other.x) &&
///         const_eq!(self.y, other.y)
///     }
///
///     pub const fn const_cmp(&self, other: &Self) -> Ordering {
///         try_equal!(const_cmp!(self.x, other.x));
///         try_equal!(const_cmp!(self.y, other.y))
///     }
/// }
///
/// const _: () = {
///     let foo = MyType{x: "hello", y: &[3, 5, 8, 13]};
///     let bar = MyType{x: "world", y: &[3, 5, 8, 13]};
///
///     assertc_eq!(const_cmp!(foo, foo), Ordering::Equal);
///     assertc_eq!(const_cmp!(foo, bar), Ordering::Less);
///     assertc_eq!(const_cmp!(bar, foo), Ordering::Greater);
///     assert!(const_eq!(foo, foo));
///     assert!(!const_eq!(foo, bar));
/// };
///
/// const _: () = {
///     let foo = MyType{x: "hello", y: &[false]};
///     let bar = MyType{x: "hello", y: &[true]};
///
///     assertc_eq!(const_cmp!(foo, foo), Ordering::Equal);
///     assertc_eq!(const_cmp!(foo, bar), Ordering::Less);
///     assertc_eq!(const_cmp!(bar, foo), Ordering::Greater);
///     assert!(const_eq!(foo, foo));
///     assert!(!const_eq!(foo, bar));
/// };
///
/// ```
///
/// [`CmpWrapper`]: crate::cmp::CmpWrapper
/// [`impl_cmp`]: crate::cmp::impl_cmp
pub trait ConstCmp {
    /// What kind of type this is, this can be one of:
    ///
    /// - [`IsStdKind`]: A standard library type.
    ///
    /// - [`IsNotStdKind`]: A type that is not from the standard library.
    ///
    type Kind: ConstCmpKind;

    /// The type after dereferencing all references.
    ///
    /// User-defined types should generally set this to `Self`.
    type This: ?Sized + ConstCmp<Kind = Self::Kind, This = Self::This>;
}

///////////////////////////////////////////////////////////////////////////////

impl<T, const N: usize> ConstCmp for [T; N] {
    type Kind = IsStdKind;
    type This = Self;
}

impl<T> ConstCmp for [T] {
    type Kind = IsStdKind;
    type This = Self;
}

impl ConstCmp for str {
    type Kind = IsStdKind;
    type This = Self;
}

impl<T> ConstCmp for &T
where
    T: ?Sized + ConstCmp,
{
    type Kind = T::Kind;
    type This = T::This;
}

impl<T> ConstCmp for &mut T
where
    T: ?Sized + ConstCmp,
{
    type Kind = T::Kind;
    type This = T::This;
}

///////////////////////////////////////////////////////////////////////////////

/// Hack used to automatically wrap standard library types inside [`CmpWrapper`],
/// while leaving user defined types unwrapped.
///
/// You're not intended to use this directly, the intended way to coerce a type
/// into a type with a `const_eq` and/or `const_cmp` method is
/// with the [`coerce_to_cmp`] macro.
///
/// `R`: Is a type that implements [`ConstCmp`]
///
/// [`coerce_to_cmp`]: crate::cmp::coerce_to_cmp
#[allow(clippy::type_complexity)]
pub struct IsAConstCmp<R: ?Sized>(PhantomData<fn() -> PhantomData<R>>);

impl<R: ?Sized> Copy for IsAConstCmp<R> {}

impl<R: ?Sized> Clone for IsAConstCmp<R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R: ?Sized + ConstCmp> IsAConstCmp<R> {
    /// Constructs an `IsAConstCmp`
    pub const NEW: Self = Self(PhantomData);
}

impl<R: ?Sized + ConstCmp> IsAConstCmp<R> {
    /// Infers the type parameter by taking a reference to it.
    #[inline(always)]
    pub const fn infer_type(self, _: &R) -> Self {
        self
    }

    /// coerces `&T::This` into as reference to a type that has a `const_eq` and/or
    /// `const_cmp` method.
    ///
    /// The return type can be either:
    /// - `&CmpWrapper<R::This>` if `R::Kind == `[`IsStdKind`].
    /// - `&R::This` if `R::Kind == `[`IsNotStdKind`].
    #[inline]
    pub const fn coerce<'a>(self, reff: &'a R::This) -> CoerceTo<'a, R::This> {
        match const { <R::Kind as ConstCmpKind>::__KIND_WITNESS.to_coercion_witness::<'a, R::This>() }
        {
            __CoercionWitness::IsStdKind(te) => te.to_left(CmpWrapper::from_ref(reff)),
            __CoercionWitness::IsNotStdKind(te) => te.to_left(reff),
        }
    }

    /// Removes layers of references by coercing the argument.
    #[inline(always)]
    pub const fn unreference(self, r: &R::This) -> &R::This {
        r
    }
}

/////////////////////////////////////////////////////////////////////////////

/// Trait for the types that are assignable to [`ConstCmp::Kind`]
pub trait ConstCmpKind: Sized {
    /// What `IsAConstCmp::coerce` coerces `&T` into, it can be either:
    /// - `&CmpWrapper<T>` if `Self == `[`IsStdKind`].
    /// - `&T` if `Self == `[`IsNotStdKind`].
    type CoerceTo<T: ?Sized>: ?Sized;

    #[doc(hidden)]
    const __KIND_WITNESS: __ConstCmpKindWitness<Self>;
}

impl ConstCmpKind for IsStdKind {
    type CoerceTo<T: ?Sized> = CmpWrapper<T>;

    #[doc(hidden)]
    const __KIND_WITNESS: __ConstCmpKindWitness<Self> =
        __ConstCmpKindWitness::IsStdKind(TypeEq::NEW);
}

impl ConstCmpKind for IsNotStdKind {
    type CoerceTo<T: ?Sized> = T;

    #[doc(hidden)]
    const __KIND_WITNESS: __ConstCmpKindWitness<Self> =
        __ConstCmpKindWitness::IsNotStdKind(TypeEq::NEW);
}

/// What `IsAConstCmp::coerce` coerces `&'a T` into, it can be either:
/// - `&'a CmpWrapper<T>` if `<T as ConstCmp>::Kind` == [`IsStdKind`].
/// - `&'a T` if `<T as ConstCmp>::Kind` == [`IsNotStdKind`].
pub type CoerceTo<'a, T> = &'a <<T as ConstCmp>::Kind as ConstCmpKind>::CoerceTo<T>;

#[doc(hidden)]
pub enum __ConstCmpKindWitness<Kind> {
    IsStdKind(TypeEq<Kind, IsStdKind>),
    IsNotStdKind(TypeEq<Kind, IsNotStdKind>),
}

impl<Kind> __ConstCmpKindWitness<Kind> {
    const fn to_coercion_witness<'a, T>(self) -> __CoercionWitness<'a, T>
    where
        T: ?Sized + ConstCmp<Kind = Kind>,
        Kind: ConstCmpKind,
    {
        typewit::type_fn! {
            struct CoerceToFn<T: ?Sized>;
            impl<K: ConstCmpKind> K => <K as ConstCmpKind>::CoerceTo<T>
        }

        match self {
            Self::IsStdKind(te) => {
                __CoercionWitness::IsStdKind(te.project::<CoerceToFn<T>>().in_ref())
            }
            Self::IsNotStdKind(te) => {
                __CoercionWitness::IsNotStdKind(te.project::<CoerceToFn<T>>().in_ref())
            }
        }
    }
}

#[doc(hidden)]
enum __CoercionWitness<'a, T: ?Sized + ConstCmp> {
    IsStdKind(TypeEq<CoerceTo<'a, T>, &'a CmpWrapper<T>>),
    IsNotStdKind(TypeEq<CoerceTo<'a, T>, &'a T>),
}

/////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
pub struct __AssertConstCmp<'a, T: ConstCmp> {
    pub reff: &'a T,
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assert_const_cmp {
    ($reff:expr) => {
        $crate::cmp::__AssertConstCmp { reff: $reff }
    };
}
