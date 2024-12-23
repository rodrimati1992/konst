//! Const equivalents of `char` functions.
//!
//! The module is called `chr` to avoid name collisions with the `char` type.
//!
//! # Removed in 0.4.0
//!
//! These items were removed in 0.4.0 because there is an equivalent
//! way to write it in const:
//!
//! - `from_u32_unchecked`: [char::from_u32_unchecked]
//! - `from_u32`: [char::from_u32]

/// A char encoded as a utf8 string.
///
/// # Example
///
/// ```rust
/// use konst::chr;
///
/// const ENC: &chr::Utf8Encoded = &chr::encode_utf8('û');
/// const ENC_STR: &str = ENC.as_str();
///
/// assert_eq!(ENC_STR, "û");
///
/// ```
pub use konst_kernel::chr::Utf8Encoded;

/// Alternative to [`char::encode_utf8`], which returns an inline-allocated string.
///
/// # Example
///
/// ```rust
/// use konst::chr;
///
/// const ENC: &chr::Utf8Encoded = &chr::encode_utf8('🤔');
/// const ENC_STR: &str = ENC.as_str();
///
/// assert_eq!(ENC_STR, "🤔");
///
/// ```
pub use konst_kernel::chr::encode_utf8;
