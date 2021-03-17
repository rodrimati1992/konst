//! Marker trait for types that implement the const formatting methods.
//!
//!

use crate::polymorphism::CmpWrapper;

use core::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////

/// Marker trait for types that implement the const comparison methods.
///
pub trait ConstCmpMarker {
    /// What kind of type this is, this can be one of:
    ///
    /// - [`IsArrayKind`]: For slices, and arrays.
    ///
    /// - [`IsStdKind`]: Any other standard library type.
    ///
    /// - [`IsNotStdKind`]: Any type that is not from the standard library.
    ///
    /// [`IsArrayKind`]: ./struct.IsArrayKind.html
    /// [`IsStdKind`]: ./struct.IsStdKind.html
    /// [`IsNotStdKind`]: ./struct.IsNotStdKind.html
    type Kind;

    /// The type after dereferencing,
    /// implemented as `type This = Self;` for all non-reference types
    type This: ?Sized;
}

/// Marker type for arrays and slices,
/// used as the [`Kind`] associated type  in [`ConstCmpMarker`].
///
/// [`Kind`]: ./trait.ConstCmpMarker.html#associatedtype.Kind
/// [`ConstCmpMarker`]: ./trait.ConstCmpMarker.html
///
pub struct IsArrayKind<T>(PhantomData<T>);

/// Marker type for the remaining standard library types,,
/// used as the [`Kind`] associated type  in [`ConstCmpMarker`].
///
/// [`Kind`]: ./trait.ConstCmpMarker.html#associatedtype.Kind
/// [`ConstCmpMarker`]: ./trait.ConstCmpMarker.html
///
pub struct IsStdKind;

/// Marker type for non-standard library types,
/// used as the [`Kind`] associated type  in [`ConstCmpMarker`].
///
/// [`Kind`]: ./trait.ConstCmpMarker.html#associatedtype.Kind
/// [`ConstCmpMarker`]: ./trait.ConstCmpMarker.html
///
pub struct IsNotStdKind;

///////////////////////////////////////////////////////////////////////////////

/// Hack used to automatically wrap standard library types inside [`CmpWrapper`],
/// while leaving user defined types unwrapped.
///
/// # Type parameters
///
/// `K` is `<R as ConstCmpMarker>::Kind`
/// The kind of type that `T` is,
/// [a slice](./struct.IsArrayKind.html),
/// [other std types](./struct.IsStdKind.html),
/// [non-std types](./struct.IsNotStdKind.html).
///
/// `T` is `<R as ConstCmpMarker>::This`,
/// the `R` type after removing all layers of references.
///
/// # Coerce Method
///
/// The `coerce` method is what does the conversion from a `&T` depending on
/// the `K` type parameter:
///
/// - [`IsArrayKind`]: the reference is coerced to a slice, and wrapped in a [`CmpWrapper`].
///
/// - [`IsStdKind`]: the referenced value is copied, and wrapped in a [`CmpWrapper`].
///
/// - [`IsNotStdKind`]: the reference is simply returned as a `&T`.
///
/// [`IsArrayKind`]: ./struct.IsArrayKind.html
/// [`IsStdKind`]: ./struct.IsStdKind.html
/// [`IsNotStdKind`]: ./struct.IsNotStdKind.html
///
/// [`CmpWrapper`]: ../struct.CmpWrapper.html
///
#[allow(clippy::type_complexity)]
pub struct IsAConstCmpMarker<K, T: ?Sized, R: ?Sized>(
    PhantomData<(
        PhantomData<fn() -> PhantomData<K>>,
        PhantomData<fn() -> PhantomData<T>>,
        PhantomData<fn() -> PhantomData<R>>,
    )>,
);

impl<K, T: ?Sized, R: ?Sized> Copy for IsAConstCmpMarker<K, T, R> {}

impl<K, T: ?Sized, R: ?Sized> Clone for IsAConstCmpMarker<K, T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> IsAConstCmpMarker<R::Kind, R::This, R>
where
    R: ?Sized + ConstCmpMarker,
{
    /// Constructs an `IsAConstCmpMarker`
    pub const NEW: Self = Self(PhantomData);
}

impl<K, T: ?Sized, R: ?Sized> IsAConstCmpMarker<K, T, R> {
    /// Infers the type parameters by taking a reference to `R` .
    ///
    /// The `K` and `T` type parameters are determined by `R` in
    /// the [`NEW`] associated constant.
    ///
    /// [`NEW`]: #associatedconstant.NEW
    #[inline(always)]
    pub const fn infer_type(self, _: &R) -> Self {
        self
    }

    /// Removes layers of references by coercing the argument.
    #[inline(always)]
    pub const fn unreference(self, r: &T) -> &T {
        r
    }
}

/////////////////////////////////////////////////////////////////////////////

impl<U, T: ?Sized, R: ?Sized> IsAConstCmpMarker<IsArrayKind<U>, T, R> {
    /// Coerces an array to a slice, then wraps the slice in a `CmpWrapper`
    #[inline(always)]
    pub const fn coerce(self, slice: &[U]) -> CmpWrapper<&[U]> {
        CmpWrapper(slice)
    }
}

impl<T: ?Sized, R: ?Sized> IsAConstCmpMarker<IsNotStdKind, T, R> {
    /// An identity function, just takes `reference` and returns it.
    #[inline(always)]
    pub const fn coerce(self, reference: &T) -> &T {
        reference
    }
}

/////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "const_generics")]
macro_rules! array_impls {
    ($($tt:tt)*) => {
        impl<T, const N: usize> ConstCmpMarker for [T; N] {
            type Kind = IsArrayKind<T>;
            type This = Self;
        }
    };
}

#[cfg(not(feature = "const_generics"))]
macro_rules! array_impls {
    ($($len:literal),* $(,)* ) => (
        $(
            impl<T> ConstCmpMarker for [T; $len] {
                type Kind = IsArrayKind<T>;
                type This = Self;
            }
        )*
    )
}

impl ConstCmpMarker for str {
    type Kind = IsStdKind;
    type This = Self;
}

impl<R: ?Sized> IsAConstCmpMarker<IsStdKind, str, R> {
    /// Wraps `reference` in a `CmpWrapper`.
    #[inline(always)]
    pub const fn coerce(self, reference: &str) -> CmpWrapper<&str> {
        CmpWrapper(reference)
    }
}

array_impls! {
    0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,
    16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,
    32,
}

impl<T> ConstCmpMarker for [T] {
    type Kind = IsArrayKind<T>;
    type This = [T];
}

impl<T> ConstCmpMarker for &T
where
    T: ?Sized + ConstCmpMarker,
{
    type Kind = T::Kind;
    type This = T::This;
}

impl<T> ConstCmpMarker for &mut T
where
    T: ?Sized + ConstCmpMarker,
{
    type Kind = T::Kind;
    type This = T::This;
}
