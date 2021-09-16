//! Generic constants for `MaybeUninit<T>`.

use core::mem::MaybeUninit;

declare_generic_const! {
    /// Generic constant for an uninitialized `MaybeUninit<T>`.
    /// Usable to safely construct a `[MaybeUninit<T>; LEN]` when `T` is non-`Copy`.
    ///
    /// As of Rust 1.51.0, `[MaybeUninit::uninit(); LEN]` is not valid for non-`Copy` types,
    /// but `[CONST; LEN]` does work, like in the example below.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::maybe_uninit::UNINIT;
    ///
    /// use std::mem::{self, MaybeUninit};
    ///
    /// // Intentionally doesn't implement `Copy`
    /// #[derive(Debug, PartialEq, Eq, Clone)]
    /// struct NonCopy(u8);
    ///
    /// const INITS: [NonCopy; 5] = {
    ///     let mut uninits = [UNINIT::<NonCopy>::V; 5];
    ///     konst::for_range!{i in 0..5=>
    ///         uninits[i] = MaybeUninit::new(NonCopy(i as u8 * 3));
    ///     }
    ///     unsafe{ mem::transmute(uninits) }
    /// };
    ///
    /// assert_eq!(INITS, [NonCopy(0), NonCopy(3), NonCopy(6), NonCopy(9), NonCopy(12)]);
    ///
    for[T]
    pub const UNINIT[T]: MaybeUninit<T> = MaybeUninit::uninit();
}

#[cfg(feature = "const_generics")]
declare_generic_const! {
    /// Generic constant for an uninitialized `[MaybeUninit<T>; N]`. Requires Rust 1.51.0.
    ///
    /// # Features
    ///
    /// This requires the "const_generics" feature, which requires Rust 1.51.0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::maybe_uninit::UNINIT_ARRAY;
    ///
    /// use std::mem::{self, MaybeUninit};
    ///
    /// const INITS: [[u8; 2]; 2] = {
    ///     let mut uninits = [UNINIT_ARRAY::<u8, 2>::V; 2];
    ///
    ///     uninits[0] = [MaybeUninit::new(3), MaybeUninit::new(5)];
    ///     uninits[1] = [MaybeUninit::new(8), MaybeUninit::new(13)];
    ///
    ///     unsafe{ mem::transmute(uninits) }
    /// };
    ///
    /// assert_eq!(INITS, [[3, 5], [8, 13]]);
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
    for[T, const N: usize]
    pub const UNINIT_ARRAY[T; N]: [MaybeUninit<T>; N] = [UNINIT::V; N];
}

/// Const equivalent of [`MaybeUninit::assume_init`](core::mem::MaybeUninit::assume_init)
///
/// # Safety
///
/// This has [the same safety requirements as `MaybeUninit::assume_init`
/// ](https://doc.rust-lang.org/1.55.0/core/mem/union.MaybeUninit.html#safety)
///
/// # Example
///
/// ```rust
/// use std::mem::MaybeUninit;
///
/// use konst::maybe_uninit;
///
/// const MU: MaybeUninit<u16> = MaybeUninit::new(12345);
/// const INIT: u16 = unsafe{ maybe_uninit::assume_init(MU) };
///
/// assert_eq!(INIT, 12345);
///
/// ```
#[cfg(feature = "rust_1_56")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_56")))]
pub const unsafe fn assume_init<T>(md: MaybeUninit<T>) -> T {
    crate::utils_156::__priv_transmute! {MaybeUninit<T>, T, md}
}

/// Const equivalent of [`MaybeUninit::assume_init_ref`](core::mem::MaybeUninit::assume_init_ref)
///
/// # Safety
///
/// This has [the same safety requirements as `MaybeUninit::assume_init_ref`
/// ](https://doc.rust-lang.org/1.55.0/core/mem/union.MaybeUninit.html#safety-3)
///
/// # Example
///
/// ```rust
/// use std::cmp::Ordering;
/// use std::mem::MaybeUninit;
///
/// use konst::maybe_uninit;
///
/// const MU: &MaybeUninit<Ordering> = &MaybeUninit::new(Ordering::Greater);
/// const INIT: &Ordering = unsafe{ maybe_uninit::assume_init_ref(MU) };
///
/// assert_eq!(INIT, &Ordering::Greater);
///
/// ```
#[cfg(feature = "rust_1_56")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_56")))]
pub const unsafe fn assume_init_ref<T>(md: &MaybeUninit<T>) -> &T {
    crate::utils_156::__priv_transmute_ref! {MaybeUninit<T>, T, md}
}

/// Const equivalent of [`MaybeUninit::as_ptr`](core::mem::MaybeUninit::as_ptr)
///
/// # Example
///
/// ```rust
/// use std::mem::MaybeUninit;
///
/// use konst::maybe_uninit;
///
/// const MU: &MaybeUninit<Option<&str>> = &MaybeUninit::new(Some("foo"));
/// const PTR: *const Option<&str> = maybe_uninit::as_ptr(MU);
///
/// unsafe {
///     assert_eq!(*PTR, Some("foo"));
/// }
///
/// ```
pub const fn as_ptr<T>(md: &MaybeUninit<T>) -> *const T {
    md as *const MaybeUninit<T> as *const T
}

/// Const equivalent of
/// [`MaybeUninit::array_assume_init`](core::mem::MaybeUninit::array_assume_init)
///
/// # Safety
///
/// This has [the same safety requirements as `MaybeUninit::array_assume_init`
/// ](https://doc.rust-lang.org/1.55.0/core/mem/union.MaybeUninit.html#safety-5)
///
/// # Example
///
/// ```rust
/// use std::mem::MaybeUninit;
///
/// use konst::maybe_uninit;
///
/// const INIT: [u16; 10] = {
///     let mut arr: [MaybeUninit<u16>; 10] = maybe_uninit::UNINIT_ARRAY::V;
///
///     let mut i = 0usize;
///     while i < 10 {
///         let x = (i as u16) + 1;
///         arr[i as usize] = MaybeUninit::new(x * x);
///         i += 1;
///     }
///
///     unsafe{ maybe_uninit::array_assume_init(arr) }
/// };
///
/// assert_eq!(INIT, [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]);
///
/// ```
#[cfg(feature = "rust_1_56")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_56")))]
pub const unsafe fn array_assume_init<T, const N: usize>(md: [MaybeUninit<T>; N]) -> [T; N] {
    crate::utils_156::__priv_transmute! {[MaybeUninit<T>; N], [T; N], md}
}
