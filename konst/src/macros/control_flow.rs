/// Emulates the [inline const feature](`const{ ... }`) in pre-1.79 versions.
///
/// As opposed to inline const, you must pass the type that the expression evaluates to.
///
/// # Limitations
///
/// This can't be used with expressions that reference generic parameters.
///
/// # Example
///
/// ```rust
/// use konst::{konst, eq_str};
///
/// const FOO: &str = "hello";
///
/// # const _: bool = konst!{bool, eq_str(FOO, "hi")};
/// #
/// // By using `konst` here, the function is unconditionally evaluated at compile-time.
/// if konst!{bool, eq_str(FOO, "hi")} {
///     panic!("The constants are equal, this wasn't supposed to happen!!");
/// }
///
/// ```
///
/// [Rust 1.79.0]: 
/// https://blog.rust-lang.org/2024/06/13/Rust-1.79.0.html#inline-const-expressions
///
/// [inline const feature]: 
/// https://blog.rust-lang.org/2024/06/13/Rust-1.79.0.html#inline-const-expressions
#[macro_export]
macro_rules! konst {
    ($type:ty, $expr:expr $(,)*) => {{
        const __KONST__: $type = $expr;
        __KONST__
    }};
}


/// Emulates by-value destructuring of a [`Some`] variant that contains a Drop type in const.
/// 
/// # Motivation
/// 
/// This macro works around the fact that this code
/// 
/// ```rust,compile_fail
/// const fn foo<T>(opt: Option<T>) -> Result<T, ()> {
///     match opt {
///         Some(x) => Ok(x),
///         None => Err(())
///     }
/// }
/// ```
/// causes this error as of Rust 1.83:
/// ```text
/// error[E0493]: destructor of `Option<T>` cannot be evaluated at compile-time
///  --> src/lib.rs:1:17
///   |
/// 1 | const fn foo<T>(opt: Option<T>) -> T {
///   |                 ^^^ the destructor for this type cannot be evaluated in constant functions
/// ...
/// 6 | }
///   | - value is dropped here
/// ```
/// 
/// # Example
/// 
/// ```rust
/// assert_eq!(ok_or_none_error(Some(10)), Ok(10));
/// assert_eq!(ok_or_none_error(None::<String>), Err(ItWasNoneError));
///
///
/// const fn ok_or_none_error<T>(opt: Option<T>) -> Result<T, ItWasNoneError> {
///     konst::if_let_Some!{x = opt => {
///         Ok(x)
///     } else {
///         Err(ItWasNoneError)
///     }}
/// }
/// 
/// #[derive(Debug, PartialEq, Eq)]
/// struct ItWasNoneError;
/// ```
/// 
#[macro_export]
macro_rules! if_let_Some {
    ($some:pat = $e:expr => $then:block $(else $else:block)?) => {
        match $e {opt => 
            if $crate::__::Option::is_some(&opt) {
                let $some = opt.unwrap();
                $then
            } else {
                $crate::__::forget(opt);
                $($else)?
            }
        }
    }
}

/// Emulates by-value a destructuring while let loop over a [`Some`] variant 
/// that contains a Drop type in const.
/// 
/// # Motivation
/// 
/// This macro works around the fact that this code
/// 
/// ```rust,compile_fail
/// use konst::array::ArrayBuilder;
/// 
/// const fn foo<T: SomeTrait>() -> [T; 3] {
///     let mut builder = ArrayBuilder::new();
///     while let Some(x) = produce_option(&builder) {
///         builder.push(x);
///     }
///     builder.build()
/// }
/// 
/// # trait SomeTrait {}
/// # const fn produce_option<T: SomeTrait, U>(_: &U) -> Option<T> {
/// #   None
/// # }
/// ```
/// causes this error as of Rust 1.83:
/// ```text
/// error[E0493]: destructor of `Option<T>` cannot be evaluated at compile-time
///   --> konst/src/macros/control_flow.rs:92:25
///    |
/// 9  |     while let Some(x) = produce_option(&builder) {
///    |                         ^^^^^^^^^^^^^^^^^^^^^^^^ the destructor for this type cannot be evaluated in constant functions
/// 10 |         builder.push(x);
/// 11 |     }
///    |     - value is dropped here
/// ```
/// 
/// # Example
/// 
/// ```rust
/// use konst::array::ArrayBuilder;
/// 
/// assert_eq!(make_strings::<1>(), [String::new()]);
/// assert_eq!(make_strings::<2>(), [String::new(), String::new()]);
/// assert_eq!(make_strings::<3>(), [String::new(), String::new(), String::new()]);
/// 
/// const fn make_strings<const N: usize>() -> [String; N] {
///     let mut builder = ArrayBuilder::new();
///     konst::while_let_Some!{x = produce_option(&builder) =>
///         builder.push(x);
///     }
///     builder.build()
/// }
/// 
/// const fn produce_option<const N: usize>(ab: &ArrayBuilder<String, N>) -> Option<String> {
///     if ab.is_full() {
///         None
///     } else {
///         Some(String::new())
///     }
/// }
/// ```
/// 
#[macro_export]
macro_rules! while_let_Some {
    ($some:pat = $e:expr => $($then:tt)*) => {
        loop {
            match $e {opt => 
                if $crate::__::Option::is_some(&opt) {
                    let $some = opt.unwrap();
                    $($then)*
                } else {
                    $crate::__::forget(opt);
                    break
                }
            }
        }
    }
}








