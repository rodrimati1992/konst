#![allow(clippy::or_fun_call)]
#![allow(clippy::useless_conversion)]
#![allow(irrefutable_let_patterns)]

extern crate proc_macro;

use proc_macro as used_proc_macro;

use std::iter;

#[allow(unused_imports)]
use used_proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

mod destructuring;

mod parsing;

mod utils;

fn parse_crate_token(tt: Option<TokenTree>) -> TokenStream {
    match tt {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::None => group.stream(),
        Some(tt @ TokenTree::Ident(_)) => std::iter::once(tt).collect(),
        Some(tt) => panic!("expected $crate, found: `{tt:?}`"),
        None => panic!("expected $crate, found nothing"),
    }
}

#[doc(hidden)]
#[expect(non_snake_case)]
#[proc_macro]
pub fn __destructure__unwrap_pats(
    input_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    crate::destructuring::macro_impl(input_tokens.into()).into()
}

#[doc(hidden)]
#[proc_macro]
pub fn __priv_bstr_start(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bstr_pattern(input_tokens.into(), StrAt::Start).into()
}

#[doc(hidden)]
#[proc_macro]
pub fn __priv_bstr_end(input_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bstr_pattern(input_tokens.into(), StrAt::End).into()
}

fn bstr_pattern(input_tokens: TokenStream, str_at: StrAt) -> TokenStream {
    use crate::utils::punct_token;

    let parsed = parsing::parse_inputs(input_tokens);

    match parsed {
        Ok(Inputs { rem_ident, strings }) => {
            let mut out = TokenStream::new();

            for (i, patt) in strings.iter().enumerate() {
                let span = patt.span(&rem_ident);

                if i != 0 {
                    out.extend(punct_token('|', span));
                }
                let tt = crate::utils::bracket(Span::call_site(), |out| match str_at {
                    StrAt::Start => {
                        output_patt(patt, out);
                        output_remainder_pat(&rem_ident, out);
                    }
                    StrAt::End => {
                        output_remainder_pat(&rem_ident, out);
                        out.extend(punct_token(',', span));
                        output_patt(patt, out);
                    }
                });

                out.extend(iter::once(tt))
            }

            out
        }
        Err(e) => e.to_compile_error(),
    }
}

fn output_patt(patt: &Pattern, out: &mut TokenStream) {
    use crate::utils::punct_token;

    match patt {
        Pattern::String { string, span } => {
            for b in string.bytes() {
                let mut lit = Literal::u8_unsuffixed(b);
                lit.set_span(*span);
                out.extend(iter::once(TokenTree::from(lit)));
                out.extend(punct_token(',', *span));
            }
        }
    }
}

fn output_remainder_pat(patt: &Ident, out: &mut TokenStream) {
    use crate::utils::{punct_joint_token2, punct_token};

    out.extend(iter::once(TokenTree::from(patt.clone())));

    out.extend(punct_token('@', patt.span()));

    out.extend(punct_joint_token2('.', '.', patt.span()));
}

struct Inputs {
    rem_ident: Ident,
    strings: Vec<Pattern>,
}

enum Pattern {
    String { string: String, span: Span },
}

enum StrAt {
    Start,
    End,
}

impl Pattern {
    fn span(&self, _rem_ident: &Ident) -> Span {
        match self {
            Pattern::String { span, .. } => *span,
        }
    }
}
