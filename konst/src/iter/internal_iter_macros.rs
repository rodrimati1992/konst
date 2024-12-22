use crate::iter::ConstIntoIter;

use core::marker::PhantomData;

#[doc(hidden)]
#[macro_export]
macro_rules! iterator_shared {
    (
        is_forward = $is_forward:ident,
        $(is_copy = $is_copy:ident,)?
        item = $Item:ty,
        iter_forward = $Self:ty,
        $(iter_reversed = $Rev:path,)?
        next($self:ident) $next_block:block,
        $(next_back $next_back_block:block,)?
        fields = $fields:tt,
    ) => {
        $crate::__::__choose_alt! {($($is_copy)? true) {
            /// Creates a clone of this iterator
            pub const fn copy(&self) -> Self {
                let Self $fields = *self;
                Self $fields
            }
        }}

        $(
            /// Reverses the iterator
            pub const fn rev(self) -> $crate::__::__choose!($is_forward $Rev $Self) {
                let Self $fields = self;
                type Type<T> = T;
                Type::<$crate::__::__choose!($is_forward $Rev $Self)> $fields
            }
        )?

        /// Advances the iterator and returns the next value.
        #[track_caller]
        pub const fn next(&mut $self) -> Option<$Item> {
            $crate::__::__choose!{
                $is_forward
                $next_block
                $($next_back_block)?
            }
        }

        $(
            /// Removes and returns an element from the end of the iterator.
            #[track_caller]
            pub const fn next_back(&mut $self) -> Option<$Item> {
                $crate::__::__choose!{
                    $is_forward
                    $next_back_block
                    $next_block
                }
            }
        )?
    };
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
        macro_rules! __cim_method_not_found_err {
            ($func:ident $($_($fn)?)* $($($_($st_func)?)*)*) => {
                $crate::__::compile_error! {$crate::__::concat!{
                    "the `",
                    $crate::__::stringify!($func),
                    "` method cannot be called in this macro",
                }}
            };
            ($func:ident $func2:ident) => {
                $crate::__::compile_error!{$crate::__::concat!(
                    "Unsupported iterator method: `",
                    $crate::__::stringify!($func),
                    "`",
                )}
            };
        }

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

        #[doc(hidden)]
        pub use __cim_method_not_found_err;
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
            var({
                let x: $crate::__::usize = $crate::__cim_assert_expr!{nth($($args)*), 0usize};
                x
            }),

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
                $crate::iter::into_iter!(
                    $crate::__cim_assert_expr!{zip($($args)*), 0usize..0}
                )
            )
        }

        ( enumerate($($args:tt)*) ) => {
            i = { 0usize }
        }

        ( take skip ($($args:tt)*) ) => {
            rem = {
                let x: $crate::__::usize = $crate::__cim_assert_expr!{take($($args)*), 0};
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

                    let elem_phantom_ty = $crate::iter::__get_item_ty(&$iter_var);
                    let $item = if let $crate::__::Some(elem_) = $iter_var.$next_fn() {
                        $crate::iter::__assert_item_ty(&elem_, elem_phantom_ty);
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
macro_rules! __cim_assert_expr {
    ($func:ident (), $def:expr) => {{
        $crate::__cim_no_expr_arg_error! {$func ()}
        $def
    }};
    ($func:ident ( $expr:expr $(,)?), $def:expr) => {
        $expr
    };
    ($func:ident ($expr:expr ,$($args:tt)+), $def:expr) => {{
        $crate::__::compile_error! {$crate::__::concat!{
            "`",
            $crate::__::stringify!($func),
            "` only takes one argument"
        }}

        $def
    }};
    ($func:ident $args:tt, $def:expr) => {{
        $crate::__cim_no_expr_arg_error! {$func $args}
        $def
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cim_no_expr_arg_error {
    ($func:ident $args:tt) => {
        $crate::__::compile_error! {$crate::__::concat!{
            "`",
            $crate::__::stringify!($func),
            "` expected an expression to be passed, passed: ",
            $crate::__cim_if_empty!{
                $args {
                    "``"
                } else {
                    $crate::__::stringify! $args
                }
            },
        }}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cim_if_empty {
    (() {$($then:tt)*} else $else:tt) => { $($then)* };
    (($($non_empty:tt)+) $then:tt else {$($else:tt)*}) => { $($else)* };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __assert_fold_accum {
    ($func:ident, $e:expr $(, $($__:tt)*)?) => {
        $e
    };
    // dummy default value, this'll error in __iter_eval
    ($func:ident, $($args:tt)*) => {
        ()
    };
}

#[doc(hidden)]
#[inline(always)]
pub const fn __get_item_ty<Iter>(_: &Iter) -> PhantomData<(Iter, Iter::Item)>
where
    Iter: ConstIntoIter,
{
    PhantomData
}

#[doc(hidden)]
#[inline(always)]
pub const fn __assert_item_ty<Iter>(_: &Iter::Item, _: PhantomData<(Iter, Iter::Item)>)
where
    Iter: ConstIntoIter,
{
}
