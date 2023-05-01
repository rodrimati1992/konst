//! Const fn equivalents of
//! [`MaybeUninit<T>`](https://doc.rust-lang.org/core/mem/union.MaybeUninit.html) methods.
//!
//! # Removed in 0.3.0
//!
//! These functions were removed in 0.3.0 because there is an equivalent
//! const fn in the standard library:
//!
//! - `as_ptr`: [`MaybeUninit::as_ptr`]
//! - `assume_init`: [`MaybeUninit::assume_init`]
//! - `assume_init_ref`: [`MaybeUninit::assume_init_ref`]
//!
//!

use core::mem::MaybeUninit;

declare_generic_const! {
    /// Generic constant for an uninitialized `MaybeUninit<T>`.
    /// Usable to safely construct a `[MaybeUninit<T>; LEN]` when `T` is non-`Copy`.
    ///
    /// As of Rust 1.65.0, `[MaybeUninit::uninit(); LEN]` is not valid for non-`Copy` types,
    /// but `[CONST; LEN]` does work, like in the example below.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::maybe_uninit::{self as mu, UNINIT};
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
    ///     unsafe{ mu::array_assume_init(uninits) }
    /// };
    ///
    /// assert_eq!(INITS, [NonCopy(0), NonCopy(3), NonCopy(6), NonCopy(9), NonCopy(12)]);
    ///
    for[T]
    pub const UNINIT[T]: MaybeUninit<T> = MaybeUninit::uninit();
}

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
    pub const UNINIT_ARRAY[T; N]: [MaybeUninit<T>; N] = [UNINIT::V; N];
}

/// Const equivalent of [`MaybeUninit::uninit_array`](core::mem::MaybeUninit::uninit_array)
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
pub use konst_kernel::maybe_uninit::uninit_array;

/// Const equivalent of [`MaybeUninit::assume_init_mut`](core::mem::MaybeUninit::assume_init_mut)
///
/// # Safety
///
/// This has [the same safety requirements as `MaybeUninit::assume_init_mut`
/// ](https://doc.rust-lang.org/1.55.0/core/mem/union.MaybeUninit.html#safety-3)
///
/// # Example
///
/// ```rust
/// # #![feature(const_mut_refs)]
/// use std::cmp::Ordering;
/// use std::mem::MaybeUninit;
///
/// use konst::maybe_uninit;
///
/// const unsafe fn mutate_mu(mu: &mut MaybeUninit<u32>) -> u32 {
///     let mutref = maybe_uninit::assume_init_mut(mu);
///     *mutref += 100;
///     *mutref
/// }
///
/// const MU: (MaybeUninit<u32>, [u32; 3]) = {
///     let mut mu = MaybeUninit::new(5);
///     let array = unsafe{
///         [mutate_mu(&mut mu), mutate_mu(&mut mu), mutate_mu(&mut mu)]
///     };
///     (mu, array)
/// };
///
/// unsafe{ assert_eq!(MU.0.assume_init(), 305); }
/// assert_eq!(MU.1, [105, 205, 305]);
///
/// ```
#[cfg(feature = "mut_refs")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "mut_refs")))]
#[inline(always)]
pub const unsafe fn assume_init_mut<T>(md: &mut MaybeUninit<T>) -> &mut T {
    &mut *(md as *mut MaybeUninit<T> as *mut T)
}

/// Const equivalent of [`MaybeUninit::write`](core::mem::MaybeUninit::write)
///
/// # Example
///
/// ```rust
/// # #![feature(const_mut_refs)]
/// use std::cmp::Ordering;
/// use std::mem::MaybeUninit;
///
/// use konst::maybe_uninit;
///
/// const fn cond_init(mu: &mut MaybeUninit<u32>, value: u32) -> Option<&mut u32> {
///     if value % 3 != 0 {
///         Some(maybe_uninit::write(mu, value))
///     } else {
///         None
///     }
/// }
///
/// let mut mu = MaybeUninit::uninit();
/// assert_eq!(cond_init(&mut mu, 0), None);
/// assert_eq!(cond_init(&mut mu, 1), Some(&mut 1));
/// assert_eq!(cond_init(&mut mu, 2), Some(&mut 2));
/// assert_eq!(cond_init(&mut mu, 3), None);
/// assert_eq!(cond_init(&mut mu, 4), Some(&mut 4));
/// assert_eq!(cond_init(&mut mu, 5), Some(&mut 5));
/// assert_eq!(cond_init(&mut mu, 6), None);
///
/// ```
#[cfg(feature = "mut_refs")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "mut_refs")))]
#[inline(always)]
pub const fn write<T>(md: &mut MaybeUninit<T>, value: T) -> &mut T {
    *md = MaybeUninit::new(value);
    unsafe { &mut *(md as *mut MaybeUninit<T> as *mut T) }
}

/// Const equivalent of [`MaybeUninit::as_mut_ptr`].
///
/// # Example
///
/// Initializing a `#[repr(u8)]` enum
///
/// ```rust
/// # #![feature(const_mut_refs)]
/// use std::mem::MaybeUninit;
///
/// use konst::maybe_uninit as mu;
///
/// const ENUM: Enum = {
///     let mut mu = MaybeUninit::<Enum>::uninit();
///     
///     let ptr = mu::as_mut_ptr(&mut mu).cast::<MaybeUninit<Discr>>();
///     unsafe{
///         *ptr = MaybeUninit::new(Discr::Bar);
///         mu.assume_init()
///     }
/// };
///
/// unsafe {
///     assert_eq!(ENUM, Enum::Bar);
/// }
///
/// #[repr(u8)]
/// enum Discr {
///     Foo,
///     Bar,
///     Baz,
/// }
///
/// #[repr(u8)]
/// #[derive(Debug, PartialEq)]
/// enum Enum {
///     Foo(u8),
///     Bar,
///     Baz{s: String},
/// }
///
/// ```
///
/// [`MaybeUninit::as_mut_ptr`]:
/// https://doc.rust-lang.org/nightly/core/mem/union.MaybeUninit.html#method.as_mut_ptr
#[cfg(feature = "mut_refs")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "mut_refs")))]
#[inline(always)]
pub const fn as_mut_ptr<T>(md: &mut MaybeUninit<T>) -> *mut T {
    md as *mut MaybeUninit<T> as *mut T
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
pub use konst_kernel::maybe_uninit::array_assume_init;
