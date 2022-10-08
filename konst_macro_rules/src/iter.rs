mod combinator_methods;
mod iter_eval_macro;

#[macro_export]
macro_rules! for_each {
    ($pattern:pat in $($rem:tt)*) => ({
        $crate::__process_iter_args!{
            ($crate::__for_each)
            (($pattern),)
            (
                item,
                'zxe7hgbnjs,
                adapter,
            )
            $($rem)*
        }
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! __for_each {
    (
        @each
        ($pattern:pat),
        ($item:ident adapter),
        $(,)? => $($code:tt)*
    ) => ({
        let $pattern = $item;
        $($code)*
    });
    (@$other:ident $($tt:tt)*) =>{};
}

#[macro_export]
macro_rules! iter_count {
    ($iter:expr $(,)*) => {{
        let mut count = 0;
        $crate::for_each! {_ in $iter => {count+=1;}}
        count
    }};
}

macro_rules! make__cim_preprocess_methods__macro {
    (
        $_:tt
        [$(
            $fn:ident [$($next_fn:ident)?] $args:tt
            $(return $ret_var:tt)?
            $(var $new_var:tt)?,
        )*]
        [$(
            ($($st_func:ident)* ($($func_args:tt)*)) => { $var:ident = $var_expr:tt }
        )*]
        $($finished_arm:tt)*
    ) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __cim_preprocess_methods {
            $($finished_arm)*

            $(
                (
                    (($_($vars_after:tt)*) $_($fixed:tt)*)
                    [$prev_next_fn:ident $_($ignored:tt)*]

                    $func_:ident $_($fn)? $args,
                    $_($rest:tt)*
                ) => ({
                    $( $crate::__assert_first_rev!{$func_ $prev_next_fn $next_fn} )?

                    $crate::iter::__cim_preprocess_methods!{
                        (
                            (
                                $(return(rets = $ret_var))?
                                $_($vars_after)*
                                $((var = $new_var))?
                            )
                            $_($fixed)*
                        )
                        [$($next_fn)? $prev_next_fn]
                        $_($rest)*
                    }
                });
            )*

            $(
                (
                    (($_($vars_before:tt)*) $_($fixed:tt)*)
                    [$prev_next_fn:ident $_($ignored:tt)*]

                    $func_:ident $($_($st_func)?)* ($($func_args)*),
                    $_($rest:tt)*
                ) => ({
                    $crate::iter::__cim_preprocess_methods!{
                        (($_($vars_before)* ($var = $var_expr)) $_($fixed)*)
                        [$prev_next_fn]
                        $_($rest)*
                    }
                });
            )*


            (
                $fixed:tt
                $prev_next_fn:tt

                $func:ident $func2:ident ($_($args_:tt)*),
                $_($rest:tt)*
            ) => {
                $crate::__::compile_error!{$crate::__::concat!(
                    "unsupported iterator method: ",
                    $crate::__::stringify!($func),
                )}
            }
        }

        #[doc(hidden)]
        pub use __cim_preprocess_methods;
    };
}

make__cim_preprocess_methods__macro! {
    $
    [
        copied[] ($($args:tt)*),
        filter[] ($($args:tt)*),
        filter_map[] ($($args:tt)*),
        flat_map[] ($($args:tt)*),
        flatten[] ($($args:tt)*),
        map[] ($($args:tt)*),
        take_while[] ($($args:tt)*),
        rev[next_back] ($($args:tt)*),

        rfind[next_back] ($($args:tt)*)
            return($crate::__::None),

        all[] ($($args:tt)*)
            return(true),

        any[] ($($args:tt)*)
            return(false),

        count[] ($($args:tt)*)
            return(0usize),

        find[] ($($args:tt)*)
            return($crate::__::None),

        find_map[] ($($args:tt)*)
            return($crate::__::None),

        rfold[next_back] ($($args:tt)*)
            return($crate::__assert_fold_accum!(rfold, $($args)*)),

        fold[] ($($args:tt)*)
            return($crate::__assert_fold_accum!(fold, $($args)*)),

        for_each[] ($($args:tt)*),

        nth[] ($($args:tt)*)
            return($crate::__::None)
            var($crate::__cim_assert_expr!{nth($($args)*)}),

        next[] ($($args:tt)*)
            return($crate::__::None),

        position[] ($($args:tt)*)
            return($crate::__::None)
            var(0usize),

        rposition[next_back] ($($args:tt)*)
            return($crate::__::None)
            var(0usize),
    ]

    [
        ( zip($($args:tt)*) ) => {
            iter = (
                $crate::into_iter_macro!(
                    $crate::__cim_assert_expr!{zip($($args)*)}
                )
            )
        }

        ( enumerate($($args:tt)*) ) => {
            i = {
                $crate::__cim_error_on_args!{enumerate($($args)*)}
                0usize
            }
        }

        ( take skip ($($args:tt)*) ) => {
            rem = {
                let x: $crate::__::usize = $crate::__cim_assert_expr!{take($($args)*)};
                x
            }
        }

        ( skip_while ($($args:tt)*) ) => {
            still_skipping = true
        }
    ]

    (
        (
            (
                $(return($ret_var:ident = $ret_val:tt))?
                ($iter_var:ident = $iter_expr:expr);
                $(($var:ident = $var_value:expr))*
            )
            ($($callback_macro:tt)*)
            ($($fixed_arguments:tt)*)
            (
                $item:ident,
                $label:lifetime,
                // adapter: analogous to iterator adapter, which return iterators
                // consumer: methods which consume the iterator without (necessarily)
                //            returning an iterator.
                $allowed_methods:ident,
            )
            ($($args:tt)*)
        )
        [$next_fn:ident $($ignored:tt)*]
    ) => ({
        $(let mut $ret_var = $ret_val;)?
        $crate::__call_iter_methods!{
            (
                ($($var)* $($ret_var)?)
                ($($callback_macro)*) ($($fixed_arguments)*)
                ($label $label)
                $next_fn
                $allowed_methods
            )
            (
                ($($var)* $($ret_var)?)
                ($($callback_macro)*) ($($fixed_arguments)*)
                ($label $label)
                $next_fn
                $allowed_methods
            )
            $item
            (
                (
                    {
                        $iter_var = $iter_expr,
                        $($var = $var_value,)*
                    }
                    let $item = if let $crate::__::Some((elem_, next_)) = $iter_var.$next_fn() {
                        $iter_var = next_;
                        elem_
                    } else {
                        break $label;
                    };
                )
            )
            $($args)*
        }
        $($ret_var)?
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assert_first_rev {
    ($func:ident next_back next_back) => {
        $crate::__::compile_error! {$crate::__::concat!(
            "cannot call two iterator-reversing methods in `konst::iter` macros,",
            " called: ",
            $crate::__::stringify!($func),
        )}
    };
    ($func:ident $prev_next_fn:ident $($next_fn:ident)?) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assert_fold_accum {
    ($func:ident, $e:expr $(, $($__:tt)*)?) => { $e };
    ($func:ident, $($args:tt)*) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "expected expression as first argument to `",
            $crate::__::stringify!($func),
            "`, found: "
            $crate::__::stringify!($($args)*),
        )}
    };
}
