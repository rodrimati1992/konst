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
#[inline(always)]
pub const fn as_inner<T>(md: &ManuallyDrop<T>) -> &T {
    // SAFETY: ManuallyDrop<T> is represented the same as T,
    //         so their pointers should be compatible with an `as` cast
    unsafe { &*(md as *const ManuallyDrop<T> as *const T) }
}

/// Const equivalent of `&mut *manually_drop`
///
/// # Example
///
/// ```rust
/// use std::mem::ManuallyDrop;
/// use konst::manually_drop;
///
/// const fn add_100(num: &mut u32 ) {
///     *num += 100;
/// }
///
/// const FOO: ManuallyDrop<u32> = {
///     let mut mu = ManuallyDrop::new(5);
///     let inner = manually_drop::as_inner_mut(&mut mu);
///     add_100(inner);
///     add_100(inner);
///     add_100(inner);
///     mu
/// };
///
/// assert_eq!(*FOO, 305);
/// ```
#[cfg(feature = "rust_1_83")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_83")))]
#[inline(always)]
pub const fn as_inner_mut<T>(md: &mut ManuallyDrop<T>) -> &mut T {
    // SAFETY: ManuallyDrop<T> is represented the same as T,
    //         so their pointers should be compatible with an `as` cast
    unsafe { &mut *(md as *mut ManuallyDrop<T> as *mut T) }
}



/// Const equivalent of [`core::mem::ManuallyDrop::take`] with the same safety requirements.
#[cfg(feature = "rust_1_83")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_83")))]
#[inline(always)]
pub const unsafe fn take<T>(md: &mut ManuallyDrop<T>) -> T {
    // SAFETY: ManuallyDrop<T> is represented the same as T,
    //         so it's valid to read T out of a pointer to ManuallyDrop<T>
    unsafe { (md as *mut ManuallyDrop<T> as *mut T).read() }
}
