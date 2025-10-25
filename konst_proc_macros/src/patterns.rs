#[allow(unused_imports)]
use crate::used_proc_macro::{
    self, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream as TS, TokenTree,
};

use crate::{
    parsing::{Parser, peek_parse_path},
    utils::{self, Error},
};

use std::{collections::VecDeque, iter::once};

pub(crate) struct Pattern {
    pub(crate) pattern_tokens: TS,

    pub(crate) var: PatternVariant,
}

pub(crate) enum PatternVariant {
    Underscore(Span),
    Rem { binding: Option<TS>, dotdot: Span },
    Ident,
    Array(Arraylike),
    Tuple(Arraylike),
    Struct(Struct),
}

pub(crate) struct Arraylike {
    pub(crate) group_span: Span,
    pub(crate) patterns: VecDeque<Pattern>,
    pub(crate) remainder_pos: Option<usize>,
}

pub(crate) struct Struct {
    pub(crate) group_span: Span,
    pub(crate) path: TS,
    pub(crate) fields: Vec<Field>,
    pub(crate) remainder: Option<Span>,
}

pub(crate) struct Field {
    pub(crate) name: TokenTree,
    pub(crate) pattern: Pattern,
}

fn usize_lit(n: usize, span: Span) -> TokenTree {
    let mut lit = Literal::usize_unsuffixed(n);
    lit.set_span(span);
    TokenTree::Literal(lit)
}

fn isize_lit(n: isize, span: Span) -> TokenTree {
    let mut lit = Literal::isize_unsuffixed(n);
    lit.set_span(span);
    TokenTree::Literal(lit)
}

fn group(delim: Delimiter, span: Span, stream: TS) -> TokenTree {
    let mut group = Group::new(delim, stream);
    group.set_span(span);
    group.into()
}

pub(crate) fn expand_pattern(pat: Pattern, out: &mut TS) {
    let mut out_t = TS::new();

    out_t.extend(once(group(
        Delimiter::Parenthesis,
        Span::call_site(),
        pat.pattern_tokens,
    )));

    match pat.var {
        PatternVariant::Underscore(span) => {
            out_t.extend(once(TokenTree::from(Ident::new("underscore", span))));
        }
        PatternVariant::Rem { .. } => {
            unreachable!()
        }
        PatternVariant::Ident => {
            out_t.extend(once(TokenTree::from(Ident::new(
                "ident",
                Span::call_site(),
            ))));
        }
        PatternVariant::Array(al) => {
            out_t.extend(once(TokenTree::from(Ident::new("array", al.group_span))));
            expand_arraylike(al, &mut out_t);
        }
        PatternVariant::Tuple(al) => {
            out_t.extend(once(TokenTree::from(Ident::new("tuple", al.group_span))));
            expand_arraylike(al, &mut out_t);
        }
        PatternVariant::Struct(struct_) => {
            out_t.extend(once(TokenTree::from(Ident::new(
                "struct",
                struct_.group_span,
            ))));
            expand_struct(struct_, &mut out_t);
        }
    }

    out.extend(once(group(
        Delimiter::Parenthesis,
        Span::call_site(),
        out_t,
    )));
}

pub(crate) fn expand_arraylike(pat: Arraylike, out_t: &mut TS) {
    let Arraylike {
        group_span,
        mut patterns,
        remainder_pos,
    } = pat;

    out_t.extend(once(utils::paren(group_span, |out_p| {
        for (i, pat) in patterns
            .drain(..remainder_pos.unwrap_or(patterns.len()))
            .enumerate()
        {
            out_p.extend(once(usize_lit(i, group_span)));
            expand_pattern(pat, out_p);
            out_p.extend(utils::punct_token(',', group_span));
        }
    })));

    if let Some(rem_pos) = remainder_pos {
        let Some(Pattern {
            var: PatternVariant::Rem { binding, dotdot },
            ..
        }) = patterns.pop_front()
        else {
            unreachable!()
        };

        out_t.extend(once(utils::paren(dotdot, |out_p| {
            out_p.extend(once(usize_lit(rem_pos, group_span)));

            match binding {
                Some(binding) => out_p.extend(binding),
                None => out_p.extend(once(TokenTree::Ident(Ident::new("_", dotdot)))),
            }
        })));

        out_t.extend(once(utils::paren(group_span, |out_p| {
            let suffix = patterns;

            for (i, pat) in (-(suffix.len() as isize)..0).zip(suffix) {
                out_p.extend(once(isize_lit(i, group_span)));
                expand_pattern(pat, out_p);
                out_p.extend(utils::punct_token(',', group_span));
            }
        })));
    }
}

