/// Emulates the [inline const feature], eg: `const{ foo() }`,
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
///
/// [inline const feature]:
/// https://doc.rust-lang.org/1.50.0/unstable-book/language-features/inline-const.html
#[macro_export]
macro_rules! konst {
    ($type:ty, $expr:expr $(,)*) => {{
        const __KONST__: $type = $expr;
        __KONST__
    }};
}
