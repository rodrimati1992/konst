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
#[macro_export]
macro_rules! unwrap_res_or {
    ($e:expr, |$(_)?| $v:expr) => {
        match $e {
            $crate::__::Ok(x) => x,
            $crate::__::Err(_) => $v,
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
#[macro_export]
macro_rules! unwrap_opt_or {
    ($e:expr, |$(_)?| $v:expr) => {
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