pub(crate) fn expand_struct(pat: Struct, out_t: &mut TS) {
    let Struct {
        group_span,
        path,
        fields,
        remainder,
    } = pat;

    out_t.extend(once(utils::paren(group_span, |out_p| {
        out_p.extend(path);
    })));

    out_t.extend(once(utils::brace(group_span, |out_p| {
        for Field { name, pattern } in fields {
            out_p.extend(once(name.clone()));
            expand_pattern(pattern, out_p);
            out_p.extend(utils::punct_token(',', name.span()));
        }
    })));

    if let Some(rem_span) = remainder {
        out_t.extend(once(utils::paren(rem_span, |out_p| {
            out_p.extend(utils::punct_joint_token2('.', '.', rem_span));
        })));
    }
}

pub(crate) fn parse_pattern(parser: &mut Parser) -> Result<Pattern, Error> {
    let mut out = TS::new();

    if let Some(TokenTree::Group(group)) = parser.peek()
        && group.delimiter() == Delimiter::None
    {
        let inner_tokens = &mut group.stream().into_iter().peekable();

        parser.next();

        return parse_pattern(inner_tokens);
    }

    if let Some(path) = peek_parse_path(parser)? {
        match parser.peek() {
            x if x.is_none_or(is_pattern_terminator) => {
                return Ok(Pattern {
                    pattern_tokens: path.clone(),
                    var: PatternVariant::Ident,
                });
            }
            Some(TokenTree::Group(group)) => {
                let struct_pat = parse_struct_pattern(path.clone(), &group)?;

                out.extend(path);
                out.extend(parser.next());

                return Ok(Pattern {
                    pattern_tokens: out,
                    var: PatternVariant::Struct(struct_pat),
                });
            }
            Some(TokenTree::Punct(p)) if p.as_char() == '@' => {
                out.extend(path.clone());
                out.extend(parser.next());

                return parse_dotdot(out, Some(path), parser);
            }
            _ => {}
        }
    }

    match parser.peek() {
        Some(TokenTree::Punct(p)) if p.as_char() == '.' => return parse_dotdot(out, None, parser),
        Some(TokenTree::Ident(ident)) if ident.to_string() == "_" => {
            let span = ident.span();

            return Ok(Pattern {
                pattern_tokens: TS::from_iter(parser.next()),
                var: PatternVariant::Underscore(span),
            });
        }
        Some(TokenTree::Group(group))
            if matches!(
                group.delimiter(),
                Delimiter::Parenthesis | Delimiter::Bracket
            ) =>
        {
            let inner_parser = &mut group.stream().into_iter().peekable();
            let delim = group.delimiter();
            let arraylike = parse_arraylike(group.span(), inner_parser)?;

            out.extend(parser.next());

            return Ok(Pattern {
                pattern_tokens: out,
                var: match delim {
                    Delimiter::Bracket => PatternVariant::Array(arraylike),
                    Delimiter::Parenthesis => PatternVariant::Tuple(arraylike),
                    _ => unreachable!(),
                },
            });
        }
        Some(tt) => return Err(Error::new(tt.span(), "expected pattern")),
        None => return Err(Error::new(Span::call_site(), "expected pattern")),
    }
}

fn parse_arraylike(group_span: Span, parser: &mut Parser) -> Result<Arraylike, Error> {
    let mut patterns = VecDeque::new();

    let mut remainder_pos = None;

    while parser.peek().is_some() {
        let pattern = parse_pattern(parser)?;

        if let PatternVariant::Rem { dotdot, .. } = pattern.var {
            if remainder_pos.is_some() {
                return Err(Error::new(dotdot, "only one `..` pattern is allowed"));
            }

            remainder_pos = Some(patterns.len());
        }

        patterns.push_back(pattern);

        parse_comma_term(parser)?;
    }

    Ok(Arraylike {
        group_span,
        patterns,
        remainder_pos,
    })
}

