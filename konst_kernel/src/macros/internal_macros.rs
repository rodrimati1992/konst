#[doc(hidden)]
#[macro_export]
macro_rules! __unparenthesize {
    (($($stuff:tt)*)) => { $($stuff)* };
    ($($stuff:tt)*) => { $($stuff)* };
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
