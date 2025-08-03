#[doc(hidden)]
#[macro_export]
macro_rules! __iter_eval {
    (
        $iter:expr
        $($(,)+ $methods:ident ($($args:tt)*))*
        $(,)?
    ) => (
        $crate::iter::__eval2_lowering!{
            {
                // instruction in the current iteration level
                []
                // previous iterator levels
                []
                // variables declared at the top level
                [[/* is_returning */] [/* regular variables */][/* iterator variables */ $iter;]]
            }

            $([$methods $methods] ($($args)*))*
        }
    );
    (
        $(, $methods:ident ($($args:tt)*))*
        $(,)?
    ) => (
        $crate::__::compile_error!{ "expected iterator argument" }
    );
}

/**
Emulates iterator method chains, by expanding to equivalent code.

For examples that use multiple methods [look here](#full-examples)

# Drop behavior

The behavior regarding dropping iterators is
[documented here](crate::iter::ConstIntoIter#dropping).

# Methods

### Consuming methods

Consuming methods are those that consume the iterator, returning a non-iterator.

The consuming methods listed alphabetically:
- [`all`](#all)
- [`any`](#any)
- [`count`](#count)
- [`find_map`](#find_map)
- [`find`](#find)
- [`fold`](#fold)
- [`for_each`](#for_each)
- [`next`](#next)
- [`nth`](#nth)
- [`position`](#position)
- [`rfind`](#rfind)
- [`rfold`](#rfold)

`rposition` has been removed since it returned the distance from the end,
instead of the start. Implementing it properly would require major changes.

### Adaptor Methods

Adaptor methods are those that transform the iterator into a different iterator.
They are shared with other `konst::iter` macros and are documented
in the  [`iterator_dsl`] module.

The iterator adaptor methods, listed alphabetically
(these links go to the [`iterator_dsl`] module):
- [`copied`](./iterator_dsl/index.html#copied)
- [`enumerate`](./iterator_dsl/index.html#enumerate)
- [`filter_map`](./iterator_dsl/index.html#filter_map)
- [`filter`](./iterator_dsl/index.html#filter)
- [`flat_map`](./iterator_dsl/index.html#flat_map)
- [`flatten`](./iterator_dsl/index.html#flatten)
- [`map`](./iterator_dsl/index.html#map)
- [`rev`](./iterator_dsl/index.html#rev)
- [`skip_while`](./iterator_dsl/index.html#skip_while)
- [`skip`](./iterator_dsl/index.html#skip)
- [`take_while`](./iterator_dsl/index.html#take_while)
- [`take`](./iterator_dsl/index.html#take)
- [`zip`](./iterator_dsl/index.html#zip)


### `all`

Const equivalent of [`Iterator::all`]

```rust
use konst::iter;
const fn all_digits(s: &str) -> bool {
    iter::eval!(s.as_bytes(),all(|c| matches!(c, b'0'..=b'9')))
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

```rust
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

```rust
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

```rust
use konst::{iter, result};

const fn find_parsable(slice: &[&str]) -> Option<u64> {
    iter::eval!(slice,find_map(|&s| result::ok!(u64::from_str_radix(s, 10))))
}

assert_eq!(find_parsable(&[]), None);
assert_eq!(find_parsable(&["foo"]), None);
assert_eq!(find_parsable(&["foo", "10"]), Some(10));
assert_eq!(find_parsable(&["10", "20"]), Some(10));

```

### `rfind`

Const equivalent of [`DoubleEndedIterator::rfind`]

Limitations iterator-reversing methods can't:
- be called more than once in the same macro invocation.
- be called after calling `take`,`take_while`,`skip`, or `skip_while`.


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

Limitation: iterator-reversing methods can't be called more than once in
the same macro invocation.

```rust
use konst::iter;

const fn concat_u16s(slice: &[u16]) -> u128 {
    iter::eval!(slice,rfold(0, |accum, &rhs| (accum << 16) + (rhs as u128)))
}

assert_eq!(concat_u16s(&[1, 2, 3]), 0x0003_0002_0001);
assert_eq!(concat_u16s(&[3, 5, 8]), 0x0008_0005_0003);


```

<span id = "full-examples"></span>
# Examples

### Second number in CSV

This example demonstrates a function that gets the second number in a CSV string,
using iterators.

```rust
use konst::{
    iter,
    result,
    string,
};

const fn second_number(s: &str) -> Option<u64> {
    iter::eval!(
        string::split(s, ","),
            filter_map(|s| result::ok!(u64::from_str_radix(s, 10))),
            nth(1),
    )
}

assert_eq!(second_number("foo,bar,baz"), None);
assert_eq!(second_number("foo,3,bar"), None);
assert_eq!(second_number("foo,3,bar,5"), Some(5));
assert_eq!(second_number("foo,8,bar,13,baz,21"), Some(13));

```


[`iterator_dsl`]: ./iterator_dsl/index.html

*/
#[doc(inline)]
pub use __iter_eval as eval;

