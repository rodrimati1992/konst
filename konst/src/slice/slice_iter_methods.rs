use crate::{option, slice};

/// Gets a const iterator over `slice`, const equivalent of
/// [`<[T]>::iter`
/// ](https://doc.rust-lang.org/std/primitive.slice.html#method.iter)
///
/// # Example
///
/// ### Normal
///
/// ```rust
/// use konst::iter::for_each_i;
/// use konst::slice;
///
/// const ARR: &[usize] = &{
///     let mut arr = [0usize; 3];
///     for_each_i!{(i, elem) in slice::iter(&["foo", "hello", "That box"]) =>
///         arr[i] = elem.len();
///     }
///     arr
/// };
///
/// assert_eq!(ARR, [3, 5, 8]);
///
/// ```
///
/// ### Reversed
///
/// ```rust
/// use konst::iter::for_each_i;
/// use konst::slice;
///
/// const ARR: &[usize] = &{
///     let mut arr = [0usize; 3];
///     for_each_i!{(i, elem) in slice::iter(&["foo", "hello", "That box"]).rev() =>
///         arr[i] = elem.len();
///     }
///     arr
/// };
///
/// assert_eq!(ARR, [8, 5, 3]);
///
/// ```
pub const fn iter<T>(slice: &[T]) -> Iter<'_, T> {
    Iter { slice }
}

macro_rules! iter_shared {
    (is_forward = $is_forward:ident) => {
        iterator_shared! {
            is_forward = $is_forward,
            item = &'a T,
            iter_forward = Iter<'a, T>,
            iter_reversed = IterRev<'a, T>,
            next = iter_next,
            next_back = iter_next_back,
            fields = {slice},
        }

        /// Accesses the remaining slice.
        pub const fn as_slice(&self) -> &'a [T] {
            self.slice
        }
    };
}

macro_rules! iter_next {
    ($self:ident) => {
        if let [elem, rem @ ..] = $self.slice {
            $self.slice = rem;
            Some((elem, $self))
        } else {
            None
        }
    };
}

macro_rules! iter_next_back {
    ($self:ident) => {
        if let [rem @ .., elem] = $self.slice {
            $self.slice = rem;
            Some((elem, $self))
        } else {
            None
        }
    };
}

pub struct Iter<'a, T> {
    slice: &'a [T],
}

pub struct IterRev<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Iter<'a, T> {
    iter_shared! {is_forward = true}
}

impl<'a, T> IterRev<'a, T> {
    iter_shared! {is_forward = false}
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "rust_1_64")]
mod requires_rust_1_64 {
    use super::*;

    #[inline(always)]
    pub(crate) const fn some_if_nonempty<T>(slice: &[T]) -> Option<&[T]> {
        if let [] = slice {
            None
        } else {
            Some(slice)
        }
    }

    /// Const equivalent of
    /// [`<[T]>::windows`
    /// ](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::iter::for_each;
    /// use konst::slice;
    ///
    /// const fn is_sorted(slice: &[u8]) -> bool {
    ///     for_each!{window in slice::windows(slice, 2) =>
    ///         if window[0] > window[1] { return false; }
    ///     }
    ///     true
    /// }
    ///
    /// assert!(is_sorted(&[3, 5, 8]));
    /// assert!(!is_sorted(&[8, 13, 0]));
    ///
    ///
    ///
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
    #[track_caller]
    pub const fn windows<T>(slice: &[T], size: usize) -> Windows<'_, T> {
        assert!(size != 0, "window size must be non-zero");

