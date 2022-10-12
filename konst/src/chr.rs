//! Const equivalents of `char` functions.
//!
//! The module is called `chr` to avoid name collisions with the `char` type.

use const_panic::fmt::char_formatting::{self, FmtChar};

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
pub struct Utf8Encoded(FmtChar);

impl Utf8Encoded {
    /// Accesses the inner char as utf8 bytes.
    pub const fn as_bytes(&self) -> &[u8] {
        crate::slice::slice_up_to(self.0.encoded(), self.0.len())
    }

    /// Accesses the inner char as a utf8 string.
    pub const fn as_str(&self) -> &str {
        // safety: `Utf8Encoded::as_bytes` is tested for all possible chars
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }
}

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
pub const fn encode_utf8(c: char) -> Utf8Encoded {
    Utf8Encoded(char_formatting::char_to_display(c))
}
