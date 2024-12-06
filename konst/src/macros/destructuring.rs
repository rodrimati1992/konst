#![expect(non_camel_case_types)]

use core::marker::PhantomData;
use core::mem::ManuallyDrop;

//////

#[doc(hidden)]
pub trait __GetImplsHelper {
    type T: ?Sized;

    fn __impls_drop_iwrhqlpnnieu8c6w(&self) -> __DoesNotImplDrop<Self::T>;
}

//////

#[doc(hidden)]
pub struct __ImplsDrop<T: ?Sized>(PhantomData<fn(T) -> T>);

//////

#[doc(hidden)]
pub struct __DoesNotImplDrop<T: ?Sized>(PhantomData<fn(T) -> T>);

impl<T: ?Sized> __DoesNotImplDrop<T> {
    pub const fn new(_: *mut T) -> Self {
        Self(PhantomData)
    }
}

//////

#[doc(hidden)]
pub struct __GetImpls_IWRHQLPNNIEU8C6W<T: ?Sized>(pub PhantomData<fn(T) -> T>);

impl<T> __GetImplsHelper for __GetImpls_IWRHQLPNNIEU8C6W<T> {
    type T = T;

    fn __impls_drop_iwrhqlpnnieu8c6w(&self) -> __DoesNotImplDrop<Self::T> {
        __DoesNotImplDrop(PhantomData)
    }
}

#[expect(drop_bounds)]
impl<T> __GetImpls_IWRHQLPNNIEU8C6W<T>
where
    // I really do mean to test for Drop,
    // because it's to destructure a struct into its fields
    T: Drop,
{
    pub fn __impls_drop_iwrhqlpnnieu8c6w(self) -> __ImplsDrop<T> {
        __ImplsDrop(PhantomData)
    }
}

//////

#[doc(hidden)]
#[inline(always)]
pub const fn cast_manuallydrop_ptr<T>(ptr: *mut ManuallyDrop<T>) -> *mut T {
    ptr.cast()
}

#[doc(hidden)]
#[inline(always)]
/// Gets a pointer to the first elem in a ManuallyDrop array
pub const fn cast_manuallydrop_array_ptr<T, const N: usize>(
    ptr: *mut ManuallyDrop<[T; N]>,
) -> *mut T {
    ptr.cast()
}

#[doc(hidden)]
#[inline(always)]
pub const fn cast_ptr_with_phantom<T, U>(ptr: *mut T, _phantom: PhantomData<fn(U) -> U>) -> *mut U {
    ptr.cast()
}

#[doc(hidden)]
#[inline(always)]
pub const fn get_phantom_len<T, const N: usize>(
    _phantom: PhantomData<fn([T; N]) -> [T; N]>,
) -> usize {
    N
}

#[doc(hidden)]
#[inline(always)]
pub const fn make_it<T>() -> T {
    loop {}
}

#[doc(hidden)]
#[inline(always)]
pub const fn fake_read<T>(_: *mut T) -> T {
    loop {}
}

#[doc(hidden)]
#[inline(always)]
pub const fn make_phantom<T>(_: *mut T) -> PhantomData<fn(T) -> T> {
    PhantomData
}

#[doc(hidden)]
#[inline(always)]
pub const fn array_into_phantom<T, const N: usize>(
    val: [T; N],
) -> PhantomData<fn([T; N]) -> [T; N]> {
    core::mem::forget(val);
    PhantomData
}

#[doc(hidden)]
#[inline(always)]
pub const fn assert_same_type<T>(this: T, that: T) {
    core::mem::forget(this);
    core::mem::forget(that);
}

//////

#[doc(hidden)]
pub type __ArrayManuallyDrop<T, const LEN: usize> = ManuallyDrop<[T; LEN]>;

//////

