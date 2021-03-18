/// Calls a `Parser` method with many alternative string literals.
///
/// If any of the literals match, the parser is mutated, otherwise it's left unmodified.
///
/// # Syntax
///
/// The general syntax for this macro is
/// `parse_any!{ <parser_expression> , <method_name> => <branches> }`
///
/// Where `<parser_expression>` is an expression (of [`Parser`] type) that can be assigned into.
///
/// Where `<method_name>` is the name of one of the [`Parser`] methods usable in this macro:
///
/// - [`strip_prefix`] and [`strip_suffix`]:
/// use the match-like syntax, running the branch of the first pattern that matches.
/// [example](#parsing-enum-example)
///
/// Where `<branches>` can be either of these, depending on the `<method_name>`:
///
/// - A match-like syntax:
/// A comma separated sequence of `<patterns> => <expression>` branches.
///
/// - Just the patterns: `<patterns>`.
///
/// Where `<patterns>` can be be any amount of `|`-separated string literals
/// (`const`ants don't work here), or `_`.
/// Note that passing a macro that expands to a literal doesn't work here,
/// `concat` and `stringify` are special cased to work.
///
/// [`strip_prefix`]: parsing/struct.Parser.html#method.strip_prefix
/// [`strip_suffix`]: parsing/struct.Parser.html#method.strip_suffix
///
///
/// # Example
///
/// <span id = "parsing-enum-example"></span>
/// ### Parsing enum
///
/// ```rust
/// use konst::{Parser, parse_any, unwrap_opt};
///
/// #[derive(Debug, PartialEq)]
/// enum Color {
///     Red,
///     Blue,
///     Green,
/// }
///
/// impl Color {
///     pub const fn try_parse(mut parser: Parser<'_>) -> Option<(Color, Parser<'_>)> {
///         parse_any!{parser, strip_prefix;
///             "Red"|"red" => Some((Color::Red, parser)),
///             "Blue"|"blue" => Some((Color::Blue, parser)),
///             "Green"|"green" => Some((Color::Green, parser)),
///             _ => None
///         }
///     }
/// }
///
/// const COLORS: [Color; 4] = {
///     let parser = Parser::from_str("BlueRedGreenGreen");
///     let (c0, parser) = unwrap_opt!(Color::try_parse(parser));
///     let (c1, parser) = unwrap_opt!(Color::try_parse(parser));
///     let (c2, parser) = unwrap_opt!(Color::try_parse(parser));
///     let (c3, _     ) = unwrap_opt!(Color::try_parse(parser));
///     
///     [c0, c1, c2, c3]
/// };
///
/// assert_eq!(COLORS, [Color::Blue, Color::Red, Color::Green, Color::Green])
///
///
/// ```
///
/// [`Parser`]: parsing/struct.Parser.html
#[macro_export]
macro_rules! parse_any {
    ($place:expr, strip_prefix; $($branches:tt)* ) => {
        $crate::__priv_pa_normalize_branches!{
            ($place, __priv_pa_strip_prefix, outside_konst)
            ()
            $($branches)*
        }
    };
    ($place:expr, strip_suffix; $($branches:tt)* ) => {
        $crate::__priv_pa_normalize_branches!{
            ($place, __priv_pa_strip_suffix, outside_konst)
            ()
            $($branches)*
        }
    };
}

#[allow(unused_macros)]
macro_rules! parse_any_priv {
    ($place:expr, $method_macro:ident; $($branches:tt)* ) => {
        $crate::__priv_pa_normalize_branches!{
            ($place, $method_macro, inside_konst)
            ()
            $($branches)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_pa_normalize_branches {
    (
        $fixed_params:tt
        ( $($prev_branch:tt)* )

        $($pattern:pat)|* => $expr:expr
        $(, $($rem:tt)* )?
    ) => {
        $crate::__priv_pa_normalize_branches!{
            $fixed_params
            (
                $($prev_branch)*

                ($($pattern)|*) => ($expr)
            )
            $($($rem)*)?
        }
    };
    (
        $fixed_params:tt
        ( $($prev_branch:tt)* )

        $($pattern:pat)|* => $expr:block
        $($rem:tt)*
    ) => {
        $crate::__priv_pa_normalize_branches!{
            $fixed_params
            (
                $($prev_branch)*

                ($($pattern)|*) => ($expr)
            )
            $($rem)*
        }
    };

    (
        ($place:expr, $method_macro:ident, $call_place:tt)
        (
            $($branches:tt)*
        )

        // Nothing left to parse
        $(,)?
    ) => {
        $crate::$method_macro!{
            ($place, $call_place)

            $($branches)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_pa_strip_prefix {
    (
        $accessor_args:tt

        $(
            ($($pattern:pat)|*)=>($e:expr)
        )*
    ) => {
        match $crate::__priv_pa_bytes_accessor!(get, $accessor_args) {
            $(
                $( $crate::__priv_bstr_start!(rem, $pattern))|* => {
                    $crate::__priv_pa_bytes_accessor!(set, $accessor_args, rem);
                    $e
                }
            )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_pa_strip_suffix {
    (
        $accessor_args:tt

        $(
            ($($pattern:pat)|*)=>($e:expr)
        )*
    ) => {
        match $crate::__priv_pa_bytes_accessor!(get, $accessor_args) {
            $(
                $( $crate::__priv_bstr_end!(rem, $pattern))|* => {
                    $crate::__priv_pa_bytes_accessor!(set, $accessor_args, rem);
                    $e
                }
            )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_pa_bytes_accessor {
    (get, ($place:expr, inside_konst)) => {
        $place.bytes
    };
    (get, ($place:expr, outside_konst)) => {
        $place.bytes()
    };
    (set, ($place:expr, inside_konst), $rem:expr) => {
        #[allow(unused_assignments)]
        {
            $place.bytes = $rem;
        }
    };
    (set, ($place:expr, outside_konst), $rem:expr) => {
        #[allow(unused_assignments)]
        {
            $place = $place.move_to_remainder($rem);
        }
    };
}
