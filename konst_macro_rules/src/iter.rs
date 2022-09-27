#[macro_export]
macro_rules! for_each {
    ($pattern:pat in $iter:expr => $($code:tt)*) => (
        match $crate::into_iter_macro!($iter) {mut iter=>{
            while let $crate::__::Some((elem, next)) = iter.next() {
                iter = next;
                let $pattern = elem;
                $($code)*
            }
        }}
    );
}

#[macro_export]
macro_rules! iter_all {
    ($iter:expr, $($closure:tt)*) => {
        $crate::__parse_closure!{
            ($crate::__iter_all) ($iter,) (elem),
            $($closure)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter_all {
    ($iter:expr, |$elem:pat| $v:expr) => {
        match $crate::into_iter_macro!($iter) {
            mut iter => loop {
                match iter.next() {
                    $crate::__::Some((elem, next)) => {
                        iter = next;
                        let $elem = elem;
                        if !$v {
                            break false;
                        }
                    }
                    $crate::__::None => break true,
                }
            },
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter_on_found {
    (
        $iter:expr,
        $ret_elem:ident,
        $next_fn:ident,
        ($($before_loop:tt)*),
        ($($on_continue:tt)*),
        $on_found:expr,
        $on_not_found:expr,
        |$elem:pat| $v:expr
    ) => (
        match $crate::into_iter_macro!($iter) { mut iter => {
            $($before_loop)*
            loop {
                match iter.$next_fn() {
                    $crate::__::Some(($ret_elem, next)) => {
                        let $elem = &$ret_elem;
                        if $v {
                            break $on_found;
                        }
                        iter = next;
                        $($on_continue)*
                    }
                    $crate::__::None => break $on_not_found,
                }
            }
        }}
    )
}

#[macro_export]
macro_rules! iter_any {
    ($iter:expr, $($closure:tt)*) => {
        $crate::__parse_closure!{
            ($crate::__iter_on_found)
            ($iter, _elem, next, (), (), true, false,)
            (elem),
            $($closure)*
        }
    }
}

#[macro_export]
macro_rules! iter_position {
    ($iter:expr, $($closure:tt)*) => {
        $crate::__parse_closure!{
            ($crate::__iter_on_found)
            ($iter, _elem, next, (let mut i = 0;), (i+=1;), $crate::__::Some(i), $crate::__::None,)
            (elem),
            $($closure)*
        }
    }
}

#[macro_export]
macro_rules! iter_rposition {
    ($iter:expr, $($closure:tt)*) => {
        $crate::__parse_closure!{
            ($crate::__iter_on_found)
            (
                $iter, _elem, next_back, (let mut i = 0;), (i+=1;),
                $crate::__::Some(i), $crate::__::None,
            )
            (elem),
            $($closure)*
        }
    }
}

#[macro_export]
macro_rules! iter_find {
    ($iter:expr, $($closure:tt)*) => {
        $crate::__parse_closure!{
            ($crate::__iter_on_found)
            ($iter, elem, next, (), (), $crate::__::Some(elem), $crate::__::None,)
            (elem),
            $($closure)*
        }
    }
}

#[macro_export]
macro_rules! iter_rfind {
    ($iter:expr, $($closure:tt)*) => {
        $crate::__parse_closure!{
            ($crate::__iter_on_found)
            ($iter, elem, next_back, (), (), $crate::__::Some(elem), $crate::__::None,)
            (elem),
            $($closure)*
        }
    }
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
        $crate::__parse_closure!{
            ($crate::__iter_fold) ($iter, next_back, $accum,) (elem),
            $($closure)*
        }
    }
}

#[macro_export]
macro_rules! iter_fold {
    ($iter:expr, $accum:expr, $($closure:tt)*) => {
        $crate::__parse_closure!{
            ($crate::__iter_fold) ($iter, next, $accum,) (elem),
            $($closure)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter_fold {
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
macro_rules! iter_reduce {
    ($iter:expr, $($closure:tt)*) => {
        $crate::__parse_closure!{
            ($crate::__iter_reduce) ($iter, ) (elem),
            $($closure)*
        }
    }
}

#[macro_export]
macro_rules! iter_find_map {
    ($iter:expr, $($closure:tt)*) => {
        $crate::__parse_closure!{
            ($crate::__iter_find_map) ($iter,) (elem),
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

#[macro_export]
macro_rules! for_each_zip {
    ($pattern:pat in $($iter:expr),* $(,)? => $($code:tt)*) => (
        $crate::__for_each_zip!{
            ($pattern => $($code)*)
            ($($iter),*)
            ()
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_zip {
    (
        $fixed:tt
        ($iter:expr $(, $($rest:tt)*)?)
        ($($prev:tt)*)
    ) => {
        $crate::__for_each_zip!{
            $fixed
            ($($($rest)*)?)
            ($($prev)* ($iter, iter, next, elem))
        }
    };
    (
        ($pattern:pat => $($code:tt)*)
        ()
        ($(($iter:expr, $iter_var:ident, $next:ident, $elem:ident))*)
    ) => {
        match ($($crate::into_iter_macro!($iter),)*) {($(mut $iter_var,)*) => {
            while let ($($crate::__::Some(($elem, $next)),)*) = ($($iter_var.next(),)*) {
                $($iter_var = $next;)*
                let $pattern = ($($elem,)*);
                $($code)*
            }
        }}
    };
}
