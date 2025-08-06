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

#[cfg(feature = "iter")]
mod option_iterators;

#[cfg(feature = "iter")]
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
        match $crate::option::__opt_and_val($e, $v) {
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
        match $crate::option::__opt($e) {
            opt => {
                // using Option::unwrap to work around the inability to
                // destructure Option<T: Drop> by value in const
                let ret = if let $crate::__::Some(_) = opt {
                    $crate::__::Option::unwrap(opt)
                } else {
                    $crate::option::__unwrap_or_else_helper(opt, $v)
                };

                ret
            }
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take no arguments")
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
        match ($crate::option::__opt($e), $v) {
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
        match $crate::option::__opt($e) {
            opt => {
                // using Option::unwrap to work around the inability to
                // destructure Option<T: Drop> by value in const
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
        $crate::__::compile_error!("expected the closure to take no arguments")
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
        match $crate::option::__opt($opt) {
            opt => {
                // using Option::unwrap to work around the inability to
                // destructure Option<T: Drop> by value in const
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
        $crate::__::compile_error!("expected the closure to take a pattern as an argument")
    };
    ($opt:expr, || $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take a pattern as an argument")
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
        match $crate::option::__opt($opt) {
            opt => {
                // using Option::unwrap to work around the inability to
                // destructure Option<T: Drop> by value in const
                if let $crate::__::Some(_) = opt {
                    let $param = $crate::__::Option::unwrap(opt);
                    let ret: $crate::__::Option<_> = $mapper;
                    ret
                } else {
                    $crate::__::forget(opt);
                    $crate::__::None
                }
            }
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take a pattern as an argument")
    };
    ($opt:expr, || $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take a pattern as an argument")
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
        match $crate::option::__opt($opt) {
            opt @ $crate::__::Some(_) => opt,
            mut opt @ $crate::__::None => {
                $crate::__utils::__overwrite(&mut opt, $mapper);
                opt
            }
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take no arguments")
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
        match $crate::option::__opt($e) {
            opt @ $crate::__::Some(x)
                if {
                    let $param = &x;
                    $v
                } =>
            {
                opt
            }
            _ => $crate::__::None,
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take a pattern as an argument")
    };
    ($opt:expr, || $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take a pattern as an argument")
    };
    ($e:expr, $function:path $(,)?) => {
        match $crate::option::__opt($e) {
            $crate::__::Some(x) if $function(&x) => $crate::__::Some(x),
            _ => $crate::__::None,
        }
    };
}

/// A const equivalent of [`Option::get_or_insert`]
///
/// # Example
///
/// ```rust
/// use konst::option;
///
/// const AA: Option<u8> = {
///     let mut ret = None;
///     *option::get_or_insert!(&mut ret, 3) += 100;
///     ret
/// };
///
/// assert_eq!(AA, Some(103));
///
///
/// const BB: Option<u8> = {
///     let mut ret = Some(5);
///     *option::get_or_insert!(&mut ret, 0) += 100;
///     ret
/// };
///
/// assert_eq!(BB, Some(105));
/// ```
///
#[doc(inline)]
pub use crate::__get_or_insert as get_or_insert;

#[doc(hidden)]
#[macro_export]
macro_rules! __get_or_insert {
    ($opt:expr, $inserted:expr $(,)?) => {
        match $crate::option::__optmut_val($opt, $inserted) {
            ($crate::__::Some(val), _inserted) => val,
            (opt @ None, inserted) => {
                $crate::option::__overwrite_some(opt, inserted);

                let ret = $crate::option::__unwrap_mut(opt);
                ret
            }
        }
    };
}

/// A const equivalent of [`Option::get_or_insert_with`]
///
/// # Example
///
/// ```rust
/// use konst::option;
///
/// const AA: Option<u8> = {
///     let mut ret = None;
///
///     // You can use a closure-like syntax to initialize the Option.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     *option::get_or_insert_with!(&mut ret, || 3) += 100;
///     ret
/// };
///
/// assert_eq!(AA, Some(103));
///
///
/// const BB: Option<u8> = {
///     let mut ret = None;
///     *option::get_or_insert_with!(&mut ret, func) += 100;
///     ret
/// };
///
/// assert_eq!(BB, Some(121));
///
/// const fn func() -> u8 {
///     21
/// }
///
///
/// const CC: Option<u8> = {
///     let mut ret = Some(5);
///     *option::get_or_insert_with!(&mut ret, || unreachable!()) += 100;
///     ret
/// };
///
/// assert_eq!(CC, Some(105));
/// ```
///
#[doc(inline)]
pub use crate::__get_or_insert_with as get_or_insert_with;

#[doc(hidden)]
#[macro_export]
macro_rules! __get_or_insert_with {
    ($opt:expr, || $default:expr $(,)?) => {
        match $crate::__optmut!($opt).reff {
            $crate::__::Some(val) => val,
            opt @ None => {
                $crate::option::__overwrite_some(opt, $default);

                let ret = $crate::option::__unwrap_mut(opt);
                ret
            }
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take no arguments")
    };
    ($opt:expr, $default:expr $(,)?) => {
        $crate::option::get_or_insert_with!($opt, || $default())
    };
}

/// A const equivalent of [`Option::insert`]
///
/// # Example
///
/// ```rust
/// use konst::option;
///
/// const AA: Option<u8> = {
///     let mut ret = None;
///     *option::insert!(&mut ret, 3) += 100;
///     ret
/// };
///
/// assert_eq!(AA, Some(103));
///
/// const BB: Option<u8> = {
///     let mut ret = Some(5);
///     *option::insert!(&mut ret, 13) += 100;
///     ret
/// };
///
/// assert_eq!(BB, Some(113));
/// ```
///
#[doc(inline)]
pub use crate::__option_insert as insert;

#[doc(hidden)]
#[macro_export]
macro_rules! __option_insert {
    ($opt:expr, $inserted:expr $(,)?) => {
        match $crate::option::__optmut_val($opt, $inserted) {
            (opt, val) => {
                *opt = $crate::__::Some(val);
                let ret = $crate::option::__unwrap_mut(opt);
                ret
            }
        }
    };
}

/// A const equivalent of [`Option::zip`]
///
/// # Example
///
/// ```rust
/// use konst::option;
///
/// const VALS: [Option<(u8, char)>; 4] = [
///     option::zip!(None, None),
///     option::zip!(None, Some('a')),
///     option::zip!(Some(3), None),
///     option::zip!(Some(3), Some('a')),
/// ];
///
/// assert_eq!(VALS, [None, None, None, Some((3, 'a'))]);
/// ```
///
#[doc(inline)]
pub use crate::__option_zip as zip;

#[doc(hidden)]
#[macro_export]
macro_rules! __option_zip {
    ($left:expr, $right:expr) => {
        // ensuring that both arguments are always evaluated
        match $crate::option::__opt_pair($left, $right) {
            (left, right) => {
                $crate::if_let_Some! {l = left => {
                    $crate::if_let_Some!{r = right => {
                        $crate::__::Some((l, r))
                    } else {
                        $crate::__::None
                    }}
                } else {
                    $crate::__::None
                }}
            }
        }
    };
}

/// A const equivalent of [`Option::unzip`]
///
/// # Example
///
/// ```rust
/// use konst::option;
///
/// const VALS: [(Option<u8>, Option<char>); 2] = [
///     option::unzip(None),
///     option::unzip(Some((3, 'a'))),
/// ];
///
/// assert_eq!(VALS, [(None, None), (Some(3), Some('a'))]);
/// ```
///
pub const fn unzip<T, U>(opt: Option<(T, U)>) -> (Option<T>, Option<U>) {
    crate::if_let_Some! {tuple = opt => {
        crate::destructure!{(l, r) = tuple}

        (Some(l), Some(r))
    } else {
        (None, None)
    }}
}

/// A const equivalent of [`Option::is_some_and`]
///
/// The `Option` argument is implicitly borrowed.
///
/// # Example
///
/// ```rust
/// use konst::option;
///
/// // You can use a closure-like syntax.
/// // `return` inside the "closure" returns from the function where this macro is called.
/// const AA: bool = option::is_some_and!(None::<u8>, |_| unreachable!());
/// assert_eq!(AA, false);
///
/// const BB: bool = option::is_some_and!(Some(3), |x| *x == 3);
/// assert_eq!(BB, true);
///
/// // explicitly borrowing the Option
/// const CC: bool = option::is_some_and!(&Some(5), is_ten);
/// assert_eq!(CC, false);
///
///
/// const fn is_ten(n: &u8) -> bool {
///     *n == 10
/// }
/// ```
///
#[doc(inline)]
pub use crate::__option_is_some_and as is_some_and;

#[doc(hidden)]
#[macro_export]
macro_rules! __option_is_some_and {
    ($opt:expr, |$param:pat_param| $pred:expr $(,)?) => {
        match $crate::__optref!(&$opt).reff {
            $crate::__::Some(reff) => {
                let $param = reff;
                $pred
            }
            $crate::__::None => false,
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take a pattern as an argument")
    };
    ($opt:expr, || $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take a pattern as an argument")
    };
    ($opt:expr, $pred:expr $(,)?) => {
        $crate::option::is_some_and!($opt, |x| $pred(x))
    };
}

/// A const equivalent of [`Option::is_none_or`]
///
/// The `Option` argument is implicitly borrowed.
///
/// # Example
///
/// ```rust
/// use konst::option;
///
/// // You can use a closure-like syntax.
/// // `return` inside the "closure" returns from the function where this macro is called.
/// const AA: bool = option::is_none_or!(None::<u8>, |_| unreachable!());
/// assert_eq!(AA, true);
///
/// const BB: bool = option::is_none_or!(Some(3), |x| *x == 3);
/// assert_eq!(BB, true);
///
/// // explicitly borrowing the Option
/// const CC: bool = option::is_none_or!(&Some(5), is_ten);
/// assert_eq!(CC, false);
///
///
/// const fn is_ten(n: &u8) -> bool {
///     *n == 10
/// }
/// ```
///
#[doc(inline)]
pub use crate::__option_is_none_or as is_none_or;

#[doc(hidden)]
#[macro_export]
macro_rules! __option_is_none_or {
    ($opt:expr, |$param:pat_param| $pred:expr $(,)?) => {
        match $crate::__optref!(&$opt).reff {
            $crate::__::Some(reff) => {
                let $param = reff;
                $pred
            }
            $crate::__::None => true,
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take a pattern as an argument")
    };
    ($opt:expr, || $($anything:tt)* ) => {
        $crate::__::compile_error!("expected the closure to take a pattern as an argument")
    };
    ($opt:expr, $pred:expr $(,)?) => {
        $crate::option::is_none_or!($opt, |x| $pred(x))
    };
}

///////////////////////////////////////////////

#[doc(hidden)]
pub struct __OptRef<'a, T> {
    pub reff: &'a Option<T>,
}

#[doc(hidden)]
#[macro_export]
macro_rules! __optref {
    ($($reff:tt)*) => {
        $crate::option::__OptRef { reff: $($reff)* }
    }
}

#[doc(hidden)]
pub struct __OptMut<'a, T> {
    pub reff: &'a mut Option<T>,
}

#[doc(hidden)]
#[macro_export]
macro_rules! __optmut {
    ($($reff:tt)*) => {
        $crate::option::__OptMut { reff: $($reff)* }
    }
}

#[inline(always)]
#[doc(hidden)]
pub const fn __opt<T>(opt: Option<T>) -> Option<T> {
    opt
}

#[inline(always)]
#[doc(hidden)]
pub const fn __opt_pair<T, U>(l: Option<T>, r: Option<U>) -> (Option<T>, Option<U>) {
    (l, r)
}

#[inline(always)]
#[doc(hidden)]
pub const fn __opt_and_val<T>(opt: Option<T>, val: T) -> (Option<T>, T) {
    (opt, val)
}

#[inline(always)]
#[doc(hidden)]
pub const fn __optmut_val<T>(opt: &mut Option<T>, val: T) -> (&mut Option<T>, T) {
    (opt, val)
}

#[inline(always)]
#[doc(hidden)]
pub const fn __unwrap_or_else_helper<T>(opt: Option<T>, val: T) -> T {
    core::mem::forget(opt);
    val
}

#[inline(always)]
#[doc(hidden)]
#[track_caller]
pub const fn __unwrap_mut<T>(opt: &mut Option<T>) -> &mut T {
    match opt {
        Some(x) => x,
        None => __panic_on_none(),
    }
}

#[inline(always)]
#[doc(hidden)]
#[track_caller]
pub const fn __overwrite_some<T>(opt: &mut Option<T>, val: T) {
    crate::__utils::__overwrite(opt, Some(val))
}

#[cold]
#[track_caller]
const fn __panic_on_none() -> ! {
    panic!("called `Option::unwrap()` on a `None` value")
}
