use konst::chr::{encode_utf8, Utf8Encoded};

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
