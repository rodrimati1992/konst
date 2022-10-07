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

macro_rules! make_parse_closure_macro {
    ($_:tt $macro_name:ident $arg_count:tt ($($pat_var:ident)*)) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! $macro_name {
            (
                ($_($macro:tt)*) ($_($args:tt)*)
                $usage_site:tt,
                |$($_$pat_var:pat),* $_(,)?| $v:expr $_(,)?
            ) => {
                $_($macro)* ! {
                    $_($args)*
                    |$($_$pat_var),*| $v
                }
            };
            ($macro:tt $args:tt $usage_site:tt, | $_($anything:tt)* ) => {
                $crate::__parse_closure_emit_error!{
                    $arg_count
                    $usage_site
                }
            };
            ($macro:tt $args:tt $usage_site:tt, || $_($anything:tt)* ) => {
                $crate::__parse_closure_emit_error!{
                    $arg_count
                    $usage_site
                }
            };
            (
                ($_($macro:tt)*) ($_($args:tt)*)
                $usage_site:tt, $v:expr $_(,)?
            ) => {
                match $v {func => {
                    $_($macro)* ! {
                        $_($args)*
                        |$($pat_var),*| func($($pat_var),*)
                    }
                }}
            };
            ( $_($anything:tt)* ) => {
                $crate::__::compile_error!("expected a closure argument")
            };
        }

        pub use $macro_name;
    }
}

make_parse_closure_macro! { $ __parse_closure_1 1 (aaa) }
make_parse_closure_macro! { $ __parse_closure_2 2 (aaa bbb) }

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_emit_error {
    ($count:tt ($($usage_site:tt)*)) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "`",
            $crate::__::stringify!($($usage_site)*),
            "` expects a closure with ",
            $count,
            " argument(s)",
        )}
    };
}

#[cfg(feature = "mut_refs")]
pub(crate) use mut_refs::{deref_raw_mut_ptr, BorrowMut};
