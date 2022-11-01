//! Const equivalents of `char` functions.
//!
//! The module is called `chr` to avoid name collisions with the `char` type.

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

/// Encodes `c` into utf8.
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

/// A const equivalent of [`core::char::from_u32_unchecked`]
///
/// # Example
///
/// ```rust
/// use konst::chr;
///
/// const AT: char = unsafe { chr::from_u32_unchecked(64) };
///
/// assert_eq!(AT, '@');
/// ```
pub use konst_kernel::chr::from_u32_unchecked;

/// A const equivalent of [`core::char::from_u32`]
///
/// # Example
///
/// ```rust
/// use konst::{chr, option};
///
/// const AT: char = option::unwrap!(chr::from_u32(64));
///
/// assert_eq!(AT, '@');
/// ```
pub use konst_kernel::chr::from_u32;
