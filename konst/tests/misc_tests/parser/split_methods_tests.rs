use konst::parsing::{ErrorKind, ParseDirection, Parser};

#[test]
fn test_split() {
    for (string, expecteds) in [
        ("foo,bar,baz", vec![("foo", 4), ("bar", 8), ("baz", 11)]),
        (
            "foo,bar,baz,",
            vec![("foo", 4), ("bar", 8), ("baz", 12), ("", 12)],
        ),
    ] {
        let mut item;
        let mut parser = Parser::new(string).skip_back(0);
        assert_eq!(parser.parse_direction(), ParseDirection::FromEnd);
        for (expected, start_offset) in expecteds {
            (item, parser) = parser.split(',').unwrap();
            assert_eq!(parser.start_offset(), start_offset);
            assert_eq!(parser.remainder(), &string[start_offset..]);
            assert_eq!(parser.parse_direction(), ParseDirection::FromStart);
            assert_eq!(item, expected);
        }

        let err = parser.split(",").unwrap_err();
        assert_eq!(err.offset(), string.len());
        assert_eq!(err.error_direction(), ParseDirection::FromStart);
        assert_eq!(err.kind(), ErrorKind::SplitExhausted);
    }
}

#[test]
fn test_rsplit() {
    for (string, expecteds) in [
        ("foo,bar,baz", vec!["baz", "bar", "foo"]),
        (",foo,bar,baz", vec!["baz", "bar", "foo", ""]),
    ] {
        let mut item;
        let mut parser = Parser::new(string).skip(0);
        assert_eq!(parser.parse_direction(), ParseDirection::FromStart);

        for expected in expecteds {
            (item, parser) = parser.rsplit(',').unwrap();
            assert_eq!(parser.start_offset(), 0);
            assert_eq!(parser.parse_direction(), ParseDirection::FromEnd);
            assert_eq!(item, expected);
        }

        let err = parser.rsplit(",").unwrap_err();
        assert_eq!(err.offset(), 0);
        assert_eq!(err.error_direction(), ParseDirection::FromEnd);
        assert_eq!(err.kind(), ErrorKind::SplitExhausted);
    }
}

#[test]
fn test_split_keep() {
    for (string, expecteds) in [
        ("foo,bar,baz", vec![("foo", 3), ("bar", 7), ("baz", 11)]),
        (
            "foo,bar,baz,",
            vec![("foo", 3), ("bar", 7), ("baz", 11), ("", 12)],
        ),
    ] {
        let mut item;
        let mut parser = Parser::new(string).skip_back(0);
        assert_eq!(parser.parse_direction(), ParseDirection::FromEnd);
        for (expected, start_offset) in expecteds {
            (item, parser) = parser.split_keep(',').unwrap();
            assert_eq!(parser.start_offset(), start_offset);
            assert_eq!(parser.remainder(), &string[start_offset..]);
            assert_eq!(parser.parse_direction(), ParseDirection::FromStart);
            assert_eq!(item, expected);

            if !parser.is_empty() {
                parser = parser.strip_prefix(',').unwrap();
            }
        }

        let err = parser.split_keep(",").unwrap_err();
        assert_eq!(err.offset(), string.len());
        assert_eq!(err.error_direction(), ParseDirection::FromStart);
        assert_eq!(err.kind(), ErrorKind::SplitExhausted);
    }
}

#[test]
fn test_split_terminator() {
    for (string, expecteds) in [("foo,bar,baz,", [("foo", 4), ("bar", 8), ("baz", 12)])] {
        let mut item;
        let mut parser = Parser::new(string).skip_back(0);
        assert_eq!(parser.parse_direction(), ParseDirection::FromEnd);
        for (expected, start_offset) in expecteds {
            (item, parser) = parser.split_terminator(',').unwrap();
            assert_eq!(parser.start_offset(), start_offset);
            assert_eq!(parser.parse_direction(), ParseDirection::FromStart);
            assert_eq!(item, expected);
        }

        let err = parser.split_terminator(",").unwrap_err();
        assert_eq!(err.offset(), string.len());
        assert_eq!(err.error_direction(), ParseDirection::FromStart);
        assert_eq!(err.kind(), ErrorKind::SplitExhausted);
    }
    {
        let parser = Parser::new("");
        let err = parser.split_terminator(",").unwrap_err();
        assert_eq!(err.offset(), 0);
        assert_eq!(err.error_direction(), ParseDirection::FromStart);
        assert_eq!(err.kind(), ErrorKind::DelimiterNotFound);
    }
    {
        let mut parser = Parser::new("foo,hello");
        parser = parser.split_terminator(",").unwrap().1;
        let err = parser.split_terminator(",").unwrap_err();
        assert_eq!(err.offset(), 4);
        assert_eq!(err.error_direction(), ParseDirection::FromStart);
        assert_eq!(err.kind(), ErrorKind::DelimiterNotFound);
        assert_eq!(parser.remainder(), "hello");
    }
    {
        let mut parser = Parser::new("foo,");
        parser = parser.split_terminator(",").unwrap().1;
        let err = parser.split_terminator(",").unwrap_err();
        assert_eq!(err.offset(), 4);
        assert_eq!(err.error_direction(), ParseDirection::FromStart);
        assert_eq!(err.kind(), ErrorKind::SplitExhausted);
        assert_eq!(parser.remainder(), "");
    }
}

#[test]
fn test_rsplit_terminator() {
    for (string, expecteds) in [(",foo,bar,baz", vec!["baz", "bar", "foo"])] {
        let mut item;
        let mut parser = Parser::new(string).skip(0);
        assert_eq!(parser.parse_direction(), ParseDirection::FromStart);

        for expected in expecteds {
            (item, parser) = parser.rsplit_terminator(',').unwrap();
            assert_eq!(parser.start_offset(), 0);
            assert_eq!(parser.parse_direction(), ParseDirection::FromEnd);
            assert_eq!(item, expected);
        }

        let err = parser.rsplit_terminator(",").unwrap_err();
        assert_eq!(err.offset(), 0);
        assert_eq!(err.error_direction(), ParseDirection::FromEnd);
        assert_eq!(err.kind(), ErrorKind::SplitExhausted);
    }
    {
        let parser = Parser::new("");
        let err = parser.rsplit_terminator(",").unwrap_err();
        assert_eq!(err.offset(), 0);
        assert_eq!(err.error_direction(), ParseDirection::FromEnd);
        assert_eq!(err.kind(), ErrorKind::DelimiterNotFound);
    }
    {
        let mut parser = Parser::new("hello,foo");
        parser = parser.rsplit_terminator(",").unwrap().1;
        let err = parser.rsplit_terminator(",").unwrap_err();
        assert_eq!(err.offset(), 5);
        assert_eq!(err.error_direction(), ParseDirection::FromEnd);
        assert_eq!(err.kind(), ErrorKind::DelimiterNotFound);
        assert_eq!(parser.remainder(), "hello");
    }
    {
        let mut parser = Parser::new(",foo");
        parser = parser.rsplit_terminator(",").unwrap().1;
        let err = parser.rsplit_terminator(",").unwrap_err();
        assert_eq!(err.offset(), 0);
        assert_eq!(err.error_direction(), ParseDirection::FromEnd);
        assert_eq!(err.kind(), ErrorKind::SplitExhausted);
        assert_eq!(parser.remainder(), "");
    }
}
