use core::fmt::{self, Debug};
use core::mem::{ManuallyDrop, MaybeUninit};

use crate::iter::{ConstIntoIter, IntoIterWrapper, IsIteratorKind, IsStdKind};

/// Const equivalent of [`core::array::IntoIter`]
/// 
/// # Example
/// 
/// ```rust
/// use konst::array::{ArrayBuilder, IntoIter};
/// 
/// assert_eq!(ARR, [21, 13, 8, 5, 3]);
/// 
/// const ARR: [u32; 5] = reverse([3, 5, 8, 13, 21]);
/// 
/// const fn reverse<T, const LEN: usize>(arr: [T; LEN]) -> [T; LEN] {
///     let mut iter = IntoIter::new(arr);
///     let mut builder = ArrayBuilder::new();
/// 
///     konst::while_let_Some!{item = iter.next_back() =>
///         builder.push(item);
///     }
/// 
///     // necessary to avoid "destructor cannot be evaluated at compile-time" error
///     iter.assert_is_empty();
/// 
///     builder.build()
/// }
/// ```
/// 
/// [`ConstIntoIter`]: crate::iter::ConstIntoIter
#[repr(C)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct IntoIter<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    taken_front: usize,
    taken_back: usize,
}

impl<T, const L: usize> ConstIntoIter for [T; L] {
    type Kind = IsStdKind;
    type IntoIter = IntoIter<T, L>;
    type Item = T;
}

impl<T, const L: usize> IntoIterWrapper<[T; L], IsStdKind> {
    /// Converts `[T; L]` into an iterator
    pub const fn const_into_iter(self) -> IntoIter<T, L> {
        IntoIter::new(ManuallyDrop::into_inner(self.iter))
    }
}

impl<T, const L: usize> ConstIntoIter for IntoIter<T, L> {
    type Kind = IsIteratorKind;
    type IntoIter = IntoIter<T, L>;
    type Item = T;
}

impl<T, const N: usize> IntoIter<T, N> {
    /// Constructs an IntoIter from an array.
    pub const fn new(array: [T; N]) -> Self {
        Self {
            array: array_into_md(array),
            taken_front: 0,
            taken_back: 0,
        }
    }

    /// Constructs an already-consumed IntoIter.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use konst::array::IntoIter;
    /// 
    /// let mut iter = IntoIter::<u8, 4>::empty();
    /// 
    /// assert_eq!(iter.next(), None);
    /// 
    /// ```
    pub const fn empty() -> Self {
        Self {
            array: crate::maybe_uninit::uninit_array(),
            taken_front: N,
            taken_back: 0,
        }
    }

    const fn is_empty(&self) -> bool {
        (N - self.taken_front - self.taken_back) == 0
    }

    const fn slice_len(&self) -> usize {
        N - self.taken_front - self.taken_back
    }

    /// Asserts that the IntoIter is empty,
    /// allows using IntoIter in const.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use konst::array::IntoIter;
    /// 
    /// assert_eq!(SUM, 16);
    /// 
    /// const SUM: u64 = summer(IntoIter::new([3, 5, 8]));
    /// 
    /// const fn summer<const N: usize>(mut iter: IntoIter<u64, N>) -> u64 {
    ///     let mut sum = 0u64;
    ///     konst::while_let_Some!{item = iter.next() =>
    ///         sum += item;
    ///     }
    ///     iter.assert_is_empty();
    ///     sum
    /// }
    /// ```
    #[track_caller]
    pub const fn assert_is_empty(self) {
        assert!(self.is_empty());

        core::mem::forget(self);
    }

