use core::mem::{ManuallyDrop, MaybeUninit};



#[doc(hidden)]
#[macro_export]
macro_rules! __array_map_by_val {
    ($array:expr, $($closure:tt)* ) => (
        match $crate::array::__array_macros_2::ArrayConsumer::new($array) {
            mut consumer => {
                $crate::__::__parse_closure_1!{
                    ($crate::__array_map2)
                    (consumer,) 
                    (array_map),
                    $($closure)*
                }
            }
        }
    );
}


#[doc(hidden)]
#[macro_export]
macro_rules! __array_map2 {
    (
        $consumer:ident,
        ($($pattern:tt)*) $(-> $ret:ty)? $mapper:block $(,)?
    ) => ({
        let mut builder = 
            $crate::array::__array_macros_2::ArrayBuilder::__new $(::<$ret>)? ();

        builder.infer_length_from_consumer(&$consumer);

        while let Some(elem) = $consumer.__next() {
            let val = $crate::__::ManuallyDrop::into_inner(elem);
            let $($pattern)* = val;
            builder.push($mapper);
        }
        $crate::__::mem::forget($consumer);

        builder.into_array()
    })
}

////////////////////////////////////////////////////////////////////////////////

pub struct ArrayConsumer<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    taken: usize,
}

impl<T, const N: usize> ArrayConsumer<T, N> {
    pub const fn new(array: [T; N]) -> Self {
        Self {
            array: array_into_mu(array),
            taken: 0,
        }
    }

    pub const fn __next(&mut self) -> Option<ManuallyDrop<T>> {
        if self.taken >= N {
            return None
        }


        // SAFETY: self.array[self.taken] is guaranteed initialized by the fact that 
        //         each element is taken in lockstep with incrementing self.taken by 1,
        //         and the assertion above.
        let ret = unsafe { take_elem(&mut self.array, self.taken) };
        
        self.taken += 1;

        Some(ret)
    }
}

impl<T, const N: usize> Drop for ArrayConsumer<T, N> {
    fn drop(&mut self) {
        unsafe {
            let left = N - self.taken;

            let ptr = self.array.as_mut_ptr().cast::<T>();

            core::ptr::slice_from_raw_parts_mut(ptr.add(self.taken), left)
                .drop_in_place();
        }
    }
}


////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct ArrayBuilder<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    inited: usize,
}

impl<const N: usize> ArrayBuilder<(), N> {
    pub const fn __new<T>() -> ArrayBuilder<T, N> {
        ArrayBuilder {
            array: crate::maybe_uninit::uninit_array(),
            inited: 0,
        }
    }
}

impl<T, const N: usize> ArrayBuilder<T, N> {
    pub const fn infer_length_from_consumer<U>(&self, _consumer: &ArrayConsumer<U, N>) {}

    pub const fn push(&mut self, val: T) {
        assert!(self.inited < N, "trying to add an element to full array");
        
        self.array[self.inited] = MaybeUninit::new(val);
        
        self.inited += 1;
    }

    pub const fn into_array(self) -> [T; N] {
        assert!(self.inited == N, "trying to unwrap a non-fully-initialized array");

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
}

impl<T, const N: usize> Drop for ArrayBuilder<T, N> {
    fn drop(&mut self) {
        unsafe {
            let inited = self.inited;

            let ptr = self.array.as_mut_ptr().cast::<T>();

            core::ptr::slice_from_raw_parts_mut(ptr, inited)
                .drop_in_place();
        }
    }
}


////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
const fn array_into_mu<T, const N: usize>(arr: [T; N]) -> [MaybeUninit<T>; N] {
    unsafe {
        crate::__::__priv_transmute!{[T; N], [MaybeUninit<T>; N], arr}
    }
}

// # Safety
// 
// arr[i] must be initialized as `T`
const unsafe fn take_elem<T, const N: usize>(
    arr: &mut [MaybeUninit<T>; N],
    i: usize
) -> ManuallyDrop<T> {
    crate::__::__priv_transmute!{
        MaybeUninit<T>, 
        ManuallyDrop<T>, 
        core::mem::replace(&mut arr[i], MaybeUninit::uninit())
    }
}


