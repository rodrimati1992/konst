//! Generic constants for types from the [`alloc`] crate, including `String` and `Vec`.
//!
//! [`alloc`]: https://doc.rust-lang.org/alloc/
//!
//! # Removed in 0.4.0
//!
//! - `VEC_NEW`: removed because the stable equivalent is `const { Vec::new() }`
//! - `STRING_NEW`: removed because the stable equivalent is `const { String::new()}`
//! - `COW_STR_NEW`: removed because the stable equivalent is `const { Cow::Borrowed("") }`
//!
//!
use alloc::borrow::Cow;

declare_generic_const! {
    /// An empty `Cow<'_, [T]>`. Usable to construct a `[Cow<'_, [T]>; N]`.
    ///
    /// As of Rust 1.88.0, `[Cow::Borrowed(&[][..]); LEN]` is not valid,
    /// because `Cow<'_, [T]>` isn't copy,
    /// but `[CONST; LEN]` does work, like in the example below.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::alloc_type::COW_SLICE_NEW;
    ///
    /// use std::borrow::Cow;
    ///
    /// const SLICES: [Cow<'_, [u64]>; 6] = [COW_SLICE_NEW::<u64>::V; 6];
    ///
    /// let mut cows = SLICES;
    ///
    /// [3, 5, 8, 13, 21, 34].iter().copied()
    ///     .enumerate()
    ///     .filter(|(i, _)| i % 2 != 0 )
    ///     .for_each(|(i, v)|{
    ///         cows[i].to_mut().push(v)
    ///     });
    ///
    /// assert_eq!(cows, [&[][..], &[5], &[], &[13], &[], &[34]])
    ///
    /// ```
    ///
    for['a, T: Clone + 'a]
    pub const COW_SLICE_NEW['a, T]: Cow<'a, [T]> = Cow::Borrowed(&[]);
}
