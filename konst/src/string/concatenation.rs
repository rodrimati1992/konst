/// Macro equivalent of `<[&str]>::concat`, which takes a constant as an argument.
///
/// This acts like a compile-time-evaluated version of this function:
/// ```rust
/// pub const fn str_concat(strings: &'static [&'static str]) -> &'static str
/// # { "" }
/// ```
///
/// # Example
///
/// ```rust
/// use konst::string::str_concat;
///
/// const S: &[&str] = &["these ", "are ", "words"];
/// assert_eq!(str_concat!(S), "these are words");
///
/// assert_eq!(str_concat!(&[]), "");
///
/// assert_eq!(str_concat!(&["foo", "bar", "baz"]), "foobarbaz");
///
/// ```
pub use konst_kernel::string_concat as str_concat;

/// Macro equivalent of `<[&str]>::join`, which takes constants as arguments.
///
/// This acts like a compile-time-evaluated version of this function:
/// ```rust
/// pub const fn str_join(
///     delimiter: &'static str,
///     strings: &'static [&'static str],
/// ) -> &'static str
/// # { "" }
/// ```
///
/// # Example
///
/// ```rust
/// use konst::string::str_join;
///
/// const COMMA: &str = ",";
/// const S: &[&str] = &["these", "are", "words"];
/// assert_eq!(str_join!(COMMA, S), "these,are,words");
///
/// assert_eq!(str_join!(",", &[]), "");
///
/// assert_eq!(str_join!(" ", &["foo", "bar", "baz"]), "foo bar baz");
///
/// ```
pub use konst_kernel::string_join as str_join;
