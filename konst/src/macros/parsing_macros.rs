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