/// Destructures a struct/tuple/array into all of its elements/fields.
///
/// [**for examples look here**](#examples)
///
/// # Motivation
///
/// This macro works around a limitation of Rust as of 1.83,
/// where in a const context, a non-`Drop` type can't be destructured into its elements/fields
/// if any of them is `Drop`.
///
/// Even simple cases like this don't compile:
///
/// ```rust,compile_fail
/// const fn foo<T>((a, b): (T, T)) -> [T; 2] {
///     [a, b]
/// }
/// ```
///
/// ```text
/// error[E0493]: destructor of `(T, T)` cannot be evaluated at compile-time
///  --> src/lib.rs:1:17
///   |
/// 1 | const fn foo<T>((a, b): (T, T)) -> [T; 2] {
///   |                 ^^^^^^ the destructor for this type cannot be evaluated in constant functions
/// 2 |     [a, b]
/// 3 | }
///   | - value is dropped here
/// ```
///
/// # Requirements/Limitations
///
/// This macro has these requirements and limitations:
/// - it does not support `..` patterns in tuples or structs
/// (because unmentioned fields would be leaked),
/// but `..` patterns are supported in arrays.
/// - it requires that passed-in structs do not impl `Drop`
/// (like built-in destructuring does),
/// but any field can impl `Drop`.
/// - it needs to be invoked multiple times
/// to destructure nested structs/tuples/arrays that have `Drop` elements/fields.
/// [(example)](#nested-destructuring)
/// - it only supports tuple structs and tuples up to 16 elements (inclusive)
///
/// # Syntax
///
/// This section uses a pseudo-macro_rules syntax for each type of input.
///
/// ### Braced structs
///
/// ```text
/// $struct_path:brace_path {$($field:tt $(: $pattern:pat)?),* $(,)?}
/// $(:$struct_ty:ty)?
/// = $val:expr
/// ```
///
/// Where `:brace_path` can be either of:
/// - `$(::)? $($path:ident)::* $(,)?`
/// - `$struct_path:path $(,)?`
///
/// [example below](#braced-struct)
///
/// ### Tuple structs
///
/// ```text
/// $struct_path:tuple_path ( $($pattern:pat),* $(,)? )
/// $(:$struct_ty:ty)?
/// = $val:expr
/// ```
///
/// Where `:tuple_path` can be either of:
/// - `$(::)? $($path:ident)::* $(,)?`
/// - `$struct_path:path ,` (braced struct patterns don't need the `,`)
///
/// [example below](#tuple-struct)
///
/// ### Tuples
///
/// ```text
/// ( $($pattern:pat),* $(,)? ) $(:$tuple_ty:ty)? = $val:expr
/// ```
/// [example below](#tuple)
///
/// ### Arrays
///
/// ```text
/// [$( $pat:elem_pat $(@ ..)? ),* $(,)?] $(:$array_ty:ty)? = $val:expr
///
/// Where `:elem_pat` can be any of:
/// - `_`
/// - `..`
/// - `$ident:ident`
/// - `($pattern:pat)`: any pattern inside of parentheses
/// ```
/// [example below](#array)
///
/// # Examples
///
/// These examples demonstrate destructuring non-Copy types in const,
/// which can't be done with built-in destructuring as of Rust 1.83.
///
/// ### Braced Struct
///
/// ```rust
/// use std::ops::Range;
///
/// assert_eq!(PAIR, (3, 5));
///
/// const PAIR: (u32, u32) = range_to_pair(3..5);
///
/// const fn range_to_pair<T>(range: Range<T>) -> (T, T) {
///     konst::destructure!{Range{start, end} = range}
///
///     (start, end)
/// }
/// ```
///
/// ### Tuple Struct
///
/// ```rust
///
/// assert_eq!(PAIR, [8, 13]);
///
/// const PAIR: [u32; 2] = Pair(8, 13).into_inner();
///
/// struct Pair<T>(T, T);
///
/// impl<T> Pair<T> {
///     const fn into_inner(self) -> [T; 2] {
///         konst::destructure!{Self(first, second) = self}
///     
///         [first, second]
///     }
/// }
/// ```
///
/// ### Tuple
///
/// ```rust
///
/// assert_eq!(PAIR, (5, String::new()));
///
/// const PAIR: (u32, String) = swap_pair((String::new(), 5));
///
/// const fn swap_pair<T, U>(pair: (T, U)) -> (U, T) {
///     konst::destructure!{(a, b) = pair}
///
///     (b, a)
/// }
/// ```
///
/// ### Array
///
/// ```rust
///
/// assert_eq!(SPLIT, (Some(String::new()), [None, None, Some(String::new())]));
///
/// const SPLIT: (Option<String>, [Option<String>; 3]) =
///     split_first([Some(String::new()), None, None, Some(String::new())]);
///
/// const fn split_first<T>(array: [T; 4]) -> (T, [T; 3]) {
///     konst::destructure!{[a, rem @ ..] = array}
///     
///     (a, rem)
/// }
/// ```
///
/// ### Nested Destructuring
///
/// ```rust
///
/// assert_eq!(TRIPLE, [3, 5, 8]);
///
/// const TRIPLE: [u8; 3] = flatten((3, (5, 8)));
///
/// const fn flatten<T>(tup: (T, (T, T))) -> [T; 3] {
///     // `tail` can't be destructured inline into `(b, c)`,
///     // it must be destructured separately
///     konst::destructure!{(a, tail) = tup}
///     
///     konst::destructure!{(b, c) = tail}
///
///     [a, b, c]
/// }
///
///
#[macro_export]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_83")))]
macro_rules! destructure {
    // braced struct struct
    ($($(@$is_path:tt)? ::)? $($path:ident)::+ $(,)?{$($braced:tt)*} $($rem:tt)*) => (
        $crate::__destructure__braced_struct_prepare_fields! {
            {$($braced)*}
            ($($(@$is_path)? ::)? $($path)::*)
            path
            $($rem)*
        }
    );

    // tuple struct
    ($($(@$is_path:tt)? ::)? $($path:ident)::+ $(,)? ($($tupled:tt)*) $($rem:tt)*) => (
        $crate::__destructure__tuple_struct_field_names!{
            (
                ($($(@$is_path)? ::)? $($path)::+)
                path
                $($rem)*
            )
            ()
            ($($tupled)*)
            (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15)
        }
    );

    // braced struct struct
    ($struct_path:path $(,)? {$($braced:tt)*} $($rem:tt)*) => (
        $crate::__destructure__braced_struct_prepare_fields! {
            {$($braced)*}
            ($struct_path)
            type
            $($rem)*
        }
    );

    // tuple struct
    ($struct_path:path, ($($tupled:tt)*) $($rem:tt)*) => (
        $crate::__destructure__tuple_struct_field_names!{
            (
                ($struct_path)
                type
                $($rem)*
            )
            ()
            ($($tupled)*)
            (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15)
        }
    );

    // tuple
    (() $(:$tuple_ty:ty)? = $val:expr) => (
        let () $(: $tuple_ty)? = $val;
    );
    (($($tupled:tt)*) $($rem:tt)*) => (
        $crate::__destructure__tuple_field_names!{
            ($($rem)*)
            ()
            ($($tupled)*)
            (0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15)
        }
    );

    // array
    ([] $(:$array_ty:ty)? = $val:expr) => (
        let [] $(: $array_ty)? = $val;
    );

    (
        [$( $pat:tt $(@ $dotdot:tt)? ),* $(,)?]

        $($rem:tt)*
    ) => (
        $crate::__destructure_array__process_fields!{
            [$( ($pat) ($($dotdot $dotdot)?) ,)*]

            $($rem)*
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructure__braced_struct_prepare_fields {
    (
        {$($field:tt $(: $pattern:pat)?),* $(,)?}
        $($rem:tt)*
    ) => {
        $crate::__destructure_struct!{
            {$(($field) $field $(: $pattern)?,)*}

            $($rem)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructure__tuple_struct_field_names {
    (($($rem:tt)*) ($($patterns:tt)*) () ($($fnames:tt)*)) => {
        $crate::__destructure_struct!{
            {$($patterns)*}
            $($rem)*
        }
    };
    ($fixed:tt $prev_patterns:tt (.. $($next_pattern:tt)*) $fnames:tt) => {
        $crate::__::compile_error!{
            "`..` patterns are not supported in top-level tuple struct patterns"
        }
    };
    (
        $fixed:tt
        ($($prev_patterns:tt)*)
        ($pattern:pat $(, $($next_pattern:tt)*)?)
        ($fname:tt $($next_fnames:tt)*)
    ) => {
        $crate::__destructure__tuple_struct_field_names!{
            $fixed
            ($($prev_patterns)* ($fname) $fname:$pattern,)
            ($($($next_pattern)*)?)
            ($($next_fnames)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructure_struct {
    (
        { /* no fields */ $(,)? }
        ($($struct_path:tt)*)
        $path_kind:ident
        $(:$struct_ty:ty)?
        = $val:expr
    ) => (
        let $($struct_path)* {} $(: $struct_ty)? = $val;
    );
    (
        {
            $(($($_fa0:ident)? $($_fa1:literal)?) $field:tt $(: $pattern:pat)?),*
            $(,)?
        }
        ($($struct_path:tt)*)
        $path_kind:ident
        $(:$struct_ty:ty)?
        = $val:expr
    ) => (
        // assert that `$struct_path` has precisely the fields that the user listed
        let val @ $($struct_path)* {$($field: _),*} $(: $struct_ty)? = $val;

        // asserts that `val` is not a reference,
        // protects against match ergonomics allowing `$val` to be a references.
        //
        // This always uses `$($struct_path)*`, even if `$struct_ty` is passed,
        // because:
        // - if this tested the type using `$struct_ty`,
        //   it would allow passing a reference to a struct when `$struct_ty == &Struct`.
        // - `$($struct_path)*` is guaranteed to be a struct due to it being used
        //   in the pattern above
        $crate::__destructuring__type_assert!{($path_kind $($struct_path)*) val}

        let mut val = $crate::__::ManuallyDrop::new(val);

        let ptr: *mut _ = $crate::macros::destructuring::cast_manuallydrop_ptr(&raw mut val);


        if false {
            _ = ||{
                use $crate::macros::destructuring::__GetImplsHelper as _;

                // assert that the struct doesn't impl Drop
                // (its fields can, just not the struct itself)
                let assertion_expected: $crate::macros::destructuring::__DoesNotImplDrop<_> =
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
            // SAFETY: the value being wrapped in a ManuallyDrop,
            //         and the asserts above, ensure that these reads are safe.
            //
            // using `read_unaligned` to support destructuring packed structs
            let $crate::__first_pat!( $($pattern,)? $field, ) = unsafe {
                $crate::__::ptr::read_unaligned(&raw mut (*ptr).$field)
            };
        )*
    );
    (
        {
            $(($($_fa0:ident)? $($_fa1:literal)?) $field0:tt $(: $_0:pat)? ,)*
            $(
                (..) $field1:tt $(: $_1:pat)? ,
                $($__anything:tt)*
            )?
        }
        $($_3:tt)*
    ) => (
        compile_error!{"`..` patterns are not supported in top-level struct patterns"}
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructuring__type_assert {
    ((path $($path:tt)*) $variable:ident) => {
        // assert that `$variable` is a struct, not a reference to a struct
        #[allow(unreachable_code)]
        if false {
            loop {}

            let expected @ $($path)* {..};

            $crate::macros::destructuring::assert_same_type(expected, $variable)
        }
    };
    ((type $type:ty) $variable:ident) => {
        let _: $type = $variable;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructure__tuple_field_names {
    (($($rem:tt)*) ($($patterns:tt)*) () ($($fnames:tt)*)) => {
        $crate::__destructure_tuple!{($($patterns)*) $($rem)*}
    };
    ($fixed:tt $prev_patterns:tt (.. $($next_pattern:tt)*) $fnames:tt) => {
        $crate::__::compile_error!{"`..` patterns are not supported in top-level tuple patterns"}
    };
    (
        $fixed:tt
        ($($prev_patterns:tt)*)
        ($pattern:pat $(, $($next_pattern:tt)*)?)
        ($fname:tt $($next_fnames:tt)*)
    ) => {
        $crate::__destructure__tuple_field_names!{
            $fixed
            ($($prev_patterns)* $fname:$pattern,)
            ($($($next_pattern)*)?)
            ($($next_fnames)*)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructure_tuple {
    (($($field:tt: $pattern:pat,)*) $(:$tuple_ty:ty)? = $val:expr) => (

        // assert that the tuple has precisely the element count that the user passed
        //
        // $tuple_ty being passed is not enough of an assertion,
        // because it might be a tuple with more elements,
        // so we construct a pattern to assert it.
        let val @ ($($crate::__first_pat!(_, $field),)*)
            : $crate::__first_ty!($($tuple_ty,)? ($($crate::__first_ty!(_, $field),)*),)
            = $val;

        // assert that `val` is a tuple, not a reference to a tuple
        #[allow(unreachable_code)]
        if false {
            loop {}

            let expected @ ($($crate::__first_pat!(_, $field),)*);

            $crate::macros::destructuring::assert_same_type(expected, val)
        }

        let mut val = $crate::__::ManuallyDrop::new(val);

        let ptr = $crate::macros::destructuring::cast_manuallydrop_ptr(&raw mut val);

        $(
            // SAFETY: the value being wrapped in a ManuallyDrop,
            //         and the asserts above, ensure that these reads are safe.
            let $pattern = unsafe { $crate::__::ptr::read(&raw mut (*ptr).$field) };
        )*
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructure_array__process_fields {
    (
        [$(
            (
                $( _ $($is_underscore:lifetime)? )?
                $( $ident:ident)?
                $( ($parenthesized:pat) )?
                $( .. $($has_rem_elems1:lifetime)? )?
            )
            ($(.. $rem_elems:tt $($has_rem_elems2:lifetime)?)?),
        )*]

        $($rem:tt)*
    ) => {
        $crate::__destructure_array! {
            [$(
                (
                    (
                        $(rem $($has_rem_elems1)?)?
                        $(rem $($has_rem_elems2)?)?
                        elem
                    )

                    $(_ $($is_underscore)?)?
                    $(_ $($has_rem_elems1)?)?
                    $( $ident )?
                    $( $parenthesized )?
                )
            )*]

            $($rem)*
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructure_array {
    (
        [
            $( ((elem $($__0:tt)*) $pat_prefix:tt) )*
            $(
                ((rem $($__1:tt)*) $pat_rem:tt)

                $( ((elem $($__2:tt)*) $pat_suffix:tt) )*
            )?
        ]

        $(:$array_ty:ty)?
        = $val:expr
    ) => {
        let array $(: $array_ty)? = $val;

        $(  let $crate::__first_pat!(rem_ty_phantom, $pat_rem) = $crate::__::PhantomData; )?

        // asserts the length of the array,
        // and computes the length of the array produced by `@ ..` patterns
        #[allow(unreachable_code)]
        if false {
            loop {}

            // assert that `array` is an array, not a reference to an array
            _ = $crate::macros::destructuring::array_into_phantom(array);

            let [
                $($crate::__first_pat!(_, $pat_prefix),)*
                $(
                    $crate::__first_pat!(rem @ .., $pat_rem),
                    $($crate::__first_pat!(_, $pat_suffix),)*
                )?
            ] = array;

            $(
                rem_ty_phantom = $crate::macros::destructuring::array_into_phantom(
                    $crate::__first_expr!(rem, $pat_rem)
                );
            )?

        }

        let mut array = $crate::macros::destructuring::__ArrayManuallyDrop::new(array);

        let ptr = $crate::macros::destructuring::cast_manuallydrop_array_ptr(&raw mut array);
        let mut i = 0;


        $crate::__destructure_array__read_elems!{unsafe, ptr, i, [$($pat_prefix),*]}

        $(

            // SAFETY: the array being wrapped in a ManuallyDrop,
            //         and the assertions above, ensure that these reads are safe.
            let $pat_rem = unsafe {
                let rem_ptr = $crate::macros::destructuring::cast_ptr_with_phantom(
                    <*mut _>::add(ptr, i),
                    rem_ty_phantom,
                );

                $crate::__::ptr::read(rem_ptr)
            };

            i += $crate::macros::destructuring::get_phantom_len(rem_ty_phantom);


            $crate::__destructure_array__read_elems!{unsafe, ptr, i, [$($pat_suffix),*]}
        )?
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructure_array__read_elems {
    ($unsafe:ident, $ptr:ident, $i:ident, [$($pattern:pat),*]) => {
        $(
            // SAFETY: the array being wrapped in a ManuallyDrop,
            //         and the assertions above, ensure that these reads are safe.
            let $pattern = $unsafe { $crate::__::ptr::read(<*mut _>::add($ptr, $i)) };

            $i += 1;
        )*
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __first_pat {
    ($first:pat, $($rem:tt)* ) => {
        $first
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __first_ty {
    ($first:ty, $($rem:tt)* ) => {
        $first
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __first_expr {
    ($first:expr, $($rem:tt)* ) => {
        $first
    };
}
