//! Marker trait for types that implement the const formatting methods.
//!
//!

use crate::polymorphism::CmpWrapper;

use core::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////

/// Marker trait for types that implement the const comparison methods.
///
/// # Implementors
///
/// Types that implement this trait are also expected to implement at least one of
/// these inherent methods:
///
/// ```ignore
/// // use std::cmp::Ordering;
///
/// const fn const_eq(&self, other: &Self) -> bool
///
/// const fn const_cmp(&self, other: &Self) -> Ordering
///
/// ```
///
/// # Coercions
///
/// The [`Kind`](#associatedtype.Kind) associated type
/// is used in the [`IsAConstCmpMarker`] marker type
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
///     polymorphism::{ConstCmpMarker, IsNotStdKind},
///     const_cmp, const_eq, try_equal,
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
/// impl ConstCmpMarker for MyType {
///     // These are the only associated types you're expected to use in
///     // impls for custom types.
///     type Kind = IsNotStdKind;
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
/// const CMPS: [(Ordering, bool); 4] = {
///     let foo = MyType{x: "hello", y: &[3, 5, 8, 13]};
///     let bar = MyType{x: "world", y: &[3, 5, 8, 13]};
///
///     [
///         (const_cmp!(foo, foo), const_eq!(foo, foo)),
///         (const_cmp!(foo, bar), const_eq!(foo, bar)),
///         (const_cmp!(bar, foo), const_eq!(bar, foo)),
///         (const_cmp!(bar, bar), const_eq!(bar, bar)),
///     ]
/// };
///
/// assert_eq!(
///     CMPS,
///     [
///         (Ordering::Equal, true),
///         (Ordering::Less, false),
///         (Ordering::Greater, false),
///         (Ordering::Equal, true),
///     ]
/// );
///
///
///
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
/// use konst::{const_cmp, const_eq, impl_cmp, try_equal};
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
/// const CMPS: [(Ordering, bool); 4] = {
///     let foo = MyType{x: "hello", y: &[3u16, 5, 8, 13]};
///     let bar = MyType{x: "world", y: &[3, 5, 8, 13]};
///
///     [
///         (const_cmp!(foo, foo), const_eq!(foo, foo)),
///         (const_cmp!(foo, bar), const_eq!(foo, bar)),
///         (const_cmp!(bar, foo), const_eq!(bar, foo)),
///         (const_cmp!(bar, bar), const_eq!(bar, bar)),
///     ]
/// };
///
/// assert_eq!(
///     CMPS,
///     [
///         (Ordering::Equal, true),
///         (Ordering::Less, false),
///         (Ordering::Greater, false),
///         (Ordering::Equal, true),
///     ]
/// );
///
/// ```
///
/// [`CmpWrapper`]: struct.CmpWrapper.html
/// [`impl_cmp`]: ../macro.impl_cmp.html
pub trait ConstCmpMarker {
    /// What kind of type this is, this can be one of:
    ///
    /// - [`IsStdKind`]: A standard library type.
    ///
    /// - [`IsRefKind`]: A reference type.
    ///
    /// - [`IsNotStdKind`]: A type that is not from the standard library.
    ///
    type Kind;
}

/// Marker type for standard library types.
pub struct IsStdKind;

/// Marker type for references.
pub struct IsRefKind;

/// Marker type for non-standard library types.
pub struct IsNotStdKind;

///////////////////////////////////////////////////////////////////////////////

impl<T, const N: usize> ConstCmpMarker for [T; N] {
    type Kind = IsStdKind;
}

impl<T> ConstCmpMarker for [T] {
    type Kind = IsStdKind;
}

impl ConstCmpMarker for str {
    type Kind = IsStdKind;
}

impl<T> ConstCmpMarker for &T
where
    T: ?Sized + ConstCmpMarker,
{
    type Kind = IsRefKind;
}

impl<T> ConstCmpMarker for &mut T
where
    T: ?Sized + ConstCmpMarker,
{
    type Kind = IsRefKind;
}

///////////////////////////////////////////////////////////////////////////////

/// A helper trait of [`ConstCmpMarker`], used for dereferencing.
pub trait ConstCmpUnref: ConstCmpMarker {
    /// What type `Self` becomes after removing all layers of references.
    type This: ?Sized + ConstCmpMarker;
}

impl<T> ConstCmpUnref for T
where
    T: ?Sized + ConstCmpMarker,
    T: ConstCmpUnrefHelper<<T as ConstCmpMarker>::Kind>,
    T::This_: ConstCmpMarker,
{
    type This = T::This_;
}

/// An implementation detail of [`ConstCmpUnref`].
pub trait ConstCmpUnrefHelper<Kind> {
    type This_: ?Sized;
}

impl<T: ?Sized> ConstCmpUnrefHelper<IsStdKind> for T {
    type This_ = T;
}

impl<T: ?Sized> ConstCmpUnrefHelper<IsNotStdKind> for T {
    type This_ = T;
}

impl<T: ?Sized + ConstCmpUnref> ConstCmpUnrefHelper<IsRefKind> for &T {
    type This_ = T::This;
}
impl<T: ?Sized + ConstCmpUnref> ConstCmpUnrefHelper<IsRefKind> for &mut T {
    type This_ = T::This;
}

///////////////////////////////////////////////////////////////////////////////

/// Hack used to automatically wrap standard library types inside [`CmpWrapper`],
/// while leaving user defined types unwrapped.
///
/// This can be constructed with he [`NEW` associated constant](#associatedconstant.NEW)
///
/// # Type parameters
///
/// `K` is `<T as ConstCmpMarker>::Kind`
/// The kind of type that `T` is,
/// [std types](./struct.IsStdKind.html),
/// [non-std types](./struct.IsNotStdKind.html).
///
/// `T` is `<R as ConstCmpUnref>::This`,
/// the `R` type after removing all layers of references.
///
/// `R`: Is a type that implements [`ConstCmpMarker`]
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

impl<R, T> IsAConstCmpMarker<T::Kind, T, R>
where
    R: ?Sized + ConstCmpUnref<This = T>,
    T: ?Sized + ConstCmpMarker,
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

impl<T: ?Sized, R: ?Sized> IsAConstCmpMarker<IsNotStdKind, T, R> {
    /// An identity function, just takes `reference` and returns it.
    #[inline(always)]
    pub const fn coerce(self, reference: &T) -> &T {
        reference
    }
}

/////////////////////////////////////////////////////////////////////////////

impl<R: ?Sized> IsAConstCmpMarker<IsStdKind, str, R> {
    /// Wraps `reference` in a `CmpWrapper`.
    #[inline(always)]
    pub const fn coerce(self, reference: &str) -> CmpWrapper<&str> {
        CmpWrapper(reference)
    }
}

impl<T, R: ?Sized> IsAConstCmpMarker<IsStdKind, [T], R> {
    /// Wraps `reference` in a `CmpWrapper`.
    #[inline(always)]
    pub const fn coerce(self, reference: &[T]) -> CmpWrapper<&[T]> {
        CmpWrapper(reference)
    }
}

impl<T, R, const N: usize> IsAConstCmpMarker<IsStdKind, [T; N], R> {
    /// Wraps `reference` in a `CmpWrapper`.
    #[inline(always)]
    pub const fn coerce(self, reference: &[T; N]) -> CmpWrapper<&[T]> {
        CmpWrapper(reference)
    }
}
