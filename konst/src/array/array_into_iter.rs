use core::fmt::{self, Debug};
use core::mem::{ManuallyDrop, MaybeUninit};

use crate::{
    drop_flavor::{DropFlavor, MayDrop, NonDrop, as_inner, as_inner_mut, wrap},
    iter::{ConstIntoIter, IntoIterWrapper, IsIteratorKind, IsStdKind},
};

use typewit::Identity;

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
///     let mut iter = IntoIter::of_drop(arr);
///     let mut builder = ArrayBuilder::of_drop();
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
pub struct IntoIter<T, const N: usize, D: DropFlavor> {
    inner: D::Wrap<IntoIterInner<T, N>>,
}

#[repr(C)]
pub struct IntoIterInner<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    taken_front: usize,
    taken_back: usize,
}

impl<T, const N: usize> IntoIterInner<T, N> {
    const fn into_ii<D: DropFlavor>(self) -> IntoIter<T, N, D> {
        IntoIter { inner: wrap(self) }
    }

    const fn slice_len(&self) -> usize {
        N - self.taken_front - self.taken_back
    }
}

impl<T, const N: usize> ConstIntoIter for [T; N] {
    type Kind = IsStdKind;
    type IntoIter = IntoIter<T, N, MayDrop>;
    type Item = T;
    const ITEMS_NEED_DROP: bool = core::mem::needs_drop::<T>();
}

impl<T, const N: usize> IntoIterWrapper<[T; N], IsStdKind> {
    /// Converts `[T; N]` into an iterator
    pub const fn const_into_iter(self) -> IntoIter<T, N, MayDrop> {
        IntoIter::of_drop(ManuallyDrop::into_inner(self.iter))
    }
}

impl<T, const N: usize, D: DropFlavor> ConstIntoIter for IntoIter<T, N, D> {
    type Kind = IsIteratorKind;
    type IntoIter = IntoIter<T, N, D>;
    type Item = T;
    const ITEMS_NEED_DROP: bool = core::mem::needs_drop::<T>();
}

impl<T, const N: usize> IntoIter<T, N, MayDrop> {
    /// Constructs an IntoIter from an array that may need dropping.
    ///
    /// The `Identity` bound emulates a type equality constraint,
    /// this allows specifying `N` through this constructor,
    /// while infering the other arguments.
    ///
    pub const fn of_drop<const N2: usize>(array: [T; N]) -> Self
    where
        Self: Identity<Type = IntoIter<T, N2, MayDrop>>,
    {
        IntoIterInner {
            array: array_into_md(array),
            taken_front: 0,
            taken_back: 0,
        }
        .into_ii()
    }
}

impl<T, const N: usize> IntoIter<T, N, NonDrop> {
    /// Constructs an IntoIter from a `Copy` array.
    ///
    /// The `Identity` bound emulates a type equality constraint,
    /// this allows specifying `N` through this constructor,
    /// while infering the other arguments.
    ///
    pub const fn of_copy<const N2: usize>(array: [T; N]) -> Self
    where
        T: Copy,
        Self: Identity<Type = IntoIter<T, N2, NonDrop>>,
    {
        IntoIterInner {
            array: array_into_md(array),
            taken_front: 0,
            taken_back: 0,
        }
        .into_ii()
    }

    /// Constructs an already-consumed IntoIter.
    ///
    /// The `Identity` bound emulates a type equality constraint,
    /// this allows specifying `N` through this constructor,
    /// while infering the other arguments.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::IntoIter;
    ///
    /// let mut iter = IntoIter::<u8, 4, _>::empty();
    ///
    /// assert_eq!(iter.next(), None);
    ///
    /// ```
    pub const fn empty<const N2: usize>() -> Self
    where
        Self: Identity<Type = IntoIter<T, N2, NonDrop>>,
    {
        IntoIterInner {
            array: crate::maybe_uninit::uninit_array(),
            taken_front: N,
            taken_back: 0,
        }
        .into_ii()
    }
}

impl<T, const N: usize, D: DropFlavor> IntoIter<T, N, D> {
    /// Gets the next element from the array
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::IntoIter;
    ///
    /// let mut iter = IntoIter::of_copy([3, 5, 8]);
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

        let this = as_inner_mut::<D, _>(&mut self.inner);

        // SAFETY: self.array[self.taken_front] is guaranteed initialized
        let ret = unsafe { this.array[this.taken_front].assume_init_read() };

        this.taken_front += 1;

