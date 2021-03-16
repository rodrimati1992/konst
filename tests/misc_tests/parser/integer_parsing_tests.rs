use konst::parsing::{ParseIntError, Parser};

use std::{
    cmp::PartialEq,
    fmt::{Debug, Display},
    iter,
};

fn check_parse<T, F>(num: T, method: F)
where
    T: PartialEq + Display + Debug,
    F: for<'a> Fn(Parser<'a>) -> Result<(T, Parser<'a>), ParseIntError>,
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
    F: for<'a> Fn(Parser<'a>) -> Result<(T, Parser<'a>), ParseIntError>,
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
fn parser_usize_isize_test() {
    check_int_parsing! {
        (isize, parse_isize)
        (usize, parse_usize)
    }
}
