#[repr(C)]
pub(crate) union __TransmuteCopy<T: Copy, U: Copy> {
    pub(crate) from: T,
    pub(crate) to: U,
}