    /// Gets the remainder of the array as a slice
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use konst::array::IntoIter;
    /// 
    /// let mut iter = IntoIter::new([3, 5, 8]);
    /// 
    /// assert_eq!(iter.as_slice(), &[3, 5, 8][..]);
    /// 
    /// assert!(iter.next().is_some());
    /// assert_eq!(iter.as_slice(), &[5, 8][..]);
    /// 
    /// assert!(iter.next().is_some());
    /// assert_eq!(iter.as_slice(), &[8][..]);
    /// 
    /// assert!(iter.next().is_some());
    /// assert_eq!(iter.as_slice(), &[][..]);
    /// 
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.as_slice(), &[][..]);
    /// ```
    pub const fn as_slice(&self) -> &[T] {
        // SAFETY: self.array is guaranteed initialized starting from self.taken_front
        //         up to `N - self.taken_back`
        unsafe {
            let ptr = self.array.as_ptr().add(self.taken_front).cast::<T>();
            core::slice::from_raw_parts(ptr, self.slice_len())
        }
    }

    /// Gets the remainder of the array as a mutable slice
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use konst::array::IntoIter;
    /// 
    /// let mut iter = IntoIter::new([3, 5, 8]);
    /// 
    /// assert_eq!(iter.as_mut_slice(), &mut [3, 5, 8][..]);
    /// 
    /// assert!(iter.next().is_some());
    /// assert_eq!(iter.as_mut_slice(), &mut [5, 8][..]);
    /// 
    /// assert!(iter.next().is_some());
    /// assert_eq!(iter.as_mut_slice(), &mut [8][..]);
    /// 
    /// assert!(iter.next().is_some());
    /// assert_eq!(iter.as_mut_slice(), &mut [][..]);
    /// 
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.as_mut_slice(), &mut [][..]);
    /// ```
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: self.array is guaranteed initialized starting from self.taken_front
        //         up to `N - self.taken_back`
        unsafe {
            let ptr = self.array.as_mut_ptr().add(self.taken_front).cast::<T>();
            core::slice::from_raw_parts_mut(ptr, self.slice_len())
        }
    }

    /// Gets a bitwise copy of this IntoIter, requires `T: Copy`.
    pub const fn copy(&self) -> Self 
    where
        T: Copy
    {
        Self {..*self}
    }

    /// Gets the next element from the array
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use konst::array::IntoIter;
    /// 
    /// let mut iter = IntoIter::new([3, 5, 8]);
    /// 
    /// assert_eq!(iter.as_slice(), &[3, 5, 8][..]);
    /// 
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.as_slice(), &[5, 8][..]);
    /// 
    /// assert_eq!(iter.next(), Some(5));
    /// assert_eq!(iter.as_slice(), &[8][..]);
    /// 
    /// assert_eq!(iter.next(), Some(8));
    /// assert_eq!(iter.as_slice(), &[][..]);
    /// 
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.as_slice(), &[][..]);
    /// ```
    pub const fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // SAFETY: self.array[self.taken_front] is guaranteed initialized
        let ret = unsafe { self.array[self.taken_front].assume_init_read() };

        self.taken_front += 1;

        Some(ret)
    }

    /// Gets the next element from the end of the array
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use konst::array::IntoIter;
    /// 
    /// let mut iter = IntoIter::new([3, 5, 8]);
    /// 
    /// assert_eq!(iter.as_slice(), &[3, 5, 8][..]);
    /// 
    /// assert_eq!(iter.next_back(), Some(8));
    /// assert_eq!(iter.as_slice(), &[3, 5][..]);
    /// 
    /// assert_eq!(iter.next_back(), Some(5));
    /// assert_eq!(iter.as_slice(), &[3][..]);
    /// 
    /// assert_eq!(iter.next_back(), Some(3));
    /// assert_eq!(iter.as_slice(), &[][..]);
    /// 
    /// assert_eq!(iter.next_back(), None);
    /// assert_eq!(iter.as_slice(), &[][..]);
    /// ```
    pub const fn next_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let index = N - self.taken_back - 1;

        // SAFETY: self.array[index] is guaranteed initialized
        let ret = unsafe { self.array[index].assume_init_read() };
        
        self.taken_back += 1;

        Some(ret)
    }

    /// Reverses the array iterator
    pub const fn rev(self) -> IntoIterRev<T, N> {
        IntoIterRev(self)
    }
}

impl<T: Debug, const N: usize> Debug for IntoIter<T, N> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_slice(), fmt)
    }
}

impl<T: Clone, const N: usize> Clone for IntoIter<T, N> {
    fn clone(&self) -> Self {
        let mut this = Self {
            array: crate::maybe_uninit::uninit_array(),
            taken_front: 0,
            taken_back: N,
        };

        for (i, elem) in self.as_slice().iter().cloned().enumerate() {
            this.array[i] = MaybeUninit::new(elem);
            this.taken_back -= 1;
        }

        this
    }
}

impl<T, const N: usize> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        unsafe {
            let slice_len = self.slice_len();

            let ptr = self.array.as_mut_ptr().cast::<T>();

            core::ptr::slice_from_raw_parts_mut(ptr.add(self.taken_front), slice_len).drop_in_place();
        }
    }
}


#[doc(hidden)]
const fn array_into_md<T, const N: usize>(arr: [T; N]) -> [MaybeUninit<T>; N] {
    unsafe {
        crate::__::__priv_transmute! {[T; N], [MaybeUninit<T>; N], arr}
    }
}


/////////////


/// Const equivalent of `core::iter::Rev<core::array::IntoIter>`
/// 
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct IntoIterRev<T, const N: usize>(IntoIter<T, N>);

impl<T, const L: usize> ConstIntoIter for IntoIterRev<T, L> {
    type Kind = IsIteratorKind;
    type IntoIter = IntoIterRev<T, L>;
    type Item = T;
}

impl<T, const N: usize> IntoIterRev<T, N> {
    /// Asserts that the IntoIterRev is empty,
    /// allows using IntoIterRev in const.
    /// 
    pub const fn assert_is_empty(self) {
        self.rev().assert_is_empty()
    }
    /// Gets the remainder of the array as a slice
    /// 
    pub const fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
    /// Gets the remainder of the array as a mutable slice
    /// 
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        self.0.as_mut_slice()
    }
    /// Gets a bitwise copy of this IntoIter, requires `T: Copy`.
    pub const fn copy(&self) -> Self 
    where
        T: Copy
    {
        Self(self.0.copy())
    }
    /// Gets the next element from the end of the array
    /// 
    pub const fn next(&mut self) -> Option<T> {
        self.0.next_back()
    }
    /// Gets the next element from the start of the array
    /// 
    pub const fn next_back(&mut self) -> Option<T> {
        self.0.next()
    }
    /// Reverses the array iterator
    pub const fn rev(self) -> IntoIter<T, N> {
        crate::destructure!{Self(x) = self}
        x
    }
}

impl<T: Debug, const N: usize> Debug for IntoIterRev<T, N> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_slice(), fmt)
    }
}

impl<T: Clone, const N: usize> Clone for IntoIterRev<T, N> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
