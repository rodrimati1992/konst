#[macro_export]
macro_rules! iter_eval {
    ($iter:expr $(, $($rem:tt)*)?) => {
        $crate::__process_iter_args!{
            ($crate::__iter_eval)
            ()
            (item, 'zxe7hgbnjs, consumer,)
            $iter, $($($rem)*)?
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter_eval {
    ($fixed:tt () $item:ident for_each($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_for_each)
            ($fixed $item,)
            (for_each),
            $($closure)*
        }
    };
    ($fixed:tt $vars:tt $item:ident any($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_any)
            ($fixed $vars $item,)
            (any),
            $($closure)*
        }
    };
    ($fixed:tt $vars:tt $item:ident all($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_all)
            ($fixed $vars $item,)
            (all),
            $($closure)*
        }
    };
    ($fixed:tt ($var:ident) $item:ident count($($args:tt)*), $(,)* ) => ({
        $crate::__cim_error_on_args!{count ($($args)*)}

        $crate::__ie_output!{
            $fixed
            { $var += 1; }
        }
    });
    // there's guaranteed to be an identifier for the method name,
    // so it is required to be either position or rposition.
    //
    // `rposition` reverses the iterator in `__cim_preprocess_methods`
    ($fixed:tt $vars:tt $item:ident $(position)? $(rposition)? ($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_position)
            ($fixed $vars $item,)
            (position, rposition),
            $($closure)*
        }
    };
    ($fixed:tt $vars:tt $item:ident find_map ($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_find_map)
            ($fixed $vars $item,)
            (find_map),
            $($closure)*
        }
    };
    ($fixed:tt $vars:tt $item:ident $(find)? $(rfind)? ($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_find)
            ($fixed $vars $item,)
            (find, rfind),
            $($closure)*
        }
    };
    ($fixed:tt $vars:tt $item:ident $(fold)? $(rfold)? ($accum:expr, $($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_2!{
            ($crate::__ie_fold)
            ($fixed $vars $item,)
            (fold, rfold),
            $($closure)*
        }
    };
    ($fixed:tt () $item:ident $(fold)? $(rfold)? ($($args:tt)*), $(,)* ) => {
        $crate::__::compile_error! {"fold/rfold methods expect 2 arguments"}
    };
    ($fixed:tt ($var:ident) $item:ident next($($args:tt)*), $(,)* ) => ({
        $crate::__cim_error_on_args!{next ($($args)*)}
        $crate::__ie_output!{
            $fixed
            {
                $var = $crate::__::Some($item);
                $crate::__ie_break!{$fixed}
            }
        }
    });
    ($fixed:tt ($nth:ident $ret:ident) $item:ident nth($($args:tt)*), $(,)* ) => ({
        $crate::__ie_output!{
            $fixed
            {
                let _: $crate::__::usize = $nth;
                if $nth == 0 {
                    $ret = $crate::__::Some($item);
                    $crate::__ie_break!{$fixed}
                } else {
                    $nth -= 1;
                }
            }
        }
    });
    ($fixed:tt () $item:ident nth($($args:tt)*), $(,)* ) => {
        $crate::__::compile_error! {"nth expects 1 argument"}
    };
    ($fixed:tt () $item:ident $comb:ident $($rem:tt)*) => {
        $crate::__::compile_error! {$crate::__::concat!(
            "Unsupported iterator method: `",
            $crate::__::stringify!($comb),
            "`",
        )}
    };
    ($fixed:tt () $item:ident $(,)*) => {
        $crate::__ie_output!{$fixed {}}
    };
    ($fixed:tt () $item:ident $($rem:tt)*) => {
        $crate::__::compile_error! {$crate::__::concat!(
            "Unsupported trailing syntax: `",
            $crate::__::stringify!($($rem)*),
            "`",
        )}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_for_each {
    ($fixed:tt $item:ident, |$elem:pat| $value:expr) => {
        $crate::__ie_output! {
            $fixed
            {let $elem = $item; $value;}
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_any {
    ($fixed:tt ($cond:ident) $item:ident, |$elem:pat| $v:expr) => {
        $crate::__ie_output! {
            $fixed
            {
                let $elem = $item;
                if $v {
                    $cond = true;
                    $crate::__ie_break!{$fixed}
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_all {
    ($fixed:tt ($cond:ident) $item:ident, |$elem:pat| $v:expr) => {
        $crate::__ie_output! {
            $fixed
            {
                let $elem = $item;
                if !$v {
                    $cond = false;
                    $crate::__ie_break!{$fixed}
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_position {
    ($fixed:tt ($i:ident $position:ident) $item:ident, |$elem:pat| $v:expr) => {
        $crate::__ie_output! {
            $fixed
            {
                let $elem = $item;
                if $v {
                    $position = $crate::__::Some($i);
                    $crate::__ie_break!{$fixed}
                } else {
                    $i += 1;
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_find_map {
    ($fixed:tt ($ret:ident) $item:ident, |$elem:pat| $v:expr) => {
        $crate::__ie_output! {
            $fixed
            {
                let $elem = $item;
                $ret = $v;
                if let $crate::__::Some(_) = $ret {
                    $crate::__ie_break!{$fixed}
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_find {
    ($fixed:tt ($ret:ident) $item:ident, |$elem:pat| $v:expr) => {
        $crate::__ie_output! {
            $fixed
            {
                let $elem = &$item;
                if $v {
                    $ret = $crate::__::Some($item);
                    $crate::__ie_break!{$fixed}
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_fold {
    ($fixed:tt ($accum:ident) $item:ident, |$accum_pat:pat, $elem:pat| $v:expr) => {
        $crate::__ie_output! {
            $fixed
            {
                let $accum_pat = $accum;
                let $elem = $item;
                $accum = $v;
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_output {
    (
        (
            ($break_label:tt $($label:tt)?)
            $item:ident
            (
                $((
                    {$($var_a:ident = $var_a_expr:expr),* $(,)?}
                    $($code:tt)*
                ))*
            )
        )
        $each:tt
    ) => ({
        match ($($( $var_a_expr,)?)*) {
            ($($(mut $var_a,)?)*) => {
                $($label:)? loop {
                    $($($code)*)*
                    $each
                }
            },
        }
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_break {
    (
        (
            ($break_label:tt $($label:tt)?)
            $item:ident
            $iter:tt
        )
        $($break_with:expr)?
    ) => {
        break $break_label $($break_with)?;
    };
}
