use core::mem::{ManuallyDrop, MaybeUninit};

#[inline(always)]
pub const fn uninit_array<T, const LEN: usize>() -> [MaybeUninit<T>; LEN] {
    union MakeMUArray<T, const LEN: usize> {
        unit: (),
        array: ManuallyDrop<[MaybeUninit<T>; LEN]>,
    }

    unsafe { ManuallyDrop::into_inner(MakeMUArray { unit: () }.array) }
}

#[inline(always)]
pub const unsafe fn array_assume_init<T, const N: usize>(md: [MaybeUninit<T>; N]) -> [T; N] {
    crate::__priv_transmute! {[MaybeUninit<T>; N], [T; N], md}
}