declare_eval2_lowering! {
    $

    # eval methods
    (
        { [$($prev_insts:tt)*] $($fixed:tt)* }
        [map] ($($args:tt)*)
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            { [ $($prev_insts)* (map (__parse_closure_1) map $($args)*)] $($fixed)* }

            $($rem)*
        }
    };

    (
        { [$($prev_insts:tt)*] $prev_levels:tt [$is_returning:tt [$($vars:tt)*] $iters:tt] }
        [enumerate] ()
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                [$($prev_insts)* (
                    map
                    (__fast_parse_closure_1)
                    enumerate
                    |item| -> _ {
                        let item = (i, item);
                        i += 1;
                        item
                    }
                )]
                $prev_levels
                [$is_returning [$($vars)* mut i = 0usize,] $iters]
            }

            $($rem)*
        }
    };

    (
        { [$($prev_insts:tt)*] $($fixed:tt)* }
        [copied] ()
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                [$($prev_insts)* (
                    map
                    (__fast_parse_closure_1)
                    copied
                    |item| -> _ {
                        let &item = item;
                        item
                    }
                )]
                $($fixed)*
            }

            $($rem)*
        }
    };

    (
        { [$($prev_insts:tt)*] $prev_levels:tt [$is_returning:tt [$($vars:tt)*] $iters:tt] }
        [skip] ($skipping:expr $(,)?)
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                [$($prev_insts)* (
                    truncating(skip)
                    |_| {
                        let _: $crate::__::usize = skipping;
                        if skipping != 0 {
                            skipping -= 1;
                            continue
                        }
                    }
                )]
                $prev_levels
                [$is_returning [$($vars)* mut skipping = $skipping,] $iters]
            }

            $($rem)*
        }
    };

    (
        { [$($prev_insts:tt)*] $prev_levels:tt [$is_returning:tt [$($vars:tt)*] $iters:tt] }
        [skip_while] ($($closure:tt)*)
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                [$($prev_insts)* (
                    truncating(skip_while)
                    |ref item| {
                        skipping = skipping && $crate::__parse_closure_1!{
                            ($crate::__eval_closure) (item,) (skip_while),
                            $($closure)*
                        };
                        if skipping {
                            continue
                        }
                    }
                )]
                $prev_levels
                [$is_returning [$($vars)* mut skipping = true,] $iters]
            }

            $($rem)*
        }
    };

    (
        { [$($prev_insts:tt)*] $prev_levels:tt [$is_returning:tt [$($vars:tt)*] $iters:tt] }
        [take] ($taking:expr $(,)?)
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                [$($prev_insts)* (
                    truncating(take)
                    |_| {
                        let _: $crate::__::usize = taking;
                        if taking == 0 {
                            __iter2_returner!{}
                        } else {
                            taking -= 1;
                        }
                    }
                )]
                $prev_levels
                [[is_returning] [$($vars)* mut taking = $taking,] $iters]
            }

            $($rem)*
        }
    };

    (
        { [$($prev_insts:tt)*] $prev_levels:tt [$is_returning:tt [$($vars:tt)*] $iters:tt] }
        [take_while] ($($closure:tt)*)
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                [$($prev_insts)* (
                    truncating(take_while)
                    |ref item| {
                        taking = taking && $crate::__parse_closure_1!{
                            ($crate::__eval_closure) (item,) (take_while),
                            $($closure)*
                        };
                        if !taking {
                            __iter2_returner!{}
                        }
                    }
                )]
                $prev_levels
                [[is_returning] [$($vars)* mut taking = true,] $iters]
            }

            $($rem)*
        }
    };

    (
        { [$($prev_insts:tt)*] $($fixed:tt)* }
        [filter] ($($args:tt)*)
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                [$($prev_insts)* (
                    __inspector
                    |ref item| {
                        let filter_in: $crate::__::bool = $crate::__parse_closure_1!{
                            ($crate::__eval_closure) (item,) (filter),
                            $($args)*
                        };

                        if !filter_in {
                            continue
                        }
                    }
                )]
                $($fixed)*
            }

            $($rem)*
        }
    };

    (
        { [$($prev_insts:tt)*] $($fixed:tt)* }
        [filter_map] ($($args:tt)*)
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                [$($prev_insts)* (
                    map
                    (__fast_parse_closure_1)
                    filter_map
                    |item| -> _ {
                        $crate::if_let_Some!{
                            item = $crate::__parse_closure_1!{
                                ($crate::__eval_closure) (item,) (filter_map),
                                $($args)*
                            } => {
                                item
                            } else {
                                continue
                            }
                        }
                    }
                )]
                $($fixed)*
            }
            $($rem)*
        }
    };

    (
        {
            [$($prev_insts:tt)*]
            $prev_levels:tt
            [ $is_returning:tt $vars:tt [$($iters:tt)*] ]
        }
        [zip] ($other_iter:expr $(,)?)
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                [$($prev_insts)* (zip (other_iter))]
                $prev_levels
                [[is_returning] $vars [$($iters)* other_iter = $other_iter,] ]
            }
            $($rem)*
        }
    };

    (
        { [$($prev_insts:tt)*] [$($prev_levels:tt)*] $vars:tt }
        [rev] ()
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                [
                    $($prev_insts)*
                    (rev =>)
                ]
                [
                    {rev => $($prev_levels)* $($prev_insts)*}
                    $($prev_levels)*
                ]
                $vars
            }

            $($rem)*
        }
    };

    (
        { [$($prev_insts:tt)*] [$($prev_levels:tt)*] $tl_vars:tt}
        [flatten] ()
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                []
                [
                    $($prev_levels)*
                    $($prev_insts)*
                    (iterate [[] [,use_item;]])
                ]
                $tl_vars
            }

            $($rem)*
        }
    };
    (
        { [$($prev_insts:tt)*] [$($prev_levels:tt)*] $tl_vars:tt}
        [flat_map] ($($args:tt)*)
        $($rem:tt)*
    ) => {
        $crate::iter::__eval2_lowering!{
            {
                []
                [
                    $($prev_levels)*
                    $($prev_insts)*
                    (map (__parse_closure_1) flat_map $($args)*)
                    (iterate [[] [,use_item;]])
                ]
                $tl_vars
            }

            $($rem)*
        }
    };

    # consumer methods


    ( fixed:tt [find] ($($closure:tt)*) $($rem:tt)* ) => {
        $crate::iter::__eval2_lowering!{
            $fixed
            [__finder __finder] (accum = $crate::__::None, |item| {
                let found = $crate::__parse_closure_1!{
                    ($crate::__eval_closure) (&item,) (find),
                    $($closure)*
                };

                if found {
                    $crate::__utils::__overwrite(&mut accum, $crate::__::Some(item));
                    __iter2_returner!{}
                }
            })
            $($rem)*
        }
    };

    ( fixed:tt [rfind] $args:tt $($rem:tt)* ) => {
        $crate::iter::__eval2_lowering!{
            $fixed
            [rev rev] ()
            [find find] $args
            $($rem)*
        }
    };

    ( fixed:tt [find_map] ($($closure:tt)*) $($rem:tt)* ) => {
        $crate::iter::__eval2_lowering!{
            $fixed
            [__finder __finder] (accum = $crate::__::None, |item| {
                $crate::if_let_Some!{found =
                    $crate::__parse_closure_1!{
                        ($crate::__eval_closure) (item,) (find_map),
                        $($closure)*
                    } => {
                        $crate::__utils::__overwrite(&mut accum, $crate::__::Some(found));
                        __iter2_returner!{}
                    }
                }
            })
            $($rem)*
        }
    };

    ( fixed:tt [position] ($($closure:tt)*) $($rem:tt)* ) => {
        $crate::iter::__eval2_lowering!{
            $fixed
            [__finder __finder] (@vars(mut i = 0usize,) accum = $crate::__::None, |item| {
                let found = $crate::__parse_closure_1!{
                    ($crate::__eval_closure) (item,) (position),
                    $($closure)*
                };

                if found {
                    accum = $crate::__::Some(i);
                    __iter2_returner!{}
                } else {
                    i += 1;
                }
            })
            $($rem)*
        }
    };

    ( fixed:tt [all] ($($closure:tt)*) $($rem:tt)* ) => {
        $crate::iter::__eval2_lowering!{
            $fixed
            [__finder __finder] (accum = true, |item| {
                accum = $crate::__parse_closure_1!{
                    ($crate::__eval_closure) (item,) (all),
                    $($closure)*
                };

                if !accum {
                    __iter2_returner!{}
                }
            })
            $($rem)*
        }
    };

    ( fixed:tt [any] ($($closure:tt)*) $($rem:tt)* ) => {
        $crate::iter::__eval2_lowering!{
            $fixed
            [__finder __finder] (accum = false, |item| {
                accum = $crate::__parse_closure_1!{
                    ($crate::__eval_closure) (item,) (any),
                    $($closure)*
                };

                if accum {
                    __iter2_returner!{}
                }
            })
            $($rem)*
        }
    };

    ( fixed:tt [nth] ($nth:expr $(,)?) $($rem:tt)* ) => {
        $crate::iter::__eval2_lowering!{
            $fixed
            [__finder __finder] (@vars(mut i = $nth,) accum = $crate::__::None, |item| {
                let _: $crate::__::usize = i;
                if let $crate::__::Some(ni) = $crate::__::usize::checked_sub(i, 1) {
                    i = ni;
                } else {
                    $crate::__utils::__overwrite(&mut accum, $crate::__::Some(item));
                    __iter2_returner!{}
                }
            })
            $($rem)*
        }
    };

    ( fixed:tt [next] () $($rem:tt)* ) => {
        $crate::iter::__eval2_lowering!{
            $fixed
            [__finder __finder] (accum = $crate::__::None, |item| {
                $crate::__utils::__overwrite(&mut accum, $crate::__::Some(item));
                __iter2_returner!{}
            })
            $($rem)*
        }
    };

    ( fixed:tt [count] () $($rem:tt)* ) => {
        $crate::iter::__eval2_lowering!{
            $fixed
            [fold fold] (@fast_parse 0usize, |accum, _item| -> _ {
                accum + 1
            })
            $($rem)*
        }
    };

    (
        { [$($prev_insts:tt)*]  $prev_levels:tt $tl_vars:tt }
        [for_each] ($($closure:tt)*)
    ) => {
        $crate::__iter2_finish_lowering!{
            []
            $prev_levels
            $tl_vars
            [ $($prev_insts)* (for_each $($closure)*) ]
        }
    };

    (
        {
            [$($prev_insts:tt)*]
            $prev_levels:tt
            [[$($is_returning2:ident $($__ir2:tt)*)?] $vars:tt $iters:tt]
        }
        [fold] (
            $(@uses_break ($accum_name:ident) $(@($uses_break:tt))? )?
            $(@fast_parse $(@($fast_parse:tt))? )?
            $default_val:expr,
            $($closure:tt)*
        )
    ) => {
        $crate::__iter2_finish_lowering!{
            [accum $($accum_name)? acc = $default_val]
            $prev_levels
            [
                [$( is_returning $($uses_break)? )? $($is_returning2)?]
                $vars
                $iters
            ]
            [
                $($prev_insts)*
                (fold
                    ($(__fast_parse_closure_2 $($fast_parse)? )? __parse_closure_2)
                    ($($accum_name)? acc)
                    $($closure)*
                )
            ]
        }
    };

    ( fixed:tt [rfold] $args:tt $($rem:tt)* ) => {
        $crate::iter::__eval2_lowering!{
            $fixed
            [rev rev] ()
            [fold fold] $args
            $($rem)*
        }
    };

    # secret methods

    (
        {
            [$($prev_insts:tt)*]
            $prev_levels:tt
            [$is_returning:tt [$($vars:tt)*] $iters:tt]
        }
        [__finder] (
            $(@vars($($more_vars:tt)*))?
            $accum:ident = $default_val:expr,
            $($closure:tt)*
        )
    ) => {
        $crate::__iter2_finish_lowering!{
            [accum $accum = $default_val]
            $prev_levels
            [[is_returning] [$($vars)* $($($more_vars)*)?] $iters]
            [
                $($prev_insts)*
                (__finder $($closure)*)
            ]
        }
    };
    (
        {
            [$($prev_insts:tt)*]
            $prev_levels:tt
            [$is_returning:tt [$($vars:tt)*] $iters:tt]
        }
        [__inspector] (
            $(@vars($($more_vars:tt)*))?
            $($accum:ident = $default_val:expr)?,
            $($closure:tt)*
        )
    ) => {
        $crate::__iter2_finish_lowering!{
            [accum $accum = $default_val]
            $prev_levels
            [$is_returning [$($vars)* $($($more_vars)*)?] $iters]
            [
                $($prev_insts)*
                (__inspector $($closure)*)
            ]
        }
    };
}

