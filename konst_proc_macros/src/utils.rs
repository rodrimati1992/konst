use std::iter::{self, Once};

#[allow(unused_imports)]
use crate::used_proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

pub(crate) fn ident_token(ident: &str, span: Span) -> Once<TokenTree> {
    let ident = Ident::new(ident, span);
    let tt = TokenTree::from(ident);
    iter::once(tt)
}

pub(crate) fn punct_token(token: char, span: Span) -> Once<TokenTree> {
    let mut token = Punct::new(token, Spacing::Alone);
    token.set_span(span);
    let tt = TokenTree::from(token);
    iter::once(tt)
}

pub(crate) fn punct_joint_token2(first: char, second: char, span: Span) -> Vec<TokenTree> {
    let tt_first = {
        let mut token = Punct::new(first, Spacing::Joint);
        token.set_span(span);
        TokenTree::from(token)
    };

    let tt_second = {
        let mut token = Punct::new(second, Spacing::Alone);
        token.set_span(span);
        TokenTree::from(token)
    };

    vec![tt_first, tt_second]
}

pub(crate) fn paren<F>(span: Span, f: F) -> TokenTree
where
    F: FnOnce(&mut TokenStream),
{
    let mut ts = TokenStream::new();
    f(&mut ts);
    let mut tt = Group::new(Delimiter::Parenthesis, ts);
    tt.set_span(span);
    TokenTree::from(tt)
}

pub(crate) fn bracket<F>(span: Span, f: F) -> TokenTree
where
    F: FnOnce(&mut TokenStream),
{
    let mut ts = TokenStream::new();
    f(&mut ts);
    let mut tt = Group::new(Delimiter::Bracket, ts);
    tt.set_span(span);
    TokenTree::from(tt)
}

pub(crate) fn brace<F>(span: Span, f: F) -> TokenTree
where
    F: FnOnce(&mut TokenStream),
{
    let mut ts = TokenStream::new();
    f(&mut ts);
    let mut tt = Group::new(Delimiter::Brace, ts);
    tt.set_span(span);
    TokenTree::from(tt)
}

///////////////////////////////////////////////////////

pub(crate) struct Error {
    span: Span,
    message: String,
}

impl Error {
    pub(crate) fn new(span: Span, message: &str) -> Self {
        Self {
            span,
            message: message.to_string(),
        }
    }

    pub(crate) fn to_compile_error(&self, krate: Option<TokenStream>) -> TokenStream {
        let Error { ref message, span } = *self;

        let mut out = TokenStream::new();

        if let Some(k) = krate {
            out.extend(k);

            out.extend(crate::utils::punct_joint_token2(':', ':', span));
        }

        out.extend(crate::utils::ident_token("compile_error", span));

        out.extend(crate::utils::punct_token('!', span));

        let msg_paren = crate::utils::paren(span, |ts| {
            let mut msg = Literal::string(message);
            msg.set_span(self.span);
            let msg = TokenTree::from(msg);
            ts.extend(iter::once(msg))
        });
        out.extend(iter::once(msg_paren));

        out
    }
}

///////////////////////////////////////////////////////

#[allow(dead_code)]
pub(crate) trait TokenTreeExt: Sized {
    fn into_token_tree(self) -> TokenTree;

    fn set_span_recursive(self, span: Span) -> TokenTree {
        let mut tt = self.into_token_tree();

        tt.set_span(span);
        if let TokenTree::Group(group) = tt {
            let delim = group.delimiter();
            let stream = group.stream().set_span_recursive(span);
            tt = TokenTree::Group(Group::new(delim, stream));
        }
        tt.set_span(span);
        tt
    }
}

impl TokenTreeExt for TokenTree {
    fn into_token_tree(self) -> TokenTree {
        self
    }
}

#[allow(dead_code)]
pub trait TokenStreamExt: Sized {
    fn into_token_stream(self) -> TokenStream;

    fn set_span_recursive(self, span: Span) -> TokenStream {
        self.into_token_stream()
            .into_iter()
            .map(|tt| tt.set_span_recursive(span))
            .collect()
    }
}

impl TokenStreamExt for TokenStream {
    fn into_token_stream(self) -> TokenStream {
        self
    }
}
