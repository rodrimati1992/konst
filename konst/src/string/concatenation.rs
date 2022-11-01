/// Macro equivalent of `<[&str]>::concat`, which takes a constant as an argument.
///
/// This acts like a compile-time-evaluated version of this function:
/// ```rust
/// # trait StrOrChar: Copy {}
/// pub const fn str_concat(strings: &'static [impl StrOrChar]) -> &'static str
/// # { "" }
/// ```
///
/// Where `impl StrOrChar` is either a `&'static str` or `char`
///
/// # Example
///
/// ```rust
/// use konst::string::str_concat;
///
/// {
///     const S: &[&str] = &["these ", "are ", "words"];
///     assert_eq!(str_concat!(S), "these are words");
///    
///     assert_eq!(str_concat!(&[]), "");
///    
///     assert_eq!(str_concat!(&["foo", "bar", "baz"]), "foobarbaz");
/// }
///
/// {
///     const C: &[char] = &['c', 'h', 'a', 'r', 's'];
///     assert_eq!(str_concat!(C), "chars");
///    
///     assert_eq!(str_concat!(&['q'; 10]), "qqqqqqqqqq");
/// }
///
///
/// ```
pub use konst_kernel::string_concat as str_concat;

/// Macro equivalent of `<[&str]>::join`, which takes constants as arguments.
///
/// This acts like a compile-time-evaluated version of this function:
/// ```rust
/// # trait StrOrChar: Copy {}
/// pub const fn str_join(
///     delimiter: impl StrOrChar,
///     strings: &'static [&'static str],
/// ) -> &'static str
/// # { "" }
/// ```
///
/// Where `impl StrOrChar` is either a `&'static str` or `char`
///
/// # Example
///
/// ```rust
/// use konst::string::str_join;
///
/// {
///     const COMMA: &str = ",";
///     const S: &[&str] = &["these", "are", "words"];
///     assert_eq!(str_join!(COMMA, S), "these,are,words");
/// }
///
/// assert_eq!(str_join!(",", &[]), "");
///
/// assert_eq!(str_join!(" ", &["foo", "bar", "baz"]), "foo bar baz");
///
/// // char separator
/// assert_eq!(str_join!(' ', &["foo", "bar", "baz"]), "foo bar baz");
///
/// ```
pub use konst_kernel::string_join as str_join;

/// Makes a `&'static str` from an const iterator over `&str`s or `char`s
///
/// # Example
///
/// ### Iterator over strings
///
/// ```rust
/// use konst::string;
///
/// const S: &str = string::from_iter!(
///     &["foo", "bar", "baz"],
///         flat_map(|s| {
///             // By value iteration over arrays isn't supported,
///             // but by-reference iteration is supported
///             &[*s, ", "]
///         })
/// );
///
/// assert_eq!(S, "foo, bar, baz, ");
///
/// ```
///
/// ### Iterator over chars
///
/// ```rust
/// use konst::{iter, string};
///
/// const S: &str = string::from_iter!('a'..='z');
///
/// assert_eq!(S, "abcdefghijklmnopqrstuvwxyz");
///
/// ```
#[cfg(feature = "iter")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "iter")))]
pub use konst_kernel::str_from_iter as from_iter;
