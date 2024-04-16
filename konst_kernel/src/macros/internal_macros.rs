#[doc(hidden)]
#[macro_export]
macro_rules! __unparenthesize {
    (($($stuff:tt)*)) => { $($stuff)* };
    ($($stuff:tt)*) => { $($stuff)* };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __unparen_pat {
    (($(|)? $($pat:pat_param)|+)) => { ($($pat)|+) };
    (($($stuff:tt)*)) => { $($stuff)* };
    ($($stuff:tt)*) => { $($stuff)* };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ty_or_und {
    () => {
        _
    };
    ($ty:ty) => {
        $ty
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __annotate_type {
    (=> $expr:expr) => {
        $expr
    };
    ($type:ty => $expr:expr) => {
        $crate::utils::TypeAnnot::<$type> { val: $expr }.val
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __choose {
    (true $then:tt $($else:tt)*) => {
        $then
    };
    (false $then:tt $else:tt) => {
        $else
    };
}
