#[allow(unused_imports)]
use crate::used_proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

pub(crate) fn macro_impl(input_tokens: TokenStream) -> TokenStream {
    let mut iter = input_tokens.into_iter();

    let krate = crate::parse_crate_token(iter.next());

    let mut out = TokenStream::new();

    loop {
        let pattern_paren = match iter.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
                group.stream()
            }
            Some(tt) => panic!("expected parentheses, found {tt}"),
            None => break out,
        };
        let pattern = match pattern_paren.into_iter().next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::None => group.stream(),
            Some(tt) => TokenStream::from(tt),
            None => panic!("expected pattern, found no more tokens"),
        };
        let expr = match iter.next() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => {
                TokenTree::Group(group)
            }
            Some(tt) => panic!("expected braced expression, found {tt}"),
            None => panic!("expected braced expression, found no more tokens"),
        };

        let var_span = Span::mixed_site();
        out.extend(crate::utils::ident_token("let", Span::mixed_site()));
        out.extend(crate::utils::ident_token("var", var_span));
        out.extend(crate::utils::punct_token('=', Span::mixed_site()));
        out.extend(std::iter::once(expr));
        out.extend(crate::utils::punct_token(';', Span::mixed_site()));

        out.extend(krate.clone());
        out.extend(crate::utils::punct_joint_token2(
            ':',
            ':',
            Span::call_site(),
        ));
        out.extend(crate::utils::ident_token(
            "__destructure__nested",
            Span::call_site(),
        ));
        out.extend(crate::utils::punct_token('!', Span::call_site()));
        out.extend(std::iter::once(crate::utils::brace(
            Span::call_site(),
            |out| {
                out.extend(crate::utils::ident_token("var", var_span));

                out.extend(pattern);
            },
        )));
    }
}
