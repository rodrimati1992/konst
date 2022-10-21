use konst::string::str_concat;

#[test]
fn str_concat_basic_test() {
    assert_eq!(str_concat!(&[]), "");

    assert_eq!(str_concat!(&["foo"]), "foo");

    assert_eq!(str_concat!(&["", "bar"]), "bar");

    assert_eq!(str_concat!(&["foo", ""]), "foo");

    assert_eq!(str_concat!(&["foo", "bar"]), "foobar");

    assert_eq!(str_concat!(&["foo", "bar", "hello"]), "foobarhello");
}

#[test]
fn str_concat_from_ref_const_test() {
    {
        const S: &[&str] = &["hello", "world"];
        assert_eq!(str_concat!(S), "helloworld");
    }
    {
        const S: &[&str; 3] = &["foo", "bar", "baz"];
        assert_eq!(str_concat!(S), "foobarbaz");
    }
}

// this test ensures that non-promotable expressions still work
#[test]
fn str_concat_from_func_test() {
    const fn func() -> [&'static str; 3] {
        ["AA", "BB", "CC"]
    }

    assert_eq!(str_concat!(&func()), "AABBCC");
}
