//! Contains items for parameterizing types by whether they need dropping.
//!
//! Refer to [`DropFlavor`] for more details

use core::mem::ManuallyDrop;

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::MayDrop {}
    impl Sealed for super::NonDrop {}
}

/// For parameterizing types by whether they need dropping.
///
/// This trait is sealed, it's implemented by [`NonDrop`] and [`MayDrop`],
/// it cannot be implemented by any other type.
///
/// # Motivation
///
/// The reason this whole module exists is to allow containers to
/// need dropping only when the type they wrap doesn't need dropping.
///
/// # Example
///
/// As you can see here, to make the container drop conditionally, you need to have
/// a `Foo` that wraps a `FooInner` in this particular way,
/// and then define constructors for types that need dropping (what `of_drop` does here)
/// and types that don't (what `of_copy` does here).
///
/// ```rust
/// use konst::drop_flavor::{self, DropFlavor, MayDrop, NonDrop};
///
///
/// const fn using_nondrop<T: Copy>(val: T) {
///     let mut container = Container::of_copy(val);
///     
///     let by_ref: &T = container.get();
///     let by_mut: &mut T = container.get_mut();
///     
///     // some code that does something useful with Container
/// }
///
/// struct Container<T, D: DropFlavor>(D::Wrap<ContainerInner<T>>);
///
/// struct ContainerInner<T>(T);
///
/// impl<T> Container<T, NonDrop> {
///     const fn of_copy(val: T) -> Self
///     where
///         T: Copy
///     {
///         Container(drop_flavor::wrap(ContainerInner(val)))
///     }
/// }
/// impl<T> Container<T, MayDrop> {
///     const fn of_drop(val: T) -> Self {
///         Container(drop_flavor::wrap(ContainerInner(val)))
///     }
/// }
///
/// impl<T, D: DropFlavor> Container<T, D> {
///     const fn get(&self) -> &T {
///         &drop_flavor::as_inner::<D, _>(&self.0).0
///     }
///     const fn get_mut(&mut self) -> &mut T {
///         &mut drop_flavor::as_inner_mut::<D, _>(&mut self.0).0
///     }
/// }
///
/// impl<T> Drop for ContainerInner<T> {
///     // only ran if `Container<T, D>` is `Container<T, MayDrop>`
///     fn drop(&mut self) {
///         println!("dropping {}!", std::any::type_name::<T>());
///     }
/// }
///
/// ```
///
/// Attempting to construct `Container` above with `of_drop` instead of `of_copy`
/// produces this compile-time error:
/// ```text
/// error[E0493]: destructor of `Container<T, MayDrop>` cannot be evaluated at compile-time
///   --> konst/src/drop_flavor.rs:25:13
///    |
/// 10 |     let mut container = Container::of_drop(val);
///    |             ^^^^^^^^^ the destructor for this type cannot be evaluated in constant functions
/// ...
/// 13 | }
///    | - value is dropped here
/// ```
///
///
pub trait DropFlavor: sealed::Sealed + 'static + Sized {
    /// This can be either:
    /// - if `Self == NonDrop`: `ManuallyDrop<T>`
    /// - if `Self == MayDrop`: `T`
    type Wrap<T>: DropFlavorWrapper<T, Flavor = Self>;
}

/// Type argument for types that may need dropping.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MayDrop {}

impl DropFlavor for MayDrop {
    type Wrap<T> = T;
}

/// Type argument for types that don't need dropping-
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonDrop {}

impl DropFlavor for NonDrop {
    type Wrap<T> = ManuallyDrop<T>;
}

/// Trait for the types that [`DropFlavor::Wrap`] can produce.
pub trait DropFlavorWrapper<T> {
    /// The DropFlavor that [`wrap`]s `T` into `Self`
    type Flavor: DropFlavor<Wrap<T> = Self>;
}

impl<T> DropFlavorWrapper<T> for T {
    type Flavor = MayDrop;
}

impl<T> DropFlavorWrapper<T> for ManuallyDrop<T> {
    type Flavor = NonDrop;
}

