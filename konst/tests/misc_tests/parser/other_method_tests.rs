use konst::parsing::{ParseDirection, Parser};

// index: 00 char: '!' len_utf8: 1
// index: 01 char: 'A' len_utf8: 1
// index: 02 char: 'q' len_utf8: 1
// index: 03 char: '¡' len_utf8: 2
// index: 05 char: '\u{7f}' len_utf8: 1
// index: 06 char: '\u{80}' len_utf8: 2
// index: 08 char: '🧡' len_utf8: 4
// index: 12 char: '🧠' len_utf8: 4
// index: 16 char: '₀' len_utf8: 3
// index: 19 char: '₁' len_utf8: 3
// index: 22 char: 'o' len_utf8: 1
// index: 23 char: 'ñ' len_utf8: 2
// index: 25 char: '个' len_utf8: 3
const S: &str = "!Aq¡🧡🧠₀₁oñ个";

#[test]
fn test_skip() {
    for (skip, start) in [
        (0, 0),
        (1, 1),
        (6, 6),
        (7, 8),
        (8, 8),
        (9, 12),
        (10, 12),
        (11, 12),
        (12, 12),
        (13, 16),
        (16, 16),
        (17, 19),
        (18, 19),
        (19, 19),
        (25, 25),
        (26, 28),
        (27, 28),
        (28, 28),
        (29, 28),
    ] {
        let mut parser = Parser::new(S);
        parser.skip(skip);
        let rem = &S[start..];
        assert_eq!(parser.remainder(), rem);
        assert_eq!(parser.start_offset(), start, "rem: {rem:?}");
        assert_eq!(parser.parse_direction(), ParseDirection::FromStart);
    }
}

#[test]
fn test_skip_back() {
    for (skip, up_to) in [
        (0, 28),
        (1, 25),
        (2, 25),
        (3, 25),
        (4, 23),
        (5, 23),
        (6, 22),
        (7, 19),
        (8, 19),
        (9, 19),
        (10, 16),
        (11, 16),
        (12, 16),
        (13, 12),
        (23, 5),
        (24, 3),
        (25, 3),
        (26, 2),
        (27, 1),
        (28, 0),
        (29, 0),
        (30, 0),
    ] {
        let mut parser = Parser::new(S);
        parser.skip_back(skip);
        assert_eq!(parser.remainder(), &S[..up_to]);
        assert_eq!(parser.start_offset(), 0);
        assert_eq!(parser.parse_direction(), ParseDirection::FromEnd);
    }
}

#[test]
fn test_with_start_offset() {
    let mut parser = Parser::with_start_offset("bar baz qux", 10);

    assert_eq!(parser.start_offset(), 10);

    for (exp_str, exp_so) in [("bar", 14), ("baz", 18), ("qux", 21)] {
        let item = parser.split(' ').unwrap();
        assert_eq!(item, exp_str);
        assert_eq!(parser.start_offset(), exp_so);
    }
}