macro_rules! declare_eval2_lowering {
    (
        $_:tt

        # eval methods
        $(
            (
                $($state0:ident :tt)? $({$($state0b:tt)*})?
                [$eval_method:ident] $($eval_m_args:tt)*
            ) => {
                $($eval_m_expansion:tt)*
            };
        )*

        # consumer methods
        $(
            (
                $($state1:ident :tt)? $({$($state1b:tt)*})?
                [$cons_method:ident] $($cons_m_args:tt)*
            ) => {
                $($cons_m_expansion:tt)*
            };
        )*

        # secret methods
        $(
            (
                $($state2:ident :tt)? $({$($state2b:tt)*})?
                [$secret_method:ident] $($secret_m_args:tt)*
            ) => {
                $($secret_m_expansion:tt)*
            };
        )*
    ) => (

        #[doc(hidden)]
        #[macro_export]
        macro_rules! __eval2_lowering_ {
            $(
                (
                    $($_ $state0)? $({$($state0b)*})?
                    [$method_name:ident $eval_method] $($eval_m_args)*
                ) => {
                    $($eval_m_expansion)*
                };
            )*
            (
                $fixed:tt
                [$method_name:ident $( $_($cons_method)? )*]
                $args:tt
                $_($rem:tt)+
            ) => {
                $crate::__::compile_error!{$crate::__::concat!{
                    "Cannot consume the iterator multiple times, ",
                    "first consumed by `", $crate::__::stringify!($method_name) ,
                    "` method"
                }}
            };
            $(
                (
                    $($_ $state1:tt)? $({$($state1b)*})?
                    [$method_name:ident $cons_method] $($cons_m_args)*
                ) => {
                    $($cons_m_expansion)*
                };
            )*
            $(
                (
                    $($_ $state2:tt)? $({$($state2b)*})?
                    [$secret_name:ident $secret_method] $($secret_m_args)*
                ) => {
                    $($secret_m_expansion)*
                };
            )*
            (
                $fixed:tt
                [$method_name:ident $( $_($eval_method)? )* $( $_($cons_method)? )*]
                ($_($args:tt)*)
                $_($rem:tt)*
            ) => {
                $crate::__::compile_error!{$crate::__::concat!{
                    "invalid argument(s) for `", $crate::__::stringify!($method_name),
                    "` method: `", $crate::__::stringify!($_($args)*), "`"
                }}
            };
            (
                $fixed:tt
                [$method_name:ident $__method_name:ident]
                $_($rem:tt)*
            ) => {
                $crate::__::compile_error!{$crate::__::concat!{
                    "method `", $crate::__::stringify!($method_name),
                    "` is unsupported, expected one of:",
                    $( "\n- ", $crate::__::stringify!($eval_method), )*
                    $( "\n- ", $crate::__::stringify!($cons_method), )*
                }}
            };
            (
                $fixed:tt
            ) => {
                $crate::__::compile_error!{$crate::__::concat!{
                    "expected one of these consumer methods at the end:`",
                    $( "\n- ", $crate::__::stringify!($cons_method), )*
                }}
            };
            (
                $_($rem:tt)+
            ) => {
                $crate::__::compile_error!{$crate::__::concat!{
                    "Invalid iter::eval lowering syntax: ", $crate::__::stringify!($_($rem)*)
                }}
            };
        }

        pub use __eval2_lowering_ as __eval2_lowering;

    )
}
use declare_eval2_lowering;

