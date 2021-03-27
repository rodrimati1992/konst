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
            $crate::__::Err{$(0: $pati,)? ..} => $v,
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

/// `?`-like macro, which allows optionally mapping errors.
///
/// `?` currently doesn't work in `const fn`s because as of Rust 1.51.0
/// trait methods don't work in `const fn`s.
///
/// # Examples
///
/// ### Basic
///
/// ```rust
/// use konst::try_;
///
/// const OK: Result<&str, u8> = expect_no_whitespace("hello");
/// assert_eq!(OK, Ok("hello"));
///
/// const ERR: Result<&str, u8> = expect_no_whitespace("hello world");
/// assert_eq!(ERR, Err(b' '));
///
///
/// const fn expect_no_whitespace(string: &str) -> Result<&str, u8> {
///     let bytes = string.as_bytes();
///     konst::for_range!{i in 0..bytes.len() =>
///         try_!(assert_not_whitespace(bytes[i]));
///     }
///     Ok(string)
/// }
///
/// const fn assert_not_whitespace(byte: u8) -> Result<(), u8> {
///     if matches!(byte, b'\t' | b'\n' | b'\r' | b' ') {
///         Err(byte)
///     } else {
///         Ok(())
///     }
/// }
///
/// ```
///
/// ### Mapping errors
///
/// ```rust
/// use konst::try_;
///
/// const EVENS: Result<[Even; 4], u32> =
///     array_to_even([0, 2, 4, 6]);
///
/// let new = |n| Even::new(n).unwrap();
/// assert_eq!(EVENS, Ok([new(0), new(2), new(4), new(6)]));
///
///
/// const UNEVEN: Result<[Even; 4], u32> =
///     array_to_even([0, 2, 5, 6]);
///
/// assert_eq!(UNEVEN, Err(5));
///
///
/// const fn array_to_even(arr: [u32; 4]) -> Result<[Even; 4], u32> {
///     let mut ret = [Even::ZERO; 4];
///     
///     konst::for_range!{i in 0..4 =>
///         ret[i] = try_!(Even::new(arr[i]), map_err = |e| e.get() );
///     }
///     
///     Ok(ret)
/// }
///
/// #[derive(Debug, PartialEq)]
/// pub struct Even(u32);
///
/// impl Even {
///     const ZERO: Even = Even(0);
///     
///     pub const fn new(number: u32) -> Result<Self, NotEven> {
///         if number % 2  == 0 {
///             Ok(Even(number))
///         } else {
///             Err(NotEven(number))
///         }
///     }
/// }
///
/// #[derive(Debug, PartialEq)]
/// pub struct NotEven(u32);
///
/// impl NotEven {
///     pub const fn get(&self) -> u32 {
///         self.0
///     }
/// }
///
/// use std::fmt::{self, Display};
///
/// impl Display for NotEven {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         fmt::Debug::fmt(self, f)
///     }
/// }
///
/// impl std::error::Error for NotEven {}
///
/// ```
///
#[macro_export]
macro_rules! try_ {
    ($e:expr, map_err = |$($pati:pat)?| $v:expr) => {
        match $e {
            $crate::__::Ok(x) => x,
            $crate::__::Err{$(0: $pati,)? ..} => return $crate::__::Err($v),
        }
    };
    ($e:expr $(,)?) => {{
        match $e {
            $crate::__::Ok(x) => x,
            $crate::__::Err(e) => return $crate::__::Err(e),
        }
    }};
}

/// `?`-like macro for `Option`s.
///
/// # Example
///
/// ```rust
/// use konst::try_opt;
///
/// const SOME: Option<u8> = sum_u8s(&[3, 5, 8, 13]);
/// assert_eq!(SOME, Some(29));
///
/// const NONE: Option<u8> = sum_u8s(&[3, 5, 8, 13, 240]);
/// assert_eq!(NONE, None);
///
/// const fn sum_u8s(mut nums: &[u8]) -> Option<u8> {
///     let mut sum = 0_u8;
///     while let [first, rem @ ..] = nums {
///         nums = rem;
///         sum = try_opt!(sum.checked_add(*first));
///     }
///     Some(sum)
/// }
///
/// ```
///
#[macro_export]
macro_rules! try_opt {
    ($opt:expr) => {
        match $opt {
            $crate::__::Some(x) => x,
            $crate::__::None => return $crate::__::None,
        }
    };
}
