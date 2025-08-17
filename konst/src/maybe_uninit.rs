//! Const fn equivalents of [`MaybeUninit<T>`](core::mem::MaybeUninit) methods.
//!
//! # Removed in 0.4.0
//!
//! These items were removed in 0.4.0 because there is an equivalent
//! way to write it in const:
//!
//! - `as_mut_ptr`: [`MaybeUninit::as_mut_ptr`]
//! - `assume_init_mut`: [`MaybeUninit::assume_init_mut`]
//! - `UNINIT`: `const { MaybeUninit::uninit() }`
//! - `write`: [`MaybeUninit::write`]
//!

use core::mem::{ManuallyDrop, MaybeUninit};

declare_generic_const! {
    /// Generic constant for an uninitialized `[MaybeUninit<T>; N]`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::maybe_uninit::UNINIT_ARRAY;
    ///
    /// use std::mem::{self, MaybeUninit};
    ///
    /// const INITS: [[u8; 2]; 2] = {
    ///     let mut uninits: [[MaybeUninit<u8>; 2]; 2] = [UNINIT_ARRAY::<u8, 2>::V; 2];
    ///
    ///     uninits[0] = [MaybeUninit::new(3), MaybeUninit::new(5)];
    ///     uninits[1] = [MaybeUninit::new(8), MaybeUninit::new(13)];
    ///
    ///     unsafe{ mem::transmute(uninits) }
    /// };
    ///
    /// assert_eq!(INITS, [[3, 5], [8, 13]]);
    /// ```
    for[T, const N: usize]
    pub const UNINIT_ARRAY[T; N]: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
}

/// Constructs an uninitialized `[MaybeUninit<T>; N]`.
///
/// # Example
///
/// ```rust
/// use konst::maybe_uninit as mu;
///
/// use std::mem::{self, MaybeUninit};
///
/// const INITS: [u8; 2] = {
///     let mut uninits = mu::uninit_array::<u8, 2>();
///
///     uninits[0] = MaybeUninit::new(21);
///     uninits[1] = MaybeUninit::new(34);
///
///     unsafe{ mu::array_assume_init(uninits) }
/// };
///
/// assert_eq!(INITS, [21, 34]);
/// ```
#[inline(always)]
pub const fn uninit_array<T, const LEN: usize>() -> [MaybeUninit<T>; LEN] {
    union MakeMUArray<T, const LEN: usize> {
        unit: (),
        array: ManuallyDrop<[MaybeUninit<T>; LEN]>,
    }

    unsafe { ManuallyDrop::into_inner(MakeMUArray { unit: () }.array) }
}

/// Stable equivalent of
/// [`MaybeUninit::array_assume_init`](core::mem::MaybeUninit::array_assume_init)
///
/// # Safety
///
/// This has the same safety requirements as [`MaybeUninit::array_assume_init`]
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
#[inline(always)]
pub const unsafe fn array_assume_init<T, const N: usize>(md: [MaybeUninit<T>; N]) -> [T; N] {
    unsafe {
        crate::__priv_transmute! {[MaybeUninit<T>; N], [T; N], md}
    }
}