fn parse_struct_pattern(path: TS, group: &Group) -> Result<Struct, Error> {
    let is_braced = match group.delimiter() {
        Delimiter::Bracket => false,
        Delimiter::Brace => true,
        _ => return Err(Error::new(group.span(), "expected struct")),
    };

    let parser = &mut group.stream().into_iter().peekable();

    let mut fields = Vec::new();

    let mut i = 0;

    let mut remainder = None;

    while let Some(first_span) = parser.peek().map(|tt| tt.span()) {
        if remainder.is_some() {
            return Err(Error::new(
                first_span,
                "no fields are allowed after `..` patterns",
            ));
        }

        if let Some(TokenTree::Punct(p)) = parser.peek()
            && p.as_char() == '.'
            && let Pattern {
                var: PatternVariant::Rem { dotdot, binding },
                ..
            } = parse_dotdot(TS::new(), None, parser)?
        {
            if binding.is_some() {
                return Err(Error::new(
                    dotdot,
                    "`@ ..` pattern not supported in struct patterns",
                ));
            }

            remainder = Some(dotdot);
        } else {
            let (name, field_pat_kind) = if is_braced {
                parse_field_name(parser)?
            } else {
                let lit = usize_lit(i, first_span);
                i += 1;
                (lit, FieldPatKind::WithPat)
            };

            let pattern = match field_pat_kind {
                FieldPatKind::OnlyName => Pattern {
                    pattern_tokens: TS::from(name.clone()),
                    var: PatternVariant::Ident,
                },
                FieldPatKind::WithPat => parse_pattern(parser)?,
            };

            fields.push(Field { name, pattern });
        }

        parse_comma_term(parser)?;
    }

    Ok(Struct {
        group_span: group.span(),
        path,
        fields,
        remainder,
    })
}

fn parse_comma_term(parser: &mut Parser) -> Result<Option<Span>, Error> {
    match parser.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => Ok(Some(p.span())),
        Some(tt) => Err(Error::new(tt.span(), "expected comma")),
        None => Ok(None),
    }
}

enum FieldPatKind {
    OnlyName,
    WithPat,
}

fn parse_field_name(parser: &mut Parser) -> Result<(TokenTree, FieldPatKind), Error> {
    let field_name = match parser.next() {
        Some(tt @ (TokenTree::Literal(_) | TokenTree::Ident(_))) => tt,
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::None => {
            return parse_field_name(&mut group.stream().into_iter().peekable());
        }
        Some(tt) => return Err(Error::new(tt.span(), "expected struct field name")),
        None => return Err(Error::new(Span::call_site(), "expected struct field name")),
    };

    match parser.peek() {
        Some(TokenTree::Punct(p)) if p.as_char() == ':' => {
            parser.next();

            Ok((field_name, FieldPatKind::WithPat))
        }
        Some(TokenTree::Punct(p)) if p.as_char() == ',' => Ok((field_name, FieldPatKind::OnlyName)),
        Some(tt) => Err(Error::new(tt.span(), "expected `:`")),
        None => Ok((field_name, FieldPatKind::OnlyName)),
    }
}

fn parse_dotdot(mut out: TS, binding: Option<TS>, parser: &mut Parser) -> Result<Pattern, Error> {
    match (parser.next(), parser.next()) {
        (Some(TokenTree::Punct(p0)), Some(TokenTree::Punct(p1)))
            if p0.as_char() == '.' && p1.as_char() == '.' =>
        {
            let span = p0.span();

            out.extend([TokenTree::Punct(p0), TokenTree::Punct(p1)]);

            Ok(Pattern {
                pattern_tokens: out,
                var: PatternVariant::Rem {
                    binding,
                    dotdot: span,
                },
            })
        }
        (Some(tt), _) => Err(Error::new(tt.span(), "expected `..`")),
        (None, _) => Err(Error::new(
            Span::call_site(),
            "expected `..`, found nothing",
        )),
    }
}

fn is_pattern_terminator(tt: &TokenTree) -> bool {
    matches!(tt, TokenTree::Punct(p) if matches!(p.as_char(), ':' | ';' | '='))
}