#[doc(hidden)]
#[macro_export]
macro_rules! __iter2_finish_lowering {
    (
        // top-level properties
        [$(accum $ret_var:ident $($__ret_var:ident)* = $ret_val:expr)?]

        // previous iterator levels
        [
            $({rev $($rev:tt)*})*
            $(($($prev_levels:tt)*))*
        ]

        // variables declared at the top
        [
            [$($is_returning:ident $($__ir2:tt)*)?]
            $vars:tt
            $iters:tt
        ]

        // instruction in the current iteration level
        [$($instrs_il:tt)*]
    ) => { match ($($ret_val,)?) {($(mut $ret_var,)?) => {
        $( let mut $is_returning = false; )?

        $(
            macro_rules! __iter2_returner {
                () => {
                    $is_returning = true;
                    break;
                }
            }
        )?

        $crate::__iter2_interpreter!{
            [item next next_back ($($is_returning)?)]
            [item next next_back ($($is_returning)?)]

            $((rev $($rev)*))*
            (iterate @top_level [$vars $iters])
            $(($($prev_levels)*))*
            $($instrs_il)*
        }

        $($ret_var)?
    }}};
    (
        $($rem:tt)+
    ) => {
        $crate::__::compile_error!{$crate::__::concat!{
            "Invalid iter::eval finish lowering syntax: ", $crate::__::stringify!($($rem)*)
        }}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter2_interpreter {
    (
        [$item:ident $next:ident $__next_back:ident ($($is_returning:ident $(@$isr:tt)?)?)]
        $fixed:tt
        (
            iterate
            $(@top_level $(@($top_level:tt))?)?
            [
                [$($var_name:pat_param = $var_val:expr,)*]
                [
                    $($iter0_val:expr)? $(,use_item $($__use_item:ident)?)?;
                    $($iter_name:ident = $iter_val:expr,)*
                ]
            ]
        )
        $($rem:tt)*
    ) => {
        match (
            $($var_val,)*
            $($iter0_val)? $($item $(@$__use_item)?)?,
            $($iter_val,)*
        ) {
            ($($var_name,)* iter0, $($iter_name,)*) => {
                let mut iter0 = $crate::iter::into_iter!(iter0);
                $(let mut $iter_name = $crate::iter::into_iter!($iter_name);)*

                let elem_phantom_ty = $crate::iter::__get_item_ty(&iter0);
                $crate::while_let_Some! { $item = iter0.$next() =>
                    $crate::iter::__assert_item_ty(&$item, elem_phantom_ty);

                    $crate::__iter2_interpreter!{
                        $fixed
                        $fixed
                        $($rem)*
                    }
                }

                #[cfg(any(false $(, true$($isr)?)?))]
                {
                    $crate::__iter2_drop_iterator!{iter0};
                    $( $crate::__iter2_drop_iterator!{$iter_name}; )*
                }

                $crate::__::forget(iter0);
                $($crate::__::forget($iter_name);)*

                #[cfg(all(true $(, false $($top_level)?)?))]
                {
                    $(
                        if $is_returning {
                            break;
                        }
                    )?
                }
            }
        }
    };

    (
        [$item:ident $next:ident $($__rem_fixed:tt)*]
        $fixed:tt
        (
            map
            ($closure_parser:ident $($__closure_parser:tt)*)
            $method_name:ident

            $($closure:tt)*
        )
        $($rem:tt)*
    ) => {
        let $item = $crate::$closure_parser!{
            ($crate::__eval_closure) ($item,) ($method_name),
            $($closure)*
        };

        $crate::__iter2_interpreter!{$fixed $fixed $($rem)*}
    };

    (
        [$item:ident $next:ident $($__rem_fixed:tt)*]
        $fixed:tt
        (zip ($other_iter:ident))
        $($rem:tt)*
    ) => {
        let $item = $crate::if_let_Some!{item2 = $other_iter.$next() => {
            ($item, item2)
        } else {
            __iter2_returner!{}
        }};

        $crate::__iter2_interpreter!{
            $fixed
            $fixed
            $($rem)*
        }
    };

    // dedicated instruction for take*/skip* so that rev can detect their presence
    (
        [$item:ident $next:ident $($__rem_fixed:tt)*]
        $fixed:tt
        (truncating ($method:ident) |$item_param:pat_param| $code:block)
        $($rem:tt)*
    ) => {
        let $item_param = $item;
        $code

        $crate::__iter2_interpreter!{$fixed $fixed $($rem)*}
    };

    (
        [$item:ident $next:ident $next_back:ident $is_returning:tt]
        $fixed:tt
        (rev => $($prev_instructions:tt)* )
        $($rem:tt)*
    ) => {
        $crate::__iter2_rev_asserts!{$($prev_instructions)*}

        $crate::__iter2_interpreter!{
            [$item $next_back $next $is_returning]
            [$item $next_back $next $is_returning]
            $($rem)*
        }
    };

    (
        [$item:ident $($__rem_fixed:tt)*]
        $fixed:tt
        (__inspector |$item_param:pat_param| $code:block)
        $($rem:tt)*
    ) => {
        let $item_param = $item;
        $code

        $crate::__iter2_interpreter!{$fixed $fixed $($rem)*}
    };

    (
        [$item:ident $($__rem_fixed:tt)*]
        $fixed:tt
        (for_each $($closure:tt)*)
    ) => {
        let _: () = $crate::__parse_closure_1!{
            ($crate::__eval_closure) ($item,) (for_each),
            $($closure)*
        };
    };
    (
        [$item:ident $($__rem_fixed:tt)*]
        $fixed:tt
        (
            fold
            ($closure_parser:ident $($__closure_parser:tt)*)
            ($acc:ident $($__acc:ident)*)
            $($closure:tt)*
        )
    ) => {
        $acc = $crate::$closure_parser!{
            ($crate::__eval_closure) (($acc, $item,),) (fold),
            $($closure)*
        };
    };

    (
        [$item:ident $($__rem_fixed:tt)*]
        $fixed:tt
        (__finder |$item_param:pat_param| $code:block)
    ) => {
        let $item_param = $item;
        $code
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter2_drop_iterator {
    ($iter:ident) => {
        if $crate::iter::__items_needs_drop(&$iter) {
            while let $crate::__::Some(_) = $iter.next() {}
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter2_rev_asserts {
    ( $({rev $(@$is_rev:tt)? => $($__0:tt)*})? $(( $($instruction:tt)* ))* ) => {
        $(
            $crate::__::compile_error!{"iterators can't be reversed twice"};
            $(@$is_rev)?
        )?

        $crate::__iter2_rev_assert_no_truncation!{
            $(( $($instruction)* ))*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter2_rev_assert_no_truncation {
    (
        (truncating ($method_name:ident) $($__0:tt)*)
        $($rem:tt)*
    ) => {
        $crate::__::compile_error!{$crate::__::concat!{
            "Cannot call `rev()` after calling `",
            $crate::__::stringify!($method_name),
            "`",
        }}

        $crate::__iter2_rev_assert_no_truncation!{ $($rem)* }
    };
    (
        ($method_name:ident $($__0:tt)*)
        $($rem:tt)*
    ) => {
        $crate::__iter2_rev_assert_no_truncation!{ $($rem)* }
    };
    () => {};
}
