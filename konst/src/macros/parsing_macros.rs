/// Assigns the value in a `Ok` variant into pre-existing variables,
/// doing nothing it it was an `Err` variant.
///
/// # Let pattern
///
/// You can declare variables usable inside this macro with `let` patterns, like this:
/// ```rust
/// # use konst::rebind_if_ok;
/// #
/// let mut bar = 0;
/// let mut number = 10;
/// let res: Result<_, ()> = Ok((10, 20));
/// rebind_if_ok!{(let foo, bar) = res =>
///     number += foo;
/// }
/// assert_eq!(bar, 20);
/// ```
/// `foo` in this invocation of `rebind_if_ok` is a macro-local variable initialized with `10`,
/// while `bar` is a pre-existing variable that is assigned `20`.
///
/// This pattern only works when destructuring tuples.
///
///
///
#[macro_export]
macro_rules! rebind_if_ok {
    (
        $pattern:tt = $expression:expr
        $( => $($code:tt)* )?
    ) => {
        if let $crate::__::v::Ok(tuple) = $expression {
            $crate::__priv_ai_preprocess_pattern!( tuple, ($pattern) );
            $($($code)*)?
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_ai_preprocess_pattern {
    ( $var:ident, (($($pat:tt)*))) => {
        $crate::__priv_assign_tuple!($var, (0 1 2 3 4 5) , $($pat)*)
    };
    ( $var:ident, ($($pat:tt)*)) => {
        $crate::__priv_assign_tuple!($var, (0 1 2 3 4 5) , $($pat)*)
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __priv_assign_tuple {
    ($var:ident, $fields:tt, let $pat:pat $(, $($rem:tt)*)?) => {
        $crate::__priv_next_ai_access!(
            (let $pat) $var, $fields, $($($rem)*)?
        )
    };
    ($var:ident, $fields:tt, _ $(, $($rem:tt)*)?) => {
        $crate::__priv_next_ai_access!(
            (let _) $var, $fields, $($($rem)*)?
        )
    };
    ($var:ident, $fields:tt, $e:expr $(, $($rem:tt)*)?) => {
        $crate::__priv_next_ai_access!(
            ($e) $var, $fields, $($($rem)*)?
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_next_ai_access {
    ( ($($lhs:tt)*) $var:ident , (0 $($rem_fields:tt)*),  ) => {
        $($lhs)* = $var;
    };
    ( ($($lhs:tt)*) $var:ident , ($field:tt $($rem_fields:tt)*),  ) => {
        $($lhs)* = $var.$field;
    };
    ( ($($lhs:tt)*) $var:ident , ($field:tt $($rem_fields:tt)*), $($rem:tt)+ ) => {
        $($lhs)* = $var.$field;
        $crate::__priv_assign_tuple!($var,($($rem_fields:tt)*), $($rem)+)
    };
}

///////////////////////////////////////////////////////////////////////////////

/// Returns on Err, otherwise assigns the value in a `Ok` variant into pre-existing variables.
///
/// # Mapping errors
///
/// You can optionally map the returned error like this:
/// ```text
/// try_rebind!{foo = bar(), map_err = |e| convert_this_error(e)}
/// ```
/// or
/// ```text
/// try_rebind!{foo = bar(), map_err = convert_this_error}
/// ```
/// The `convert_this_error` function is expected to return the error type that
/// the current function returns.
///
/// # Let pattern
///
/// You can declare new variables with `let` patterns like this:
/// ```text
/// try_rebind!{(let foo, bar) = Ok((10, 20))}
/// try_rebind!{(let (a, b, c), bar) = Ok(((10, 10, 10), 20))}
/// ```
/// `foo` in here is a new variable initialized with `10` (same for `a`, `b`, and `c`),
/// while `bar` is a pre-existing variable that is assigned `20`.
///
/// This pattern only works when destructuring tuples.
///
/// # Examples
///
/// ### Parsing
///
/// Inside of `parse_int_pair`, `parser` always refers to the same variable.
///
/// ```rust
/// use konst::{
///     parsing::{Parser, ParseValueResult},
///     try_rebind, unwrap_ctx,
/// };
///
/// const fn parse_int_pair(mut parser: Parser<'_>) -> ParseValueResult<'_, (u64, u64)> {
///     try_rebind!{(let aa, parser) = parser.parse_u64()}
///     try_rebind!{parser = parser.strip_prefix_u8(b',')}
///     try_rebind!{(let bb, parser) = parser.parse_u64()}
///     Ok(((aa, bb), parser))
/// }
///
/// const PAIR: (u64, u64) = {
///     let parser = Parser::from_str("100,200");
///     unwrap_ctx!(parse_int_pair(parser)).0
/// };
///
/// assert_eq!(PAIR, (100, 200));
///
/// ```
///
#[macro_export]
macro_rules! try_rebind {
    (
        $pattern:tt = $expression:expr $(, $($map_err:tt)* )?
    ) => {
        let tuple = match $expression {
            $crate::__::v::Ok(tuple) => tuple,
            $crate::__::v::Err(_e) => return $crate::__::v::Err(
                $crate::__priv_try_map_err!(_e $(, $($map_err)* )? )
            ),
        };
        $crate::__priv_ai_preprocess_pattern!( tuple, ($pattern) );
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __priv_try_map_err {
    ($error:ident, $(map_err = )? |$pat:pat| $map_err:expr ) => {{
        let $pat = $error;
        $map_err
    }};
    ($error:ident, $(map_err = )? $map_err:path ) => {{
        let $pat = $error;
        $map_err($error)
    }};
    ($error:ident $(,)?) => {
        $error
    };
}

///////////////////////////////////////////////////////////////////////////////

macro_rules! partial{
    (
        $new_macro:ident = $macro:ident ! ($($prefix:tt)*)
    ) => {
        __priv_partial!(
            ($)
            $new_macro = $macro ! ($($prefix)*)
        )
    };
}

macro_rules! __priv_partial {
    (
        ($_:tt)
        $new_macro:ident = $macro:ident ! ($($prefix:tt)*)
    ) => {
        macro_rules! $new_macro {
            ($_($_ args:tt)*) => {
                $macro!($($prefix)* $_($_ args)* )
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

macro_rules! try_parsing {
    ( $parser:ident, $parse_direction:ident $(,$ret:ident)*; $($code:tt)* ) => ({
        #![allow(unused_parens, unused_labels, unused_macros)]

        let copy = $parser;

        let ($($ret),*) = 'ret: loop {
            partial!{throw = throw_out!(copy, $parse_direction,)}
            partial!{ret_ = return_!('ret,)}

            #[allow(unreachable_code)]
            break {
                $($code)*
            };
        };

        update_offset!($parser, copy, $parse_direction);
        Ok(($($ret,)* $parser))
    })
}

macro_rules! parsing {
    ( $parser:ident, $parse_direction:ident $(,$ret:ident)*; $($code:tt)* ) => ({
        #![allow(unused_parens, unused_labels, unused_macros)]

        let copy = $parser;

        let ($($ret),*) = 'ret: loop {
            partial!{ret_ = return_!('ret,)}

            break {
                $($code)*
            };
        };

        update_offset!($parser, copy, $parse_direction);
        ($($ret,)* $parser)
    })
}

macro_rules! throw_out {
    ($copy:ident, $parse_direction:ident, $kind:expr) => {
        return Err(crate::parsing::ParseError::new(
            $copy,
            ParseDirection::$parse_direction,
            $kind,
        ));
    };
    ($copy:ident, $parse_direction:ident, $kind:expr, map_err = $func:ident) => {
        return Err($func(crate::parsing::ParseError::new(
            $copy,
            ParseDirection::$parse_direction,
            $kind,
        )));
    };
}
macro_rules! return_ {
    ($ret:lifetime, $($val:expr)?) => {{
        break $ret $($val)?
    }};
}
macro_rules! update_offset {
    ($parser:ident, $copy:ident, FromStart) => {
        $parser.start_offset += ($copy.bytes.len() - $parser.bytes.len()) as u32;
    };
    ($parser:ident, $copy:ident, FromEnd) => {
        $parser.end_offset -= ($copy.bytes.len() - $parser.bytes.len()) as u32;
    };
}
