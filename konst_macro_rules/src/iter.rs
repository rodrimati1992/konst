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
macro_rules! iter_position {
    ($iter:expr, $($closure:tt)*) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__iter_position) ($iter, next,) (position),
            $($closure)*
        }
    }
}

#[macro_export]
macro_rules! iter_rposition {
    ($iter:expr, $($closure:tt)*) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__iter_position) ($iter, next_back,) (rposition),
            $($closure)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter_position {
    ($iter:expr, $next_fn:ident, |$elem:pat| $v:expr) => {
        match $crate::into_iter_macro!($iter) {
            mut iter => {
                let mut i = 0;
                loop {
                    match iter.$next_fn() {
                        $crate::__::Some((elem, next)) => {
                            let $elem = elem;
                            if $v {
                                break $crate::__::Some(i);
                            }
                            iter = next;
                            i += 1;
                        }
                        $crate::__::None => break $crate::__::None,
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! iter_find {
    ($iter:expr, $($closure:tt)*) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__iter_find) ($iter, next,) (find),
            $($closure)*
        }
    }
}

#[macro_export]
macro_rules! iter_rfind {
    ($iter:expr, $($closure:tt)*) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__iter_find) ($iter, next_back,) (rfind),
            $($closure)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter_find {
    (
        $iter:expr,
        $next_fn:ident,
        |$elem:pat| $v:expr
    ) => {
        match $crate::into_iter_macro!($iter) {
            mut iter => loop {
                match iter.$next_fn() {
                    $crate::__::Some((elem, next)) => {
                        let $elem = &elem;
                        if $v {
                            break $crate::__::Some(elem);
                        }
                        iter = next;
                    }
                    $crate::__::None => break $crate::__::None,
                }
            },
        }
    };
}

#[macro_export]
macro_rules! iter_count {
    ($iter:expr $(,)*) => {{
        let mut count = 0;
        $crate::for_each! {_ in $iter => {count+=1;}}
        count
    }};
}

#[macro_export]
macro_rules! iter_nth {
    ($iter:expr, $nth:expr $(,)*) => {{
        match ($crate::into_iter_macro!($iter), $nth) {
            (mut iter, nth) => {
                let mut n = 0;
                loop {
                    match iter.next() {
                        $crate::__::Some((elem, next)) => {
                            if n == nth {
                                break $crate::__::Some(elem);
                            } else {
                                iter = next;
                                n += 1;
                            }
                        }
                        $crate::__::None => break $crate::__::None,
                    }
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! iter_rfold {
    ($iter:expr, $accum:expr, $($closure:tt)*) => {
        $crate::__parse_closure_2!{
            ($crate::__iter_fold) ($iter, next_back, $accum,) (rfold),
            $($closure)*
        }
    }
}

#[macro_export]
macro_rules! iter_fold {
    ($iter:expr, $accum:expr, $($closure:tt)*) => {
        $crate::__parse_closure_2!{
            ($crate::__iter_fold) ($iter, next, $accum,) (fold),
            $($closure)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter_fold {
    ($iter:expr, $next_fn:ident, $accum:expr, |$($accum_pat:pat)? $(,)*| $v:expr) => {
        $crate::__::compile_error!("expected a two-argument closure")
    };
    ($iter:expr, $next_fn:ident, $accum:expr, |$accum_pat:pat, $elem:pat| $v:expr) => {
        match ($crate::into_iter_macro!($iter), $accum) {
            (mut iter, mut accum) => loop {
                match iter.$next_fn() {
                    $crate::__::Some((elem, next)) => {
                        iter = next;
                        let $elem = elem;
                        let $accum_pat = accum;
                        accum = $v;
                    }
                    $crate::__::None => break accum,
                }
            },
        }
    };
}

#[macro_export]
macro_rules! iter_find_map {
    ($iter:expr, $($closure:tt)*) => {
        $crate::utils::__parse_closure_1!{
            ($crate::__iter_find_map) ($iter,) (find_map),
            $($closure)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter_find_map {
    ($iter:expr, |$elem:pat| $v:expr) => {
        match $crate::into_iter_macro!($iter) {
            mut iter => loop {
                match iter.next() {
                    $crate::__::Some((elem, next)) => {
                        let $elem = elem;
                        if let opt @ $crate::__::Some(_) = $v {
                            break opt;
                        }
                        iter = next;
                    }
                    $crate::__::None => break $crate::__::None,
                }
            },
        }
    };
}

macro_rules! make__cim_detect_rev_method__macro {
    (
        $_:tt
        [$($fn:ident,)*]
        [$($reversing_fn:ident,)*]
        $($finished_arm:tt)*
    ) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __cim_detect_rev_method {
            $($finished_arm)*
            (
                $fixed:tt
                $next_fn:ident

                $func:ident $($_($fn)?)* ,
                $_($rest:tt)*
            ) => {
                $crate::iter::__cim_detect_rev_method!{
                    $fixed
                    $next_fn
                    $_($rest)*
                }
            };
            (
                $fixed:tt
                next

                $func:ident $($_($reversing_fn)?)* ,
                $_($rest:tt)*
            ) => {
                $crate::iter::__cim_detect_rev_method!{
                    $fixed
                    next_back
                    $_($rest)*
                }
            };
            (
                $fixed:tt
                next_back

                $func:ident $($_($reversing_fn)?)* ,
                $_($rest:tt)*
            ) => {
                $crate::__::compile_error!{$crate::__::concat!(
                    "cannot call two iterator-reversing methods in `konst::iter` macros,",
                    " called: ",
                    $crate::__::stringify!($func),
                )}
            };
                        (
                $fixed:tt
                $next_fn:ident

                $func:ident $func2:ident,
                $_($rest:tt)*
            ) => {
                $crate::__::compile_error!{$crate::__::concat!(
                    "unsupported iterator method: ",
                    $crate::__::stringify!($func),
                )}
            }
        }

        #[doc(hidden)]
        pub use __cim_detect_rev_method;
    };
}

make__cim_detect_rev_method__macro! {
    $
    [
        copied,
        enumerate,
        filter,
        filter_map,
        flat_map,
        flatten,
        map,
        skip,
        skip_while,
        take,
        take_while,
        zip,

        all,
        any,
        count,
        find,
        find_map,
        fold,
        for_each,
        nth,
        next,
        position,
    ]
    [
        rev,

        rfind,
        rfold,
        rposition,
    ]

    (
        (
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
            ($iter:expr)
            ($($args:tt)*)
        )
        $next_fn:ident
    ) => {
        $crate::__call_iter_methods!{
            (
                ($($callback_macro)*) ($($fixed_arguments)*)
                ($label $label) $next_fn $allowed_methods
            )
            (
                ($($callback_macro)*) ($($fixed_arguments)*)
                ($label $label) $next_fn $allowed_methods
            )
            $item
            (
                (
                    iter = $iter
                    {}
                    let $item = if let $crate::__::Some((elem_, next_)) = iter.$next_fn() {
                        iter = next_;
                        elem_
                    } else {
                        break $label;
                    };
                )
            )
            $($args)*
        }
    };
}
