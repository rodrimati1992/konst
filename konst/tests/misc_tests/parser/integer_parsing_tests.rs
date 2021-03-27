use konst::parsing::{ParseError, Parser};

use std::{
    cmp::PartialEq,
    fmt::{Debug, Display},
    iter,
};

fn check_parse<T, F>(num: T, method: F)
where
    T: PartialEq + Display + Debug,
    F: for<'a> Fn(Parser<'a>) -> Result<(T, Parser<'a>), ParseError>,
{
    for suffix in ["", ";", "-", "--"].iter().copied() {
        let mut string = num.to_string();
        string.push_str(suffix);

        let parser = Parser::from_str(&string);
        let (parsed_num, parser) = method(parser).unwrap();

        assert_eq!(num, parsed_num);
        assert_eq!(parser.bytes(), suffix.as_bytes());
    }
}

fn check_type<T, F>(min: T, max: T, method: F)
where
    T: PartialEq + Display + Debug + Copy,
    F: for<'a> Fn(Parser<'a>) -> Result<(T, Parser<'a>), ParseError>,
{
    for num in [min, max].iter().copied() {
        check_parse(num, &method);

        let mut string = num.to_string();
        let last_digit = string.pop().unwrap();
        let add_one = (last_digit as u8 + 1) as char;
        assert!(add_one.is_ascii_digit());
        string.push(add_one);

        let parser = Parser::from_str(&string);
        assert!(method(parser).is_err());
    }

    for notnum in ["", "-", "#", " "].iter().copied() {
        let parser = Parser::from_str(notnum);
        assert!(method(parser).is_err());
    }
}

macro_rules! check_int_parsing {
    (
        ($signed:ident, $parse_signed:ident)
        ($unsigned:ident, $parse_unsigned:ident)
    ) => {{
        {
            for base in iter::successors(Some(1), |n: &$unsigned| n.checked_mul(10)) {
                let sadd = |x: $unsigned| base.saturating_add(x);
                let ssub = |x: $unsigned| base.saturating_sub(x);
                for &n in [ssub(2), ssub(1), base, sadd(1), sadd(2)].iter() {
                    check_parse(n, |x| Parser::$parse_unsigned(x));
                }
            }

            check_type($unsigned::MAX, $unsigned::MAX, |x| {
                Parser::$parse_unsigned(x)
            });
        }
        {
            for base in iter::successors(Some(1), |n: &$signed| n.checked_mul(10)) {
                let sadd = |x: $signed| base.saturating_add(x);
                let ssub = |x: $signed| base.saturating_sub(x);
                for &n in [ssub(2), ssub(1), base, sadd(1), sadd(2)].iter() {
                    check_parse(n, |x| Parser::$parse_signed(x));
                    check_parse(n.wrapping_neg(), |x| Parser::$parse_signed(x));
                }
            }

            check_type($signed::MIN, $signed::MAX, |x| Parser::$parse_signed(x));
        }
    }};
}

#[test]
fn parser_u128_i128_test() {
    check_int_parsing! {
        (i128, parse_i128)
        (u128, parse_u128)
    }
}

#[test]
fn parser_u64_i64_test() {
    check_int_parsing! {
        (i64, parse_i64)
        (u64, parse_u64)
    }
}

#[test]
fn parser_u32_i32_test() {
    check_int_parsing! {
        (i32, parse_i32)
        (u32, parse_u32)
    }
}

#[test]
fn parser_u16_i16_test() {
    check_int_parsing! {
        (i16, parse_i16)
        (u16, parse_u16)
    }
}

#[test]
fn parser_u8_i8_test() {
    check_int_parsing! {
        (i8, parse_i8)
        (u8, parse_u8)
    }
}

#[test]
fn parser_usize_isize_test() {
    check_int_parsing! {
        (isize, parse_isize)
        (usize, parse_usize)
    }
}

#[test]
fn ensure_correct_delegation() {
    use konst::primitive;

    {
        let arr = [
            ("0", Some(0u8)),
            ("12", Some(12)),
            ("123", Some(123)),
            ("1234", None),
        ];

        for (input, output) in arr.iter().copied() {
            assert_eq!(primitive::parse_u8(input).ok(), output);
            assert_eq!(primitive::parse_u8_b(input.as_bytes()).ok(), output);
        }
    }
    {
        let arr = &[
            ("-129", None),
            ("-128", Some(-128i8)),
            ("-13", Some(-13)),
            ("-1", Some(-1)),
            ("0", Some(0)),
            ("4", Some(4)),
            ("48", Some(48)),
            ("127", Some(127)),
            ("128", None),
        ];

        for (input, output) in arr.iter().copied() {
            assert_eq!(primitive::parse_i8(input).ok(), output);
            assert_eq!(primitive::parse_i8_b(input.as_bytes()).ok(), output);
        }
    }

    macro_rules! check_unsigned_parser {
        (
            $type:ty, $str_fn:ident, $byte_fn:ident
        ) => {{
            let arr: &[(&str, Option<$type>)] = &[
                ("0", Some(0)),
                ("12", Some(12)),
                ("123", Some(123)),
                ("400000000000000000000000000099000000000", None),
                ("A", None),
            ];

            for (input, output) in arr.iter().copied() {
                assert_eq!(primitive::$str_fn(input).ok(), output);
                assert_eq!(primitive::$byte_fn(input.as_bytes()).ok(), output);
            }
        }};
    }

    check_unsigned_parser! {u16, parse_u16, parse_u16_b}
    check_unsigned_parser! {u32, parse_u32, parse_u32_b}
    check_unsigned_parser! {u64, parse_u64, parse_u64_b}
    check_unsigned_parser! {u128, parse_u128, parse_u128_b}
    check_unsigned_parser! {usize, parse_usize, parse_usize_b}

    macro_rules! check_unsigned_parser {
        (
            $type:ty, $str_fn:ident, $byte_fn:ident
        ) => {{
            let arr: &[(&str, Option<$type>)] = &[
                ("A", None),
                ("-200000000000000000000000000099000000000", None),
                ("-128", Some(-128)),
                ("-13", Some(-13)),
                ("-1", Some(-1)),
                ("0", Some(0)),
                ("4", Some(4)),
                ("48", Some(48)),
                ("127", Some(127)),
                ("200000000000000000000000000099000000000", None),
                ("-", None),
            ];

            for (input, output) in arr.iter().copied() {
                assert_eq!(primitive::$str_fn(input).ok(), output);
                assert_eq!(primitive::$byte_fn(input.as_bytes()).ok(), output);
            }
        }};
    }

    check_unsigned_parser! {i16, parse_i16, parse_i16_b}
    check_unsigned_parser! {i32, parse_i32, parse_i32_b}
    check_unsigned_parser! {i64, parse_i64, parse_i64_b}
    check_unsigned_parser! {i128, parse_i128, parse_i128_b}
    check_unsigned_parser! {isize, parse_isize, parse_isize_b}
}
