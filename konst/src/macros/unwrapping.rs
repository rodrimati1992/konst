/// For unwrapping `Result`s in const contexts with some error message.
///
/// The error type must have a method with this signature:
///
/// ```rust
/// # struct Foo;
/// # impl Foo {
/// pub const fn panic(&self) -> ! {
/// #   loop{}
/// # }
/// # }
/// ```
///
/// The Results returned by [`Parser`] methods can all be used with this macro.
///
/// # Example
///
#[cfg_attr(feature = "parsing_no_proc", doc = "```rust")]
#[cfg_attr(not(feature = "parsing_no_proc"), doc = "```ignore")]
/// use konst::{Parser, unwrap_ctx};
///
/// let mut parser = Parser::from_str("hello world");
///
/// parser = unwrap_ctx!(parser.strip_prefix("hello "));
///
/// assert_eq!(parser.bytes(), "world".as_bytes());
///
/// ```
///
/// [`Parser`]: ./parsing/struct.Parser.html
#[macro_export]
macro_rules! unwrap_ctx {
    ($e:expr) => {
        match $e {
            $crate::__::Ok(x) => x,
            $crate::__::Err(e) => e.panic(),
        }
    };
}

/// For unwrapping `Result`s in const contexts,
/// with a default value when it's an error.
///
/// # Example
///
/// ### `unwrap_or`
///
/// ```rust
/// use konst::unwrap_res_or;
///
/// let ok: Result<i32, i32> = Ok(3);
/// let err: Result<i32, i32> = Err(5);
///
/// assert_eq!(unwrap_res_or!(ok, 100), 3);
/// assert_eq!(unwrap_res_or!(err, 13), 13);
///
/// ```
///
/// ### `unwrap_or_else`
///
/// ```rust
/// use konst::unwrap_res_or;
///
/// let ok: Result<i32, i32> = Ok(3);
/// let err: Result<i32, i32> = Err(5);
///
/// assert_eq!(unwrap_res_or!(ok, |_| loop{}), 3);
/// assert_eq!(unwrap_res_or!(err, |e| expensive_function(e)), 10);
///
/// # const fn expensive_function(n: i32) -> i32 {
/// #   n * 2
/// # }
/// ```
///
#[macro_export]
macro_rules! unwrap_res_or {
    ($e:expr, |$($pati:pat)?| $v:expr) => {
        match $e {
            $crate::__::Ok(x) => x,
            $crate::__::Err($($pati)?) => $v,
        }
    };
    ($e:expr, $v:expr) => {{
        let value = $v;
        match $e {
            $crate::__::Ok(x) => x,
            $crate::__::Err(_) => value,
        }
    }};
}

/// For unwrapping `Option`s in const contexts, with a default value when it's a `None`.
///
/// # Example
///
/// ### `unwrap_or`
///
/// ```rust
/// use konst::unwrap_opt_or;
///
/// let some: Option<i32> = Some(3);
/// let none: Option<i32> = None;
///
/// assert_eq!(unwrap_opt_or!(some, 100), 3);
/// assert_eq!(unwrap_opt_or!(none, 13), 13);
///
/// ```
///
/// ### `unwrap_or_else`
///
/// ```rust
/// use konst::unwrap_opt_or;
///
/// let some: Option<i32> = Some(3);
/// let none: Option<i32> = None;
///
/// assert_eq!(unwrap_opt_or!(some, |_| loop{}), 3);
/// assert_eq!(unwrap_opt_or!(none, || expensive_function()), 34);
///
/// # const fn expensive_function() -> i32 {
/// #   34
/// # }
/// ```
///
#[macro_export]
macro_rules! unwrap_opt_or {
    ($e:expr, || $v:expr) => {
        match $e {
            $crate::__::Some(x) => x,
            $crate::__::None => $v,
        }
    };
    ($e:expr, |_| $v:expr) => {
        match $e {
            $crate::__::Some(x) => x,
            $crate::__::None => $v,
        }
    };
    ($e:expr, $v:expr) => {{
        let value = $v;
        match $e {
            $crate::__::Some(x) => x,
            $crate::__::None => value,
        }
    }};
}
