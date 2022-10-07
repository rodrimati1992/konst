/**

Emulates most iterator methods.


# Methods

`konst::iter::eval` supports emulating iterator methods by expanding to equivalent code.

This macro supports the methods described below and 
the ones described in the [`iterator_dsl`] module.

### `all`

Const equivalent of [`Iterator::all`]

```rust
use konst::iter;
const fn all_digits(s: &str) -> bool {
    iter::eval!(s.as_bytes(),all(|c| c.is_ascii_digit()))
}

assert!(all_digits("123456"));
assert!(!all_digits("0x123456"));

```

### `any`

Const equivalent of [`Iterator::any`]


```rust
use konst::iter;

const fn contains_pow2(s: &[u64]) -> bool {
    iter::eval!(s,any(|c| c.is_power_of_two()))
}

assert!(contains_pow2(&[2, 3, 5]));
assert!(!contains_pow2(&[13, 21, 34]));

```

### `for_each`

Alternative way to write [`konst::iter::for_each`](crate::iter::for_each)

```rust
use konst::iter;

const fn sum_elems(s: &[u64]) -> u64 {
    let mut sum = 0u64;
    iter::eval!{s,copied(),for_each(|x| sum+=x)}
    sum
}

assert_eq!(sum_elems(&[]), 0);
assert_eq!(sum_elems(&[2]), 2);
assert_eq!(sum_elems(&[3, 5]), 8);

```
### `next`

Gets the first element in the iterator,
only needed when intermediate Iterator methods are used.

```rust
use konst::iter;

const fn first_elem(s: &[u64]) -> Option<u64> {
    iter::eval!(s,copied(),next())
}

assert_eq!(first_elem(&[]), None);
assert_eq!(first_elem(&[2]), Some(2));
assert_eq!(first_elem(&[3, 5]), Some(3));

```

[`iterator_dsl`]: ../iterator_dsl/index.html


### `position`

Const equivalent of [`Iterator::position`]

```rust
use konst::iter;

const fn find_num(slice: &[u64], n: u64) -> Option<usize> {
    iter::eval!(slice,position(|&elem| elem == n))
}

assert_eq!(find_num(&[3, 5, 8], 0), None);
assert_eq!(find_num(&[3, 5, 8], 3), Some(0));
assert_eq!(find_num(&[3, 5, 8], 5), Some(1));
assert_eq!(find_num(&[3, 5, 8], 8), Some(2));

```

### `rposition`

Const equivalent of [`Iterator::rposition`]

```rust
use konst::iter;

const fn rfind_num(slice: &[u64], n: u64) -> Option<usize> {
    iter::eval!(slice,rposition(|&elem| elem == n))
}

assert_eq!(rfind_num(&[3, 5, 8], 0), None);
assert_eq!(rfind_num(&[3, 5, 8], 3), Some(2));
assert_eq!(rfind_num(&[3, 5, 8], 5), Some(1));
assert_eq!(rfind_num(&[3, 5, 8], 8), Some(0));

```

### `find`

Const equivalent of [`Iterator::find`]

```rust
use konst::iter;

const fn find_odd(slice: &[u64], n: u64) -> Option<&u64> {
    iter::eval!(slice,find(|&&elem| elem % 2 == 1))
}

assert_eq!(find_odd(&[], 0), None);
assert_eq!(find_odd(&[2, 4], 0), None);
assert_eq!(find_odd(&[3, 5, 8], 3), Some(&3));
assert_eq!(find_odd(&[8, 12, 13], 3), Some(&13));

```

### `find_map`

Const equivalent of [`Iterator::find_map`]

This example requires the `"parsing_no_proc"` feature.

*/
#[cfg_attr(not(feature = "parsing_no_proc"), doc = "```ignore")]
#[cfg_attr(feature = "parsing_no_proc", doc = "```rust")]
/**
use konst::{iter, result};
use konst::primitive::parse_u64;

const fn find_parsable(slice: &[&str]) -> Option<u64> {
    iter::eval!(slice,find_map(|&s| result::ok!(parse_u64(s))))
}

assert_eq!(find_parsable(&[]), None);
assert_eq!(find_parsable(&["foo"]), None);
assert_eq!(find_parsable(&["foo", "10"]), Some(10));
assert_eq!(find_parsable(&["10", "20"]), Some(10));

```

### `rfind`

Const equivalent of [`DoubleEndedIterator::rfind`]

```rust
use konst::iter;

const fn sum_u64(slice: &[u64]) -> Option<&u64> {
    iter::eval!(slice,rfind(|&elem| elem.is_power_of_two()))
}

assert_eq!(sum_u64(&[]), None);
assert_eq!(sum_u64(&[2]), Some(&2));
assert_eq!(sum_u64(&[2, 5, 8]), Some(&8));

```

*/
pub use konst_macro_rules::iter_eval as eval;