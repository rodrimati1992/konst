use konst::parsing::{ParseDirection, Parser};

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[test]
fn trim_start_end_matches_test() {
    #[track_caller]
    fn assertion(string: &str, needle: &str, returned: &str) {
        {
            let parser = Parser::from_str(string);
            let trimmed = parser.trim_start_matches(needle);
            assert_eq!(trimmed.bytes(), returned.as_bytes(), "normal");
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
            let parser = Parser::from_str(&rev_string);
            let rev_needle = &*reverse(needle);
            let rev_returned = &*reverse(returned);
            let trimmed = parser.trim_end_matches(rev_needle);

            assert_eq!(trimmed.bytes(), rev_returned.as_bytes(), "rev");

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
fn trim_start_end_matches_u8_test() {
    #[track_caller]
    fn assertion(string: &str, needle: u8, returned: &str) {
        {
            let parser = Parser::from_str(string);
            assert_eq!(
                parser.trim_start_matches_u8(needle).bytes(),
                returned.as_bytes(),
                "normal"
            );
        }
        {
            let rev_string = &*reverse(string);
            let parser = Parser::from_str(&rev_string);
            let rev_returned = &*reverse(returned);

            assert_eq!(
                parser.trim_end_matches_u8(needle).bytes(),
                rev_returned.as_bytes(),
                "rev"
            );
        }
    }

    assertion("hello", b'h', "ello");

    assertion("hhhhello", b'h', "ello");

    assertion("ello", b'h', "ello");

    assertion("", b'h', "");
}

////////////////////////////////////////////

#[test]
fn strip_prefix_suffix_test() {
    #[track_caller]
    fn assertion(string: &str, needle: &str, returned: Option<&str>) {
        {
            let parser = Parser::from_str(string);
            let returned = returned.map(|x| x.as_bytes());
            let stripped = parser.strip_prefix(needle).ok();
            assert_eq!(stripped.map(|x| x.bytes()), returned, "normal");

            if let Some(stripped) = stripped {
                assert_eq!(stripped.start_offset(), needle.len());
                assert_eq!(stripped.end_offset(), string.len());
                assert_eq!(stripped.parse_direction(), ParseDirection::FromStart);
            }
        }
        {
            let rev_string = &*reverse(string);
            let parser = Parser::from_str(&rev_string);
            let rev_needle = &*reverse(needle);
            let rev_returned = returned.map(|x| reverse(x));
            let rev_returned = rev_returned.as_ref().map(|x| x.as_bytes());

            let stripped = parser.strip_suffix(rev_needle).ok();
            assert_eq!(stripped.map(|x| x.bytes()), rev_returned, "rev");

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

#[test]
fn strip_prefix_suffix_u8_test() {
    #[track_caller]
    fn assertion(string: &str, needle: u8, returned: Option<&str>) {
        {
            let parser = Parser::from_str(string);
            let returned = returned.map(|x| x.as_bytes());
            assert_eq!(
                parser.strip_prefix_u8(needle).map(|x| x.bytes()).ok(),
                returned,
                "normal"
            );
        }
        {
            let rev_string = &*reverse(string);
            let parser = Parser::from_str(&rev_string);
            let rev_returned = returned.map(|x| reverse(x));
            let rev_returned = rev_returned.as_ref().map(|x| x.as_bytes());

            assert_eq!(
                parser.strip_suffix_u8(needle).map(|x| x.bytes()).ok(),
                rev_returned,
                "rev"
            );
        }
    }

    assertion("", b'l', None);

    assertion("hellllllo", b'l', None);
    assertion("hellooworld", b'x', None);
    assertion("hellooworld", b'h', Some("ellooworld"));
    assertion("  Hi!", b' ', Some(" Hi!"));
}

////////////////////////////////////////////

#[test]
fn find_skip_test() {
    #[track_caller]
    fn assertion(string: &str, needle: &str, returned: Option<&str>) {
        {
            let parser = Parser::from_str(string);
            let returned = returned.map(|x| x.as_bytes());
            assert_eq!(
                parser.find_skip(needle).map(|x| x.bytes()).ok(),
                returned,
                "normal"
            );
        }
        {
            let rev_string = &*reverse(string);
            let parser = Parser::from_str(&rev_string);
            let rev_needle = &*reverse(needle);
            let rev_returned = returned.map(|x| reverse(x));
            let rev_returned = rev_returned.as_ref().map(|x| x.as_bytes());

            let trimmed = parser.rfind_skip(rev_needle).map(|x| x.bytes());
            assert_eq!(trimmed.ok(), rev_returned, "rev");
        }
    }

    assertion("hhehelhellhelloworld", "lo", Some("world"));
    assertion("hhehelhellhelloworld", "hello", Some("world"));
    assertion("hehelhellhelloworld", "hello", Some("world"));
    assertion("helhellhelloworld", "hello", Some("world"));
    assertion("hellhelloworld", "hello", Some("world"));
    assertion("helloworld", "hello", Some("world"));

    assertion("helloworld", "", Some("helloworld"));

    assertion("helloworld", "z", None);
    assertion("helloworld", "hella", None);
    assertion("helloworld", "lowa", None);

    assertion("ell", "ell", Some(""));
    assertion("ell", "ella", None);

    assertion("woowooa-that-", "wooa", Some("-that-"));
}

#[test]
fn find_skip_u8_test() {
    #[track_caller]
    fn assertion(string: &str, needle: u8, returned: Option<&str>) {
        {
            let parser = Parser::from_str(string);
            let returned = returned.map(|x| x.as_bytes());
            assert_eq!(
                parser.find_skip_u8(needle).map(|x| x.bytes()).ok(),
                returned,
                "normal"
            );
        }
        {
            let rev_string = &*reverse(string);
            let parser = Parser::from_str(&rev_string);
            let rev_returned = returned.map(|x| reverse(x));
            let rev_returned = rev_returned.as_ref().map(|x| x.as_bytes());

            assert_eq!(
                parser.rfind_skip_u8(needle).map(|x| x.bytes()).ok(),
                rev_returned,
                "rev"
            );
        }
    }

    assertion("helloworld", b'l', Some("loworld"));

    assertion("helloworld", b'w', Some("orld"));

    assertion("helloworld", b'x', None);

    assertion("", b'x', None);
}
