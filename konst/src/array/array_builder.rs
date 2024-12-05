use core::mem::{ManuallyDrop, MaybeUninit};

use crate::array::ArrayConsumer;


/// For constructing an array element by element.
/// 
/// # Example
/// 
/// ```rust
/// use konst::array::ArrayBuilder;
/// 
/// assert_eq!(ARR, [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
/// 
/// const ARR: [u32; 10] = {
///     let mut builder = ArrayBuilder::new();
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
#[repr(C)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "rust_1_83")))]
pub struct ArrayBuilder<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    inited: usize,
}

impl<T, const N: usize> ArrayBuilder<T, N> {
    /// Constructs an empty ArrayBuilder
    pub const fn new() -> Self {
        ArrayBuilder {
            array: crate::maybe_uninit::uninit_array(),
            inited: 0,
        }
    }

    /// The amount of initialized elements in the array
    pub const fn len(&self) -> usize {
        self.inited
    }

    /// Whether the array has been fully initialized
    pub const fn is_full(&self) -> bool {
        self.inited == N
    }

    /// Gets the initialized part of the array as a slice
    pub const fn as_slice(&self) -> &[T] {
        // SAFETY: self.array is guaranteed initialized up to self.inited - 1 inclusive
        unsafe {
            core::slice::from_raw_parts(self.array.as_ptr().cast::<T>(), self.inited)
        }
    }

    /// Gets the initialized part of the array as a mutable slice
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: self.array is guaranteed initialized up to self.inited - 1 inclusive
        unsafe {
            core::slice::from_raw_parts_mut(self.array.as_mut_ptr().cast::<T>(), self.inited)
        }
    }

    /// Appends the `val` element to the array.
    /// 
    /// # Panic
    /// 
    /// Panics if `self.len() == N`, i.e.: the array is fully initialized.
    /// 
    pub const fn push(&mut self, val: T) {
        assert!(self.inited < N, "trying to add an element to full array");

        self.array[self.inited] = MaybeUninit::new(val);

        self.inited += 1;
    }

    /// Unwraps this ArrayBuilder into an array.
    /// 
    /// # Panic
    /// 
    /// Panics if `self.len() != N`, i.e.: the array is not fully initialized.
    /// 
    pub const fn build(self) -> [T; N] {
        assert!(self.is_full(), "trying to unwrap a non-fully-initialized array");

        // SAFETY: self.array is guaranteed fully initialized by the fact that
        //         each element is inited in lockstep with incrementing self.inited by 1,
        //         and the assertion above.
        unsafe {
            let mut this = ManuallyDrop::new(self);

            // this cast is guaranteed correct becaue this struct is `#[repr(C))]`
            // and the first field is a `[MaybeUninit<T>; N]`
            (&raw mut this).cast::<[T; N]>().read()
        }
    }

    /// Helper for inferring the length of the built array from an `ArrayConsumer`.
    pub const fn infer_length_from_consumer<U>(&self, _consumer: &ArrayConsumer<U, N>) {}
}

impl<T, const N: usize> Drop for ArrayBuilder<T, N> {
    fn drop(&mut self) {
        unsafe {
            let inited = self.inited;

            let ptr = self.array.as_mut_ptr().cast::<T>();

            core::ptr::slice_from_raw_parts_mut(ptr, inited).drop_in_place();
        }
    }
}
