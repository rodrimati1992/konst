use const_panic::{flatten_panicvals as f_pvs, ArrayString, FmtArg};

use konst::parsing::{ErrorKind, ParseDirection, ParseError, Parser};

type Buff = ArrayString<256>;

#[test]
fn free_parse_bool_test() {
    let err = konst::primitive::parse_bool("--").unwrap_err();

    macro_rules! case {
        ($fmtarg:ident, $fmtstring:literal) => {{
            assert_eq!(
                Buff::from_panicvals(&err.to_panicvals(FmtArg::$fmtarg)).unwrap(),
                *format!(concat!("{:", $fmtstring, "}"), err),
            );
        }};
    }

    case! {DEBUG, "?"}
    case! {ALT_DEBUG, "#?"}
    case! {DISPLAY, ""}
    case! {ALT_DISPLAY, "#"}
}

#[test]
fn parser_error_fmt_equiv() {
    let parser = Parser::new(" - - - ");

    for error_kind in [
        ErrorKind::ParseInteger,
        ErrorKind::ParseBool,
        ErrorKind::Find,
        ErrorKind::Strip,
        ErrorKind::SplitExhausted,
        ErrorKind::DelimiterNotFound,
        ErrorKind::Other,
    ] {
        for pd in [
            ParseDirection::FromStart,
            ParseDirection::FromEnd,
            ParseDirection::FromBoth,
        ] {
            let mut parser = parser.copy();
            match pd {
                ParseDirection::FromStart => parser.trim_start(),
                ParseDirection::FromEnd => parser.trim_end(),
                ParseDirection::FromBoth => parser.trim(),
            };

            let pe = match error_kind {
                ErrorKind::Other => parser.copy().to_other_error(&"oh hi!"),
                _ => parser.copy().into_error(error_kind),
            };

            assert_eq!(pe.kind(), error_kind);
            assert_eq!(pe.error_direction(), pd);

            {
                let debug_f = Buff::from_panicvals(&pe.to_panicvals(FmtArg::DEBUG)).unwrap();
                if pe.kind() == ErrorKind::Other {
                    assert!(debug_f.to_str().contains("oh hi!"), "{debug_f:?}");
                }
                assert_eq!(debug_f, *format!("{pe:?}"));
            }

            assert_eq!(
                Buff::from_panicvals(&pe.to_panicvals(FmtArg::ALT_DEBUG)).unwrap(),
                *format!("{pe:#?}"),
            );

            {
                let display_f = Buff::from_panicvals(&pe.to_panicvals(FmtArg::DISPLAY)).unwrap();
                if pe.kind() == ErrorKind::Other {
                    assert!(display_f.to_str().contains("oh hi!"), "{display_f:?}");
                }
                assert_eq!(display_f, *format!("{pe:}"));
            }

            assert_eq!(
                Buff::from_panicvals(&pe.to_panicvals(FmtArg::ALT_DISPLAY)).unwrap(),
                *format!("{pe:#}"),
            );
        }
    }
}
