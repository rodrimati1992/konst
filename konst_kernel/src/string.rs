#[cfg(feature = "__for_konst")]
pub mod string_for_konst;

#[cfg(feature = "__for_konst")]
pub use self::string_for_konst::*;

#[inline]
pub const fn str_up_to(string: &str, len: usize) -> &str {
    let bytes = string.as_bytes();
    if __is_char_boundary_forgiving(bytes, len) {
        // Safety: __is_char_boundary_forgiving checks that `len` falls on a char boundary.
        unsafe { __from_u8_subslice_of_str(crate::slice::slice_up_to(bytes, len)) }
    } else {
        non_char_boundary_panic("index", len)
    }
}

#[inline]
pub const fn str_from(string: &str, start: usize) -> &str {
    let bytes = string.as_bytes();
    if __is_char_boundary_forgiving(bytes, start) {
        // Safety: __is_char_boundary_forgiving checks that `start` falls on a char boundary.
        unsafe { __from_u8_subslice_of_str(crate::slice::slice_from(bytes, start)) }
    } else {
        non_char_boundary_panic("start", start)
    }
}

#[inline]
pub const fn str_range(string: &str, start: usize, end: usize) -> &str {
    let bytes = string.as_bytes();
    let start_inbounds = __is_char_boundary_forgiving(bytes, start);
    if start_inbounds && __is_char_boundary_forgiving(bytes, end) {
        // Safety: __is_char_boundary_forgiving checks that
        // `start` and `end` fall on a char boundaries.
        unsafe { __from_u8_subslice_of_str(crate::slice::slice_range(bytes, start, end)) }
    } else if start_inbounds {
        non_char_boundary_panic("end", end)
    } else {
        non_char_boundary_panic("start", start)
    }
}

#[inline]
pub const fn is_char_boundary(string: &str, position: usize) -> bool {
    __is_char_boundary_bytes(string.as_bytes(), position)
}

macro_rules! byte_is_char_boundary {
    ($b:expr) => {
        ($b as i8) >= -0x40
    };
}

#[doc(hidden)]
#[inline]
pub const fn __is_char_boundary_bytes(bytes: &[u8], position: usize) -> bool {
    position == bytes.len() || position < bytes.len() && byte_is_char_boundary!(bytes[position])
}

#[inline]
const fn __is_char_boundary_forgiving(bytes: &[u8], position: usize) -> bool {
    position >= bytes.len() || byte_is_char_boundary!(bytes[position])
}

#[doc(hidden)]
pub const fn __find_next_char_boundary(bytes: &[u8], mut position: usize) -> usize {
    loop {
        position += 1;

        if __is_char_boundary_forgiving(bytes, position) {
            break position;
        }
    }
}

#[doc(hidden)]
pub const fn __find_prev_char_boundary(bytes: &[u8], mut position: usize) -> usize {
    position = position.saturating_sub(1);

    while !__is_char_boundary_forgiving(bytes, position) {
        position -= 1;
    }

    position
}

#[doc(hidden)]
#[track_caller]
pub const unsafe fn __from_u8_subslice_of_str(s: &[u8]) -> &str {
    #[cfg(any(feature = "debug", test))]
    if !s.is_empty() {
        if !byte_is_char_boundary!(s[0]) {
            panic!("string doesn't start at a byte boundary")
        }

        let cb = __find_prev_char_boundary(s, s.len() - 1);
        if let Err(_) = core::str::from_utf8(crate::slice::slice_from(s, cb)) {
            panic!("string doesn't end at a byte boundary")
        }
    }

    core::str::from_utf8_unchecked(s)
}

#[cold]
#[track_caller]
#[doc(hidden)]
const fn non_char_boundary_panic(extreme: &str, index: usize) -> ! {
    use crate::utils::PanikVal;

    crate::utils::basic_panic(&[
        PanikVal::Str(extreme),
        PanikVal::Str(" `"),
        PanikVal::Usize(index),
        PanikVal::Str("` is not on a char boundary"),
    ])
}
