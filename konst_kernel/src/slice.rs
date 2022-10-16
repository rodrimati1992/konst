#[doc(hidden)]
#[macro_export]
macro_rules! __slice_from_impl {
    ($slice:ident, $start:ident, $as_ptr:ident, $from_raw_parts:ident, $on_overflow:expr) => {{
        #[allow(unused_variables)]
        let (rem, overflowed) = $slice.len().overflowing_sub($start);

        if overflowed {
            return $on_overflow;
        }

        unsafe { core::slice::$from_raw_parts($slice.$as_ptr().offset($start as _), rem) }
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

/// Defines `konst_kernel::slice` items without docs
pub mod items {
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
}

#[macro_export]
#[doc(hidden)]
macro_rules! __slice_from_docs {
    ($mod:literal => $item:item) => {
        /// A const equivalent of `&slice[start..]`.
        ///
        /// If `slice.len() < start`, this simply returns an empty slice.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use ", $mod, "::slice_from;")]
        ///
        /// const FIBB: &[u16] = &[3, 5, 8, 13, 21, 34, 55, 89];
        ///
        /// const TWO: &[u16] = slice_from(FIBB, 2);
        /// const FOUR: &[u16] = slice_from(FIBB, 4);
        /// const ALL: &[u16] = slice_from(FIBB, 0);
        /// const NONE: &[u16] = slice_from(FIBB, 1000);
        ///
        /// assert_eq!(TWO, &[8, 13, 21, 34, 55, 89]);
        /// assert_eq!(FOUR, &[21, 34, 55, 89]);
        /// assert_eq!(ALL, FIBB);
        /// assert_eq!(NONE, &[]);
        ///
        /// ```
        $item
    };
}
__slice_from_docs! {
    "konst_kernel::slice" =>
    #[doc(inline)]
    pub use items::slice_from;
}

#[macro_export]
#[doc(hidden)]
macro_rules! __slice_up_to_docs {
    ($mod:literal => $item:item) => {
        /// A const equivalent of `&slice[..len]`.
        ///
        /// If `slice.len() < len`, this simply returns `slice` back.
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use ", $mod, "::slice_up_to;")]
        ///
        /// const FIBB: &[u16] = &[3, 5, 8, 13, 21, 34, 55, 89];
        ///
        /// const TWO: &[u16] = slice_up_to(FIBB, 2);
        /// const FOUR: &[u16] = slice_up_to(FIBB, 4);
        /// const NONE: &[u16] = slice_up_to(FIBB, 0);
        /// const ALL: &[u16] = slice_up_to(FIBB, 1000);
        ///
        /// assert_eq!(TWO, &[3, 5]);
        /// assert_eq!(FOUR, &[3, 5, 8, 13]);
        /// assert_eq!(NONE, &[]);
        /// assert_eq!(ALL, FIBB);
        ///
        /// ```
        $item
    };
}

__slice_up_to_docs! {
    "konst_kernel::slice" =>
    #[doc(inline)]
    pub use items::slice_up_to;
}

#[macro_export]
#[doc(hidden)]
macro_rules! __slice_range_docs {
    ($mod:literal => $item:item) => {
        /// A const equivalent of `&slice[start..end]`.
        ///
        /// If `start >= end ` or `slice.len() < start `, this returns an empty slice.
        ///
        /// If `slice.len() < end`, this returns the slice from `start`.
        ///
        /// # Alternatives
        ///
        /// For a const equivalent of `&slice[start..]` there's [`slice_from`].
        ///
        /// For a const equivalent of `&slice[..end]` there's [`slice_up_to`].
        ///
        /// # Example
        ///
        /// ```rust
        #[doc = concat!("use ", $mod, "::slice_range;")]
        ///
        /// const FIBB: &[u16] = &[3, 5, 8, 13, 21, 34, 55, 89];
        ///
        /// const TWO: &[u16] = slice_range(FIBB, 2, 4);
        /// const FOUR: &[u16] = slice_range(FIBB, 4, 7);
        /// const NONE: &[u16] = slice_range(FIBB, 0, 0);
        /// const ALL: &[u16] = slice_range(FIBB, 0, 1000);
        ///
        /// assert_eq!(TWO, &[8, 13]);
        /// assert_eq!(FOUR, &[21, 34, 55]);
        /// assert_eq!(NONE, &[]);
        /// assert_eq!(ALL, FIBB);
        ///
        /// ```
        $item
    };
}

__slice_up_to_docs! {
    "konst_kernel::slice" =>
    #[doc(inline)]
    pub use items::slice_range;
}
