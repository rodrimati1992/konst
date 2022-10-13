use konst::parsing::{ErrorKind, ParseDirection, Parser};

#[test]
fn test_split() {
    for (string, expecteds) in [
        ("foo,bar,baz", vec!["foo", "bar", "baz"]),
        ("foo,bar,baz,", vec!["foo", "bar", "baz", ""]),
    ] {
        let mut item;
        let mut parser = Parser::new(string);
        for expected in expecteds {
            (item, parser) = parser.split(',').unwrap();
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
        let mut parser = Parser::new(string);
        for expected in expecteds {
            (item, parser) = parser.rsplit(',').unwrap();
            assert_eq!(item, expected);
        }

        let err = parser.rsplit(",").unwrap_err();
        assert_eq!(err.offset(), 0);
        assert_eq!(err.error_direction(), ParseDirection::FromEnd);
        assert_eq!(err.kind(), ErrorKind::SplitExhausted);
    }
}

#[test]
fn test_split_terminator() {
    for (string, expecteds) in [("foo,bar,baz,", ["foo", "bar", "baz"])] {
        let mut item;
        let mut parser = Parser::new(string);
        for expected in expecteds {
            (item, parser) = parser.split_terminator(',').unwrap();
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
        let mut parser = Parser::new(string);
        for expected in expecteds {
            (item, parser) = parser.rsplit_terminator(',').unwrap();
            assert_eq!(item, expected);
        }

        let err = parser.rsplit_terminator(",").unwrap_err();
        assert_eq!(err.offset(), 0);
        assert_eq!(err.error_direction(), ParseDirection::FromEnd);
        assert_eq!(err.kind(), ErrorKind::SplitExhausted);
    }
    {
        let mut parser = Parser::new("");
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
