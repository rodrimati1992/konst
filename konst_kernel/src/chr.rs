mod char_formatting;

#[cfg(test)]
mod tests;

pub use char_formatting::*;

// this isn't documented at all, so this lint seems redundant here <_<
#[allow(clippy::missing_safety_doc)]
pub const unsafe fn from_u32_unchecked(n: u32) -> char {
    core::mem::transmute(n)
}
pub const fn from_u32(n: u32) -> Option<char> {
    if n < 0xD800 || 0xE000 <= n && n <= 0x10FFFF {
        unsafe { Some(from_u32_unchecked(n)) }
    } else {
        None
    }
}

// this isn't really dead though, it's used for the static assertion below
#[allow(dead_code)]
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
