#![expect(clippy::empty_loop)]

#[doc(hidden)]
#[inline(always)]
pub const fn split_array_ptr_len<T, U, const N: usize>(
    ptr: *mut U,
    _phantom: core::marker::PhantomData<fn([T; N]) -> [T; N]>,
) -> (*mut T, usize) {
    (ptr.cast(), N)
}

#[doc(hidden)]
#[inline(always)]
pub const fn fake_read_array_ref<T, const N: usize>(_ptr: &[T; N]) -> [T; N] {
    loop {}
}

//////

/// TODO
#[macro_export]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "konst_proc_macros")))]
macro_rules! destructure_rec {
    ($($tt:tt)*) => (
        $crate::__::__destructure__unwrap_pats!{$crate () $($tt)*}
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructure_rec__inner {
    (
        $(#[forget_ignored_fields $(@$forget_ignored_fields:tt)?])?
        {$patterns:tt}
        $(: $type:ty )?
        = $expr:expr
    ) => (
        let mut val $(: $crate::__::ManuallyDrop<$type>)? = $crate::__::ManuallyDrop::new($expr);

        let ptr: *mut _ = $crate::macros::destructuring::cast_manuallydrop_ptr(&raw mut val);

        $crate::__destructure_rec__recursive! {
            [
                $(#[forget_ignored_fields $(@$forget_ignored_fields)?])?
            ]
            [
                $(#[forget_ignored_fields $(@$forget_ignored_fields)?])?
            ]

            {ptr}

            $patterns
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructure_rec__recursive {
    (
        $fixed:tt $fixed2:tt {$ptr:expr}
        ( $pattern:tt binding )
    ) => {
        let $pattern = unsafe { <*mut _>::read_unaligned($ptr) };
    };

    (
        $fixed:tt [] $ptr:tt
        ( $pattern:tt struct $path:tt $fields:tt ($($dotdot:tt)*) )
    ) => {
        $crate::__::compile_error!{
            "`..` patterns are not supported in struct patterns by default,\
             because they can forget fields\
            "
        }
    };

    (
        $fixed:tt
        [ $(#[forget_ignored_fields])? ]
        {$ptr:expr}

        (
            $pattern:tt
            struct
            ($path:path)
            {$($field_name:tt $field_pat:tt,)*}
            $(($($dotdot:tt)*))?
        )
    ) => {
        let ptr = $ptr;

        // assert that `*ptr` is a struct, not a reference to a struct,
        // and that it has all the fields the user listed.
        #[allow(unreachable_code)]
        if false {
            loop {}

            let expected @ $path {$($field_name: _,)* $($($dotdot)*)?};

            // SAFETY: dead code
            let read_out = unsafe {
                <*mut _>::read_unaligned(ptr)
            };

            $crate::macros::destructuring::assert_same_type(expected, read_out);

            _ = ||{
                use $crate::macros::destructuring::__GetImplsHelper as _;

                // assert that the struct doesn't impl Drop
                // (its fields can, just not the struct itself)
                let _assertion_expected: $crate::macros::destructuring::__DoesNotImplDrop<_> =
                    if false {
                        $crate::macros::destructuring::__DoesNotImplDrop::new(ptr)
                    } else {
                        let assertion = $crate::macros
                            ::destructuring
                            ::__GetImpls_IWRHQLPNNIEU8C6W(
                                $crate::macros::destructuring::make_phantom(ptr)
                            ).__impls_drop_iwrhqlpnnieu8c6w();

                        assertion
                    };
            };
        }

        $(
            $crate::__destructure_rec__recursive! {
                $fixed
                $fixed
                {unsafe { &raw mut (*ptr).$field_name }}
                $field_pat
            }
        )*
    };

    (
        $fixed:tt [] $ptr:tt
        ( $pattern:tt tuple $fields:tt $dotdot:tt ())
    ) => {
        $crate::__::compile_error!{
            "`..` patterns are not supported in tuple patterns by default,\
             because they can forget fields\
            "
        }
    };

    (
        $fixed:tt [] $ptr:tt
        ( $pattern:tt tuple $fields:tt $dotdot:tt $suffix_fields:tt)
    ) => {
        $crate::__::compile_error!{
            "tuple patterns do not support `..` with trailing fields"
        }
    };

    (
        $fixed:tt
        [ $(#[forget_ignored_fields])? ]
        {$ptr:expr}

        (
            $pattern:tt
            tuple
            ($($field:tt $field_pat:tt,)*)
            $(
                ($($dotdot:tt)*)
                ($($suffix_fields:tt)*)
            )?
        )
    ) => {
        let ptr = $ptr;

        // assert that `*ptr` is a tuple, not a reference to a tuple
        #[allow(unreachable_code)]
        if false {
            loop {}

            let expected @ (
                $($crate::__first_pat!(_, $field),)*
                $($crate::__first_pat!(_, $($dotdot)*),)?
            );

            // SAFETY: dead code
            let read_out = unsafe {
                <*mut _>::read_unaligned(ptr)
            };

            $crate::macros::destructuring::assert_same_type(expected, read_out)
        }

        $(
            $crate::__destructure_rec__recursive! {
                $fixed
                $fixed
                {unsafe { &raw mut (*ptr).$field }}
                $field_pat
            }
        )*
    };

    (
        $fixed:tt
        $fixed2:tt
        {$ptr:expr}

        (
            $pattern:tt
            array
            ($($pre_index:tt $pre_pat:tt,)*)
            $(
                ($rem_index:tt $($rem_pat:tt)*)
                ($($post_index:tt $post_pat:tt,)*)
            )?
        )
    ) => {
        let ptr = $ptr;

        let arr_type_len_phantom = $crate::__::PhantomData;
        $(  let $crate::__first_pat!(rem_ty_phantom, $($rem_pat)*) = $crate::__::PhantomData; )?

        // asserts the length of the array,
        // and computes the length of the array produced by `@ ..` patterns
        #[allow(unreachable_code)]
        if false {
            loop {}

            let [
                $($crate::__first_pat!(_, $pre_pat),)*
                $(
                    rem @ ..,
                    $($crate::__first_pat!(_, $post_pat),)*
                )?
            ] = unsafe {

                // assert that `*ptr` is an array, not a reference to an array
                arr_type_len_phantom = $crate::macros::destructuring::array_into_phantom({
                    let array = <*mut _>::read_unaligned(ptr);
                    array
                });

                // SAFETY: unreachable code
                $crate::macros::destructuring_rec::fake_read_array_ref(&*ptr)
            };

            $(
                rem_ty_phantom = $crate::macros::destructuring::array_into_phantom(
                    $crate::__first_expr!(rem, $($rem_pat)*)
                );
            )?

        }

        let (ptr_elem, len) = $crate::macros::destructuring_rec::split_array_ptr_len(
            ptr,
            arr_type_len_phantom,
        );

        $(
            $crate::__destructure_rec__recursive! {
                $fixed $fixed
                {unsafe { <*mut _>::add(ptr_elem, $pre_index) }}
                $pre_pat
            }
        )*

        $(

            // SAFETY: the array being wrapped in a ManuallyDrop,
            //         and the assertions above, ensure that these reads are safe.
            let $($rem_pat)* = unsafe {
                let rem_ptr = $crate::macros::destructuring::cast_ptr_with_phantom(
                    <*mut _>::add(ptr_elem, $rem_index),
                    rem_ty_phantom,
                );

                <*mut _>::read_unaligned(rem_ptr)
            };

            $(
                $crate::__destructure_rec__recursive! {
                    $fixed $fixed
                    {unsafe { <*mut _>::add(ptr_elem, len.wrapping_sub($post_index)) }}
                    $post_pat
                }
            )*
        )?
    };
}
