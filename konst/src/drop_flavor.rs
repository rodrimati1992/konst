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
/// # Example
///
/// ```rust
/// use konst::drop_flavor::{self, DropFlavor, MayDrop, NonDrop};
///
///
/// const fn using_nondrop<T: Copy>(val: T) {
///     let container = Container::of_copy(val);
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
///   --> konst/src/drop_flavor.rs:25:9
///    |
/// 10 |     let container = Container::of_drop(val);
///    |         ^^^^^^^^^ the destructor for this type cannot be evaluated in constant functions
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
pub enum MayDrop {}

impl DropFlavor for MayDrop {
    type Wrap<T> = T;
}

/// Type argument for types that don't need dropping-
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
pub const fn unwrap<D, T>(wrapper: D::Wrap<T>) -> T
where
    D: DropFlavor,
{
    // SAFETY: because DropFlavor is sealed, `D::Wrap<T>` is transmutable to `T`
    // because it's either `T` or a `ManuallyDrop<T>`
    unsafe { crate::__priv_transmute!(D::Wrap<T>, T, wrapper) }
}

/// Coerces [`&D::Wrap<T>`](DropFlavor::Wrap) into its contained `&T`
pub const fn as_inner<D, T>(wrapper: &D::Wrap<T>) -> &T
where
    D: DropFlavor,
{
    // SAFETY: because DropFlavor is sealed, `&D::Wrap<T>` is castable to `&T`
    // because it's either `&T` or a `&ManuallyDrop<T>`
    unsafe { &*(wrapper as *const D::Wrap<T> as *const T) }
}

/// Coerces [`&mut D::Wrap<T>`](DropFlavor::Wrap) into its contained `&mut T`
pub const fn as_inner_mut<D, T>(wrapper: &mut D::Wrap<T>) -> &mut T
where
    D: DropFlavor,
{
    // SAFETY: because DropFlavor is sealed, `&mut D::Wrap<T>` is castable to `&mut T`
    // because it's either `&mut T` or a `&mut ManuallyDrop<T>`
    unsafe { &mut *(wrapper as *mut D::Wrap<T> as *mut T) }
}

/// Converts `T` into either `T` or `ManuallyDrop<T>` as determined by the return type.
pub const fn wrap<U, T>(wrapper: T) -> U
where
    U: DropFlavorWrapper<T>,
{
    // SAFETY: `T` is transmutable to `U`
    // because it's either `T` or a `ManuallyDrop<T>`
    unsafe { crate::__priv_transmute!(T, U, wrapper) }
}
