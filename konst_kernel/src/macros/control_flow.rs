#[macro_export]
#[doc(hidden)]
macro_rules! __for_range_docs {
    ($mod:literal) => {concat!(r#"
For loop over a range

# Example
    
```rust"#,
"\nuse ", $mod, "::for_range;\n",
r#"
const LEN: usize = 10;
const ARR: [u32; LEN] = {
    let mut ret = [1; LEN];
    for_range!{i in 2..LEN =>
        ret[i] = ret[i - 1] + ret[i - 2];
    }
    ret
};

assert_eq!(ARR, [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
```
"#
    )}
}

#[doc = __for_range_docs!("konst_kernel")]
pub mod for_range{}

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