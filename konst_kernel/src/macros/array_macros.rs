use core::mem::MaybeUninit;

#[macro_export]
macro_rules! array_map {
    ($array:expr, $($closure:tt)* ) => (
        match $array {
            ref array => {
                let array = $crate::__::assert_array(array);

                $crate::utils::__parse_closure_1!{
                    ($crate::__array_map) (array, |i| array[i],) (array_map),
                    $($closure)*
                }
            }
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __array_map {
    (
        $array:ident,
        |$i:ident| $get_input:expr,
        ($($pattern:tt)*) $(-> $ret:ty)? $mapper:block $(,)?
    ) => ({
        let len = $array.len();
        let mut out = $crate::__::uninit_array_of_len(&$array);

        let mut $i = 0usize;
        while $i < len {
            let $($pattern)* = $get_input;
            out[$i] = $crate::__::MaybeUninit $(::<$ret>)? ::new($mapper);
            $i += 1;
        }
        // protecting against malicious `$mapper`s that break out of the `while` loop
        $crate::__::assert!($i == len);

        unsafe{
            $crate::__::array_assume_init(out)
        }
    })
}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! array_from_fn {
    ($($args:tt)*) => ({
        $crate::__split_array_type_and_closure!{
            (($crate::__array_from_fn_inner) ())
            ()
            ($($args)*)
        }
    });
}

#[macro_export]
macro_rules! __array_from_fn_inner {
    (($($($type:tt)+)?) $($closure_unparsed:tt)*) => ({
        let input = $crate::__::unit_array();

        let arr $(: $crate::__unparenthesize_ty!($($type)*))? =
            $crate::utils::__parse_closure_1!{
                ($crate::__array_map) (input, |i| i,) (array_from_fn),
                $($closure_unparsed)*
            };

        arr
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! __split_array_type_and_closure {
    ((($($callback:tt)*) ($($args:tt)*)) $before:tt (=> $($rem:tt)*)) => {
        $($callback)* ! {$($args)* $before $($rem)*}
    };
    ((($($callback:tt)*) ($($args:tt)*)) ($($before:tt)*) ($(| $($rem:tt)*)?)) => {
        $($callback)* ! {$($args)* () $($before)* $(| $($rem)*)?}
    };
    ($callback:tt ($($before:tt)*) ($token:tt $($rem:tt)*)) => {
        $crate::__split_array_type_and_closure! {$callback ($($before)* $token) ($($rem)*)}
    };
}

////////////////////////////////////////////////////////////////////////////////

#[inline(always)]
pub const fn assert_array<T, const N: usize>(array: &[T; N]) -> &[T; N] {
    array
}

#[inline(always)]
pub const fn uninit_array_of_len<T, U, const N: usize>(_input: &[T; N]) -> [MaybeUninit<U>; N] {
    crate::maybe_uninit::uninit_array()
}

#[inline(always)]
pub const fn unit_array<const N: usize>() -> [(); N] {
    [(); N]
}
