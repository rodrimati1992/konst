use crate::{
    iter::{ConstIntoIter, IsIteratorKind},
    option, slice,
};

use konst_kernel::iterator_shared;

/// Gets a const iterator over `slice`, const equivalent of
/// [`<[T]>::iter`
/// ](https://doc.rust-lang.org/std/primitive.slice.html#method.iter)
///
/// # Example
///
/// ### Normal
///
/// ```rust
/// use konst::iter::for_each;
/// use konst::slice;
///
/// const ARR: &[usize] = &{
///     let mut arr = [0usize; 3];
///     // the `slice::iter` call here is unnecessary,
///     // you can pass a slice reference to `for_each`
///     for_each!{(i, elem) in slice::iter(&["foo", "hello", "That box"]), enumerate() =>
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
/// use konst::iter::for_each;
/// use konst::slice;
///
/// const ARR: &[usize] = &{
///     let mut arr = [0usize; 3];
///     for_each!{(i, elem) in slice::iter(&["foo", "hello", "That box"]).rev(),enumerate() =>
///         arr[i] = elem.len();
///     }
///     arr
/// };
///
/// assert_eq!(ARR, [8, 5, 3]);
///
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::into_iter::slice_into_iter::iter;

/// Const equivalent of [`core::slice::Iter`].
///
/// This is constructed in either of these ways:
/// ```rust
/// # let a_slice = &[3];
/// # let _ = (
/// konst::slice::iter(a_slice)
/// # ,
/// konst::iter::into_iter!(a_slice)
/// # );
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::into_iter::slice_into_iter::Iter;

/// Const equivalent of `core::iter::Rev<core::slice::Iter<_>>`
///
/// This is constructed in either of these ways:
/// ```rust
/// # let a_slice = &[3];
/// # let _ = (
/// konst::slice::iter(a_slice).rev()
/// # ,
/// konst::iter::into_iter!(a_slice).rev()
/// # );
/// ```
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::into_iter::slice_into_iter::IterRev;

/// A const equivalent of `slice.iter().copied()`
///
/// # Example
///
/// ```rust
/// use konst::{iter, slice};
///
/// const fn find_even(slice: &[u32]) -> Option<u32> {
///     iter::eval!(slice::iter_copied(slice),find(|elem| *elem % 2 == 0))
/// }
///
/// assert_eq!(find_even(&[]), None);
/// assert_eq!(find_even(&[1]), None);
/// assert_eq!(find_even(&[1, 2]), Some(2));
/// assert_eq!(find_even(&[5, 4, 3, 2, 1]), Some(4));
///
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::into_iter::slice_into_iter::iter_copied;

/// A const equivalent of `iter::Copied<slice::Iter<'a, T>>`.
///
/// This const iterator can be created with [`iter_copied`].
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::into_iter::slice_into_iter::IterCopied;

/// A const equivalent of `iter::Rev<iter::Copied<slice::Iter<'a, T>>>`
///
/// This const iterator can be created with
/// ```rust
/// # let slice = &[3, 5, 8];
/// # let _: konst::slice::IterCopiedRev<'_, u32> =
/// konst::slice::iter_copied(slice).rev()
/// # ;
/// ```
///
/// # Example
///
/// ```rust
/// use konst::iter;
/// use konst::slice::{self, IterCopiedRev};
///
/// const fn rfind_even(slice: &[u32]) -> Option<u32> {
///     let iter: IterCopiedRev<'_, u32> = slice::iter_copied(slice).rev();
///     iter::eval!(iter,find(|&elem| elem % 2 == 0))
/// }
///
/// assert_eq!(rfind_even(&[]), None);
/// assert_eq!(rfind_even(&[1]), None);
/// assert_eq!(rfind_even(&[1, 2]), Some(2));
/// assert_eq!(rfind_even(&[1, 2, 3, 4, 5]), Some(4));
///
/// ```
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::into_iter::slice_into_iter::IterCopiedRev;

