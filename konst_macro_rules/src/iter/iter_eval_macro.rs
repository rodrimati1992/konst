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
    ($fixed:tt $item:ident for_each($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_for_each)
            ($fixed $item,)
            (for_each),
            $($closure)*
        }
    };
    ($fixed:tt $item:ident any($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_any)
            ($fixed $item,)
            (any),
            $($closure)*
        }
    };
    ($fixed:tt $item:ident all($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_all)
            ($fixed $item,)
            (all),
            $($closure)*
        }
    };
    // there's guaranteed to be an identifier for the method name,
    // so it is required to be either position or rposition.
    //
    // `rposition` reverses the iterator in `__cim_detect_rev_method`
    ($fixed:tt $item:ident $(position)? $(rposition)? ($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_position)
            ($fixed $item,)
            (position or rposition),
            $($closure)*
        }
    };
    ($fixed:tt $item:ident find_map ($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_find_map)
            ($fixed $item,)
            (find or rfind),
            $($closure)*
        }
    };
    ($fixed:tt $item:ident $(find)? $(rfind)? ($($closure:tt)*), $(,)* ) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__ie_find)
            ($fixed $item,)
            (find or rfind),
            $($closure)*
        }
    };
    ($fixed:tt $item:ident next($($args:tt)*), $(,)* ) => ({
        $crate::__cim_error_on_args!{next ($($args)*)}
        $crate::__ie_output!{
            $fixed
            { let mut next = $crate::__::None;}
            {
                next = $crate::__::Some($item);
                $crate::__ie_break!{$fixed}
            }
            { next }
        }
    });
    ($fixed:tt $item:ident $comb:ident $($rem:tt)*) => {
        $crate::__::compile_error! {$crate::__::concat!(
            "Unsupported iterator method: `",
            $crate::__::stringify!($comb),
            "`",
        )}
    };
    ($fixed:tt $item:ident $(,)*) => {
        $crate::__ie_output!{$fixed {} {} {}}
    };
    ($fixed:tt $item:ident $($rem:tt)*) => {
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
            {}
            {let $elem = $item; $value;}
            {}
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_any {
    ($fixed:tt $item:ident, |$elem:pat| $v:expr) => {
        $crate::__ie_output! {
            $fixed
            { let mut cond = false;}
            {
                let $elem = $item;
                if $v {
                    cond = true;
                    $crate::__ie_break!{$fixed}
                }
            }
            { cond }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_all {
    ($fixed:tt $item:ident, |$elem:pat| $v:expr) => {
        $crate::__ie_output! {
            $fixed
            { let mut cond = true; }
            {
                let $elem = $item;
                if !$v {
                    cond = false;
                    $crate::__ie_break!{$fixed}
                }
            }
            { cond }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_position {
    ($fixed:tt $item:ident, |$elem:pat| $v:expr) => {
        $crate::__ie_output! {
            $fixed
            {
                let mut position = $crate::__::None;
                let mut i = 0;
            }
            {
                let $elem = $item;
                if $v {
                    position = $crate::__::Some(i);
                    $crate::__ie_break!{$fixed}
                } else {
                    i += 1;
                }
            }
            { position }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_find_map {
    ($fixed:tt $item:ident, |$elem:pat| $v:expr) => {
        $crate::__ie_output! {
            $fixed
            {
                let mut val = $crate::__::None;
            }
            {
                let $elem = $item;
                val = $v;
                if let $crate::__::Some(_) = val {
                    $crate::__ie_break!{$fixed}
                }
            }
            { val }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ie_find {
    ($fixed:tt $item:ident, |$elem:pat| $v:expr) => {
        $crate::__ie_output! {
            $fixed
            {
                let mut val = $crate::__::None;
            }
            {
                let $elem = &$item;
                if $v {
                    val = $crate::__::Some($item);
                    $crate::__ie_break!{$fixed}
                }
            }
            { val }
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
                    $($iter_var:ident = $iter_expr:tt)?
                    {$($init:tt)*}
                    $($code:tt)*
                ))*
            )
        )
        {$($extra_init:tt)*}
        $each:tt
        $finish:tt
    ) => ({
        match ($($( $crate::into_iter_macro!($iter_expr) ,)?)*) {
            ($($(mut $iter_var,)?)*) => {
                $($($init)*)*
                $($extra_init)*
                $($label:)? loop {
                    $($($code)*)*
                    $each
                }
                $finish
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
