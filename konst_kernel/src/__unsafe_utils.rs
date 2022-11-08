use core::mem::ManuallyDrop;

#[repr(C)]
pub union Transmuter<F, T> {
    pub from: ManuallyDrop<F>,
    pub to: ManuallyDrop<T>,
}

#[repr(C)]
pub union PtrToRef<'a, P: ?Sized> {
    pub ptr: *const P,
    pub reff: &'a P,
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_transmute {
    ($from:ty, $to:ty, $value:expr) => {{
        $crate::__::ManuallyDrop::into_inner(
            $crate::__unsafe_utils::Transmuter::<$from, $to> {
                from: $crate::__::ManuallyDrop::new($value),
            }
            .to,
        )
    }};
}
