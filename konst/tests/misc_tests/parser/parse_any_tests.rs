use konst::{parse_any, Parser};

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

macro_rules! match_any_test {
    (
        $string:ident, $expected:ident, $expected_val:expr,
        $parser:ident, $method:ident <-> $method_rev:ident;

        $(
            ($($normal_pat:tt)*) <-> ($($rev_pat:tt)*) $(=> $code:block)?
        )*
    ) => ({
        #![allow(unused_braces)]

        #[allow(unused_mut)]
        {
            let mut $parser = Parser::from_str($string);
            let val = parse_any!{$parser, $method;
                $(
                    $($normal_pat)* $( => $code )?
                )*
            };
            assert_eq!(val, $expected_val);
            assert_eq!($parser.bytes(), $expected.as_bytes());
        }
        #[allow(unused_mut)]
        {
            let $string = &*reverse($string);
            let $expected = &*reverse($expected);

            let mut $parser = Parser::from_str($string);
            let val = parse_any!{$parser, $method_rev;
                $(
                    $($rev_pat)* $( => $code )?
                )*
            };
            assert_eq!(val, $expected_val);
            assert_eq!($parser.bytes(), $expected.as_bytes());
        }
    });
}

#[test]
fn strip_prefix_suffix_test() {
    #[track_caller]
    fn empty<'p>(s: &str) {
        let expected = s;
        match_any_test! {
            s, expected, (), parser, strip_prefix <-> strip_suffix;

            ("") <-> ("") => {
                assert_eq!(parser.bytes(), s.as_bytes());
            }
            (_) <-> (_) => { unreachable!() }
        }
        match_any_test! {
            s, expected, (), parser, strip_prefix <-> strip_suffix;

            (_) <-> (_) => {
                assert_eq!(parser.bytes(), s.as_bytes());
            }
        }
    }

    empty("hello");
    empty("world");
    empty("x");
    empty("");

    #[track_caller]
    fn hello_pat(s: &str, expected: &str, expected_value: u32) {
        match_any_test! {
            s, expected, expected_value, parser, strip_prefix <-> strip_suffix;
            (concat!("hel", r#"lo"#)) <-> (concat!("ol", r#"leh"#)) => {
                3
            }
            ("world" | stringify!(foo)) <-> ("dlrow" | stringify!(oof)) => {
                5
            }
            (_) <-> (_) => {
                8
            }
        }
    }

    hello_pat("helloheloworld", "heloworld", 3);

    hello_pat("worldly", "ly", 5);
    hello_pat("fooey", "ey", 5);

    hello_pat("worl", "worl", 8);
    hello_pat("elloheloworld", "elloheloworld", 8);
    hello_pat("", "", 8);
}

#[test]
fn trim_start_end_matches_test() {
    #[track_caller]
    fn empty<'p>(s: &str) {
        let expected = s;
        match_any_test! {
            s, expected, (), parser, trim_start_matches <-> trim_end_matches;
            ("") <-> ("")
        }
    }

    empty("hello");
    empty("world");
    empty("x");
    empty("");

    #[track_caller]
    fn unreachable_pat(s: &str, expected: &str) {
        match_any_test! {
            s, expected, (), parser, trim_start_matches <-> trim_end_matches;
            ("hello" | "" | "world") <-> ("olleh" | "" | "dlrow")
        }
    }

    unreachable_pat("", "");
    unreachable_pat("worldhello", "worldhello");
    unreachable_pat("world", "world");
    unreachable_pat("helloworld", "world");
    unreachable_pat("hellohelloworld", "world");

    #[track_caller]
    fn multiple_pats(s: &str, expected: &str) {
        match_any_test! {
            s, expected, (), parser, trim_start_matches <-> trim_end_matches;
            ("foo" | "bar" | "baz") <-> ("oof" | "rab" | "zab")
        }
    }

    multiple_pats("foo", "");
    multiple_pats("bar", "");
    multiple_pats("baz", "");
    multiple_pats("_foo", "_foo");
    multiple_pats("_bar", "_bar");
    multiple_pats("_baz", "_baz");
    multiple_pats("barfoo", "");
    multiple_pats("bazfoobar", "");
    multiple_pats("_bazfoobar", "_bazfoobar");
    multiple_pats("baz_foobar", "_foobar");
    multiple_pats("bazfo_obar", "fo_obar");
    multiple_pats("bazfoo_bar", "_bar");
    multiple_pats("bazfoob_ar", "b_ar");
    multiple_pats("bazfoobar_", "_");
}

#[test]
fn find_skip_test() {
    #[track_caller]
    fn empty<'p>(s: &str) {
        let expected = s;
        match_any_test! {
            s, expected, (), parser, find_skip <-> rfind_skip;

            ("") <-> ("") => {
                assert_eq!(parser.bytes(), s.as_bytes());
            }
            (_) <-> (_) => { unreachable!() }
        }
        match_any_test! {
            s, expected, (), parser, find_skip <-> rfind_skip;

            (_) <-> (_) => {
                assert_eq!(parser.bytes(), s.as_bytes());
            }
        }
    }

    empty("");
    empty("a");
    empty("ab");
    empty("abc");

    #[track_caller]
    fn lo_pat(s: &str, expected: &str, expected_value: u32) {
        match_any_test! {
            s, expected, expected_value, parser, find_skip <-> rfind_skip;
            ("lo") <-> ("ol") => {
                3
            }
            (_) <-> (_) => {
                8
            }
        }
    }

    lo_pat("hhehelhellhelloworld", "world", 3);
    lo_pat("hello", "", 3);
    lo_pat("helloot", "ot", 3);
    lo_pat("looo", "oo", 3);
    lo_pat("lloee", "ee", 3);
    lo_pat("hel", "hel", 8);
    lo_pat("worlds", "worlds", 8);

    #[track_caller]
    fn hello_pat(s: &str, expected: &str, expected_value: u32) {
        match_any_test! {
            s, expected, expected_value, parser, find_skip <-> rfind_skip;
            ("hello") <-> ("olleh") => {
                3
            }
            (_) <-> (_) => {
                8
            }
        }
    }

    hello_pat("hhehelhellhelloworld", "world", 3);
    hello_pat("hehelhellhelloworld", "world", 3);
    hello_pat("helhellhelloworld", "world", 3);
    hello_pat("hellhelloworld", "world", 3);
    hello_pat("helloworld", "world", 3);
    hello_pat("wow", "wow", 8);
    hello_pat("hell", "hell", 8);

    #[track_caller]
    fn wooa_pat(s: &str, expected: &str, expected_value: u32) {
        match_any_test! {
            s, expected, expected_value, parser, find_skip <-> rfind_skip;
            ("wooa") <-> ("aoow") => {
                3
            }
            (_) <-> (_) => {
                8
            }
        }
    }

    wooa_pat("woowooa-that-", "-that-", 3);
    wooa_pat("wooa-that-", "-that-", 3);
    wooa_pat("woo-that-", "woo-that-", 8);
}
