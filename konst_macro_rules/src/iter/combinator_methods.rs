#[doc(hidden)]
#[macro_export]
macro_rules! __process_iter_args {
    (
        ($($callback_macro:tt)*)
        ($($fixed_arguments:tt)*)
        (
            $item:ident,
            $label:lifetime,
            // adapters: analogous to iterator adapters, which return iterators
            // consumers: methods which consume the iterator without (necessarily)
            //            returning an iterator.
            $allowed_methods:ident,
        )
        $iter:expr,
        $($args:tt)*
    ) => (
        $crate::__call_iter_methods!{
            (($($callback_macro)*) ($($fixed_arguments)*) ($label $label) $allowed_methods)
            (($($callback_macro)*) ($($fixed_arguments)*) ($label $label) $allowed_methods)
            $item
            (
                (
                    iter = $iter
                    {}
                    let $item = if let $crate::__::Some((elem_, next_)) = iter.next() {
                        iter = next_;
                        elem_
                    } else {
                        break $label;
                    };
                )
            )
            $($args)*
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __call_iter_methods {
    (
        $fixed:tt $fixedb:tt $item:ident ($($iters:tt)*)
        $(,)? zip($iter:expr) $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                iter = $iter
                {/*init*/}

                let $item = if let $crate::__::Some((elem_, next_)) = iter.next() {
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
        $(,)? enumerate() $($rem:tt)*
    ) => (
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
    );
    (
        $fixed:tt
        $fixedb:tt
        $item:ident
        ($($iters:tt)*)
        $(,)? take($amount:expr $(,)?) $($rem:tt)*
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
        $(,)? take_while($($pred:tt)*) $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {}
                if !$crate::__parse_closure!(
                    ($crate::__cim_filter) ($item,) (elem),
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
        $(,)? skip($amount:expr $(,)?) $($rem:tt)*
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
        $(,)? skip_while($($pred:tt)*) $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {let mut still_skipping = true; }
                still_skipping = still_skipping && $crate::__parse_closure!(
                    ($crate::__cim_filter) ($item,) (elem),
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
        $(,)? filter($($pred:tt)*) $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {}
                if !$crate::__parse_closure!(
                    ($crate::__cim_filter) ($item,) (elem),
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
        $(,)? filter_map($($args:tt)*) $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {}
                let $item =
                    match $crate::__parse_closure!(
                        ($crate::__cim_map) ($item,) (elem),
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
        $(,)? map($($args:tt)*) $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {}
                let $item = $crate::__parse_closure!(
                    ($crate::__cim_map) ($item,) (elem),
                    $($args)*
                );
            ))
            $($rem)*
        }
    );
    (
        $fixed:tt $fixedb:tt $item:ident ($($iters:tt)*)
        $(,)? copied() $($rem:tt)*
    ) => (
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
    );
    (
        $fixed:tt $fixedb:tt $item:ident $iters:tt
        $(,)? flat_map($($args:tt)*) $($rem:tt)*
    ) => {
        $crate::__cim_output_layer!{
            $fixed
            $item
            $iters
            {
                $crate::__parse_closure!{
                    ($crate::__cim_flat_map) ($fixed $item ($($rem)*)) (elem),
                    $($args)*
                }
            }
            {}
        }
    };
    (
        $fixed:tt $fixedb:tt $item:ident $iters:tt
        $(,)? flatten() $($rem:tt)*
    ) => {
        $crate::__cim_output_layer!{
            $fixed
            $item
            $iters
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
    };
    (
        $fixed:tt
        ($macro:tt $prev_args:tt $label:tt adapters)
        $item:tt $iters:tt
        $(,)? $comb:ident $($rem:tt)*
    ) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "Unsupported iterator combinator: `",
            $crate::__::stringify!($comb),
            "`",
        )}
    };
    (
        $fixed:tt
        ($macro:tt $prev_args:tt $label:tt consumers)
        $($rem:tt)*
    ) => {
        $crate::__::__call_iter_consumer_methods!{
            $fixed $fixed $($rem:tt)*
        }
    };
    (
        $fixed:tt
        (($($macro:tt)*) ($($prev_args:tt)*) $label:tt $allowed_methods:ident)
        $item:ident
        $iters:tt
        $($rem:tt)*
    ) => {
        $crate::__cim_output_layer!{
            $fixed
            $item
            $iters
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
macro_rules! __call_iter_consumer_methods {
    (
        $fixed:tt $fixedb:tt $item:tt $iters:tt
        $(,)? $comb:ident $($rem:tt)*
    ) => {
        $crate::__::compile_error! {$crate::__::concat!(
            "Unsupported iterator method: `",
            $crate::__::stringify!($comb),
            "`",
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
        $each:tt
        $finish:tt
    ) => ({
        match ($($( $crate::into_iter_macro!($iter_expr) ,)?)*) {
            ($($(mut $iter_var,)?)*) => {
                $($($init)*)*
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
        // avoiding lifetime extension
        let v = $v;
        v
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cim_break {
    ((
        $macro:tt
        $prev_args:tt
        ($break_label:tt $($label:tt)?)
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
            $allowed_methods:ident
        )
        $item:ident
        ($($rem:tt)*)
        |$elem:pat| $v:expr
    ) => ({
        let $elem = $item;
        let v = {
            // avoiding lifetime extension
            let v = $v;
            v
        };

        $crate::__call_iter_methods!{
            ($macro $prev_args ($break_label) $allowed_methods)
            ($macro $prev_args ($break_label) $allowed_methods)
            $item
            (
                (
                    iter = v
                    {}
                    let $item = if let $crate::__::Some((elem_, next_)) = iter.next() {
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
