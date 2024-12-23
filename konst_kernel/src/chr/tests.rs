use crate::chr::{self, encode_utf8};

#[test]
fn test_chars_as_str() {
    let mut buffer = [0u8; 10];

    for c in "fooñ个人bar\u{100000}baz".chars() {
        let std_encoded = c.encode_utf8(&mut buffer);
        assert_eq!(encode_utf8(c).as_str(), std_encoded);
    }
}

#[cfg(not(miri))]
#[test]
fn test_all_chars_as_byets() {
    let mut buffer = [0u8; 10];
    for c in '\0'..=char::MAX {
        let std_encoded = c.encode_utf8(&mut buffer);
        assert_eq!(encode_utf8(c).as_bytes(), std_encoded.as_bytes());
    }
}
