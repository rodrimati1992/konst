use konst::parsing::{ParseDirection, Parser};

use konst::string;

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[test]
fn trim_start_end_matches_test() {
    #[track_caller]
    fn assertion(string: &str, needle: &str, returned: &str) {
        {
            let parser = Parser::new(string);
            let trimmed = parser.trim_start_matches(needle);
            assert_eq!(trimmed.remainder(), returned, "normal");

            assert_eq!(string::trim_start_matches(string, needle), returned, "norm");

            {
                let start_offset = string.len() - string.trim_start_matches(needle).len();
                assert_eq!(trimmed.start_offset(), start_offset, "{}", line!());
                assert_eq!(trimmed.end_offset(), string.len(), "{}", line!());
                assert_eq!(
                    trimmed.parse_direction(),
                    ParseDirection::FromStart,
                    "{}",
                    line!()
                );
            }
        }
        {
            let rev_string = &*reverse(string);
            let parser = Parser::new(&rev_string);
            let rev_needle = &*reverse(needle);
            let rev_returned = &*reverse(returned);
            let trimmed = parser.trim_end_matches(rev_needle);

            assert_eq!(trimmed.remainder(), rev_returned, "rev");

            assert_eq!(
                string::trim_end_matches(rev_string, rev_needle),
                rev_returned,
                "rev-"
            );

            {
                let end_offset = rev_string.trim_end_matches(rev_needle).len();
                assert_eq!(trimmed.start_offset(), 0, "{}", line!());
                assert_eq!(trimmed.end_offset(), end_offset, "{}", line!());
                assert_eq!(
                    trimmed.parse_direction(),
                    ParseDirection::FromEnd,
                    "{}",
                    line!()
                );
            }
        }
    }

    assertion("helloheloworld", "hello", "heloworld");

    assertion("fooofooworld", "foo", "ofooworld");
    assertion("foofooworld", "foo", "world");
    assertion("fofooworld", "foo", "fofooworld");
    assertion("ffooworld", "foo", "ffooworld");
    assertion("fooworld", "foo", "world");

    assertion("hihihiho", "hi", "ho");

    assertion("hello", "ello", "hello");

    assertion("ell", "ello", "ell");

    assertion("ell", "ell", "");

    assertion("ell", "el", "l");

    assertion("", "", "");

    assertion("e", "", "e");

    assertion("", "e", "");

    assertion("", "ello", "");
}

#[test]
fn trim_start_end_test() {
    #[track_caller]
    fn assertion(string: &str, returned: &str) {
        {
            let parser = Parser::new(string);
            assert_eq!(parser.trim_start().remainder(), returned, "normal");

            assert_eq!(string::trim_start(string), returned, "normal-c");
        }
        {
            let rev_string = &*reverse(string);
            let parser = Parser::new(&rev_string);
            let rev_returned = &*reverse(returned);

            assert_eq!(parser.trim_end().remainder(), rev_returned, "rev");

            assert_eq!(string::trim_end(rev_string), rev_returned, "rev-c");
        }
    }

    assertion(" fooo bar ", "fooo bar ");
    assertion("  fooo bar ", "fooo bar ");

    assertion("\tfooo bar\t", "fooo bar\t");
    assertion("\t\tfooo bar\t\t", "fooo bar\t\t");

    assertion("\nfooo bar\n", "fooo bar\n");
    assertion("\n\nfooo bar\n\n", "fooo bar\n\n");

    assertion("\rfooo bar\r", "fooo bar\r");
    assertion("\r\rfooo bar\r\r", "fooo bar\r\r");

    assertion("\r\n \t-FOO BAR     ", "-FOO BAR     ");
}

////////////////////////////////////////////

