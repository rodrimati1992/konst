//! `const` equivalents of `Result` methods.

/// A const equivalent of [`Result::unwrap_or`]
///
/// # Example
///
/// ```rust
/// use konst::result;
///
/// // Necessary for type inference reasons.
/// type Res = Result<u32, u32>;
///
/// const ARR: &[u32] = &[
///     result::unwrap_or!(Res::Ok(3), 5),
///     result::unwrap_or!(Res::Err(8), 13),
/// ];
///
/// assert_eq!(ARR, &[3, 13]);
///
/// ```
///
#[doc(inline)]
pub use crate::__res_unwrap_or as unwrap_or;

#[doc(hidden)]
#[macro_export]
macro_rules! __res_unwrap_or {
    ($res:expr, $v:expr $(,)?) => {
        match $crate::__ResT!($res, $v) {
            $crate::__PResT!($crate::__::Ok(x), _) => x,
            $crate::__PResT!($crate::__::Err(_), value) => value,
        }
    };
}

/// A const equivalent of [`Result::unwrap_or_else`]
///
#[doc = crate::docs::closure_arg_pattern_limitations_docs!("")]
///
/// # Example
///
/// ```rust
/// use konst::result;
///
/// // Necessary for type inference reasons.
/// type Res = Result<u32, u32>;
///
/// const ARR: &[u32] = &[
///     // You can use a closure-like syntax to run code when the Result argument is Err.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     result::unwrap_or_else!(Res::Ok(3), |_| loop{}),
///     result::unwrap_or_else!(Res::Err(8), |x| x + 5),
///
///     // You can also pass functions
///     result::unwrap_or_else!(Res::Ok(21), add_34),
///     result::unwrap_or_else!(Res::Err(55), add_34),
/// ];
///
/// assert_eq!(ARR, &[3, 13, 21, 89]);
///
/// const fn add_34(n: u32) -> u32 {
///     n + 34
/// }
/// ```
///
#[doc(inline)]
pub use crate::__res_unwrap_or_else as unwrap_or_else;

#[doc(hidden)]
#[macro_export]
macro_rules! __res_unwrap_or_else {
    ($res:expr, |$param:pat_param| $expr:expr $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok(x) => x,
            $crate::__::Err($param) => $expr,
        }
    };
    ($res:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, || $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, $function:path $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok(x) => x,
            $crate::__::Err(x) => $function(x),
        }
    };
}

/// Unwraps the `Err` variant of `$res`.
/// If `$res` is an `Ok` it calls the closure argument to convert it into an error.
///
#[doc = crate::docs::closure_arg_pattern_limitations_docs!("")]
///
/// # Example
///
/// ```rust
/// use konst::result;
///
/// // Necessary for type inference reasons.
/// type Res = Result<u16, u32>;
///
/// const ARR: &[u32] = &[
///     // You can use a closure-like syntax to run code when the Result argument is Ok.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     result::unwrap_err_or_else!(Res::Ok(3), |x| (x + 2) as u32),
///     result::unwrap_err_or_else!(Res::Err(8), |_| loop{}),
///
///     // You can also pass functions
///     result::unwrap_err_or_else!(Res::Ok(16), add_34),
///     result::unwrap_err_or_else!(Res::Err(55), add_34),
/// ];
///
/// assert_eq!(ARR, &[5, 8, 50, 55]);
///
/// const fn add_34(n: u16) -> u32 {
///     (n + 34) as u32
/// }
/// ```
///
#[doc(inline)]
pub use crate::__res_unwrap_err_or_else as unwrap_err_or_else;

#[doc(hidden)]
#[macro_export]
macro_rules! __res_unwrap_err_or_else {
    ($res:expr, |$param:pat_param| $expr:expr $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok($param) => $expr,
            $crate::__::Err(x) => x,
        }
    };
    ($res:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, || $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, $function:path $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok(x) => $function(x),
            $crate::__::Err(x) => x,
        }
    };
}

/// A const equivalent of [`Result::ok`]
///
/// # Example
///
/// ```rust
/// use konst::result;
///
/// // Necessary for type inference reasons.
/// type Res = Result<u32, u32>;
///
/// const ARR: &[Option<u32>] = &[
///     result::ok!(Res::Ok(3)),
///     result::ok!(Res::Err(8)),
/// ];
///
/// assert_eq!(ARR, &[Some(3), None]);
///
/// ```
///
#[doc(inline)]
pub use crate::__res_ok as ok;

