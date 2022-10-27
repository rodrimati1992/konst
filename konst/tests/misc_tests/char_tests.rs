use konst::chr::{self, encode_utf8, Utf8Encoded};

#[test]
fn test_chars_as_str() {
    let mut buffer = String::new();

    for c in "fooñ个人bar\u{100000}baz".chars() {
        buffer.clear();
        buffer.push(c);
        assert_eq!(encode_utf8(c).as_str(), buffer.as_str());
    }
}

#[cfg(not(miri))]
#[test]
fn test_all_chars_as_byets() {
    let mut buffer = String::new();
    for c in '\0'..=char::MAX {
        buffer.clear();
        buffer.push(c);

        let encoded = encode_utf8(c);
        assert_eq!(encoded.as_bytes(), buffer.as_bytes());
    }
}

#[cfg(not(miri))]
#[test]
fn test_all_chars_from_u32() {
    for c in 0..(u32::from(char::MAX) + 10) {
        assert_eq!(chr::from_u32(c), core::char::from_u32(c));
    }
}

#[test]
fn test_some_chars_from_u32() {
    let vals = [
        0x0, 0x1, 0x7F, 0xFF, 0x7FF, 0x1000, 0x8000, 0xD7FF, 0xD800, 0xDFFF, 0xE000, 0xE001,
        0x10FFFE, 0x10FFFF, 0x110000, 0x110001,
    ];
    for c in vals {
        let chr = chr::from_u32(c);
        assert_eq!(chr, core::char::from_u32(c), "number: {c:x}");

        if let Some(chr) = chr {
            unsafe {
                assert_eq!(chr, chr::from_u32_unchecked(c), "number: {c:x}");
            }
        }
    }
}
