use core::mem::ManuallyDrop;

pub struct TypeAnnot<T> {
    pub val: T,
}

#[repr(C)]
pub union Transmuter<F, T> {
    pub from: ManuallyDrop<F>,
    pub to: ManuallyDrop<T>,
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_transmute {
    ($from:ty, $to:ty, $value:expr) => {{
        $crate::__::ManuallyDrop::into_inner(
            $crate::__utils::Transmuter::<$from, $to> {
                from: $crate::__::ManuallyDrop::new($value),
            }
            .to,
        )
    }};
}
