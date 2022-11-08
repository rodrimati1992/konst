/// For loop over a range
/// 
/// # Example
///     
/// ```rust
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
/// ```
/// 
pub use konst_kernel::for_range;