#[test]
fn strip_prefix_suffix_test() {
    #[track_caller]
    fn assertion(string: &str, needle: &str, returned: Option<&str>) {
        {
            let parser = Parser::new(string);
            let stripped = parser.strip_prefix(needle).ok();
            {
                let stripped = stripped.map(|x| x.remainder());
                assert_eq!(stripped, returned, "normala");
            }

            if let Some(stripped) = stripped {
                assert_eq!(stripped.start_offset(), needle.len());
                assert_eq!(stripped.end_offset(), string.len());
                assert_eq!(stripped.parse_direction(), ParseDirection::FromStart);
            }
        }
        {
            let rev_string = &*reverse(string);
            let parser = Parser::new(&rev_string);
            let rev_needle = &*reverse(needle);
            let rev_returned = returned.map(|x| reverse(x));

            let stripped = parser.strip_suffix(rev_needle).ok();
            {
                let stripped = stripped.map(|x| x.remainder());
                assert_eq!(stripped, rev_returned.as_deref(), "reva");
            }

            if let (Some(stripped), Some(returned)) = (stripped, rev_returned) {
                assert_eq!(stripped.start_offset(), 0);
                assert_eq!(stripped.end_offset(), returned.len());
                assert_eq!(stripped.parse_direction(), ParseDirection::FromEnd);
            }
        }
    }

    assertion("hellllllo", "hello", None);
    assertion("helloworld", "x", None);

    assertion("hellooworld", "hello", Some("oworld"));
    assertion("helloworld", "", Some("helloworld"));
    assertion("", "helloworld", None);

    assertion("", "", Some(""));
    assertion("a", "", Some("a"));
    assertion("", "a", None);
    assertion("a", "a", Some(""));
    assertion("aa", "a", Some("a"));
    assertion("aaa", "a", Some("aa"));
}

////////////////////////////////////////////

#[test]
fn find_skip_test() {
    #[track_caller]
    fn assertion(string: &str, needle: &str, returned: Option<&str>, returned_keep: Option<&str>) {
        {
            let parser = Parser::new(string);

            assert_eq!(
                parser.find_skip(needle).map(|x| x.remainder()).ok(),
                returned,
                "normal"
            );

            assert_eq!(string::find_skip(string, needle), returned, "normal-3");

            assert_eq!(
                string::find_keep(string, needle),
                returned_keep.as_deref(),
                "normal-5"
            );
        }
        {
            let rev_string = &*reverse(string);
            let parser = Parser::new(&rev_string);
            let rev_needle = &*reverse(needle);
            let rev_returned = returned.map(|x| reverse(x));

            let rev_returned_keep = returned_keep.map(|x| reverse(x));

            let trimmed = parser.rfind_skip(rev_needle).map(|x| x.remainder());
            assert_eq!(trimmed.ok(), rev_returned.as_deref(), "rev");

            assert_eq!(
                string::rfind_skip(rev_string, rev_needle),
                rev_returned.as_deref(),
                "rev-3"
            );

            assert_eq!(
                string::rfind_keep(rev_string, rev_needle),
                rev_returned_keep.as_deref(),
                "rev-5"
            );
        }
    }

    assertion("hhehelhellhelloworld", "lo", Some("world"), Some("loworld"));
    assertion(
        "hhehelhellhelloworld",
        "hello",
        Some("world"),
        Some("helloworld"),
    );
    assertion(
        "hehelhellhelloworld",
        "hello",
        Some("world"),
        Some("helloworld"),
    );
    assertion(
        "helhellhelloworld",
        "hello",
        Some("world"),
        Some("helloworld"),
    );
    assertion("hellhelloworld", "hello", Some("world"), Some("helloworld"));
    assertion(
        "hellhellohelloworld",
        "hello",
        Some("helloworld"),
        Some("hellohelloworld"),
    );
    assertion("helloworld", "hello", Some("world"), Some("helloworld"));

    assertion("helloworld", "", Some("helloworld"), Some("helloworld"));

    assertion("_lolololfoo", "lolol", Some("olfoo"), Some("lolololfoo"));
    assertion("l_lolololfoo", "lolol", Some("olfoo"), Some("lolololfoo"));
    assertion("lo_lolololfoo", "lolol", Some("olfoo"), Some("lolololfoo"));
    assertion("lol_lolololfoo", "lolol", Some("olfoo"), Some("lolololfoo"));
    assertion(
        "lolo_lolololfoo",
        "lolol",
        Some("olfoo"),
        Some("lolololfoo"),
    );

    assertion("helloworld", "z", None, None);
    assertion("helloworld", "hella", None, None);
    assertion("helloworld", "lowa", None, None);

    assertion("ell", "ell", Some(""), Some("ell"));
    assertion("ell", "ella", None, None);

    assertion("woowooa-that-", "wooa", Some("-that-"), Some("wooa-that-"));
}
