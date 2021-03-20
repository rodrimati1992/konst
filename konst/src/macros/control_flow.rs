/// For loop over a range
///
/// # Example
///
/// ```
/// use konst::for_range;    
///     
/// const LEN: usize = 10;
/// const ARR: [u32; LEN] = {
///     let mut ret = [1; LEN];
///     for_range!{i in 2..LEN =>
///         ret[i] = ret[i - 1] + ret[i - 2];
///     }
///     ret
/// };
///
/// assert_eq!(ARR, [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
///
/// ```
#[macro_export]
macro_rules! for_range {
    ($pat:pat in $range:expr => $($code:tt)*) => {
        let $crate::__::Range{mut start, end} = $range;

        while start < end {
            let $pat = start;

            $($code)*

            start+=1;
        }
    };
}
