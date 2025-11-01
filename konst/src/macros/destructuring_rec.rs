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

/// Nested destructuring of structs/tuples/arrays (composite types).
///
/// Use this macro over [`destructure`] if you need to destructure composite types
/// into *nested* fields that may need dropping.
///
/// [**for examples look here**](#examples)
///
/// # Motivation
///
/// This macro works around a limitation of Rust as of 1.91,
/// where in a const context, a non-`Drop` type can't be destructured into its elements/fields
/// if any of them is `Drop`.
///
/// Even simple cases like this don't compile:
///
/// ```rust,compile_fail
/// const fn foo<T>(((a, b), c): ((T, T), T)) -> [T; 3] {
///     [a, b, c]
/// }
/// ```
///
/// ```text
/// error[E0493]: destructor of `((T, T), T)` cannot be evaluated at compile-time
///  --> src/lib.rs:1:17
///   |
/// 1 | const fn foo<T>(((a, b), c): ((T, T), T)) -> [T; 3] {
///   |                 ^^^^^^^^^^^ the destructor for this type cannot be evaluated in constant functions
/// 2 |     [a, b, c]
/// 3 | }
///   | - value is dropped here
/// ```
///
/// # Requirements/Limitations
///
/// This macro has these requirements and limitations:
/// - it only supports `..` patterns in tuples or structs
///   when the `#[forget_ignored_fields]` attribute is used,
///   which forgets unmentioned fields.
///   (arrays always support the `..` pattern, dropping unmentioned elements)
/// - it requires that passed-in structs do not impl `Drop`
///   (like built-in destructuring does).
/// - it always moves the expression on the right-hand-side,
///   even if the pattern doesn't require it.
///
/// # Syntax
///
/// This section uses a pseudo-macro_rules syntax, the allowed syntax is
/// ```text
/// $(#[forget_ignored_fields])?
/// $pattern:dr_pat $( : $type:ty )? = $val:expr
/// ```
///
/// `dr_pat` can be any of:
/// - `_ $( @ $nested_pat:dr_pat )?`
/// - `$(ref)? $(mut)? $ident:ident $( @ $nested_pat:dr_pat )?`
/// - `& $(mut)? $pat:pat`
/// - `$typename:path`
/// - `$typename:path { $($field_name:ident : $field_pat:dr_pat),* $(, ..)? $(,)? }`
/// - `$typename:path ( $($field_pat:dr_pat),* $(, ..)? $(,)? )`
/// - `[ $($pref_elem:dr_pat),* $(, $($ident:ident @)? ..)? $(, $suff_elem:dr_pat)* $(,)? ]`
/// - `( $($pref_elem:dr_pat),* $(, ..)?  $(,)? )`
///
/// The `#[forget_ignored_fields]` attribute turns the macro from forbidding
/// tuple and struct patterns that use a trailing `..` to allowing
/// the pattern, and to cause unmentioned fields to *not* be dropped.
///
/// # Examples
///
/// These examples demonstrate destructuring non-Copy types in const,
/// which can't be done with built-in destructuring as of Rust 1.91.
///
/// ### Braced Struct
///
/// ```rust
/// use std::ops::Range;
///
/// assert_eq!(TUP, (3, 5, 8, 13));
///
/// const TUP: (u32, u32, u32, u32) = ranges_to_tuple([3..5, 8..13]);
///
/// const fn ranges_to_tuple<T>(ranges: [Range<T>; 2]) -> (T, T, T, T) {
///     konst::destructure_rec!{
///         [Range { start: a, end: b }, Range { start: c, end: d }] = ranges
///     }
///
///     (a, b, c, d)
/// }
/// ```
///
/// ### Flatten
///
/// ```rust
/// assert_eq!(FLAT, [3, 5, 8, 13]);
///
/// const FLAT: [u32; 4] = flatten([[3, 5], [8, 13]]);
///
/// const fn flatten<T>(array: [[T; 2]; 2]) -> [T; 4] {
///     konst::destructure_rec!{[[a, b], [c, d]] = array}
///     
///     [a, b, c, d]
/// }
/// ```
///
/// [`destructure`]: crate::destructure
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
        = $expr:expr $(;)?
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
        // SAFETY:
        // ptr is a valid pointer to the type that is read out,
        // the pointee comes from a ManuallyDrop,
        // and it's only read out once.
        //
        // uses `read_unaligned` because this might be a pointer into a packed struct
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
                // uses `read_unaligned` because this might be a pointer into a packed struct
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
                {
                    // SAFETY: ptr is a pointer to a struct with the `$field_name` field
                    unsafe { &raw mut (*ptr).$field_name }
                }
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
                // uses `read_unaligned` because this might be a pointer into a packed struct
                <*mut _>::read_unaligned(ptr)
            };

            $crate::macros::destructuring::assert_same_type(expected, read_out)
        }

        $(
            $crate::__destructure_rec__recursive! {
                $fixed
                $fixed
                {
                    // SAFETY: ptr is a pointer to a tuple with the `$field` field
                    unsafe { &raw mut (*ptr).$field }
                }
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
                // SAFETY: unreachable code

                // assert that `*ptr` is an array, not a reference to an array
                arr_type_len_phantom = $crate::macros::destructuring::array_into_phantom({
                    // uses `read_unaligned` because this might be a pointer into a packed struct
                    let array = <*mut _>::read_unaligned(ptr);
                    array
                });

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
                {
                    // SAFETY:
                    // `ptr_elem` is a pointer into the start of an array of `len` elements.
                    // `$pre_index` is in-bounds for the array
                    unsafe { <*mut _>::add(ptr_elem, $pre_index) }
                }
                $pre_pat
            }
        )*

        $(

            // SAFETY:
            // the array being wrapped in a ManuallyDrop,
            // and the assertions above, ensure that this read is safe.
            let $($rem_pat)* = unsafe {
                let rem_ptr = $crate::macros::destructuring::cast_ptr_with_phantom(
                    <*mut _>::add(ptr_elem, $rem_index),
                    rem_ty_phantom,
                );

                // uses `read_unaligned` because this might be a pointer into a packed struct
                <*mut _>::read_unaligned(rem_ptr)
            };

            $(
                $crate::__destructure_rec__recursive! {
                    $fixed $fixed
                    {
                        // SAFETY:
                        // `ptr_elem` is a pointer into the start of an array of `len` elements.
                        //
                        // `len.wrapping_sub($post_index)` doesn't overflow because
                        // the `__destructure__unwrap_pats` macro passes in-bounds indices,
                        // and the `if false` above asserts the length of the array.
                        unsafe { <*mut _>::add(ptr_elem, len.wrapping_sub($post_index)) }
                    }
                    $post_pat
                }
            )*
        )?
    };
}
