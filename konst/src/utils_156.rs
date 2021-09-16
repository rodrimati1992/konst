use core::mem::ManuallyDrop;

#[repr(C)]
pub(crate) union Transmuter<F, T> {
    pub(crate) from: ManuallyDrop<F>,
    pub(crate) to: ManuallyDrop<T>,
}

#[repr(C)]
pub(crate) union PtrToRef<'a, P: ?Sized> {
    pub(crate) ptr: *const P,
    pub(crate) reff: &'a P,
}

macro_rules! __priv_transmute {
    ($from:ty, $to:ty, $value:expr) => {{
        core::mem::ManuallyDrop::into_inner(
            crate::utils_156::Transmuter::<$from, $to> {
                from: core::mem::ManuallyDrop::new($value),
            }
            .to,
        )
    }};
}
pub(crate) use __priv_transmute;

macro_rules! __priv_transmute_ref {
    ($from:ty, $to:ty, $reference:expr) => {
        match $reference {
            ptr => {
                let ptr: *const $from = ptr;
                crate::utils_156::PtrToRef::<$to> {
                    ptr: ptr as *const $to,
                }
                .reff
            }
        }
    };
}
pub(crate) use __priv_transmute_ref;
