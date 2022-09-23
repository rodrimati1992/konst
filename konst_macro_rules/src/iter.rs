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
