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
            (($($callback_macro)*) ($($fixed_arguments)*) ($label) $allowed_methods)
            (($($callback_macro)*) ($($fixed_arguments)*) ($label) $allowed_methods)
            $item
            (
                (
                    iter = $iter
                    {}
                    let $item = if let $crate::__::Some((elem_, next_)) = iter.next() {
                        iter = next_;
                        elem_
                    } else {
                        break;
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
                    break;
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
        $fixed:tt $fixedb:tt $item:ident ($($iters:tt)*)
        $(,)? filter($($args:tt)*) $($rem:tt)*
    ) => (
        $crate::__call_iter_methods!{
            $fixed
            $fixedb
            $item
            ( $($iters)* (
                {}
                if !$crate::__parse_closure!(
                    ($crate::__cim_filter) ($item,) (elem),
                    $($args)*
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
        ($macro:tt $prev_args:tt ($($label:lifetime)?) $allowed_methods:ident)
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
macro_rules! __cim_flat_map {
    (
        ($macro:tt $prev_args:tt $_label:tt $allowed_methods:ident)
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
            ($macro $prev_args () $allowed_methods)
            ($macro $prev_args () $allowed_methods)
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
