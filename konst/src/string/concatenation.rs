/// Macro equivalent of `<[&str]>::concat`,
/// which takes a `&[&str]` constant as input and returns a `&'static str`.
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
