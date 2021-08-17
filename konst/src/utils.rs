#[doc(hidden)]
#[repr(C)]
#[cfg(feature = "constant_time_slice")]
pub(crate) union Dereference<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}

#[cfg(all(feature = "constant_time_slice", feature = "mut_refs"))]
mod mut_refs {
    use core::mem::ManuallyDrop;

    #[repr(C)]
    #[doc(hidden)]
    pub(crate) union BorrowMut<'a, T: ?Sized> {
        ptr: *mut T,
        reff: ManuallyDrop<&'a mut T>,
    }

    pub(crate) const unsafe fn deref_raw_mut_ptr<'a, T: ?Sized>(ptr: *mut T) -> &'a mut T {
        ManuallyDrop::into_inner(BorrowMut { ptr }.reff)
    }

    pub(crate) const unsafe fn slice_from_raw_parts_mut<'a, T>(
        ptr: *mut T,
        len: usize,
    ) -> &'a mut [T] {
        let ptr = core::ptr::slice_from_raw_parts_mut(ptr, len);
        ManuallyDrop::into_inner(BorrowMut { ptr }.reff)
    }
}

#[doc(hidden)]
#[cfg(all(feature = "constant_time_slice", feature = "mut_refs"))]
pub(crate) use mut_refs::{deref_raw_mut_ptr, slice_from_raw_parts_mut, BorrowMut};

#[doc(hidden)]
#[cfg(feature = "constant_time_slice")]
pub(crate) const unsafe fn slice_from_raw_parts<'a, T>(ptr: *const T, len: usize) -> &'a [T] {
    let ptr = core::ptr::slice_from_raw_parts(ptr, len);
    Dereference { ptr }.reff
}

#[allow(dead_code)]
#[inline]
pub(crate) const fn saturating_sub(l: usize, r: usize) -> usize {
    let (sub, overflowed) = l.overflowing_sub(r);
    if overflowed {
        0
    } else {
        sub
    }
}

#[inline]
#[cfg(feature = "constant_time_slice")]
pub(crate) const fn min_usize(l: usize, r: usize) -> usize {
    if l < r {
        l
    } else {
        r
    }
}
