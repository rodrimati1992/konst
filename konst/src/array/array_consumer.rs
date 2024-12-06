use core::fmt::{self, Debug};
use core::mem::{ManuallyDrop, MaybeUninit};

/// Const analog of [`core::array::IntoIter`]
/// 
/// This isn't called `IntoIter` because it does not implement the [`ConstIntoIter`] trait,
/// as this type does not have [the API that that trait requires](
/// https://docs.rs/konst/latest/konst/iter/trait.ConstIntoIter.html#isiteratorkind )
/// 
/// # Example
/// 
/// ```rust
/// use konst::array::{ArrayBuilder, ArrayConsumer};
/// 
/// use core::mem::ManuallyDrop as MD;
/// 
/// assert_eq!(ARR, [21, 13, 8, 5, 3]);
/// 
/// const ARR: [u32; 5] = reverse([3, 5, 8, 13, 21]);
/// 
/// const fn reverse<T, const LEN: usize>(arr: [T; LEN]) -> [T; LEN] {
///     let mut iter = ArrayConsumer::new(arr);
///     let mut builder = ArrayBuilder::new();
/// 
///     while let Some(item) = iter.next_back() {
///         builder.push(MD::into_inner(item));
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_83")))]
pub struct ArrayConsumer<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    taken_front: usize,
    taken_back: usize,
}

impl<T, const N: usize> ArrayConsumer<T, N> {
    /// Constructs an ArrayConsumer from an array.
    pub const fn new(array: [T; N]) -> Self {
        Self {
            array: array_into_md(array),
            taken_front: 0,
            taken_back: 0,
        }
    }

    /// Constructs an already-consumed ArrayConsumer.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use konst::array::ArrayConsumer;
    /// 
    /// let mut iter = ArrayConsumer::<u8, 4>::empty();
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

    /// Asserts that the ArrayConsumer is empty,
    /// allows using ArrayConsumer in const.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use konst::array::ArrayConsumer;
    /// 
    /// use core::mem::ManuallyDrop as MD;
    /// 
    /// assert_eq!(SUM, 16);
    /// 
    /// const SUM: u64 = summer(ArrayConsumer::new([3, 5, 8]));
    /// 
    /// const fn summer<const N: usize>(mut iter: ArrayConsumer<u64, N>) -> u64 {
    ///     let mut sum = 0u64;
    ///     while let Some(item) = iter.next() {
    ///         sum += MD::into_inner(item);
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
    /// use konst::array::ArrayConsumer;
    /// 
    /// let mut iter = ArrayConsumer::new([3, 5, 8]);
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
    /// use konst::array::ArrayConsumer;
    /// 
    /// let mut iter = ArrayConsumer::new([3, 5, 8]);
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

    /// Gets a bitwise copy of this ArrayConsumer, requires `T: Copy`.
    pub const fn copy(&self) -> Self 
    where
        T: Copy
    {
        Self {..*self}
    }

    /// Gets the next element from the array
    /// 
    /// Due to limitations of const eval as of Rust 1.83.0,
    /// this function returns a `ManuallyDrop<T>` to be able to return a `T: Drop` in an `Option`,
    /// you'll need to call [`ManuallyDrop::into_inner`] to get `T` and avoid leaking it. 
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use konst::array::ArrayConsumer;
    /// 
    /// use std::mem::ManuallyDrop as MD;
    /// 
    /// let mut iter = ArrayConsumer::new([3, 5, 8]);
    /// 
    /// assert_eq!(iter.as_slice(), &[3, 5, 8][..]);
    /// 
    /// assert_eq!(iter.next(), Some(MD::new(3)));
    /// assert_eq!(iter.as_slice(), &[5, 8][..]);
    /// 
    /// assert_eq!(iter.next(), Some(MD::new(5)));
    /// assert_eq!(iter.as_slice(), &[8][..]);
    /// 
    /// assert_eq!(iter.next(), Some(MD::new(8)));
    /// assert_eq!(iter.as_slice(), &[][..]);
    /// 
    /// assert_eq!(iter.next(), None);
    /// assert_eq!(iter.as_slice(), &[][..]);
    /// ```
    pub const fn next(&mut self) -> Option<ManuallyDrop<T>> {
        if self.is_empty() {
            return None;
        }

        // SAFETY: self.array[self.taken_front] is guaranteed initialized
        let ret = unsafe { self.array[self.taken_front].assume_init_read() };

        self.taken_front += 1;

        Some(ManuallyDrop::new(ret))
    }

    /// Gets the next element from the end of the array
    /// 
    /// Due to limitations of const eval as of Rust 1.83.0,
    /// this function returns a `ManuallyDrop<T>` to be able to return a `T: Drop` in an `Option`,
    /// you'll need to call [`ManuallyDrop::into_inner`] to get `T` and avoid leaking it. 
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use konst::array::ArrayConsumer;
    /// 
    /// use std::mem::ManuallyDrop as MD;
    /// 
    /// let mut iter = ArrayConsumer::new([3, 5, 8]);
    /// 
    /// assert_eq!(iter.as_slice(), &[3, 5, 8][..]);
    /// 
    /// assert_eq!(iter.next_back(), Some(MD::new(8)));
    /// assert_eq!(iter.as_slice(), &[3, 5][..]);
    /// 
    /// assert_eq!(iter.next_back(), Some(MD::new(5)));
    /// assert_eq!(iter.as_slice(), &[3][..]);
    /// 
    /// assert_eq!(iter.next_back(), Some(MD::new(3)));
    /// assert_eq!(iter.as_slice(), &[][..]);
    /// 
    /// assert_eq!(iter.next_back(), None);
    /// assert_eq!(iter.as_slice(), &[][..]);
    /// ```
    pub const fn next_back(&mut self) -> Option<ManuallyDrop<T>> {
        if self.is_empty() {
            return None;
        }

        let index = N - self.taken_back - 1;

        // SAFETY: self.array[index] is guaranteed initialized
        let ret = unsafe { self.array[index].assume_init_read() };
        
        self.taken_back += 1;

        Some(ManuallyDrop::new(ret))
    }
}

impl<T: Debug, const N: usize> Debug for ArrayConsumer<T, N> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_slice(), fmt)
    }
}

impl<T: Clone, const N: usize> Clone for ArrayConsumer<T, N> {
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

impl<T, const N: usize> Drop for ArrayConsumer<T, N> {
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
