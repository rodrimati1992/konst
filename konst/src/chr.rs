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
pub use konst_kernel::char_formatting::Utf8Encoded;

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
pub use konst_kernel::char_formatting::encode_utf8;

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
pub const unsafe fn from_u32_unchecked(n: u32) -> char {
    core::mem::transmute(n)
}

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