///////////////////////////////////////////////////////////////////////////////

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
    /// # Panics
    ///
    /// Panics if `size == 0`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{iter, slice};
    ///
    /// const fn is_sorted(slice: &[u8]) -> bool {
    ///     iter::eval!(slice::windows(slice, 2),all(|w| w[1] > w[0]))
    /// }
    ///
    /// assert!(is_sorted(&[3, 5, 8]));
    /// assert!(!is_sorted(&[8, 13, 0]));
    ///
    ///
    ///
    /// ```
    #[track_caller]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
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
                next(self){
                    if self.slice.len() < self.size {
                        None
                    } else {
                        let up_to = slice::slice_up_to(self.slice, self.size);
                        self.slice = slice::slice_from(self.slice, 1);
                        Some((up_to, self))
                    }
                },
                next_back {
                    let len = self.slice.len();
                    if len < self.size {
                        None
                    } else {
                        let up_to = slice::slice_from(self.slice, len - self.size);
                        self.slice = slice::slice_up_to(self.slice, len - 1);
                        Some((up_to, self))
                    }
                },
                fields = {slice, size},
            }
        };
    }

    /// Const equivalent of [`core::slice::Windows`]
    ///
    /// This is constructed with [`windows`] like this:
    /// ```rust
    /// # let slice = &[3];
    /// # let _ =
    /// konst::slice::windows(slice, 1)
    /// # ;
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub struct Windows<'a, T> {
        slice: &'a [T],
        size: usize,
    }
    impl<'a, T> ConstIntoIter for Windows<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T];
    }

    /// Const equivalent of `core::iter::Rev<core::slice::Windows>`
    ///
    /// This is constructed with [`windows`] like this:
    /// ```rust
    /// # let slice = &[3];
    /// # let _ =
    /// konst::slice::windows(slice, 1).rev()
    /// # ;
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub struct WindowsRev<'a, T> {
        slice: &'a [T],
        size: usize,
    }
    impl<'a, T> ConstIntoIter for WindowsRev<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T];
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
    /// [`<[T]>::array_chunks`
    /// ](https://doc.rust-lang.org/std/primitive.slice.html#method.array_chunks)
    ///
    /// # Panics
    ///
    /// Panics if `N == 0`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::slice;
    ///
    /// let arr = [3, 5, 8, 13, 21, 34, 55];
    ///
    /// let iter = slice::array_chunks::<_, 2>(&arr);
    ///
    /// let (val0, iter) = iter.next().unwrap();
    /// let (val1, iter) = iter.next().unwrap();
    /// let (val2, iter) = iter.next().unwrap();
    ///
    /// let out: [[u8; 2]; 3] = [*val0, *val1, *val2];
    /// assert_eq!(out, [[3, 5], [8, 13], [21, 34]]);
    ///
    /// assert_eq!(iter.remainder(), &[55][..]);
    /// ```
    pub const fn array_chunks<'a, T, const N: usize>(slice: &'a [T]) -> ArrayChunks<'a, T, N> {
        let (arrays, rem) = slice::as_chunks(slice);

        ArrayChunks { arrays, rem }
    }

    macro_rules! array_chunks_shared {
        (is_forward = $is_forward:ident) => {
            iterator_shared! {
                is_forward = $is_forward,
                item = &'a [T; N],
                iter_forward = ArrayChunks<'a, T, N>,
                iter_reversed = ArrayChunksRev<'a, T, N>,
                next(self) {
                    match self.arrays {
                        [elem, arrays @ ..] => Some((elem, Self {arrays, rem: self.rem})),
                        [] => None,
                    }
                },
                next_back {
                    match self.arrays {
                        [arrays @ .., elem] => Some((elem, Self {arrays, rem: self.rem})),
                        [] => None,
                    }
                },
                fields = {arrays, rem},
            }
        };
    }

    /// Const equivalent of [`core::slice::ArrayChunks`]
    pub struct ArrayChunks<'a, T, const N: usize> {
        arrays: &'a [[T; N]],
        rem: &'a [T],
    }
    impl<'a, T, const N: usize> ConstIntoIter for ArrayChunks<'a, T, N> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T; N];
    }

    /// Const equivalent of `core::iter::Rev<core::slice::ArrayChunks>`
    pub struct ArrayChunksRev<'a, T, const N: usize> {
        arrays: &'a [[T; N]],
        rem: &'a [T],
    }
    impl<'a, T, const N: usize> ConstIntoIter for ArrayChunksRev<'a, T, N> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T; N];
    }

    impl<'a, T, const N: usize> ArrayChunks<'a, T, N> {
        array_chunks_shared! {is_forward = true}

        /// Accesses the trailing part of the slice that's not returned by the iterator,
        /// because it's shorter than `Ǹ` elements long.
        pub const fn remainder(&self) -> &'a [T] {
            self.rem
        }
    }

    impl<'a, T, const N: usize> ArrayChunksRev<'a, T, N> {
        array_chunks_shared! {is_forward = false}
    }

    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    /// Const equivalent of
    /// [`<[T]>::chunks`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks)
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size == 0`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::iter::collect_const;
    /// use konst::slice;
    ///
    /// const CHUNKS: [&[u8]; 3] = collect_const!{&[u8] =>
    ///     slice::chunks(&[3, 5, 8, 13, 21, 34, 55, 89], 3)
    /// };
    ///
    /// let expected: &[&[u8]] = &[&[3, 5, 8], &[13, 21, 34], &[55, 89]];
    ///
    /// assert_eq!(CHUNKS, expected)
    ///
    /// ```
    ///
    #[track_caller]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub const fn chunks<T>(slice: &[T], chunk_size: usize) -> Chunks<'_, T> {
        assert!(chunk_size != 0, "chunk size must be non-zero");

        Chunks {
            slice: some_if_nonempty(slice),
            chunk_size,
        }
    }

    macro_rules! chunks_shared {
        (is_forward = $is_forward:ident) => {
            iterator_shared! {
                is_forward = $is_forward,
                item = &'a [T],
                iter_forward = Chunks<'a, T>,
                iter_reversed = ChunksRev<'a, T>,
                next(self) {
                    option::map!(self.slice, |slice| {
                        let (ret, next) = slice::split_at(slice, self.chunk_size);
                        self.slice = some_if_nonempty(next);
                        (ret, self)
                    })
                },
                next_back{
                    option::map!(self.slice, |slice| {
                        let at = (slice.len() - 1) / self.chunk_size * self.chunk_size;
                        let (next, ret) = slice::split_at(slice, at);
                        self.slice = some_if_nonempty(next);
                        (ret, self)
                    })
                },
                fields = {slice, chunk_size},
            }
        };
    }

    /// Const equivalent of [`core::slice::Chunks`]
    ///
    /// This is constructed with [`chunks`] like this:
    /// ```rust
    /// # let slice = &[3];
    /// # let _ =
    /// konst::slice::chunks(slice, 1)
    /// # ;
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub struct Chunks<'a, T> {
        slice: Option<&'a [T]>,
        chunk_size: usize,
    }
    impl<'a, T> ConstIntoIter for Chunks<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T];
    }

    /// Const equivalent of `core::iter::Rev<core::slice::Chunks>`
    ///
    /// This is constructed with [`chunks`] like this:
    /// ```rust
    /// # let slice = &[3];
    /// # let _ =
    /// konst::slice::chunks(slice, 1).rev()
    /// # ;
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub struct ChunksRev<'a, T> {
        slice: Option<&'a [T]>,
        chunk_size: usize,
    }
    impl<'a, T> ConstIntoIter for ChunksRev<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T];
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
    /// [`<[T]>::rchunks`](https://doc.rust-lang.org/std/primitive.slice.html#method.rchunks)
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size == 0`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::iter::collect_const;
    /// use konst::slice;
    ///
    /// const CHUNKS: [&[u8]; 3] = collect_const!{&[u8] =>
    ///     slice::rchunks(&[3, 5, 8, 13, 21, 34, 55, 89], 3)
    /// };
    ///
    /// let expected: &[&[u8]] = &[&[34, 55, 89], &[8, 13, 21], &[3, 5]];
    ///
    /// assert_eq!(CHUNKS, expected)
    ///
    /// ```
    #[track_caller]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub const fn rchunks<T>(slice: &[T], chunk_size: usize) -> RChunks<'_, T> {
        assert!(chunk_size != 0, "chunk size must be non-zero");

        RChunks {
            slice: some_if_nonempty(slice),
            chunk_size,
        }
    }

    macro_rules! rchunks_shared {
        (is_forward = $is_forward:ident) => {
            iterator_shared! {
                is_forward = $is_forward,
                item = &'a [T],
                iter_forward = RChunks<'a, T>,
                iter_reversed = RChunksRev<'a, T>,
                next(self) {
                    option::map!(self.slice, |slice| {
                        let at = slice.len().saturating_sub(self.chunk_size);
                        let (next, ret) = slice::split_at(slice, at);
                        self.slice = some_if_nonempty(next);
                        (ret, self)
                    })
                },
                next_back{
                    option::map!(self.slice, |slice| {
                        let rem = slice.len() % self.chunk_size;
                        let at = if rem == 0 { self.chunk_size } else { rem };
                        let (ret, next) = slice::split_at(slice, at);
                        self.slice = some_if_nonempty(next);
                        (ret, self)
                    })
                },
                fields = {slice, chunk_size},
            }
        };
    }

    /// Const equivalent of [`core::slice::RChunks`]
    ///
    /// This is constructed with [`rchunks`] like this:
    /// ```rust
    /// # let slice = &[3];
    /// # let _ =
    /// konst::slice::rchunks(slice, 1)
    /// # ;
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub struct RChunks<'a, T> {
        slice: Option<&'a [T]>,
        chunk_size: usize,
    }
    impl<'a, T> ConstIntoIter for RChunks<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T];
    }

    /// Const equivalent of `core::iter::Rev<core::slice::RChunks>`
    ///
    /// This is constructed with [`rchunks`] like this:
    /// ```rust
    /// # let slice = &[3];
    /// # let _ =
    /// konst::slice::rchunks(slice, 1).rev()
    /// # ;
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub struct RChunksRev<'a, T> {
        slice: Option<&'a [T]>,
        chunk_size: usize,
    }
    impl<'a, T> ConstIntoIter for RChunksRev<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T];
    }

    impl<'a, T> RChunks<'a, T> {
        rchunks_shared! {is_forward = true}
    }

    impl<'a, T> RChunksRev<'a, T> {
        rchunks_shared! {is_forward = false}
    }

    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    /// Const equivalent of
    /// [`<[T]>::chunks_exact`
    /// ](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact)
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size == 0`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{option, slice};
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
    #[track_caller]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub const fn chunks_exact<T>(slice: &[T], chunk_size: usize) -> ChunksExact<'_, T> {
        assert!(chunk_size != 0, "chunk size must be non-zero");

        let at = slice.len() - slice.len() % chunk_size;
        let (slice, rem) = slice::split_at(slice, at);

        ChunksExact {
            slice,
            rem,
            chunk_size,
        }
    }

    macro_rules! chunks_exact_shared {
        (is_forward = $is_forward:ident) => {
            iterator_shared! {
                is_forward = $is_forward,
                item = &'a [T],
                iter_forward = ChunksExact<'a, T>,
                iter_reversed = ChunksExactRev<'a, T>,
                next(self) {
                    if self.slice.is_empty() {
                        None
                    } else {
                        let (ret, next) = slice::split_at(self.slice, self.chunk_size);
                        self.slice = next;
                        Some((ret, self))
                    }
                },
                next_back {
                    if self.slice.is_empty() {
                        None
                    } else {
                        let at = self.slice.len() - self.chunk_size;
                        let (next, ret) = slice::split_at(self.slice, at);
                        self.slice = next;
                        Some((ret, self))
                    }
                },
                fields = {slice, rem, chunk_size},
            }

            /// Returns the remainder of the slice that's not returned by [`next`](Self::next),
            /// because it is shorter than the chunk size.
            pub const fn remainder(&self) -> &'a [T] {
                self.rem
            }
        };
    }

    /// Const equivalent of [`core::slice::ChunksExact`]
    ///
    /// This is constructed with [`chunks_exact`] like this:
    /// ```rust
    /// # let slice = &[3];
    /// # let _ =
    /// konst::slice::chunks_exact(slice, 1)
    /// # ;
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub struct ChunksExact<'a, T> {
        slice: &'a [T],
        rem: &'a [T],
        chunk_size: usize,
    }
    impl<'a, T> ConstIntoIter for ChunksExact<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T];
    }

    /// Const equivalent of `core::iter::Rev<core::slice::ChunksExact>`
    ///
    /// This is constructed with [`chunks_exact`] like this:
    /// ```rust
    /// # let slice = &[3];
    /// # let _ =
    /// konst::slice::chunks_exact(slice, 1).rev()
    /// # ;
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub struct ChunksExactRev<'a, T> {
        slice: &'a [T],
        rem: &'a [T],
        chunk_size: usize,
    }
    impl<'a, T> ConstIntoIter for ChunksExactRev<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T];
    }

    impl<'a, T> ChunksExact<'a, T> {
        chunks_exact_shared! {is_forward = true}
    }

    impl<'a, T> ChunksExactRev<'a, T> {
        chunks_exact_shared! {is_forward = false}
    }

    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    /// Const equivalent of
    /// [`<[T]>::rchunks_exact`
    /// ](https://doc.rust-lang.org/std/primitive.slice.html#method.rchunks_exact)
    ///
    /// # Panics
    ///
    /// Panics if `chunk_size == 0`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::{option, slice};
    ///
    /// const FOUND: [&[u8]; 3] = {
    ///     let iter = slice::rchunks_exact(&[3, 5, 8, 13, 21, 34, 55, 89], 3);
    ///     let (elem0, iter) = option::unwrap!(iter.next());
    ///     let (elem1, iter) = option::unwrap!(iter.next());
    ///     [elem0, elem1, iter.remainder()]
    /// };
    ///
    /// let expected: [&[u8]; 3] = [&[34, 55, 89], &[8, 13, 21], &[3, 5]];
    ///
    /// assert_eq!(FOUND, expected);
    ///
    /// ```
    ///
    #[track_caller]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub const fn rchunks_exact<T>(slice: &[T], chunk_size: usize) -> RChunksExact<'_, T> {
        assert!(chunk_size != 0, "chunk size must be non-zero");

        let (rem, slice) = slice::split_at(slice, slice.len() % chunk_size);

        RChunksExact {
            slice,
            rem,
            chunk_size,
        }
    }

    macro_rules! rchunks_exact_shared {
        (is_forward = $is_forward:ident) => {
            iterator_shared! {
                is_forward = $is_forward,
                item = &'a [T],
                iter_forward = RChunksExact<'a, T>,
                iter_reversed = RChunksExactRev<'a, T>,
                next(self) {
                    if self.slice.is_empty() {
                        None
                    } else {
                        let at = self.slice.len() - self.chunk_size;
                        let (next, ret) = slice::split_at(self.slice, at);
                        self.slice = next;
                        Some((ret, self))
                    }
                },
                next_back {
                    if self.slice.is_empty() {
                        None
                    } else {
                        let (ret, next) = slice::split_at(self.slice, self.chunk_size);
                        self.slice = next;
                        Some((ret, self))
                    }
                },
                fields = {slice, rem, chunk_size},
            }

            /// Returns the remainder of the slice that's not returned by [`next`](Self::next),
            /// because it is shorter than the chunk size.
            pub const fn remainder(&self) -> &'a [T] {
                self.rem
            }
        };
    }

    /// Const equivalent of [`core::slice::RChunksExact`]
    ///
    /// This is constructed with [`rchunks_exact`] like this:
    /// ```rust
    /// # let slice = &[3];
    /// # let _ =
    /// konst::slice::rchunks_exact(slice, 1)
    /// # ;
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub struct RChunksExact<'a, T> {
        slice: &'a [T],
        rem: &'a [T],
        chunk_size: usize,
    }
    impl<'a, T> ConstIntoIter for RChunksExact<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T];
    }

    /// Const equivalent of `core::iter::Rev<core::slice::RChunksExact>`
    ///
    /// This is constructed with [`rchunks_exact`] like this:
    /// ```rust
    /// # let slice = &[3];
    /// # let _ =
    /// konst::slice::rchunks_exact(slice, 1).rev()
    /// # ;
    /// ```
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
    pub struct RChunksExactRev<'a, T> {
        slice: &'a [T],
        rem: &'a [T],
        chunk_size: usize,
    }
    impl<'a, T> ConstIntoIter for RChunksExactRev<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = &'a [T];
    }

    impl<'a, T> RChunksExact<'a, T> {
        rchunks_exact_shared! {is_forward = true}
    }

    impl<'a, T> RChunksExactRev<'a, T> {
        rchunks_exact_shared! {is_forward = false}
    }
}

#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use requires_rust_1_64::*;
