/**

Emulates most iterator methods.


# Methods

`konst::iter::eval` supports emulating iterator methods by expanding to equivalent code.

### Other Methods

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

### `count`

Const equivalent of [`Iterator::count`]

This example requires the `"rust_1_64"` crate feature,
because it calls `string::split`.

*/
#[cfg_attr(not(feature = "rust_1_64"), doc = "```ignore")]
#[cfg_attr(feature = "rust_1_64", doc = "```rust")]
/**
use konst::{iter, string};

const fn count_csv(s: &str) -> usize {
    iter::eval!(string::split(s, ","),count())
}

assert_eq!(count_csv("foo"), 1);
assert_eq!(count_csv("foo,bar"), 2);
assert_eq!(count_csv("foo,bar,baz"), 3);

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

### `nth`

Const equivalent of [`Iterator::nth`]

This example requires the `"rust_1_64"` crate feature,
because it calls `string::split`.

*/
#[cfg_attr(not(feature = "rust_1_64"), doc = "```ignore")]
#[cfg_attr(feature = "rust_1_64", doc = "```rust")]
/**
use konst::{iter, string};

const fn nth_csv(s: &str, nth: usize) -> Option<&str> {
    iter::eval!(string::split(s, ","),nth(nth))
}

assert_eq!(nth_csv("foo,bar,baz", 0), Some("foo"));
assert_eq!(nth_csv("foo,bar,baz", 1), Some("bar"));
assert_eq!(nth_csv("foo,bar,baz", 2), Some("baz"));
assert_eq!(nth_csv("foo,bar,baz", 3), None);

```

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

### `fold`

Const equivalent of [`Iterator::fold`]

```rust
use konst::iter;

const fn sum_u64(slice: &[u64]) -> u64 {
    iter::eval!(slice,fold(0, |accum, &rhs| accum + rhs))
}

assert_eq!(sum_u64(&[]), 0);
assert_eq!(sum_u64(&[3]), 3);
assert_eq!(sum_u64(&[3, 5]), 8);
assert_eq!(sum_u64(&[3, 5, 8]), 16);


```

### `rfold`

Const equivalent of [`DoubleEndedIterator::rfold`]

```rust
use konst::iter;     

const fn concat_u16s(slice: &[u16]) -> u128 {
    iter::eval!(slice,rfold(0, |accum, &rhs| (accum << 16) + (rhs as u128)))
}

assert_eq!(concat_u16s(&[1, 2, 3]), 0x0003_0002_0001);
assert_eq!(concat_u16s(&[3, 5, 8]), 0x0008_0005_0003);


```



[`iterator_dsl`]: ./iterator_dsl/index.html

*/
pub use konst_macro_rules::iter_eval as eval;