#[doc(hidden)]
#[macro_export]
macro_rules! __res_ok {
    ($res:expr $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok(x) => $crate::__::Some(x),
            $crate::__::Err(_) => $crate::__::None,
        }
    };
}

/// A const equivalent of [`Result::err`]
///
/// # Example
///
/// ```rust
/// use konst::result;
///
/// // Necessary for type inference reasons.
/// type Res = Result<u32, u32>;
///
/// const ARR: &[Option<u32>] = &[
///     result::err!(Res::Ok(3)),
///     result::err!(Res::Err(8)),
/// ];
///
/// assert_eq!(ARR, &[None, Some(8)]);
///
/// ```
///
#[doc(inline)]
pub use crate::__res_err as err;

#[doc(hidden)]
#[macro_export]
macro_rules! __res_err {
    ($res:expr $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok(_) => $crate::__::None,
            $crate::__::Err(x) => $crate::__::Some(x),
        }
    };
}

/// A const equivalent of [`Result::map`]
///
#[doc = crate::docs::closure_arg_pattern_limitations_docs!("")]
///
/// # Example
///
/// ```rust
/// use konst::result;
///
/// // Necessary for type inference reasons.
/// type Res = Result<u32, u32>;
///
/// const ARR: &[Res] = &[
///     // You can use a closure-like syntax to run code when the Result argument is Ok.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     result::map!(Res::Ok(3), |x| x + 2),
///     result::map!(Res::Err(8), |_| loop{}),
///
///     // You can also pass functions
///     result::map!(Res::Ok(16), add_34),
///     result::map!(Res::Err(55), add_34),
/// ];
///
/// assert_eq!(ARR, &[Ok(5), Err(8), Ok(50), Err(55)]);
///
/// const fn add_34(n: u32) -> u32 {
///     n + 34
/// }
/// ```
///
#[doc(inline)]
pub use crate::__res_map as map;

#[doc(hidden)]
#[macro_export]
macro_rules! __res_map {
    ($res:expr, |$param:pat_param| $expr:expr $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok($param) => $crate::__::Ok($expr),
            $crate::__::Err(x) => $crate::__::Err(x),
        }
    };
    ($res:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, || $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, $function:path $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok(param) => $crate::__::Ok($function(param)),
            $crate::__::Err(x) => $crate::__::Err(x),
        }
    };
}

/// A const equivalent of [`Result::map_err`]
///
#[doc = crate::docs::closure_arg_pattern_limitations_docs!("")]
///
/// # Example
///
/// ```rust
/// use konst::result;
///
/// // Necessary for type inference reasons.
/// type Res = Result<u32, u32>;
///
/// const ARR: &[Res] = &[
///     // You can use a closure-like syntax to run code when the Result argument is Ok.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     result::map_err!(Res::Ok(3), |_| loop{}),
///     result::map_err!(Res::Err(8), |x| x + 5),
///
///     // You can also pass functions
///     result::map_err!(Res::Ok(16), add_34),
///     result::map_err!(Res::Err(55), add_34),
/// ];
///
/// assert_eq!(ARR, &[Ok(3), Err(13), Ok(16), Err(89)]);
///
/// const fn add_34(n: u32) -> u32 {
///     n + 34
/// }
/// ```
///
#[doc(inline)]
pub use crate::__res_map_err as map_err;

#[doc(hidden)]
#[macro_export]
macro_rules! __res_map_err {
    ($res:expr, |$param:pat_param| $expr:expr $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok(x) => $crate::__::Ok(x),
            $crate::__::Err($param) => $crate::__::Err($expr),
        }
    };
    ($res:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, || $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, $function:path $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok(x) => $crate::__::Ok(x),
            $crate::__::Err(x) => $crate::__::Err($function(x)),
        }
    };
}

