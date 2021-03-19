use konst::{parse_any, Parser};

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

macro_rules! match_any_test {
    (
        $string:ident, $expected:ident, $expected_val:expr,
        $parser:ident, $method:ident <-> $method_rev:ident;

        $(
            ($($normal_pat:tt)*) <-> ($($rev_pat:tt)*) => $code:block
        )*
    ) => ({
        #![allow(unused_braces)]

        {
            let mut $parser = Parser::from_str($string);
            let val = parse_any!{$parser, $method;
                $(
                    $($normal_pat)* => $code
                )*
            };
            assert_eq!(val, $expected_val);
            assert_eq!($parser.bytes(), $expected.as_bytes());
        }
        {
            let $string = &*reverse($string);
            let $expected = &*reverse($expected);

            let mut $parser = Parser::from_str($string);
            let val = parse_any!{$parser, $method_rev;
                $(
                    $($rev_pat)* => $code
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
