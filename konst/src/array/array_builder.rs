use core::fmt::{self, Debug};
use core::mem::{ManuallyDrop, MaybeUninit};

use crate::{
    array::IntoIter,
    drop_flavor::{DropFlavor, MayDrop, NonDrop, as_inner, as_inner_mut, wrap},
};

use typewit::Identity;

/// For constructing an array element by element.
///
/// This type can be constructed with these constructors:
/// - [`of_copy`](Self::of_copy): for building an array of Copy elements,
///   needed for using `ArrayBuilder` in functions that have early returns.
/// - [`of_drop`](Self::of_drop):
///   for building any type, useful in functions without early returns.
///
///
/// # Example
///
/// ```rust
/// use konst::array::ArrayBuilder;
///
/// assert_eq!(ARR, [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
///
/// const ARR: [u32; 10] = {
///     let mut builder = ArrayBuilder::of_copy();
///     builder.push(1);
///     builder.push(1);
///
///     while !builder.is_full() {
///         let [.., a, b] = *builder.as_slice() else { unreachable!() };
///
///         builder.push(a + b);
///     }
///
///     builder.build()
/// };
/// ```
#[repr(transparent)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub struct ArrayBuilder<T, const N: usize, D: DropFlavor> {
    inner: D::Wrap<ArrayBuilderInner<T, N>>,
}

#[repr(C)]
pub struct ArrayBuilderInner<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    inited: usize,
}

impl<T, const N: usize> ArrayBuilderInner<T, N> {
    const fn into_builder<D: DropFlavor>(self) -> ArrayBuilder<T, N, D> {
        ArrayBuilder { inner: wrap(self) }
    }
}

impl<T, const N: usize> ArrayBuilder<T, N, MayDrop> {
    /// Constructs an empty ArrayBuilder of an element type that may need dropping
    ///
    /// The `N2` const parameter and `Identity` bound are hacks to allow
    /// specifying `N` through this constructor.
    #[inline(always)]
    pub const fn of_drop<const N2: usize>() -> Self
    where
        Self: Identity<Type = ArrayBuilder<T, N2, MayDrop>>,
    {
        Self::of_any()
    }
}

impl<T, const N: usize> ArrayBuilder<T, N, NonDrop> {
    /// Constructs an empty ArrayBuilder of Copy element types.
    ///
    /// The `N2` const parameter and `Identity` bound are hacks to allow
    /// specifying `N` through this constructor.
    #[inline(always)]
    pub const fn of_copy<const N2: usize>() -> Self
    where
        T: Copy,
        Self: Identity<Type = ArrayBuilder<T, N2, NonDrop>>,
    {
        Self::of_any()
    }
}

impl<T, const N: usize, D: DropFlavor> ArrayBuilder<T, N, D> {
    // Constructs an empty ArrayBuilder of any flavor.
    const fn of_any() -> Self {
        ArrayBuilderInner {
            array: crate::maybe_uninit::uninit_array(),
            inited: 0,
        }
        .into_builder()
    }

    /// The amount of initialized elements in the array
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::ArrayBuilder;
    ///
    /// let mut builder = ArrayBuilder::of_copy::<3>();
    ///
    /// assert_eq!(builder.len(), 0);
    ///
    /// builder.push(3);
    /// assert_eq!(builder.len(), 1);
    ///
    /// builder.push(5);
    /// assert_eq!(builder.len(), 2);
    ///
    /// builder.push(8);
    /// assert_eq!(builder.len(), 3);
    /// ```
    ///
    pub const fn len(&self) -> usize {
        as_inner::<D, _>(&self.inner).inited
    }

    /// Whether the array has at least one initialized element.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::ArrayBuilder;
    ///
    /// let mut builder = ArrayBuilder::of_copy::<3>();
    ///
    /// assert!( builder.is_empty());
    ///
    /// builder.push(3);
    /// assert!(!builder.is_empty());
    ///
    /// builder.push(5);
    /// assert!(!builder.is_empty());
    ///
    /// builder.push(8);
    /// assert!(!builder.is_empty());
    /// ```
    ///
    pub const fn is_empty(&self) -> bool {
        as_inner::<D, _>(&self.inner).inited == 0
    }

    /// Whether the array has been fully initialized
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::ArrayBuilder;
    ///
    /// let mut builder = ArrayBuilder::of_copy::<3>();
    ///
    /// assert!(!builder.is_full());
    ///
    /// builder.push(3);
    /// assert!(!builder.is_full());
    ///
    /// builder.push(5);
    /// assert!(!builder.is_full());
    ///
    /// builder.push(8);
    /// assert!(builder.is_full());
    /// ```
    ///
    pub const fn is_full(&self) -> bool {
        as_inner::<D, _>(&self.inner).inited == N
    }

    /// Gets the initialized part of the array as a slice
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::ArrayBuilder;
    ///
    /// let mut builder = ArrayBuilder::of_copy::<3>();
    ///
    /// assert_eq!(builder.as_slice(), [].as_slice());
    ///
    /// builder.push(3);
    /// assert_eq!(builder.as_slice(), [3].as_slice());
    ///
    /// builder.push(5);
    /// assert_eq!(builder.as_slice(), [3, 5].as_slice());
    ///
    /// builder.push(8);
    /// assert_eq!(builder.as_slice(), [3, 5, 8].as_slice());
    /// ```
    ///
    pub const fn as_slice(&self) -> &[T] {
        // SAFETY: self.array is guaranteed initialized up to this.inited - 1 inclusive
        unsafe {
            let this = as_inner::<D, _>(&self.inner);

            core::slice::from_raw_parts(this.array.as_ptr().cast::<T>(), this.inited)
        }
    }

