#[doc(hidden)]
#[macro_export]
macro_rules! __process_iter_args {
    (
        $callback_macro:tt
        $fixed_arguments:tt
        $other_args:tt
        $iter:expr $(, $method:ident ($($args:tt)*) )* $(,)*
        =>
        $($rem:tt)*
    ) => (
        $crate::iter::__cim_detect_rev_method !{
            (
                $callback_macro
                $fixed_arguments
                $other_args
                ( $iter )
                (
                    $($method($($args)*),)*
                    => $($rem)*
                )
            )

            next

            $($method $method,)*
        }
    );
    (
        $callback_macro:tt
        $fixed_arguments:tt
        $other_args:tt
        $iter:expr $(, $method:ident ($($args:tt)*) )* $(,)*
    ) => (
        $crate::iter::__cim_detect_rev_method !{
            (
                $callback_macro
                $fixed_arguments
                $other_args
                ($iter)
                ( $($method($($args)*),)* )
            )

            next

            $($method $method,)*
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __call_iter_methods {
    // this method is detected by `__cim_detect_rev_method`,
    // and causes the iterator method to change from next to next_back
    (
        $fixed:tt $fixedb:tt $item:ident $iters:tt
        rev($($args:tt)*), $($rem:tt)*
    ) => ({
        $crate::__cim_error_on_args!{rev($($args)*)}

        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            $iters
            $($rem)*
        }
    });
    (
        $fixed:tt
        ($macro:tt $prev_args:tt $label:tt $next_fn:tt $allowed_methods:ident)
        $item:ident
        ($($iters:tt)*)
        zip($iter:expr), $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixed
            $item
            ( $($iters)* (
                iter = $iter
                {/*init*/}

                let $item = if let $crate::__::Some((elem_, next_)) = iter.$next_fn() {
                    iter = next_;

                    ($item, elem_)
                } else {
                    $crate::__cim_break!{$fixed}
                };
            ))

            $($rem)*
        }
    );
    (
        $fixed:tt $fixedb:tt $item:ident ($($iters:tt)*)
        enumerate($($args:tt)*), $($rem:tt)*
    ) => ({
        $crate::__cim_error_on_args!{enumerate($($args)*)}

        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {let mut i = 0usize;}
                let $item = (i, $item);
                i+=1;
            ))
            $($rem)*
        }
    });
    (
        $fixed:tt
        $fixedb:tt
        $item:ident
        ($($iters:tt)*)
        take($amount:expr $(,)?), $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                { let mut rem: $crate::__::usize = $amount; }
                if rem == 0 {
                    $crate::__cim_break!{$fixed}
                } else {
                    rem -= 1;
                }
            ))
            $($rem)*
        }
    );
    (
        $fixed:tt $fixedb:tt $item:ident ($($iters:tt)*)
        take_while($($pred:tt)*), $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {}
                if !$crate::utils::__parse_closure_1!(
                    ($crate::__cim_filter) ($item,) (take_while),
                    $($pred)*
                ) {
                    $crate::__cim_break!{$fixed}
                }
            ))
            $($rem)*
        }
    );
    (
        $fixed:tt $fixedb:tt $item:ident ($($iters:tt)*)
        skip($amount:expr $(,)?), $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                { let mut rem: $crate::__::usize = $amount; }
                if rem != 0 {
                    rem -= 1;
                    continue;
                }
            ))
            $($rem)*
        }
    );
    (
        $fixed:tt $fixedb:tt $item:ident ($($iters:tt)*)
        skip_while($($pred:tt)*), $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {let mut still_skipping = true; }
                still_skipping = still_skipping && $crate::utils::__parse_closure_1!(
                    ($crate::__cim_filter) ($item,) (skip_while),
                    $($pred)*
                );

                if still_skipping {
                    continue;
                }
            ))
            $($rem)*
        }
    );
    (
        $fixed:tt $fixedb:tt $item:ident ($($iters:tt)*)
        filter($($pred:tt)*), $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {}
                if !$crate::utils::__parse_closure_1!(
                    ($crate::__cim_filter) ($item,) (filter),
                    $($pred)*
                ) {
                    continue;
                }
            ))
            $($rem)*
        }
    );
    (
        $fixed:tt $fixedb:tt $item:ident ($($iters:tt)*)
        filter_map($($args:tt)*), $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {}
                let $item =
                    match $crate::utils::__parse_closure_1!(
                        ($crate::__cim_map) ($item,) (filter_map),
                        $($args)*
                    ) {
                        $crate::__::Some(x) => x,
                        $crate::__::None => continue,
                    };
            ))
            $($rem)*
        }
    );
    (
        $fixed:tt $fixedb:tt $item:ident ($($iters:tt)*)
        map($($args:tt)*), $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {}
                let $item = $crate::utils::__parse_closure_1!(
                    ($crate::__cim_map) ($item,) (map),
                    $($args)*
                );
            ))
            $($rem)*
        }
    );
    (
        $fixed:tt $fixedb:tt $item:ident ($($iters:tt)*)
        copied($($args:tt)*), $($rem:tt)*
    ) => ({
        $crate::__cim_error_on_args!{copied($($args)*)}

        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {}
                let $item = *$item;
            ))
            $($rem)*
        }
    });
    (
        $fixed:tt $fixedb:tt $item:ident $iters:tt
        flat_map($($args:tt)*), $($rem:tt)*
    ) => {
        $crate::__cim_output_layer!{
            $fixed
            $item
            $iters
            {}
            {
                $crate::utils::__parse_closure_1!{
                    ($crate::__cim_flat_map) ($fixed $item ($($rem)*)) (flat_map),
                    $($args)*
                }
            }
            {}
        }
    };
    (
        $fixed:tt $fixedb:tt $item:ident $iters:tt
        flatten($($args:tt)*), $($rem:tt)*
    ) => ({
        $crate::__cim_error_on_args!{flatten($($args)*)}
        $crate::__cim_output_layer!{
            $fixed
            $item
            $iters
            {}
            {
                $crate::__cim_flat_map! {
                    $fixed
                    $item
                    ($($rem)*)
                    |elem| elem
                }
            }
            {}
        }
    });
    (
        $fixed:tt
        ($macro:tt $prev_args:tt $label:tt $next_fn:tt adapter)
        $item:tt $iters:tt
        $comb:ident ($($args:tt)*), $($rem:tt)*
    ) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "Unsupported iterator combinator: `",
            $crate::__::stringify!($comb),
            "`",
        )}
    };
    (
        $fixed:tt
        (($($macro:tt)*) ($($prev_args:tt)*) $label:tt $next_fn:tt consumer)
        $item:ident
        $iters:tt
        $($rem:tt)*
    ) => {
        $($macro)* ! {
            $($prev_args)*
            ($label $item $iters)
            $item
            $($rem)*
        }
    };
    (
        $fixed:tt
        (($($macro:tt)*) ($($prev_args:tt)*) $label:tt $next_fn:tt $allowed_methods:ident)
        $item:ident
        $iters:tt
        $($rem:tt)*
    ) => {
        $crate::__cim_output_layer!{
            $fixed
            $item
            $iters
            {  }
            { $($macro)* ! {@each $($prev_args)* ($item $allowed_methods), $($rem)*} }
            { $($macro)* ! {@end $($prev_args)* ($item $allowed_methods), $($rem)*} }
        }
    };
    ($($tt:tt)*) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "Unsupported arguments: ",
            $crate::__::stringify!($($tt)*),
        )}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cim_output_layer {
    (
        (
            $macro:tt
            $prev_args:tt
            ($break_label:lifetime $($label:lifetime)?)
            $next_fn:tt
            $allowed_methods:ident
        )
        $item:ident
        (
            $((
                $($iter_var:ident = $iter_expr:tt)?
                {$($init:tt)*}
                $($code:tt)*
            ))*
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
macro_rules! __cim_filter {
    ($item:ident, |$elem:pat| $v:expr) => {{
        let $elem = &$item;
        // avoiding lifetime extension
        let v = $v;
        v
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cim_map {
    ($item:ident, |$elem:pat| $v:expr) => {{
        let $elem = $item;
        // allowing for lifetime extension of temporaries
        $v
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cim_break {
    ((
        $macro:tt
        $prev_args:tt
        ($break_label:tt $($label:tt)?)
        $next_fn:tt
        $allowed_methods:ident
    )) => {
        break $break_label;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cim_flat_map {
    (
        (
            $macro:tt
            $prev_args:tt
            ($break_label:tt $($label:tt)?)
            $next_fn:tt
            $allowed_methods:ident
        )
        $item:ident
        ($($rem:tt)*)
        |$elem:pat| $v:expr
    ) => ({
        let $elem = $item;
        // allowing for lifetime extension of temporaries
        let v = $v;

        $crate::__call_iter_methods!{
            ($macro $prev_args ($break_label) $next_fn $allowed_methods)
            ($macro $prev_args ($break_label) $next_fn $allowed_methods)
            $item
            (
                (
                    iter = v
                    {}
                    let $item = if let $crate::__::Some((elem_, next_)) = iter.$next_fn() {
                        iter = next_;
                        elem_
                    } else {
                        break;
                    };
                )
            )
            $($rem)*
        }
    });

}

#[doc(hidden)]
#[macro_export]
macro_rules! __cim_error_on_args {
    ($func:ident()) => ();
    ($func:ident ($($args:tt)*)) => {
        $crate::__::compile_error!{$crate::__::concat!{
            "`",
            $crate::__::stringify!($func),
            "` does not take arguments, passed: ",
            $crate::__::stringify!($($args)*),
        }}
    };
}