        Windows { slice, size }
    }

    macro_rules! windows_shared {
        (is_forward = $is_forward:ident) => {
            iterator_shared! {
                is_forward = $is_forward,
                item = &'a [T],
                iter_forward = Windows<'a, T>,
                iter_reversed = WindowsRev<'a, T>,
                next = windows_next,
                next_back = windows_next_back,
                fields = {slice, size},
            }
        };
    }

    macro_rules! windows_next {
        ($self:ident) => {
            if $self.slice.len() < $self.size {
                None
            } else {
                let up_to = slice::slice_up_to($self.slice, $self.size);
                $self.slice = slice::slice_from($self.slice, 1);
                Some((up_to, $self))
            }
        };
    }

    macro_rules! windows_next_back {
        ($self:ident) => {
            let len = $self.slice.len();
            if len < $self.size {
                None
            } else {
                let up_to = slice::slice_from($self.slice, len - $self.size);
                $self.slice = slice::slice_up_to($self.slice, len - 1);
                Some((up_to, $self))
            }
        };
    }

    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
    pub struct Windows<'a, T> {
        slice: &'a [T],
        size: usize,
    }

    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
    pub struct WindowsRev<'a, T> {
        slice: &'a [T],
        size: usize,
    }

    impl<'a, T> Windows<'a, T> {
        windows_shared! {is_forward = true}
    }

    impl<'a, T> WindowsRev<'a, T> {
        windows_shared! {is_forward = false}
    }

    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    /// Const equivalent of
    /// [`<[T]>::chunks`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks)
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::iter::for_each_i;
    /// use konst::slice;
    ///
    /// const CHUNKS: &[&[u8]] = &{
    ///     let mut out = [&[] as &[u8]; 3] ;
    ///     for_each_i!{(i, chunk) in slice::chunks(&[3, 5, 8, 13, 21, 34, 55, 89], 3) =>
    ///         out[i] = chunk;
    ///     }
    ///     out
    /// };
    ///
    /// let expected: &[&[u8]] = &[&[3, 5, 8], &[13, 21, 34], &[55, 89]];
    ///
    /// assert_eq!(CHUNKS, expected)
    ///
    /// ```
    ///
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
    #[track_caller]
    pub const fn chunks<T>(slice: &[T], size: usize) -> Chunks<'_, T> {
        assert!(size != 0, "chunk size must be non-zero");

        Chunks {
            slice: Some(slice),
            size,
        }
    }

    macro_rules! chunks_shared {
        (is_forward = $is_forward:ident) => {
            iterator_shared! {
                is_forward = $is_forward,
                item = &'a [T],
                iter_forward = Chunks<'a, T>,
                iter_reversed = ChunksRev<'a, T>,
                next = chunks_next,
                next_back = chunks_next_back,
                fields = {slice, size},
            }
        };
    }

    macro_rules! chunks_next {
        ($self:ident) => {
            option::map!($self.slice, |slice| {
                let (ret, next) = slice::split_at(slice, $self.size);
                $self.slice = some_if_nonempty(next);
                (ret, $self)
            })
        };
    }

    macro_rules! chunks_next_back {
        ($self:ident) => {
            option::map!($self.slice, |slice| {
                let at = slice.len().saturating_sub($self.size);
                let (next, ret) = slice::split_at(slice, at);
                $self.slice = some_if_nonempty(next);
                (ret, $self)
            })
        };
    }

    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
    pub struct Chunks<'a, T> {
        slice: Option<&'a [T]>,
        size: usize,
    }

    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
    pub struct ChunksRev<'a, T> {
        slice: Option<&'a [T]>,
        size: usize,
    }

    impl<'a, T> Chunks<'a, T> {
        chunks_shared! {is_forward = true}
    }

    impl<'a, T> ChunksRev<'a, T> {
        chunks_shared! {is_forward = false}
    }

    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    /// Const equivalent of
    /// [`<[T]>::chunks_exact`
    /// ](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact)
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{for_range, option, slice};
    ///
    /// const FOUND: [&[u8]; 3] = {
    ///     let iter = slice::chunks_exact(&[3, 5, 8, 13, 21, 34, 55, 89], 3);
    ///     let (elem0, iter) = option::unwrap!(iter.next());
    ///     let (elem1, iter) = option::unwrap!(iter.next());
    ///     [elem0, elem1, iter.remainder()]
    /// };
    ///
    /// let expected: [&[u8]; 3] = [&[3u8, 5, 8], &[13, 21, 34], &[55, 89]];
    ///
    /// assert_eq!(FOUND, expected);
    ///
    /// ```
    ///
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
    #[track_caller]
    pub const fn chunks_exact<T>(slice: &[T], size: usize) -> ChunksExact<'_, T> {
        assert!(size != 0, "chunk size must be non-zero");

        ChunksExact { slice, size }
    }

    macro_rules! chunks_exact_shared {
        (is_forward = $is_forward:ident) => {
            iterator_shared! {
                is_forward = $is_forward,
                item = &'a [T],
                iter_forward = ChunksExact<'a, T>,
                iter_reversed = ChunksExactRev<'a, T>,
                next = chunks_exact_next,
                next_back = chunks_exact_next_back,
                fields = {slice, size},
            }

            /// Returns the remainder of the slice that not returned by [`next`](Self::next),
            /// because it is shorter than the chunk size.
            pub const fn remainder(&self) -> &'a [T] {
                self.slice
            }
        };
    }

    macro_rules! chunks_exact_next {
        ($self:ident) => {
            if $self.slice.len() < $self.size {
                None
            } else {
                let (ret, next) = slice::split_at($self.slice, $self.size);
                $self.slice = next;
                Some((ret, $self))
            }
        };
    }

    macro_rules! chunks_exact_next_back {
        ($self:ident) => {
            if let Some(at) = $self.slice.len().checked_sub($self.size) {
                let (next, ret) = slice::split_at($self.slice, at);
                $self.slice = next;
                Some((ret, $self))
            } else {
                None
            }
        };
    }

    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
    pub struct ChunksExact<'a, T> {
        slice: &'a [T],
        size: usize,
    }

    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_64")))]
    pub struct ChunksExactRev<'a, T> {
        slice: &'a [T],
        size: usize,
    }

    impl<'a, T> ChunksExact<'a, T> {
        chunks_exact_shared! {is_forward = true}
    }

    impl<'a, T> ChunksExactRev<'a, T> {
        chunks_exact_shared! {is_forward = false}
    }
}

#[cfg(feature = "rust_1_64")]
pub use requires_rust_1_64::*;