/// Unwraps [`D::Wrap<T>`](DropFlavor::Wrap) into `T`
///
/// # Example
///
/// ```rust
/// use konst::drop_flavor::{self, DropFlavor, MayDrop, NonDrop};
///
/// use std::mem::ManuallyDrop;
///
///
/// assert_eq!(unwrap_foo::<_, MayDrop>(Foo(10)), 10);
///
/// assert_eq!(unwrap_foo::<_, NonDrop>(Foo(ManuallyDrop::new(10))), 10);
///
///
/// fn unwrap_foo<T, D: DropFlavor>(foo: Foo<T, D>) -> T {
///     drop_flavor::unwrap::<D, _>(foo.0)
/// }
///
/// struct Foo<T, D: DropFlavor>(D::Wrap<T>);
/// ```
///
pub const fn unwrap<D, T>(wrapper: D::Wrap<T>) -> T
where
    D: DropFlavor,
{
    // SAFETY: because DropFlavor is sealed, `D::Wrap<T>` is transmutable to `T`
    // because it's either `T` or a `ManuallyDrop<T>`
    unsafe { crate::__priv_transmute!(D::Wrap<T>, T, wrapper) }
}

/// Coerces [`&D::Wrap<T>`](DropFlavor::Wrap) into its contained `&T`
///
/// # Example
///
/// ```rust
/// use konst::drop_flavor::{self, DropFlavor, MayDrop, NonDrop};
///
/// use std::mem::ManuallyDrop;
///
///
/// assert_eq!(foo_as_inner::<_, MayDrop>(&Foo(10)), &10);
///
/// assert_eq!(foo_as_inner::<_, NonDrop>(&Foo(ManuallyDrop::new(10))), &10);
///
///
/// fn foo_as_inner<T, D: DropFlavor>(foo: &Foo<T, D>) -> &T {
///     drop_flavor::as_inner::<D, _>(&foo.0)
/// }
///
/// struct Foo<T, D: DropFlavor>(D::Wrap<T>);
/// ```
///
pub const fn as_inner<D, T>(wrapper: &D::Wrap<T>) -> &T
where
    D: DropFlavor,
{
    // SAFETY: because DropFlavor is sealed, `&D::Wrap<T>` is castable to `&T`
    // because it's either `&T` or a `&ManuallyDrop<T>`
    unsafe { &*(wrapper as *const D::Wrap<T> as *const T) }
}

/// Coerces [`&mut D::Wrap<T>`](DropFlavor::Wrap) into its contained `&mut T`
///
/// # Example
///
/// ```rust
/// use konst::drop_flavor::{self, DropFlavor, MayDrop, NonDrop};
///
/// use std::mem::ManuallyDrop;
///
///
/// assert_eq!(foo_as_inner_mut::<_, MayDrop>(&mut Foo(10)), &mut 10);
///
/// assert_eq!(foo_as_inner_mut::<_, NonDrop>(&mut Foo(ManuallyDrop::new(10))), &mut 10);
///
///
/// fn foo_as_inner_mut<T, D: DropFlavor>(foo: &mut Foo<T, D>) -> &mut T {
///     drop_flavor::as_inner_mut::<D, _>(&mut foo.0)
/// }
///
/// struct Foo<T, D: DropFlavor>(D::Wrap<T>);
/// ```
///
pub const fn as_inner_mut<D, T>(wrapper: &mut D::Wrap<T>) -> &mut T
where
    D: DropFlavor,
{
    // SAFETY: because DropFlavor is sealed, `&mut D::Wrap<T>` is castable to `&mut T`
    // because it's either `&mut T` or a `&mut ManuallyDrop<T>`
    unsafe { &mut *(wrapper as *mut D::Wrap<T> as *mut T) }
}

/// Converts `T` into either `T` or `ManuallyDrop<T>` as determined by the return type.
///
/// # Example
///
/// ```rust
/// use konst::drop_flavor::{self, DropFlavor, MayDrop, NonDrop};
///
/// use std::mem::ManuallyDrop;
///
///
/// assert_eq!(make_foo::<MayDrop, _>(3), Foo(3));
///
/// assert_eq!(make_foo::<NonDrop, _>(5), Foo(ManuallyDrop::new(5)));
///
///
/// const fn make_foo<D: DropFlavor, T>(val: T) -> Foo<D, T> {
///     Foo(drop_flavor::wrap(val))
/// }
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct Foo<D: DropFlavor, T>(D::Wrap<T>);
/// ```
///
pub const fn wrap<W, T>(wrapper: T) -> W
where
    W: DropFlavorWrapper<T>,
{
    // SAFETY: `T` is transmutable to `W`
    // because it's either `T` or a `ManuallyDrop<T>`
    unsafe { crate::__priv_transmute!(T, W, wrapper) }
}
