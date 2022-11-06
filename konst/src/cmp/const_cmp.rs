use crate::cmp::{CmpWrapper, IsNotStdKind, IsRefKind, IsStdKind};

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
///     cmp::{ConstCmp, IsNotStdKind},
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
/// impl ConstCmp for MyType {
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
/// const _: () = {
///     let foo = MyType{x: "hello", y: &[3, 5, 8, 13]};
///     let bar = MyType{x: "world", y: &[3, 5, 8, 13]};
///
///     assert!(matches!(const_cmp!(foo, foo), Ordering::Equal));
///     assert!(matches!(const_cmp!(foo, bar), Ordering::Less));
///     assert!(matches!(const_cmp!(bar, foo), Ordering::Greater));
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
/// const _: () = {
///     let foo = MyType{x: "hello", y: &[3, 5, 8, 13]};
///     let bar = MyType{x: "world", y: &[3, 5, 8, 13]};
///
///     assert!(matches!(const_cmp!(foo, foo), Ordering::Equal));
///     assert!(matches!(const_cmp!(foo, bar), Ordering::Less));
///     assert!(matches!(const_cmp!(bar, foo), Ordering::Greater));
///     assert!(const_eq!(foo, foo));
///     assert!(!const_eq!(foo, bar));
/// };
///
/// const _: () = {
///     let foo = MyType{x: "hello", y: &[false]};
///     let bar = MyType{x: "hello", y: &[true]};
///
///     assert!(matches!(const_cmp!(foo, foo), Ordering::Equal));
///     assert!(matches!(const_cmp!(foo, bar), Ordering::Less));
///     assert!(matches!(const_cmp!(bar, foo), Ordering::Greater));
///     assert!(const_eq!(foo, foo));
///     assert!(!const_eq!(foo, bar));
/// };
///
/// ```
///
/// [`CmpWrapper`]: struct.CmpWrapper.html
/// [`impl_cmp`]: ../macro.impl_cmp.html
pub trait ConstCmp {
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

///////////////////////////////////////////////////////////////////////////////

impl<T, const N: usize> ConstCmp for [T; N] {
    type Kind = IsStdKind;
}

impl<T> ConstCmp for [T] {
    type Kind = IsStdKind;
}

impl ConstCmp for str {
    type Kind = IsStdKind;
}

impl<T> ConstCmp for &T
where
    T: ?Sized + ConstCmp,
{
    type Kind = IsRefKind;
}

impl<T> ConstCmp for &mut T
where
    T: ?Sized + ConstCmp,
{
    type Kind = IsRefKind;
}

///////////////////////////////////////////////////////////////////////////////

/// A helper trait of [`ConstCmp`], used for dereferencing.
pub trait ConstCmpUnref: ConstCmp {
    /// What type `Self` becomes after removing all layers of references.
    ///
    /// Examples:
    /// - `u32::This == u32`
    /// - `<&u32>::This == u32`
    /// - `<&&u32>::This == u32`
    type This: ?Sized + ConstCmp;
}

impl<T> ConstCmpUnref for T
where
    T: ?Sized + ConstCmp,
    T: ConstCmpUnrefHelper<<T as ConstCmp>::Kind>,
    T::This_: ConstCmp,
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
/// `K` is `<T as ConstCmp>::Kind`
/// The kind of type that `T` is: either [`IsStdKind`] or
/// [`IsNotStdKind`](crate::cmp::IsNotStdKind).
///
/// `T` is `<R as ConstCmpUnref>::This`,
/// the `R` type after removing all layers of references.
///
/// `R`: Is a type that implements [`ConstCmp`]
///
#[allow(clippy::type_complexity)]
pub struct IsAConstCmp<K, T: ?Sized, R: ?Sized>(
    PhantomData<(
        PhantomData<fn() -> PhantomData<K>>,
        PhantomData<fn() -> PhantomData<T>>,
        PhantomData<fn() -> PhantomData<R>>,
    )>,
);

impl<K, T: ?Sized, R: ?Sized> Copy for IsAConstCmp<K, T, R> {}

impl<K, T: ?Sized, R: ?Sized> Clone for IsAConstCmp<K, T, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R, T> IsAConstCmp<T::Kind, T, R>
where
    R: ?Sized + ConstCmpUnref<This = T>,
    T: ?Sized + ConstCmp,
{
    /// Constructs an `IsAConstCmp`
    pub const NEW: Self = Self(PhantomData);
}

impl<K, T: ?Sized, R: ?Sized> IsAConstCmp<K, T, R> {
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

impl<T: ?Sized, R: ?Sized> IsAConstCmp<IsNotStdKind, T, R> {
    /// An identity function, just takes `reference` and returns it.
    #[inline(always)]
    pub const fn coerce(self, reference: &T) -> &T {
        reference
    }
}

/////////////////////////////////////////////////////////////////////////////

impl<R: ?Sized> IsAConstCmp<IsStdKind, str, R> {
    /// Wraps `reference` in a `CmpWrapper`.
    #[inline(always)]
    pub const fn coerce(self, reference: &str) -> CmpWrapper<&str> {
        CmpWrapper(reference)
    }
}

impl<T, R: ?Sized> IsAConstCmp<IsStdKind, [T], R> {
    /// Wraps `reference` in a `CmpWrapper`.
    #[inline(always)]
    pub const fn coerce(self, reference: &[T]) -> CmpWrapper<&[T]> {
        CmpWrapper(reference)
    }
}

impl<T, R, const N: usize> IsAConstCmp<IsStdKind, [T; N], R> {
    /// Wraps `reference` in a `CmpWrapper`.
    #[inline(always)]
    pub const fn coerce(self, reference: &[T; N]) -> CmpWrapper<&[T]> {
        CmpWrapper(reference)
    }
}
