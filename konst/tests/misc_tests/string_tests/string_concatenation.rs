use konst::string::{self, str_concat, str_join};

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

////////////////////////////////////////////////////////////////////////////////

#[test]
fn str_join_empty_sep_test() {
    assert_eq!(str_join!("", &[]), "");

    assert_eq!(str_join!("", &["foo"]), "foo");

    assert_eq!(str_join!("", &["", "bar"]), "bar");

    assert_eq!(str_join!("", &["foo", ""]), "foo");

    assert_eq!(str_join!("", &["foo", "bar"]), "foobar");

    assert_eq!(str_join!("", &["foo", "bar", "hello"]), "foobarhello");
}

#[test]
fn str_join_comma_sep_test() {
    assert_eq!(str_join!(",", &[]), "");

    assert_eq!(str_join!(",", &["foo"]), "foo");

    assert_eq!(str_join!(",", &["", "bar"]), ",bar");

    assert_eq!(str_join!(",", &["foo", ""]), "foo,");

    assert_eq!(str_join!(",", &["foo", "bar"]), "foo,bar");

    assert_eq!(str_join!(",", &["foo", "bar", "hello"]), "foo,bar,hello");
}

#[test]
fn str_join_longer_sep_test() {
    assert_eq!(str_join!("-_-", &[]), "");

    assert_eq!(str_join!("-_-", &["foo"]), "foo");

    assert_eq!(str_join!("-_-", &["", "bar"]), "-_-bar");

    assert_eq!(str_join!("-_-", &["foo", ""]), "foo-_-");

    assert_eq!(str_join!("-_-", &["foo", "bar"]), "foo-_-bar");

    assert_eq!(
        str_join!("-_-", &["foo", "bar", "hello"]),
        "foo-_-bar-_-hello"
    );
}

#[test]
fn str_join_from_ref_const_test() {
    {
        const SEP: &str = "  ";
        const S: &[&str] = &["hello", "world"];
        assert_eq!(str_join!(SEP, S), "hello  world");
    }
    {
        const SEP: &str = "--";
        const S: &[&str; 3] = &["foo", "bar", "baz"];
        assert_eq!(str_join!(SEP, S), "foo--bar--baz");
    }
}

// this test ensures that non-promotable expressions still work
#[test]
fn str_join_from_func_test() {
    const fn sep() -> &'static str {
        "yep"
    }
    const fn strings() -> [&'static str; 3] {
        ["AA", "BB", "CC"]
    }

    assert_eq!(str_join!(sep(), &strings()), "AAyepBByepCC");
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn str_from_iter_basic_test() {
    {
        const S: &str = string::from_iter!(&[""; 0]);
        assert_eq!(S, "");
    }

    assert_eq!(string::from_iter!(&[""]), "");
    assert_eq!(string::from_iter!(&["foo"]), "foo");
    assert_eq!(string::from_iter!(&["foo", "bar"]), "foobar");
}

#[test]
fn str_from_iter_flat_mapped_test() {
    {
        let str = string::from_iter!(
            0..5,
            flat_map(|i| &[konst::string::str_up_to("abcd", i), "."])
        );
        assert_eq!(str, ".a.ab.abc.abcd.");
    }
}
