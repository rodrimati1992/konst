// Shamelessly copy pasted test cases from the const_format crate.

use konst::string;

#[track_caller]
fn split_case(string: &str, delim: &str, expected: &[&str]) {
    let rev_expected: &[&str] = &expected.iter().rev().copied().collect::<Vec<&str>>();

    // split_once
    {
        let first = expected[0];
        if let Some((before, after)) = string::split_once(string, delim) {
            assert_eq!(before, first, "split before");
            assert_eq!(&string[before.len() + delim.len()..], after, "split rem");
        } else {
            assert!(
                !string.contains(delim),
                "split none. string: {:?}\ndelim: {:?}",
                string,
                delim
            );
        }
    }

    // rsplit_once
    {
        let last = *expected.last().unwrap();
        if let Some((before, after)) = string::rsplit_once(string, delim) {
            assert_eq!(after, last, "rsplit before");
            assert_eq!(
                &string[..string.len() - after.len() - delim.len()],
                before,
                "rsplit rem.\nstring: {:?}\ndelim: {:?}",
                string,
                delim
            );
        } else {
            assert!(
                !string.contains(delim),
                "rsplit none. string: {:?}\ndelim: {:?}",
                string,
                delim
            );
        }
    }

    // [r]split_terminator
    {
        // split_terminator
        //
        // SplitTerminator, unlike Split,
        // doesn't yield the empty string after the delimiter at the end of the string
        let fwd_expected = if string.ends_with(delim) {
            let (&last, expected) = expected.split_last().unwrap();
            assert_eq!(last, "", "last-split-terminator");
            expected
        } else {
            expected
        };

        assert_eq!(
            collect_const_iter!(string::split_terminator(string, delim)),
            fwd_expected
        );

        // rsplit_terminator
        //
        // RSplitTerminator, unlike RSplit,
        // doesn't yield the empty string before the delimiter at the start of the string
        let bck_expected = if string.starts_with(delim) {
            let (&last, rev_expected) = rev_expected.split_last().unwrap();
            assert_eq!(last, "", "last-rsplit-terminator");
            rev_expected
        } else {
            rev_expected
        };

        assert_eq!(
            collect_const_iter!(string::rsplit_terminator(string, delim)),
            bck_expected,
        );
    }

    // split
    assert_eq!(collect_const_iter!(string::split(string, delim)), expected);

    // rsplit
    assert_eq!(
        collect_const_iter!(string::rsplit(string, delim)),
        rev_expected
    );
}

#[test]
fn test_str_split_with_empty_str_arg() {
    split_case("", "", &["", ""]);
    split_case("f", "", &["", "f", ""]);
    split_case("fo", "", &["", "f", "o", ""]);
    split_case("fob", "", &["", "f", "o", "b", ""]);

    split_case(
        "!Aq¡🧡🧠₀₁oñ个",
        "",
        &[
            "", "!", "A", "q", "¡", "", "", "🧡", "🧠", "₀", "₁", "o", "ñ", "个", "",
        ],
    );
}

#[test]
fn test_str_split_with_space_str_arg() {
    split_case("fob", " ", &["fob"]);
    split_case(" fob", " ", &["", "fob"]);
    split_case(" fob ", " ", &["", "fob", ""]);
    split_case("foo bar baz", " ", &["foo", "bar", "baz"]);
    split_case("foo  bar baz", " ", &["foo", "", "bar", "baz"]);
}

#[test]
fn test_str_split_with_dash_str_arg() {
    split_case("fob", "-", &["fob"]);
    split_case("-fob", "-", &["", "fob"]);
    split_case("-fob-", "-", &["", "fob", ""]);
    split_case("foo-bar-baz", "-", &["foo", "bar", "baz"]);
    split_case("foo--bar-baz", "-", &["foo", "", "bar", "baz"]);
}

#[test]
fn test_str_split_with_word_arg() {
    split_case("fob", "XY", &["fob"]);
    split_case("XYfob", "XY", &["", "fob"]);
    split_case("XYfobXY", "XY", &["", "fob", ""]);
    split_case("fooXYbarXYbaz", "XY", &["foo", "bar", "baz"]);
    split_case("fooXY bar XYbaz", "XY", &["foo", " bar ", "baz"]);
}

#[test]
fn test_str_split_with_ascii_char_arg() {
    split_case("fob", "-", &["fob"]);
    split_case("-fob", "-", &["", "fob"]);
    split_case("-fob-", "-", &["", "fob", ""]);
    split_case("foo-bar-baz", "-", &["foo", "bar", "baz"]);
    split_case("foo- bar -baz", "-", &["foo", " bar ", "baz"]);
}

#[test]
fn test_str_split_with_non_ascii_char_arg() {
    {
        split_case("fob", "", &["fob"]);
        split_case("fob", "", &["", "fob"]);
        split_case("fob", "", &["", "fob", ""]);
        split_case("foobarbaz", "", &["foo", "bar", "baz"]);
        split_case("foo bar baz", "", &["foo", " bar ", "baz"]);
    }
    {
        split_case("fob", "ñ", &["fob"]);
        split_case("ñfob", "ñ", &["", "fob"]);
        split_case("ñfobñ", "ñ", &["", "fob", ""]);
        split_case("fooñbarñbaz", "ñ", &["foo", "bar", "baz"]);
        split_case("fooñ bar ñbaz", "ñ", &["foo", " bar ", "baz"]);
    }
    {
        split_case("fob", "₀", &["fob"]);
        split_case("₀fob", "₀", &["", "fob"]);
        split_case("₀fob₀", "₀", &["", "fob", ""]);
        split_case("foo₀bar₀baz", "₀", &["foo", "bar", "baz"]);
        split_case("foo₀ bar ₀baz", "₀", &["foo", " bar ", "baz"]);
    }
    {
        split_case("fob", "🧡", &["fob"]);
        split_case("🧡fob", "🧡", &["", "fob"]);
        split_case("🧡fob🧡", "🧡", &["", "fob", ""]);
        split_case("foo🧡bar🧡baz", "🧡", &["foo", "bar", "baz"]);
        split_case("foo🧡 bar 🧡baz", "🧡", &["foo", " bar ", "baz"]);
    }
}

