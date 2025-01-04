
#[doc(hidden)]
#[macro_export]
macro_rules! __slice_from_impl {
    ($slice:ident, $start:ident, $split_at:ident, $on_overflow:expr) => {{
        #[allow(unused_variables, clippy::ptr_offset_with_cast)]
        let (_, overflowed) = $slice.len().overflowing_sub($start);

        if overflowed {
            return $on_overflow;
        }

        $slice.$split_at($start as _).1
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __slice_up_to_impl {
    ($slice:ident, $len:ident, $split_at:ident, $on_overflow:expr) => {{
        #[allow(unused_variables)]
        let (_, overflowed) = $slice.len().overflowing_sub($len);

        if overflowed {
            return $on_overflow;
        }

        $slice.$split_at($len as _).0
    }};
}

#[inline]
pub const fn slice_from<T>(slice: &[T], start: usize) -> &[T] {
    crate::__slice_from_impl!(slice, start, split_at, &[])
}

#[inline]
pub const fn slice_up_to<T>(slice: &[T], len: usize) -> &[T] {
    crate::__slice_up_to_impl!(slice, len, split_at, slice)
}

#[inline]
pub const fn slice_range<T>(slice: &[T], start: usize, end: usize) -> &[T] {
    slice_from(slice_up_to(slice, end), start)
}
