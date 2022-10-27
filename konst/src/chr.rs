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
#[derive(Copy, Clone)]
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

/// A const equivalent of [`core::char::from_u32_unchecked`]
///
/// # Example
///
/// ```rust
/// use konst::ctr;
///
/// const AT: char = unsafe { chr::from_u32_unchecked(64) };
///
/// assert_eq!(AT, '@');
/// ```
pub const unsafe fn from_u32_unchecked(n: u32) -> char {
    core::mem::transmute(n)
}

/// A const equivalent of [`core::char::from_u32`]
///
/// # Example
///
/// ```rust
/// use konst::{ctr, option};
///
/// const AT: char = option::unwrap!(chr::from_u32(64));
///
/// assert_eq!(AT, '@');
/// ```
pub const fn from_u32(n: u32) -> Option<char> {
    if n < 0xD800 || 0xE000 <= n && n <= 0x10FFFF {
        unsafe { Some(from_u32_unchecked(n)) }
    } else {
        None
    }
}

#[track_caller]
const fn assert_char_repr_as_u32(c: char) {
    let num = unsafe { core::mem::transmute::<char, u32>(c) };
    assert!(c as u32 == num);
}

const _: () = {
    assert_char_repr_as_u32('\0');
    assert_char_repr_as_u32('\u{D7FF}');
    assert_char_repr_as_u32('\u{E000}');
    assert_char_repr_as_u32('\u{10FFFF}');
    assert_char_repr_as_u32(char::MAX);
};
