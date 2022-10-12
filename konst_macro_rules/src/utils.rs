#[doc(hidden)]
#[repr(C)]
pub union Dereference<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}

macro_rules! make_parse_closure_macro {
    ($_:tt $macro_name:ident $arg_count:tt ($($pat_var:ident)*)) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! $macro_name {
            (
                $macro:tt $args:tt $usage_site:tt,
                |$_($pat_var_:pat_param),* $_(,)?|  $_(,)?
            ) => {
                $crate::__parse_closure_no_expr_error!{$usage_site}
            };
            (
                ($_($macro:tt)*) ($_($args:tt)*)
                $usage_site:tt,
                |$($_$pat_var:pat_param),* $_(,)?| $v:expr $_(,)*
            ) => {
                $_($macro)* ! {
                    $_($args)*
                    |$($_$pat_var),*| $v
                }
            };
            (
                $macro:tt $args:tt $usage_site:tt,
                |$_($pat_var_:pat_param),* $_(,)?| $v:expr,
                $trailing_expr:expr $_(, $_($rem:tt)*)?
            ) => {
                $crate::__parse_closure_trailing_expr_error!{$usage_site}
            };
            ($macro:tt $args:tt $usage_site:tt, | $_($anything:tt)* ) => {
                $crate::__parse_closure_emit_error!{
                    $arg_count
                    $usage_site
                }
            };
            ($macro:tt $args:tt $usage_site:tt, || $_($anything:tt)* ) => {
                $crate::__parse_closure_emit_error!{
                    $arg_count
                    $usage_site
                }
            };
            (
                ($_($macro:tt)*) ($_($args:tt)*)
                $usage_site:tt, $v:expr $_(,)?
            ) => {
                match $v {func => {
                    $_($macro)* ! {
                        $_($args)*
                        |$($pat_var),*| func($($pat_var),*)
                    }
                }}
            };
            ($macro:tt $args:tt $usage_site:tt $_($rem:tt)*) => {
                $crate::__parse_closure_emit_error!{
                    $arg_count
                    $usage_site
                }
            };

            ( $_($anything:tt)* ) => {
                $crate::__::compile_error!("expected a closure argument")
            };
        }

        pub use $macro_name;
    }
}

make_parse_closure_macro! { $ __parse_closure_1 1 (aaa) }
make_parse_closure_macro! { $ __parse_closure_2 2 (aaa bbb) }

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
    (($($usage_site:tt)*)) => {
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
