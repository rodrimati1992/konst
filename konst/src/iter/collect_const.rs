use typewit::{MakeTypeWitness, TypeEq, TypeWitnessTypeArg};

#[doc(hidden)]
#[macro_export]
macro_rules! __collect_const_hidden {
    (
        $Item:ty =>
        $($rem:tt)*
    ) => {{
        $crate::__collect_const_iter_with!{
            $Item ,
            {let item = $crate::__::MaybeUninit::new(item);},
            |array, length, item| array[length] = item,
            elem_length = 1,
            =>
            $($rem)*
        }

        __ARR81608BFNA5
    }}
}

/**
Collects an iterator constant into an array

# Drop behavior

The behavior regarding dropping iterators is
[documented here](crate::iter::ConstIntoIter#dropping).

# Iterator methods

This macro supports emulating iterator methods by expanding to equivalent code.

The supported iterator methods are documented in the [`iterator_dsl`] module,
because they are also supported by other `konst::iter` macros.

# Syntax

The syntax of this macro is:

```text
collect_const!(
    $Item:ty => $into_iterator:expr
        $(, $iterator_method:ident ($($method_args:tt)*) )*
        $(,)?
)
```
Where `$Item` is the type of the elements that'll be collected into an array.

Where `$into_iterator` is any type that can be converted into a const iterator, with
[`konst::iter::into_iter`](crate::iter::into_iter).

Where `$iterator_method` is any of the supported methods described in
the [`iterator_dsl`] module.

# Examples

### Iterating over a range

```rust
use konst::iter;

const ARR: [u64; 8] = iter::collect_const!(u64 =>
    10..,
        filter(|n| *n % 2 == 0),
        skip(5),
        take(8),
);

assert_eq!(ARR, [20, 22, 24, 26, 28, 30, 32, 34]);
```

### Iterating over an array

```rust
use konst::iter;

const ARR: [u8; 6] = iter::collect_const!(u8 =>
    [10, 20, 30],flat_map(|n| [n - 1, n + 1]),
);

assert_eq!(ARR, [9, 11, 19, 21, 29, 31]);
```



[`iterator_dsl`]: crate::iter::iterator_dsl
*/
#[doc(inline)]
pub use __collect_const_hidden as collect_const;

#[doc(hidden)]
#[macro_export]
macro_rules! __collect_const_iter_with {
    (
        $Item:ty,
        {$($reassign_item:tt)*},
        |$array:ident, $length:ident, $item:ident| $elem_initer:expr,
        elem_length = $elem_length:expr,
        =>
        $($rem:tt)*
    ) => {
        const fn __func_zxe7hgbnjs<Ret_KO9Y329U2U, const CAP_KO9Y329U2U: $crate::__::usize>(
            cmd: $crate::__::CollectorCmd<Ret_KO9Y329U2U, $Item, CAP_KO9Y329U2U>
        ) -> Ret_KO9Y329U2U {
            let mut $array = $crate::maybe_uninit::uninit_array::<_, CAP_KO9Y329U2U>();
            let mut $length = 0usize;

            $crate::iter::eval!{
                $($rem)*
                ,for_each(|$item| {
                    $($reassign_item)*
                    if let $crate::__::CollectorCmd::BuildArray(teq) = cmd {
                        teq.reachability_hint(());

                        $elem_initer
                    }

                    $length += $elem_length;
                })
            }

            match cmd {
                $crate::__::CollectorCmd::ComputeLength(teq) => {
                    $crate::__::forget($array);
                    teq.to_right($length)
                }
                $crate::__::CollectorCmd::BuildArray(teq) => {
                    $crate::__::assert!{
                        $length == CAP_KO9Y329U2U,
                        "initialization was skipped somehow",
                    }

                    // SAFETY: The above assert ensures that
                    // all of the array is initialized
                    let $array = unsafe{ $crate::maybe_uninit::array_assume_init($array) };
                    teq.to_right($array)
                }
            }
        }

        const __COUNT81608BFNA5: $crate::__::usize =
            __func_zxe7hgbnjs($crate::__::MakeTypeWitness::MAKE);

        const __ARR81608BFNA5: [$Item; __COUNT81608BFNA5] =
            __func_zxe7hgbnjs($crate::__::MakeTypeWitness::MAKE);


    };
}

#[doc(hidden)]
pub enum CollectorCmd<Ret, T, const CAP: usize> {
    ComputeLength(TypeEq<usize, Ret>),
    BuildArray(TypeEq<[T; CAP], Ret>),
}

impl<Ret, T, const CAP: usize> TypeWitnessTypeArg for CollectorCmd<Ret, T, CAP> {
    type Arg = Ret;
}

impl<T> MakeTypeWitness for CollectorCmd<usize, T, 0> {
    const MAKE: Self = Self::ComputeLength(TypeEq::NEW);
}

impl<T, const CAP: usize> MakeTypeWitness for CollectorCmd<[T; CAP], T, CAP> {
    const MAKE: Self = Self::BuildArray(TypeEq::NEW);
}
