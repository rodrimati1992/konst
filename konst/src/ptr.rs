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
/// - `from_ref`: [core::ptr::NonNull::from_ref]
/// - `from_mut`: [core::ptr::NonNull::from_mut]
///
/// `new` was removed because it was deprecated in 0.3.0 for unsoundness.
///
pub mod nonnull {}