    /// Gets the initialized part of the array as a mutable slice
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::ArrayBuilder;
    ///
    /// let mut builder = ArrayBuilder::of_copy::<3>();
    ///
    /// assert_eq!(builder.as_mut_slice(), [].as_mut_slice());
    ///
    /// builder.push(3);
    /// assert_eq!(builder.as_mut_slice(), [3].as_mut_slice());
    ///
    /// builder.push(5);
    /// assert_eq!(builder.as_mut_slice(), [3, 5].as_mut_slice());
    ///
    /// builder.push(8);
    /// assert_eq!(builder.as_mut_slice(), [3, 5, 8].as_mut_slice());
    /// ```
    ///
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        let this = as_inner_mut::<D, _>(&mut self.inner);

        // SAFETY: this.array is guaranteed initialized up to this.inited - 1 inclusive
        unsafe { core::slice::from_raw_parts_mut(this.array.as_mut_ptr().cast::<T>(), this.inited) }
    }

    /// Appends `val` to the array.
    ///
    /// # Panic
    ///
    /// Panics if `self.len() == N`, i.e.: the array is fully initialized.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::ArrayBuilder;
    ///
    /// let mut builder = ArrayBuilder::of_copy::<3>();
    ///
    /// builder.push(3);
    /// builder.push(5);
    /// builder.push(8);
    ///
    /// assert_eq!(builder.build(), [3, 5, 8]);
    /// ```
    ///
    pub const fn push(&mut self, val: T) {
        let this = as_inner_mut::<D, _>(&mut self.inner);

        assert!(this.inited < N, "trying to add an element to full array");

        this.array[this.inited] = MaybeUninit::new(val);

        this.inited += 1;
    }

    /// Unwraps this ArrayBuilder into an array.
    ///
    /// # Panic
    ///
    /// Panics if `self.len() != N`, i.e.: the array is not fully initialized.
    ///
    /// # Example
    ///
    /// ```rust
    /// use konst::array::ArrayBuilder;
    ///
    /// assert_eq!(ARR, [3, 5, 8]);
    ///
    /// const ARR: [u8; 3] = {
    ///     let mut builder = ArrayBuilder::of_copy();
    ///     
    ///     builder.push(3);
    ///     builder.push(5);
    ///     builder.push(8);
    ///
    ///     builder.build()
    /// };
    /// ```
    ///
    pub const fn build(self) -> [T; N] {
        assert!(
            self.is_full(),
            "trying to unwrap a non-fully-initialized array"
        );

        // SAFETY: self.array is guaranteed fully initialized by the fact that
        //         each element is inited in lockstep with incrementing self.inited by 1,
        //         and the assertion above.
        unsafe {
            let mut this = ManuallyDrop::new(self);

            // this cast is guaranteed correct because
            // `[MaybeUninit<T>; N]` is at offset 0
            (&raw mut this).cast::<[T; N]>().read()
        }
    }

    /// Gets a bitwise copy of this Builder, requires `T: Copy`.
    pub const fn copy(&self) -> Self
    where
        T: Copy,
    {
        ArrayBuilderInner {
            ..*as_inner::<D, _>(&self.inner)
        }
        .into_builder()
    }

    /// Converts this `ArrayBuilder` to have a `MayDrop` drop flavor.
    pub const fn into_may_drop(self) -> ArrayBuilder<T, N, MayDrop> {
        self.into_any_flavor()
    }

    /// Converts this `ArrayBuilder` to have any flavor.
    const fn into_any_flavor<D2: DropFlavor>(self) -> ArrayBuilder<T, N, D2> {
        // SAFETY: changing the D type parameter does not change the layout of the type
        unsafe { crate::__priv_transmute!(ArrayBuilder<T, N, D>, ArrayBuilder<T, N, D2>, self) }
    }

    /// Helper for inferring the length of the built array from an [`IntoIter`].
    pub const fn infer_length_from_consumer<U>(&self, _consumer: &IntoIter<U, N>) {}
}

impl<T, const N: usize> Default for ArrayBuilder<T, N, MayDrop> {
    fn default() -> Self {
        Self::of_drop()
    }
}

impl<T: Copy, const N: usize> Default for ArrayBuilder<T, N, NonDrop> {
    fn default() -> Self {
        Self::of_copy()
    }
}

impl<T: Debug, const N: usize, D: DropFlavor> Debug for ArrayBuilder<T, N, D> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = as_inner::<D, _>(&self.inner);

        fmt.debug_struct("ArrayBuilder")
            .field("array", &self.as_slice())
            .field("uninit_len", &(N - this.inited))
            .finish()
    }
}

impl<T: Clone, const N: usize, D: DropFlavor> Clone for ArrayBuilder<T, N, D> {
    fn clone(&self) -> Self {
        let mut this = Self::of_any();
        for elem in self.as_slice() {
            this.push(elem.clone());
        }
        this
    }
}

impl<T, const N: usize> Drop for ArrayBuilderInner<T, N> {
    fn drop(&mut self) {
        unsafe {
            let inited = self.inited;

            let ptr = self.array.as_mut_ptr().cast::<T>();

            core::ptr::slice_from_raw_parts_mut(ptr, inited).drop_in_place();
        }
    }
}
