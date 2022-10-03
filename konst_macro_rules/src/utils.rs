#[doc(hidden)]
#[repr(C)]
pub union Dereference<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}

#[cfg(feature = "mut_refs")]
mod mut_refs {
    use core::mem::ManuallyDrop;

    #[doc(hidden)]
    #[repr(C)]
    pub(crate) union BorrowMut<'a, T: ?Sized> {
        ptr: *mut T,
        reff: ManuallyDrop<&'a mut T>,
    }

    pub(crate) const unsafe fn deref_raw_mut_ptr<'a, T: ?Sized>(ptr: *mut T) -> &'a mut T {
        ManuallyDrop::into_inner(BorrowMut { ptr }.reff)
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure {
    (
        ($($macro:tt)*)
        ($($args:tt)*)
        $expected_args:tt,
        |$($pattern:pat),* $(,)?| $v:expr $(,)?
    ) => {
        $($macro)* ! {
            $($args)*
            |$($pattern),*| $v
        }
    };
    (($($macro:tt)*) ($($args:tt)*) ($($exp_arg:ident),*), $v:expr $(,)?) => {
        match $v {func => {
            $($macro)* ! {
                $($args)*
                |$($exp_arg),*| func($($exp_arg),*)
            }
        }}
    };
    ($arg:tt, $($anything:tt)* ) => {
        compile_error!("expected a closure argument")
    };
}

#[cfg(feature = "mut_refs")]
pub(crate) use mut_refs::{deref_raw_mut_ptr, BorrowMut};
