#[allow(unused_imports)]
use crate::used_proc_macro::{
    self, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

use crate::utils::Error;

use super::peek_parse_path_or_under;

fn pp_path(s: &str) -> Result<Option<String>, Error> {
    let ts = s.parse::<TokenStream>().unwrap();
    let parser = &mut ts.into_iter().peekable();
    peek_parse_path_or_under(parser)
        .map(|res| res.map(|ts| ts.into_tokens().0.to_string().replace(" ", "")))
}

#[test]
fn test_peek_parse_path_or_under() {
    assert_eq!(pp_path("foo::bar::baz").unwrap().unwrap(), "foo::bar::baz");

    assert_eq!(
        pp_path("foo<::bar::baz>").unwrap().unwrap(),
        "foo<::bar::baz>"
    );

    assert_eq!(
        pp_path("foo<(::bar::baz, u32, u64)>").unwrap().unwrap(),
        "foo<(::bar::baz,u32,u64)>"
    );

    assert_eq!(
        pp_path("foo<{(::bar::baz, u32)}, u64<<F>::foo<bar>>>")
            .unwrap()
            .unwrap(),
        "foo<{(::bar::baz,u32)},u64<<F>::foo<bar>>>"
    );
}
