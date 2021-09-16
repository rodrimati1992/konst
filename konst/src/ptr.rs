//! Const equivalents of raw pointer and [`NonNull`](core::ptr::NonNull) methods.

use core::ptr::NonNull;

/// Const equivalent of `&*raw_pointer`.
///
///
/// # Safety
///
/// This function has the safety requirements of
/// [`<*const>::as_ref`](https://doc.rust-lang.org/1.55.0/std/primitive.pointer.html#safety),
/// in addition to requiring that `ptr` is not null.
///
/// # Example
///
/// ```rust
/// use konst::ptr;
///
/// const F: &u8 = unsafe{ ptr::deref("foo".as_ptr()) };
/// assert_eq!(F, &b'f');
///
/// const BAR: &[u8; 3] = unsafe{ ptr::deref("bar".as_ptr().cast::<[u8; 3]>()) };
/// assert_eq!(BAR, b"bar");
///
///
/// ```
#[cfg(feature = "rust_1_56")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_56")))]
pub const unsafe fn deref<'a, T: ?Sized>(ptr: *const T) -> &'a T {
    core::mem::transmute(ptr)
}

/// Const equivalent of
/// [`<*const>::as_ref`](https://doc.rust-lang.org/std/primitive.pointer.html#method.as_ref)
///
/// # Safety
///
/// This function has the same safety requirements as
/// [`<*const>::as_ref`](https://doc.rust-lang.org/1.55.0/std/primitive.pointer.html#safety)
///
/// # Example
///
/// ```rust
/// use konst::ptr;
///
/// use core::ptr::null;
///
/// const NONE: Option<&u8> = unsafe{ ptr::as_ref(null()) };
/// const SOME: Option<&u8> = unsafe{ ptr::as_ref(&100) };
///
/// assert_eq!(NONE, None);
/// assert_eq!(SOME, Some(&100));
///
///
/// ```
#[cfg(feature = "rust_1_56")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_56")))]
pub const unsafe fn as_ref<'a, T: ?Sized>(ptr: *const T) -> Option<&'a T> {
    core::mem::transmute(ptr)
}

/// Const equivalent of
/// [`<*const>::is_null`](https://doc.rust-lang.org/std/primitive.pointer.html#method.is_null)
///
/// # Example
///
/// ```rust
/// use konst::ptr;
///
/// use core::ptr::null;
///
/// const NULL_IS_NULL: bool = unsafe{ ptr::is_null(null::<u8>()) };
/// const REFF_IS_NULL: bool = unsafe{ ptr::is_null(&100) };
///
/// assert_eq!(NULL_IS_NULL, true);
/// assert_eq!(REFF_IS_NULL, false);
///
///
/// ```
#[cfg(feature = "rust_1_56")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_56")))]
pub const fn is_null<'a, T: ?Sized>(ptr: *const T) -> bool {
    unsafe {
        matches!(
            core::mem::transmute::<*const T, Option<NonNull<T>>>(ptr),
            None
        )
    }
}

/// Const equivalents of [`NonNull`](core::ptr::NonNull) methods.
pub mod nonnull {
    use core::ptr::NonNull;

    /// Const equivalent of [`NonNull::new`](core::ptr::NonNull::new).
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::ptr::nonnull;
    ///
    /// use core::ptr::{NonNull, null_mut};
    ///
    /// const NONE: Option<NonNull<u8>> = unsafe{ nonnull::new(null_mut()) };
    /// const SOME: Option<NonNull<u8>> = unsafe{ nonnull::new(&100 as *const _ as *mut _) };
    ///
    /// assert!(NONE.is_none());
    /// assert_eq!(SOME.map(|x|unsafe{*x.as_ptr()}), Some(100));
    ///
    ///
    /// ```
    #[cfg(feature = "rust_1_56")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_56")))]
    pub const fn new<T: ?Sized>(ptr: *mut T) -> Option<NonNull<T>> {
        unsafe { core::mem::transmute(ptr) }
    }

    /// Const equivalent of [`NonNull::as_ref`](core::ptr::NonNull::as_ref).
    ///
    /// # Safety
    ///
    /// This has [the same safety requirements as `NonNull::as_ref`
    /// ](https://doc.rust-lang.org/1.55.0/core/ptr/struct.NonNull.html#safety-3)
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::ptr::nonnull;
    ///
    /// use core::{
    ///     ptr::NonNull,
    ///     marker::PhantomData,
    /// };
    ///
    /// const A: NonNull<u8> = nonnull::from_ref(&3);
    /// const A_REF: &u8 = unsafe{ nonnull::as_ref(A) };
    /// assert_eq!(A_REF, &3);
    ///
    /// const B: NonNull<str> = nonnull::from_ref("hello");
    /// const B_REF: &str = unsafe{ nonnull::as_ref(B) };
    /// assert_eq!(B_REF, "hello");
    ///
    /// ```
    ///
    #[cfg(feature = "rust_1_56")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_56")))]
    pub const unsafe fn as_ref<'a, T: ?Sized>(ptr: NonNull<T>) -> &'a T {
        core::mem::transmute(ptr)
    }

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
    /// const H: NonNull<str> = unsafe{ nonnull::from_ref("hello") };
    /// const W: NonNull<str> = unsafe{ nonnull::from_ref("world") };
    ///
    /// unsafe{
    ///     assert_eq!(H.as_ref(), "hello");
    ///     assert_eq!(W.as_ref(), "world");
    /// }
    /// ```
    pub const fn from_ref<T: ?Sized>(reff: &T) -> NonNull<T> {
        unsafe { NonNull::new_unchecked(reff as *const _ as *mut _) }
    }
}