        Some(ret)
    }

    /// Gets the next element from the end of the array
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::IntoIter;
    ///
    /// let mut iter = IntoIter::of_copy([3, 5, 8]);
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

        let this = as_inner_mut::<D, _>(&mut self.inner);

        let index = N - this.taken_back - 1;

        // SAFETY: this.array[index] is guaranteed initialized
        let ret = unsafe { this.array[index].assume_init_read() };

        this.taken_back += 1;

        Some(ret)
    }

    /// Reverses the array iterator
    pub const fn rev(self) -> IntoIterRev<T, N, D> {
        IntoIterRev(self)
    }

    const fn is_empty(&self) -> bool {
        let this = as_inner::<D, _>(&self.inner);

        (N - this.taken_front - this.taken_back) == 0
    }

    /// Asserts that the IntoIter is empty,
    /// allows using IntoIter in const.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::IntoIter;
    /// use konst::drop_flavor::NonDrop;
    ///
    /// assert_eq!(SUM, 16);
    ///
    /// const SUM: u64 = summer(IntoIter::of_copy([3, 5, 8]));
    ///
    /// const fn summer<const N: usize>(mut iter: IntoIter<u64, N, NonDrop>) -> u64 {
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
    /// let mut iter = IntoIter::of_copy([3, 5, 8]);
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
        // SAFETY: this.array is guaranteed initialized starting from this.taken_front
        //         up to `N - this.taken_back`
        unsafe {
            let this = as_inner::<D, _>(&self.inner);

            let ptr = this.array.as_ptr().add(this.taken_front).cast::<T>();
            core::slice::from_raw_parts(ptr, this.slice_len())
        }
    }

    /// Gets the remainder of the array as a mutable slice
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::IntoIter;
    ///
    /// let mut iter = IntoIter::of_copy([3, 5, 8]);
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
            let this = as_inner_mut::<D, _>(&mut self.inner);
            let slice_len = this.slice_len();

            let ptr = this.array.as_mut_ptr().add(this.taken_front).cast::<T>();
            core::slice::from_raw_parts_mut(ptr, slice_len)
        }
    }

    /// Gets a bitwise copy of this IntoIter, requires `T: Copy`.
    pub const fn copy(&self) -> Self
    where
        T: Copy,
    {
        IntoIterInner {
            ..*as_inner::<D, _>(&self.inner)
        }
        .into_ii()
    }

    /// Converts this `IntoIter` to have a `MayDrop` drop flavor.
    pub const fn into_drop(self) -> IntoIter<T, N, MayDrop> {
        self.into_any_flavor()
    }

    /// Converts this `IntoIter` to have a `NonDrop` drop flavor
    /// by requiring that `T` is `Copy`.
    pub const fn into_copy(self) -> IntoIter<T, N, NonDrop>
    where
        T: Copy,
    {
        self.into_any_flavor()
    }

    /// Converts this `IntoIter` to have any flavor.
    const fn into_any_flavor<D2: DropFlavor>(self) -> IntoIter<T, N, D2> {
        // SAFETY: changing the D type parameter does not change the layout of the type
        unsafe { crate::__priv_transmute!(IntoIter<T, N, D>, IntoIter<T, N, D2>, self) }
    }
}

impl<T: Debug, const N: usize, D: DropFlavor> Debug for IntoIter<T, N, D> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_slice(), fmt)
    }
}

impl<T: Clone, const N: usize, D: DropFlavor> Clone for IntoIter<T, N, D> {
    fn clone(&self) -> Self {
        let mut this = IntoIterInner {
            array: crate::maybe_uninit::uninit_array(),
            taken_front: 0,
            taken_back: N,
        };

        for (i, elem) in self.as_slice().iter().cloned().enumerate() {
            this.array[i] = MaybeUninit::new(elem);
            this.taken_back -= 1;
        }

        this.into_ii()
    }
}

impl<T, const N: usize> Drop for IntoIterInner<T, N> {
    fn drop(&mut self) {
        unsafe {
            let slice_len = self.slice_len();

            let ptr = self.array.as_mut_ptr().cast::<T>();

            core::ptr::slice_from_raw_parts_mut(ptr.add(self.taken_front), slice_len)
                .drop_in_place();
        }
    }
}

#[doc(hidden)]
const fn array_into_md<T, const N: usize>(arr: [T; N]) -> [MaybeUninit<T>; N] {
    unsafe {
        crate::__priv_transmute! {[T; N], [MaybeUninit<T>; N], arr}
    }
}

/////////////

/// Const equivalent of `core::iter::Rev<core::array::IntoIter>`
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct IntoIterRev<T, const N: usize, D: DropFlavor>(IntoIter<T, N, D>);

impl<T, const N: usize, D: DropFlavor> ConstIntoIter for IntoIterRev<T, N, D> {
    type Kind = IsIteratorKind;
    type IntoIter = IntoIterRev<T, N, D>;
    type Item = T;
    const ITEMS_NEED_DROP: bool = core::mem::needs_drop::<T>();
}

impl<T, const N: usize, D: DropFlavor> IntoIterRev<T, N, D> {
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
        T: Copy,
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
    pub const fn rev(self) -> IntoIter<T, N, D> {
        crate::destructure! {Self(x) = self}
        x
    }
}

impl<T: Debug, const N: usize, D: DropFlavor> Debug for IntoIterRev<T, N, D> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_slice(), fmt)
    }
}

impl<T: Clone, const N: usize, D: DropFlavor> Clone for IntoIterRev<T, N, D> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