/// A const equivalent of [`Result::and_then`]
///
#[doc = crate::docs::closure_arg_pattern_limitations_docs!("")]
///
/// # Example
///
/// ```rust
/// use konst::result;
///
/// // Necessary for type inference reasons.
/// type Res = Result<u32, u32>;
///
/// const ARR: &[Res] = &[
///     // You can use a closure-like syntax to run code when the Result argument is Ok.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     result::and_then!(Res::Ok(1), |x| Ok(x + 2)),
///     result::and_then!(Res::Ok(10), |x| Err(x + 4)),
///     result::and_then!(Res::Err(20), |_| loop{}),
///
///     // You can also pass functions
///     result::and_then!(Res::Ok(40), add_2),
///     result::and_then!(Res::Ok(40), add_5),
///     result::and_then!(Res::Err(60), add_5),
/// ];
///
/// assert_eq!(ARR, &[Ok(3), Err(14), Err(20), Ok(42), Err(45), Err(60)]);
///
/// const fn add_2(n: u32) -> Res {
///     Ok(n + 2)
/// }
/// const fn add_5(n: u32) -> Res {
///     Err(n + 5)
/// }
/// ```
///
#[doc(inline)]
pub use crate::__res_and_then as and_then;

#[doc(hidden)]
#[macro_export]
macro_rules! __res_and_then {
    ($res:expr, |$param:pat_param| $expr:expr $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok($param) => $expr,
            $crate::__::Err(x) => $crate::__::Err(x),
        }
    };
    ($res:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, || $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, $function:path $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok(param) => $function(param),
            $crate::__::Err(x) => $crate::__::Err(x),
        }
    };
}

/// A const equivalent of [`Result::or_else`]
///
#[doc = crate::docs::closure_arg_pattern_limitations_docs!("")]
///
/// # Example
///
/// ```rust
/// use konst::result;
///
/// // Necessary for type inference reasons.
/// type Res = Result<u32, u32>;
///
/// const ARR: &[Res] = &[
///     // You can use a closure-like syntax to run code when the Result argument is Err.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     result::or_else!(Res::Ok(1), |_| loop{}),
///     result::or_else!(Res::Err(20), |x| Ok(x + 5)),
///     result::or_else!(Res::Err(20), |x| Err(x + 7)),
///
///     // You can also pass functions
///     result::or_else!(Res::Ok(40), add_2),
///     result::or_else!(Res::Err(60), add_2),
///     result::or_else!(Res::Err(60), add_5),
/// ];
///
/// assert_eq!(ARR, &[Ok(1), Ok(25), Err(27), Ok(40), Ok(62), Err(65)]);
///
/// const fn add_2(n: u32) -> Res {
///     Ok(n + 2)
/// }
/// const fn add_5(n: u32) -> Res {
///     Err(n + 5)
/// }
/// ```
///
#[doc(inline)]
pub use crate::__res_or_else as or_else;

#[doc(hidden)]
#[macro_export]
macro_rules! __res_or_else {
    ($res:expr, |$param:pat_param| $expr:expr $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok(x) => $crate::__::Ok(x),
            $crate::__::Err($param) => $expr,
        }
    };
    ($res:expr, | $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, || $($anything:tt)* ) => {
        compile_error!("expected the closure to take a pattern as an argument")
    };
    ($res:expr, $function:path $(,)?) => {
        match $crate::__Res!($res).res {
            $crate::__::Ok(x) => $crate::__::Ok(x),
            $crate::__::Err(x) => $function(x),
        }
    };
}

#[doc(no_inline)]
pub use const_panic::unwrap_ok as unwrap;

////////////////////////////////////////////////////////////

// for asserting that a macro argument is a Result
#[doc(hidden)]
pub struct __Res<T, E> {
    pub res: Result<T, E>,
}

#[doc(hidden)]
#[macro_export]
macro_rules! __Res {
    ($($res:tt)*) => { $crate::result::__Res { res: $($res)* } };
}

// for asserting that a macro argument is a pair of Result<T, E> and T
#[doc(hidden)]
pub struct __ResT<T, E> {
    pub res: Result<T, E>,
    pub val: T,
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ResT {
    ($res:tt, $val:tt) => {
        $crate::result::__ResT {
            res: $res,
            val: $val,
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __PResT {
    ($res:pat, $val:pat) => {
        $crate::result::__ResT {
            res: $res,
            val: $val,
        }
    };
}
