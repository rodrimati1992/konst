#[allow(unused_imports)]
use crate::used_proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

use crate::{
    patterns::{self, ParseLocation},
    utils::Error,
};

pub(crate) fn macro_impl(input_tokens: TokenStream) -> Result<TokenStream, (Error, TokenStream)> {
    let mut iter = input_tokens.into_iter().peekable();

    let krate = crate::unwrap_crate_token(iter.next());

    let macro_prefix_args = match iter.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            group.stream()
        }
        Some(tt) => panic!("expected parentheses, found {tt}"),
        None => panic!("expected parentheses, found nothing"),
    };

    let attrs = crate::parsing::parse_attrs(&mut iter).map_err(|e| (e, krate.clone()))?;

    let pattern = patterns::parse_pattern(ParseLocation::TopLevel, &mut iter)
        .map_err(|e| (e, krate.clone()))?;

    let mut out = TokenStream::new();

    out.extend(krate);
    out.extend(crate::utils::punct_joint_token2(
        ':',
        ':',
        Span::call_site(),
    ));
    out.extend(crate::utils::ident_token(
        "__destructure_rec__inner",
        Span::call_site(),
    ));
    out.extend(crate::utils::punct_token('!', Span::call_site()));
    out.extend(std::iter::once(crate::utils::brace(
        Span::call_site(),
        |out| {
            out.extend(macro_prefix_args);

            out.extend(attrs.into_iter().flat_map(|attr| [attr.hash, attr.bracket]));

            out.extend(std::iter::once(crate::utils::brace(
                Span::call_site(),
                |out| {
                    crate::patterns::expand_pattern(pattern, out);
                },
            )));

            out.extend(iter);
        },
    )));

    Ok(out)
}
