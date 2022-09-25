#[macro_export]
macro_rules! for_each {
    ($pattern:pat in $iter:expr => $($code:tt)*) => (
        match $iter {mut iter=>{
            while let $crate::__::Some((elem, next)) = iter.next() {
                iter = next;
                let $pattern = elem;
                $($code)*
            }
        }}
    );
}

#[macro_export]
macro_rules! for_each_i {
    ($pattern:pat in $iter:expr => $($code:tt)*) => (
        match $iter {mut iter=>{
            let mut i = 0;
            while let $crate::__::Some((elem, next)) = iter.next() {
                iter = next;
                let $pattern = (i, elem);
                $($code)*
                i += 1;
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
        match $iter {
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
        ($($before_loop:tt)*),
        ($($on_continue:tt)*),
        $on_found:expr,
        $on_not_found:expr,
        |$elem:pat| $v:expr
    ) => (
        match $iter { mut iter => {
            $($before_loop)*
            loop {
                match iter.next() {
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
            ($iter, _elem, (), (), true, false,)
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
            ($iter, _elem, (let mut i = 0;), (i+=1;), $crate::__::Some(i), $crate::__::None,)
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
            ($iter, elem, (), (), $crate::__::Some(elem), $crate::__::None,)
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
        match ($iter, $nth) {
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
