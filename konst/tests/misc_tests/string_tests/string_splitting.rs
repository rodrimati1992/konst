// Shamelessly copy pasted test cases from the const_format crate.

#[track_caller]
fn split_case(string: &str, delim: &str, expected: &[&str]) {
    {
        let mut items = Vec::new();

        konst::iter::for_each! {item in konst::string::split(string, delim) =>
            items.push(item);
        }

        assert_eq!(items, expected);
    }

    {
        let mut items = Vec::new();

        konst::iter::for_each! {item in konst::string::rsplit(string, delim) =>
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
