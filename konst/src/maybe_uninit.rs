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
