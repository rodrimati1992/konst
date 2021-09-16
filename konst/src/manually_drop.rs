//! Const fn equivalents of [`ManuallyDrop<T>`](core::mem::ManuallyDrop) methods.

use core::mem::ManuallyDrop;

/// Const equivalent of `&*manually_drop`
///
/// # Example
///
/// ```rust
/// use std::mem::ManuallyDrop;
/// use konst::manually_drop;
///
/// const FOO: &Foo<u64> = &Foo::new(123456);
/// const FOO_REF: &u64 = FOO.get();
/// assert_eq!(FOO.get(), &123456);
/// assert_eq!(FOO_REF, &123456);
///
/// const MD: &ManuallyDrop<u64> = &ManuallyDrop::new(654321);
/// assert_eq!(manually_drop::as_inner(MD), &654321);
///
/// pub struct Foo<T>(ManuallyDrop<T>);
///
/// impl<T> Foo<T> {
///     pub const fn new(value: T) -> Self {
///         Self(ManuallyDrop::new(value))
///     }
///     
///     pub const fn get(&self) -> &T {
///         manually_drop::as_inner(&self.0)
///     }
/// }
/// ```
pub const fn as_inner<T>(md: &ManuallyDrop<T>) -> &T {
    unsafe {
        crate::utils_156::__priv_transmute_ref! {ManuallyDrop<T>, T, md}
    }
}
