#[cfg(feature = "iter")]
use crate::string::chars_methods::{string_to_char, string_to_usv};

// index: 00 char: '🧡' len_utf8: 4
// index: 04 char: '🧠' len_utf8: 4
// index: 08 char: '₀' len_utf8: 3
// index: 11 char: '₁' len_utf8: 3
// index: 14 char: 'o' len_utf8: 1
// index: 15 char: 'ñ' len_utf8: 2
// index: 17 char: '个' len_utf8: 3
const S: &str = "🧡🧠₀₁oñ个";
const B: &[u8] = S.as_bytes();

#[test]
#[should_panic]
fn invalid_start() {
    // SAFETY: this is a slice of a string
    let _ = unsafe { super::__from_u8_subslice_of_str(&B[1..]) };
}

#[test]
#[should_panic]
fn invalid_end() {
    // SAFETY: this is a slice of a string
    let _ = unsafe { super::__from_u8_subslice_of_str(&B[..B.len() - 1]) };
}

#[test]
#[should_panic]
fn invalid_both() {
    // SAFETY: this is a slice of a string
    let _ = unsafe { super::__from_u8_subslice_of_str(&B[1..B.len() - 1]) };
}

#[test]
#[cfg(feature = "iter")]
fn str_to_char_test() {
    assert_eq!(string_to_char("🧡"), '🧡');
    assert_eq!(string_to_char("🧠"), '🧠');
    assert_eq!(string_to_char("₀"), '₀');
    assert_eq!(string_to_char("₁"), '₁');
    assert_eq!(string_to_char("o"), 'o');
    assert_eq!(string_to_char("ñ"), 'ñ');
    assert_eq!(string_to_char("个"), '个');
}

#[test]
#[cfg(feature = "iter")]
#[cfg(not(miri))]
fn str_to_codepoint_test() {
    for c in '\0'..=char::MAX {
        let mut arr = [0u8; 8];
        let string = c.encode_utf8(&mut arr);

        let found = core::char::from_u32(string_to_usv(string)).unwrap();
        assert_eq!(found, c, "{c:?}");
    }
}
