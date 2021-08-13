pub union Dereference<'a, T> {
    pub ptr: *const T,
    pub reff: &'a T,
}

#[cfg(feature = "mut_refs")]
mod mut_refs {
    use core::mem::ManuallyDrop;

    #[doc(hidden)]
    pub(crate) union BorrowMut<'a, T: ?Sized> {
        ptr: *mut T,
        reff: ManuallyDrop<&'a mut T>,
    }

    pub(crate) const unsafe fn deref_raw_mut_ptr<'a, T: ?Sized>(ptr: *mut T) -> &'a mut T {
        ManuallyDrop::into_inner(BorrowMut { ptr }.reff)
    }
}

#[cfg(feature = "mut_refs")]
pub(crate) use mut_refs::{deref_raw_mut_ptr, BorrowMut};
