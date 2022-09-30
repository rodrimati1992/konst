/**
Collects an iterator constant into an array

# Example

### Range

```rust
const ARR: [usize; 4] = konst::iter::collect_const!(usize = 10..14);

assert_eq!(ARR, [10, 11, 12, 13]);
```

*/
pub use konst_macro_rules::iter_collect_const as collect_const;