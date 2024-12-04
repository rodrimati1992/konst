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
