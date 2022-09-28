// Shamelessly copy pasted test cases from the const_format crate.

use konst::{option, string};

#[track_caller]
fn split_case(string: &str, delim: &str, expected: &[&str]) {
    {
        let mut items = Vec::new();

        konst::iter::for_each! {item in string::split(string, delim) =>
            items.push(item);
        }

        assert_eq!(items, expected);
    }

    {
        let mut items = Vec::new();

        konst::iter::for_each! {item in string::rsplit(string, delim) =>
            items.push(item);
        }

        let rev_expected = expected.iter().rev().copied().collect::<Vec<&str>>();

        assert_eq!(items, rev_expected);
    }
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

    for iter in [
        string::split(string, "-"),
        string::split(string, "-").copy(),
        string::rsplit(string, "-").rev(),
    ] {
        let _: string::Split<'_, '_> = iter;

        let (elem, iter) = iter.next().unwrap();
        assert_eq!(elem, "foo");
        assert_eq!(iter.remainder(), "bar-baz");

        let (elem, iter) = iter.next().unwrap();
        assert_eq!(elem, "bar");
        assert_eq!(iter.remainder(), "baz");

        let (elem, iter) = iter.next().unwrap();
        assert_eq!(elem, "baz");
        assert_eq!(iter.remainder(), "");

        assert!(iter.next().is_none());
    }
    
    for iter in [
        string::split(string, "-").rev(),
        string::rsplit(string, "-"),
        string::rsplit(string, "-").copy(),
    ] {
        let _: string::RSplit<'_, '_> = iter;
        
        let (elem, iter) = iter.next().unwrap();
        assert_eq!(elem, "baz");
        assert_eq!(iter.remainder(), "foo-bar");

        let (elem, iter) = iter.next().unwrap();
        assert_eq!(elem, "bar");
        assert_eq!(iter.remainder(), "foo");

        let (elem, iter) = iter.next().unwrap();
        assert_eq!(elem, "foo");
        assert_eq!(iter.remainder(), "");

        assert!(iter.next().is_none());
    }
}

#[test]
fn next_back_basic() {
    let string = "foo-bar-baz";
    
    for iter in [
        string::split(string, "-"),
        string::split(string, "-").copy(),
        string::rsplit(string, "-").rev(),
    ] {
        let _: string::Split<'_, '_> = iter;
        
        let (elem, iter) = iter.next_back().unwrap();
        assert_eq!(elem, "baz");
        assert_eq!(iter.remainder(), "foo-bar");

        let (elem, iter) = iter.next_back().unwrap();
        assert_eq!(elem, "bar");
        assert_eq!(iter.remainder(), "foo");

        let (elem, iter) = iter.next_back().unwrap();
        assert_eq!(elem, "foo");
        assert_eq!(iter.remainder(), "");

        assert!(iter.next_back().is_none());
    }

    for iter in [
        string::split(string, "-").rev(),
        string::rsplit(string, "-"),
    ] {
        let _: string::RSplit<'_, '_> = iter;

        let (elem, iter) = iter.next_back().unwrap();
        assert_eq!(elem, "foo");
        assert_eq!(iter.remainder(), "bar-baz");

        let (elem, iter) = iter.next_back().unwrap();
        assert_eq!(elem, "bar");
        assert_eq!(iter.remainder(), "baz");

        let (elem, iter) = iter.next_back().unwrap();
        assert_eq!(elem, "baz");
        assert_eq!(iter.remainder(), "");

        assert!(iter.next_back().is_none());
    }
}




#[test]
fn methods_are_const() {
    const fn __(string: &str, delim: &str) {
        {
            let iter: string::Split<'_, '_> = string::split(string, delim);
            let _ = iter.copy().next();
            let _ = iter.copy().next_back();
            let _: string::RSplit<'_, '_> = iter.copy().rev();
        }
        {
            let iter: string::RSplit<'_, '_> = string::rsplit(string, delim);
            let _ = iter.copy().next();
            let _ = iter.copy().next_back();
            let _: string::Split<'_, '_> = iter.copy().rev();
        }
    }
}

