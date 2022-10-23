use core::mem::MaybeUninit;

#[macro_export]
macro_rules! array_map {
    ($array:expr, $($closure:tt)* ) => (
        $crate::utils::__alt_parse_closure_1!{
            ($crate::__array_map) ($array,) (array_map),
            $($closure)*
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __array_map {
    ($array:expr, |$param:tt $(: $type:ty)? $(,)?| $(-> $ret:ty)? $mapper:block $(,)? ) => {
        match $array {
            ref array => {
                let array = $crate::__::assert_array(array);
                let len = array.len();
                let mut out = $crate::__::uninit_copy_array_of_len(&array);

                let mut i = 0;
                while i < len {
                    let $param $(: $type)? = array[i];
                    out[i] = $crate::__::MaybeUninit $(::<$ret>)? ::new($mapper);
                    i += 1;
                }

                unsafe{
                    $crate::__::array_assume_init(out)
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[inline(always)]
pub const fn assert_array<T, const N: usize>(array: &[T; N]) -> &[T; N] {
    array
}

#[inline(always)]
pub const fn uninit_copy_array_of_len<T, U, const N: usize>(_input: &[T; N]) -> [MaybeUninit<U>; N]
where
    U: Copy,
{
    crate::maybe_uninit::uninit_array()
}
