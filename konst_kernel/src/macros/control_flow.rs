#[macro_export]
macro_rules! for_range {
    ($pat:pat_param in $range:expr => $($code:tt)*) => {
        let $crate::__::Range{mut start, end} = $range;

        while start < end {
            let $pat = start;

            start+=1;

            $($code)*
        }
    };
}
