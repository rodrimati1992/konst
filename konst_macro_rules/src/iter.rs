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