#[test]
fn next_basic() {
    let string = "foo-bar-baz";

    for mut iter in [
        string::split(string, "-"),
        string::split(string, "-").copy(),
        string::rsplit(string, "-").rev(),
    ] {
        let _: string::Split<'_, '_, &str> = iter;
        assert_eq!(iter.remainder(), "foo-bar-baz");

        assert_eq!(iter.next().unwrap(), "foo");
        assert_eq!(iter.remainder(), "bar-baz");

        assert_eq!(iter.next().unwrap(), "bar");
        assert_eq!(iter.remainder(), "baz");

        assert_eq!(iter.next().unwrap(), "baz");
        assert_eq!(iter.remainder(), "");

        assert!(iter.next().is_none());
    }

    for mut iter in [
        string::split(string, "-").rev(),
        string::rsplit(string, "-"),
        string::rsplit(string, "-").copy(),
    ] {
        let _: string::RSplit<'_, '_, &str> = iter;
        assert_eq!(iter.remainder(), "foo-bar-baz");

        assert_eq!(iter.next().unwrap(), "baz");
        assert_eq!(iter.remainder(), "foo-bar");

        assert_eq!(iter.next().unwrap(), "bar");
        assert_eq!(iter.remainder(), "foo");

        assert_eq!(iter.next().unwrap(), "foo");
        assert_eq!(iter.remainder(), "");

        assert!(iter.next().is_none());
    }
}

#[test]
fn next_basic_terminated() {
    let fwd_string = "foo-bar-baz-";
    let rev_string = "-foo-bar-baz";

    for mut iter in [
        string::split_terminator(fwd_string, "-"),
        string::split_terminator(fwd_string, "-").copy(),
    ] {
        let _: string::SplitTerminator<'_, '_, &str> = iter;
        assert_eq!(iter.remainder(), "foo-bar-baz-");

        assert_eq!(iter.next().unwrap(), "foo");
        assert_eq!(iter.remainder(), "bar-baz-");

        assert_eq!(iter.next().unwrap(), "bar");
        assert_eq!(iter.remainder(), "baz-");

        assert_eq!(iter.next().unwrap(), "baz");
        assert_eq!(iter.remainder(), "");

        assert!(iter.next().is_none());
    }

    for mut iter in [
        string::rsplit_terminator(rev_string, "-"),
        string::rsplit_terminator(rev_string, "-").copy(),
    ] {
        let _: string::RSplitTerminator<'_, '_, &str> = iter;
        assert_eq!(iter.remainder(), "-foo-bar-baz");

        assert_eq!(iter.next().unwrap(), "baz");
        assert_eq!(iter.remainder(), "-foo-bar");

        assert_eq!(iter.next().unwrap(), "bar");
        assert_eq!(iter.remainder(), "-foo");

        assert_eq!(iter.next().unwrap(), "foo");
        assert_eq!(iter.remainder(), "");

        assert!(iter.next().is_none());
    }
}

#[test]
fn next_back_basic() {
    let string = "foo-bar-baz";

    for mut iter in [
        string::split(string, "-"),
        string::split(string, "-").copy(),
        string::rsplit(string, "-").rev(),
    ] {
        let _: string::Split<'_, '_, &str> = iter;

        assert_eq!(iter.next_back().unwrap(), "baz");
        assert_eq!(iter.remainder(), "foo-bar");

        assert_eq!(iter.next_back().unwrap(), "bar");
        assert_eq!(iter.remainder(), "foo");

        assert_eq!(iter.next_back().unwrap(), "foo");
        assert_eq!(iter.remainder(), "");

        assert!(iter.next_back().is_none());
    }

    for mut iter in [
        string::split(string, "-").rev(),
        string::rsplit(string, "-"),
    ] {
        let _: string::RSplit<'_, '_, &str> = iter;

        assert_eq!(iter.next_back().unwrap(), "foo");
        assert_eq!(iter.remainder(), "bar-baz");

        assert_eq!(iter.next_back().unwrap(), "bar");
        assert_eq!(iter.remainder(), "baz");

        assert_eq!(iter.next_back().unwrap(), "baz");
        assert_eq!(iter.remainder(), "");

        assert!(iter.next_back().is_none());
    }
}

#[test]
fn methods_are_const() {
    const fn __(string: &str, delim: &str) {
        {
            let iter: string::Split<'_, '_, &str> = string::split(string, delim);
            let _ = iter.copy().next();
            let _ = iter.copy().next_back();
            let _: string::RSplit<'_, '_, &str> = iter.copy().rev();
        }
        {
            let iter: string::RSplit<'_, '_, &str> = string::rsplit(string, delim);
            let _ = iter.copy().next();
            let _ = iter.copy().next_back();
            let _: string::Split<'_, '_, &str> = iter.copy().rev();
        }
        {
            let iter: string::SplitTerminator<'_, '_, &str> =
                string::split_terminator(string, delim);
            let _ = iter.copy().next();
        }
        {
            let iter: string::RSplitTerminator<'_, '_, &str> =
                string::rsplit_terminator(string, delim);
            let _ = iter.copy().next();
        }
    }
}
