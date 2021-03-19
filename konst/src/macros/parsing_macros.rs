/// Assigns the value in a `Some`/`Ok`/`Err` variant into pre-existing variables.
///
/// You can opt out of assigning into pre-existing variables,
/// to define a variable scoped to this macro, by writing `let <pattern>`.
#[macro_export]
macro_rules! assign_if {
    (
        $variant:ident ($($pattern:tt)*) = $expression:expr
        $( => $( $then:block )? $( else $else:block )? )?
    ) => {
        if let $crate::__::v::$variant(tuple) = $expression {
            $crate::__priv_ai_preprocess_pattern!( tuple, $($pattern)* );
            $($($then)?)?
        } $( $( else  $else )? )?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_ai_preprocess_pattern {
    ( $var:ident, ($($pat:tt)*)) => {
        $crate::__priv_assign_tuple!($var.0 , $($pat)*)
    };
    ( $var:ident, $($pat:tt)*) => {
        $crate::__priv_assign_tuple!($var.0 , $($pat)*)
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __priv_assign_tuple {
    ($var:ident . $field:tt, let $pat:pat $(, $($rem:tt)*)?) => {
        $crate::__priv_next_ai_access!(
            (let $pat) $var . $field , $($($rem)*)?
        )
    };
    ($var:ident . $field:tt, _ $(, $($rem:tt)*)?) => {
        $crate::__priv_next_ai_access!(
            (let _) $var . $field , $($($rem)*)?
        )
    };
    ($var:ident . $field:tt, $e:expr $(, $($rem:tt)*)?) => {
        $crate::__priv_next_ai_access!(
            ($e) $var . $field , $($($rem)*)?
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_next_ai_access {
    ( ($($lhs:tt)*) $var:ident . 0,  ) => {
        $($lhs)* = $var;
    };
    ( ($($lhs:tt)*) $var:ident . $field:tt,  ) => {
        $($lhs)* = $var. $field;
    };
    ( ($($lhs:tt)*) $var:ident . 0, $($rem:tt)+ ) => {
        $($lhs)* = $var.0;
        $crate::__priv_assign_tuple!($var.1 , $($rem)+)
    };
    ( ($($lhs:tt)*) $var:ident . 1, $($rem:tt)+ ) => {
        $($lhs)* = $var.1;
        $crate::__priv_assign_tuple!($var.2 , $($rem)+)
    };
}

///////////////////////////////////////////////////////////////////////////////

macro_rules! partial{
    (
        $new_macro:ident = $macro:ident ! ($($prefix:tt)*)
    ) => {
        $crate::__priv_partial!(
            ($)
            $new_macro = $macro ! ($($prefix)*)
        )
    };
}

#[macro_export]
#[doc(hidden)]
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

#[doc(hidden)]
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

#[doc(hidden)]
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
    ($copy:ident, $parse_direction:ident,) => {
        return Err(ParseError::new($copy, ParseDirection::$parse_direction));
    };
    ($copy:ident, $parse_direction:ident, map_err = $func:ident) => {
        return Err($func(ParseError::new(
            $copy,
            ParseDirection::$parse_direction,
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
