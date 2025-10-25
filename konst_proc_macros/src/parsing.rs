#[allow(unused_imports)]
use crate::used_proc_macro::{
    self, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

use crate::utils::Error;

#[cfg(test)]
mod parsing_tests;

pub(crate) type Parser = std::iter::Peekable<used_proc_macro::token_stream::IntoIter>;

pub(crate) struct Attribute {
    pub(crate) hash: TokenTree,
    pub(crate) bracket: TokenTree,
}

pub(crate) fn parse_attrs(parser: &mut Parser) -> Result<Vec<Attribute>, Error> {
    let mut out = Vec::new();

    while let Some(TokenTree::Punct(p)) = parser.peek()
        && p.as_char() == '#'
    {
        let hash = TokenTree::Punct(p.clone());
        parser.next();

        match parser.peek() {
            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => {}
            _ => break,
        }

        let bracket = parser.next().unwrap();

        out.push(Attribute { hash, bracket });
    }

    Ok(out)
}

pub(crate) fn peek_parse_path(parser: &mut Parser) -> Result<Option<TokenStream>, Error> {
    let start_span = match parser.peek() {
        Some(TokenTree::Punct(p))
            if matches!(
                (p.spacing(), p.as_char()),
                (_, '<' | '>') | (Spacing::Joint, ':')
            ) =>
        {
            p.span()
        }
        Some(TokenTree::Ident(ident)) if ident.to_string() == "_" => return Ok(None),
        Some(TokenTree::Ident(ident)) => ident.span(),
        _ => return Ok(None),
    };

    let mut level = 0usize;

    let mut out = TokenStream::new();
    let mut last_span = start_span;
    let mut prev_token_spacing = Spacing::Joint;

    loop {
        let tt = parser.peek();

        let mut curr_token_spacing = Spacing::Joint;

        match tt {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '<' => {
                level += 1;
            }
            Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
                level = level
                    .checked_sub(1)
                    .ok_or_else(|| Error::new(punct.span(), "unexpected '>'"))?;
            }
            Some(tt) if level == 0 && is_path_terminator(tt, prev_token_spacing) => {
                break;
            }
            Some(TokenTree::Punct(punct)) if level == 0 => {
                curr_token_spacing = punct.spacing();
            }
            _ => {}
        }

        prev_token_spacing = curr_token_spacing;

        if let Some(tt) = parser.next() {
            last_span = tt.span();
            out.extend(std::iter::once(tt));
        } else {
            break;
        }
    }

    if level == 0 {
        Ok(Some(out))
    } else {
        Err(Error::new(last_span, "incomplete path"))
    }
}

fn is_path_terminator(tt: &TokenTree, prev_token_spacing: Spacing) -> bool {
    match tt {
        TokenTree::Punct(p) => {
            !(p.as_char() == ':' && [prev_token_spacing, p.spacing()].contains(&Spacing::Joint))
        }
        TokenTree::Group(_) => true,
        TokenTree::Ident(_) => false,
        TokenTree::Literal(_) => true,
    }
}
