#![expect(non_camel_case_types)]

use core::marker::PhantomData;
use core::mem::ManuallyDrop;

#[doc(hidden)]
pub trait __GetImplsHelper {
    type T: ?Sized;

    fn __impls_drop_iwrhqlpnnieu8c6w(&self) -> __DoesNotImplDrop<Self::T>;
}

//////

#[doc(hidden)]
pub struct __ImplsDrop<T: ?Sized>(PhantomData<fn() -> T>);

//////

#[doc(hidden)]
pub struct __DoesNotImplDrop<T: ?Sized>(PhantomData<fn() -> T>);

impl<T: ?Sized> __DoesNotImplDrop<T> {
    pub const fn new(_: *mut T) -> Self {
        Self(PhantomData)
    }
}

//////

#[doc(hidden)]
pub struct __GetImpls_IWRHQLPNNIEU8C6W<T: ?Sized>(pub PhantomData<T>);

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
pub const fn fake_read<T>(_: *mut T) -> T {
    loop {}
}

#[doc(hidden)]
#[inline(always)]
pub const fn make_phantom<T>(_: *mut T) -> PhantomData<T> {
    PhantomData
}

//////

/// Destructures a struct/tuple/array into all of its elements/fields.
///
/// # Motivation
///
/// This macro works around a limitation as of Rust 1.83,
/// where non-Copy type can't be destructured into its elements/fields in a const context.
///
/// # Requirements
///
/// This macro has these requirements:
/// - it requires writing all elements/fields,
/// because they would be leaked if they weren't mentioned.
/// - it requires that the passed-in type does not impl `Drop`
/// (like built-in destructuring does)
///
/// # Example
///
/// ### Braced Struct
///
/// This example demonstrates destructuring a non-Copy struct in const,
/// which can't be done with built-in destructuring as of Rust 1.83.
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
#[macro_export]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_83")))]
macro_rules! destructure {
    (
        $struct_path:path $(,)? {$($braced:tt)*} $($rem:tt)*
    ) => (
        $crate::__destructure_struct! {$struct_path, {$($braced)*} $($rem)*}
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! __destructure_struct {
    (
        $struct_path:path, {$($field:tt $(: $pattern:pat)?),* $(,)?}
        $(:$struct_ty:ty)?
        = $val:expr
    ) => (

        // assert that `$struct_path` has precisely the fields that the user listed
        let val @ $struct_path {$($field: _),*} $(: $struct_ty)? = $val;
        let mut val = $crate::__::ManuallyDrop::new(val);

        let ptr = $crate::macros::destructuring::cast_manuallydrop_ptr(&raw mut val);

        if false {
            _ = ||{
                use $crate::macros::destructuring::{__GetImplsHelper as _};

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
    )
}

#[macro_export]
#[doc(hidden)]
macro_rules! __first_pat {
    ($first:pat, $($rem:tt)* ) => {
        $first
    };
}
