#[allow(unused_imports)]
use crate::used_proc_macro::{
    self, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

use crate::utils::Error;

use super::peek_parse_path;

fn pp_path(s: &str) -> Result<Option<String>, Error> {
    let ts = s.parse::<TokenStream>().unwrap();
    let parser = &mut ts.into_iter().peekable();
    peek_parse_path(parser).map(|res| res.map(|ts| ts.to_string().replace(" ", "")))
}

#[test]
fn test_peek_parse_path() {
    assert_eq!(pp_path("foo::bar::baz").unwrap().unwrap(), "foo::bar::baz");
    assert_eq!(
        pp_path("foo<::bar::baz>").unwrap().unwrap(),
        "foo<::bar::baz>"
    );
}
