//! Const equivalents of raw pointer and [`NonNull`](core::ptr::NonNull) methods.
//!
//! # Removed in 0.4.0
//!
//! - `is_null` was removed because it was deprecated in 0.3.0 for unsoundness.
//! - `as_ref`: [method was const-stabilized in 1.84.0](
//!              https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref)
//! - `as_mut`: [method was const-stabilized in 1.84.0](
//!              https://doc.rust-lang.org/std/primitive.pointer.html#method.as_mut)

/// Const equivalents of [`NonNull`](core::ptr::NonNull) methods.
///
/// # Removed in 0.4.0
///
/// These functions were removed in 0.4.0 because there is an equivalent
/// const fn in the standard library:
///
/// - `as_ref`: [core::ptr::NonNull::as_ref]
/// - `as_mut`: [core::ptr::NonNull::as_mut]
///
/// `new` was removed because it was deprecated in 0.3.0 for unsoundness.
///
pub mod nonnull {
    use core::ptr::NonNull;

    /// Const equivalent of
    /// [`<NonNull<T> as From<&T>>::from`
    /// ](https://doc.rust-lang.org/1.55.0/std/ptr/struct.NonNull.html#impl-From%3C%26%27_%20T%3E)
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::ptr::nonnull;
    ///
    /// use core::ptr::NonNull;
    ///
    /// const H: NonNull<str> = nonnull::from_ref("hello");
    /// const W: NonNull<str> = nonnull::from_ref("world");
    ///
    /// unsafe{
    ///     assert_eq!(H.as_ref(), "hello");
    ///     assert_eq!(W.as_ref(), "world");
    /// }
    /// ```
    pub const fn from_ref<T: ?Sized>(reff: &T) -> NonNull<T> {
        // SAFETY: `&T` is non-null, which is all that `NonNull::new_unchecked` requires
        unsafe { NonNull::new_unchecked(reff as *const _ as *mut _) }
    }

    /// Const equivalent of
    /// [`<NonNull<T> as From<&mut T>>::from`
    /// ](https://doc.rust-lang.org/1.55.0/std/ptr/struct.NonNull.html#impl-From%3C%26%27_%20mut%20T%3E)
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::ptr::nonnull as nn;
    ///
    /// use core::ptr::NonNull;
    ///
    /// assert_eq!(ARR, (5, 8, 3));
    ///
    /// const ARR: (u8, u8, u8) = unsafe {
    ///     let mut tup = (3, 5, 8);
    ///     swap(nn::from_mut(&mut tup.0), nn::from_mut(&mut tup.1));
    ///     swap(nn::from_mut(&mut tup.1), nn::from_mut(&mut tup.2));
    ///     tup
    /// };
    ///
    /// const unsafe fn swap(mut x: NonNull<u8>, mut y: NonNull<u8>) {
    ///     let xm = x.as_mut();
    ///     let ym = y.as_mut();
    ///     let tmp = *xm;
    ///     *xm = *ym;
    ///     *ym = tmp;
    /// }
    ///
    /// ```
    ///
    pub const fn from_mut<T: ?Sized>(mutt: &mut T) -> NonNull<T> {
        // SAFETY: `&mut T` is non-null, which is all that `NonNull::new_unchecked` requires
        unsafe { NonNull::new_unchecked(mutt) }
    }
}
