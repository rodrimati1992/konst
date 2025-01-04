#[cfg(test)]
mod utils_tests;

pub struct TypeAnnot<T> {
    pub val: T,
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_2 {
    (
        $macro:tt $args:tt $usage_site:tt,
        ||  $(,)?
    ) => {
        $crate::__parse_closure_no_expr_error!{$usage_site}
    };
    (
        $macro:tt $args:tt $usage_site:tt,
        |$pat1:tt $(: $pat1_ty:ty)?, $pat2:tt $(: $pat2_ty:ty)? $(,)?| $($rem:tt)*
    ) => (
        $crate::__parse_closure_expr! {
            $usage_site $macro $args
            (
                ($crate::__unparen_pat!($pat1), $crate::__unparen_pat!($pat2)):
                ($crate::__ty_or_und!($($pat1_ty)?), $crate::__ty_or_und!($($pat2_ty)?))
            ),
            $($rem)*
        }
    );
    (
        $macro:tt $args:tt $usage_site:tt,
        |$pat1:pat_param, $pat2:pat_param $(,)?| $($rem:tt)*
    ) => (
        $crate::__parse_closure_expr! {
            $usage_site $macro $args (($pat1, $pat2)),
            $($rem)*
        }
    );
    ($macro:tt $args:tt $usage_site:tt, | $($anything:tt)* ) => {
        $crate::__parse_closure_emit_error!{2 $usage_site}
    };
    ($macro:tt $args:tt $usage_site:tt, || $($anything:tt)* ) => {
        $crate::__parse_closure_emit_error!{2 $usage_site}
    };
    (
        ($($macro:tt)*) ($($args:tt)*) $usage_site:tt,
        $v:expr $(,)?
    ) => {
        match $v {func => {
            $($macro)* ! {
                $($args)*
                ((__x, __y)) -> _ {func(__x, __y)}
            }
        }}
    };
    ($macro:tt $args:tt $usage_site:tt $($rem:tt)*) => {
        $crate::__parse_closure_emit_error!{2 $usage_site}
    };

    ( $($anything:tt)* ) => {
        $crate::__::compile_error!("expected a closure argument")
    };
}

pub use __parse_closure_2;

////////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_1 {
    (
        $macro:tt $args:tt $usage_site:tt,
        ||  $(,)?
    ) => {
        $crate::__parse_closure_no_expr_error!{$usage_site}
    };
    (
        $macro:tt $args:tt $usage_site:tt,
        |$pat:tt $(: $pat_ty:ty)? $(,)?| $($rem:tt)*
    ) => (
        $crate::__parse_closure_expr! {
            $usage_site $macro $args ($pat $(: $pat_ty)?),
            $($rem)*
        }
    );
    (
        $macro:tt $args:tt $usage_site:tt,
        |$pat:pat_param $(,)?| $($rem:tt)*
    ) => (
        $crate::__parse_closure_expr! {
            $usage_site $macro $args ($pat),
            $($rem)*
        }
    );
    ($macro:tt $args:tt $usage_site:tt, | $($anything:tt)* ) => {
        $crate::__parse_closure_emit_error!{1 $usage_site}
    };
    ($macro:tt $args:tt $usage_site:tt, || $($anything:tt)* ) => {
        $crate::__parse_closure_emit_error!{1 $usage_site}
    };
    (
        ($($macro:tt)*) ($($args:tt)*) $usage_site:tt,
        $v:expr $(,)?
    ) => {
        match $v {func => {
            $($macro)* ! {
                $($args)*
                (__x) -> _ {func(__x)}
            }
        }}
    };
    ($macro:tt $args:tt $usage_site:tt $($rem:tt)*) => {
        $crate::__parse_closure_emit_error!{1 $usage_site}
    };

    ( $($anything:tt)* ) => {
        $crate::__::compile_error!("expected a closure argument")
    };
}

pub use __parse_closure_1;

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_expr {
    (
        $usage_site:tt ($($macro:tt)*) ($($args:tt)*) ($($pattern:tt)*),
        $(,)?
    ) => {
        $crate::__parse_closure_no_expr_error!{$usage_site}
    };

    (
        $usage_site:tt ($($macro:tt)*) ($($args:tt)*) $pattern:tt,
        $v:expr $(, $($rem:expr $(, $($rem_tt:tt)*)? )? )?
    ) => ({
        $($(
            $crate::__parse_closure_trailing_expr_error!{$usage_site $rem}
        )?)?

        $($macro)* ! {
            $($args)*
            $pattern { $v }
        }
    });

    (
        $usage_site:tt
        ($($macro:tt)*) ($($args:tt)*) $pattern:tt,
        -> $ret_ty:ty $v:block $(, $($rem:expr $(, $($rem_tt:tt)*)? )? )?
    ) => ({
        $($(
            $crate::__parse_closure_trailing_expr_error!{$usage_site $rem}
        )?)?

        $($macro)* ! {
            $($args)*
            $pattern -> $ret_ty { $v }
        }
    });
}

////////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_no_expr_error {
    (($($usage_site:tt)*)) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "`",
            $crate::__::stringify!($($usage_site)*),
            "` expects valid closure syntax, passed closure without return value",
        )}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_trailing_expr_error {
    (($($usage_site:tt)*) $($rem:tt)*) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "`",
            $crate::__::stringify!($($usage_site)*),
            "` expects no arguments after closure argument",
        )}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_emit_error {
    ($count:tt ($($usage_site:tt)*)) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "`",
            $crate::__::stringify!($($usage_site)*),
            "` expects to be passed a ",
            $count,
            "-parameter closure",
        )}
    };
}
