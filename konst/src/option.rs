//! `const` equivalents of `Option` methods.
//!
//! # Removed in 0.4.0
//!
//! These items were removed in 0.4.0 because there is an equivalent
//! way to write it in const:
//!
//! - `copied`: [`Option::copied`]
//! - `flatten`: [`Option::flatten`]
//! - `unwrap`: [`Option::unwrap`]
//! - `NONE`: `const { None }`

mod option_iterators;

pub use self::option_iterators::*;

/// A const equivalent of [`Option::unwrap_or`]
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[u32] = &[
///     option::unwrap_or!(Some(3), 10000),
///     option::unwrap_or!(None, 5),
/// ];
///
/// assert_eq!(ARR, &[3, 5]);
///
/// ```
///
#[doc(inline)]
pub use crate::__opt_unwrap_or as unwrap_or;

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_unwrap_or {
    ($e:expr, $v:expr $(,)?) => {
        match ($e, $v) {
            ($crate::__::Some(x), _) => x,
            ($crate::__::None, value) => value,
        }
    };
}

/// A const equivalent of [`Option::unwrap_or_else`]
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[u32] = &[
///     // You can use a closure-like syntax to run code when the Option argument is None.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::unwrap_or_else!(Some(3), || loop{}),
///     option::unwrap_or_else!(None, || 5),
///
///     // You can also pass functions
///     option::unwrap_or_else!(Some(8), thirteen),
///     option::unwrap_or_else!(None, thirteen),
/// ];
///
/// assert_eq!(ARR, &[3, 5, 8, 13]);
///
/// const fn thirteen() -> u32 {
///     13
/// }
/// ```
///
#[doc(inline)]
pub use crate::__opt_unwrap_or_else as unwrap_or_else;

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_unwrap_or_else {
    ($e:expr, || $v:expr $(,)?) => {
        match $e {
            opt => {
                if let $crate::__::Some(_) = opt {
                    $crate::__::Option::unwrap(opt)
                } else {
                    $crate::__::forget(opt);
                    $v
                }
            }
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take no arguments")
    };
    ($e:expr, $v:expr $(,)?) => {
        $crate::__opt_unwrap_or_else! {$e, || $v()}
    };
}

/// A const equivalent of [`Option::ok_or`]
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Result<u32, u32>] = &[
///     option::ok_or!(Some(3), 10000),
///     option::ok_or!(None, 5),
/// ];
///
/// assert_eq!(ARR, &[Ok(3), Err(5)]);
///
/// ```
#[doc(inline)]
pub use crate::__opt_ok_or as ok_or;

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_ok_or {
    ($e:expr, $v:expr $(,)?) => {
        match ($e, $v) {
            ($crate::__::Some(x), _) => $crate::__::Ok(x),
            ($crate::__::None, value) => $crate::__::Err(value),
        }
    };
}

/// A const equivalent of [`Option::ok_or_else`]
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Result<u32, u32>] = &[
///     // You can use a closure-like syntax to run code when the Option argument is None.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::ok_or_else!(Some(3), || loop{}),
///     option::ok_or_else!(None, || 5),
///
///     // You can also pass functions
///     option::ok_or_else!(Some(8), thirteen),
///     option::ok_or_else!(None, thirteen),
/// ];
///
/// assert_eq!(ARR, &[Ok(3), Err(5), Ok(8), Err(13)]);
///
/// const fn thirteen() -> u32 {
///     13
/// }
/// ```
#[doc(inline)]
pub use crate::__opt_ok_or_else as ok_or_else;

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_ok_or_else {
    ($e:expr, || $v:expr $(,)?) => {
        match $e {
            opt => {
                if let $crate::__::Some(_) = opt {
                    $crate::__::Ok($crate::__::Option::unwrap(opt))
                } else {
                    $crate::__::forget(opt);
                    $crate::__::Err($v)
                }
            }
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take no arguments")
    };
    ($e:expr, $v:expr $(,)?) => {
        $crate::__opt_ok_or_else! {$e, || $v()}
    };
}

/// A const equivalent of [`Option::map`]
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Option<u32>] = &[
///     // You can use a closure-like syntax to pass code that maps the Some variant.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::map!(Some(3), |x| x * 3),
///     option::map!(None::<u32>, |_| loop{}),
///
///     // You can also pass functions
///     option::map!(Some(8), double),
///     option::map!(None::<u32>, double),
/// ];
///
/// assert_eq!(ARR, &[Some(9), None, Some(16), None]);
///
/// const fn double(x: u32) -> u32 {
///     x * 2
/// }
///
/// ```
#[doc(inline)]
pub use crate::__opt_map as map;

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_map {
    ($opt:expr, |$param:pat_param| $mapper:expr $(,)? ) => {
        match $opt {
            opt => {
                if let $crate::__::Some(_) = opt {
                    let $param = $crate::__::Option::unwrap(opt);
                    $crate::__::Some($mapper)
                } else {
                    $crate::__::forget(opt);
                    $crate::__::None
                }
            }
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($opt:expr, $function:path $(,)?) => {
        $crate::__opt_map! {$opt, |x| $function(x)}
    };
}

/// A const equivalent of [`Option::and_then`]
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Option<u32>] = &[
///     // You can use a closure-like syntax to pass code that uses the value in the Some variant.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::and_then!(Some(3), |x| Some(x * 3)),
///     option::and_then!(Some(3), |_| None),
///     option::and_then!(None::<u32>, |_| loop{}),
///
///     // You can also pass functions
///     option::and_then!(Some(23), checked_sub),
///     option::and_then!(Some(9), checked_sub),
///     option::and_then!(None::<u32>, checked_sub),
/// ];
///
/// assert_eq!(ARR, &[Some(9), None, None, Some(13), None, None]);
///
/// const fn checked_sub(x: u32) -> Option<u32> {
/// # /*
///     x.checked_sub(10)
/// # */
/// #   let (ret, overflowed) = x.overflowing_sub(10);
/// #   if overflowed { None } else { Some(ret) }
/// }
///
/// ```
#[doc(inline)]
pub use crate::__opt_and_then as and_then;

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_and_then {
    ($opt:expr, |$param:pat_param| $mapper:expr $(,)? ) => {
        match $opt {
            opt => {
                if let $crate::__::Some(_) = opt {
                    let $param = $crate::__::Option::unwrap(opt);
                    $mapper
                } else {
                    $crate::__::forget(opt);
                    $crate::__::None
                }
            }
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($opt:expr, $function:path $(,)?) => {
        $crate::__opt_and_then! {$opt, |x| $function(x)}
    };
}

/// A const equivalent of [`Option::or_else`]
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Option<u32>] = &[
///     // You can use a closure-like syntax to pass code that runs on None.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::or_else!(Some(3), || loop{}),
///     option::or_else!(None::<u32>, || Some(5)),
///
///     // You can also pass functions
///     option::or_else!(Some(8), thirteen),
///     option::or_else!(None::<u32>, thirteen),
/// ];
///
/// assert_eq!(ARR, &[Some(3), Some(5), Some(8), Some(13)]);
///
/// const fn thirteen() -> Option<u32> {
///     Some(13)
/// }
///
/// ```
#[doc(inline)]
pub use crate::__opt_or_else as or_else;

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_or_else {
    ($opt:expr, || $mapper:expr $(,)? ) => {
        match $opt {
            opt @ $crate::__::Some(_) => opt,
            opt @ $crate::__::None => {
                $crate::__::forget(opt);
                $mapper
            }
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take no arguments")
    };
    ($opt:expr, $function:path $(,)?) => {
        $crate::__opt_or_else! {$opt, || $function()}
    };
}

/// A const equivalent of [`Option::filter`]
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Option<u32>] = &[
///     // You can use a closure-like syntax to pass code that filters the Some variant.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::filter!(Some(0), |&x| x == 0),
///     option::filter!(Some(1), |x| *x == 0),
///     option::filter!(None, |_| loop{}),
///
///     // You can also pass functions
///     option::filter!(Some(3), is_odd),
///     option::filter!(Some(4), is_odd),
///     option::filter!(None, is_odd),
/// ];
///
/// assert_eq!(ARR, &[Some(0), None, None, Some(3), None, None]);
///
/// const fn is_odd(x: &u32) -> bool {
///     *x % 2 == 1
/// }
///
/// ```
#[doc(inline)]
pub use crate::__opt_filter as filter;

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_filter {
    ($e:expr, |$param:pat_param| $v:expr $(,)?) => {
        match $e {
            $crate::__::Some(x)
                if {
                    let $param = &x;
                    $v
                } =>
            {
                $crate::__::Some(x)
            }
            _ => $crate::__::None,
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($e:expr, $function:path $(,)?) => {
        match $e {
            $crate::__::Some(x) if $function(&x) => $crate::__::Some(x),
            _ => $crate::__::None,
        }
    };
}
