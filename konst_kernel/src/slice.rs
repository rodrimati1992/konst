#[cfg(feature = "__for_konst")]
pub mod slice_for_konst;

#[cfg(feature = "__for_konst")]
pub use self::slice_for_konst::*;

#[doc(hidden)]
#[macro_export]
macro_rules! __slice_from_impl {
    ($slice:ident, $start:ident, $as_ptr:ident, $from_raw_parts:ident, $on_overflow:expr) => {{
        #[allow(unused_variables, clippy::ptr_offset_with_cast)]
        let (rem, overflowed) = $slice.len().overflowing_sub($start);

        if overflowed {
            return $on_overflow;
        }

        #[allow(clippy::ptr_offset_with_cast)]
        unsafe {
            core::slice::$from_raw_parts($slice.$as_ptr().offset($start as _), rem)
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __slice_up_to_impl {
    ($slice:ident, $len:ident, $as_ptr:ident, $from_raw_parts:ident, $on_overflow:expr) => {{
        #[allow(unused_variables)]
        let (rem, overflowed) = $slice.len().overflowing_sub($len);

        if overflowed {
            return $on_overflow;
        }

        // Doing this to get a slice up to length at compile-time
        unsafe { core::slice::$from_raw_parts($slice.$as_ptr(), $len) }
    }};
}

#[inline]
pub const fn slice_from<T>(slice: &[T], start: usize) -> &[T] {
    crate::__slice_from_impl!(slice, start, as_ptr, from_raw_parts, &[])
}

#[inline]
pub const fn slice_up_to<T>(slice: &[T], len: usize) -> &[T] {
    crate::__slice_up_to_impl!(slice, len, as_ptr, from_raw_parts, slice)
}

#[inline]
pub const fn slice_range<T>(slice: &[T], start: usize, end: usize) -> &[T] {
    slice_from(slice_up_to(slice, end), start)
}
