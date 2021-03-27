//! `const` equivalents of `Option` methods.

/// A const equivalent of `Option::unwrap_or`
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[u32] = &[
///     option::unwrap_or!(Some(3), 10000),
///     option::unwrap_or!(None, 5),
/// ];
///
/// assert_eq!(ARR, &[3, 5]);
///
/// ```
///
#[doc(inline)]
pub use konst_macro_rules::opt_unwrap_or as unwrap_or;

/// A const equivalent of `Option::unwrap_or_else`
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[u32] = &[
///     // You can use a closure-like syntax to run code when the Option argument is None.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::unwrap_or_else!(Some(3), || loop{}),
///     option::unwrap_or_else!(None, || 5),
///
///     // You can also pass functions
///     option::unwrap_or_else!(Some(8), thirteen),
///     option::unwrap_or_else!(None, thirteen),
/// ];
///
/// assert_eq!(ARR, &[3, 5, 8, 13]);
///
/// const fn thirteen() -> u32 {
///     13
/// }
/// ```
///
#[doc(inline)]
pub use konst_macro_rules::opt_unwrap_or_else as unwrap_or_else;

/// A const equivalent of `Option::ok_or`
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Result<u32, u32>] = &[
///     option::ok_or!(Some(3), 10000),
///     option::ok_or!(None, 5),
/// ];
///
/// assert_eq!(ARR, &[Ok(3), Err(5)]);
///
/// ```
#[doc(inline)]
pub use konst_macro_rules::opt_ok_or as ok_or;

/// A const equivalent of `Option::ok_or_else`
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Result<u32, u32>] = &[
///     // You can use a closure-like syntax to run code when the Option argument is None.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::ok_or_else!(Some(3), || loop{}),
///     option::ok_or_else!(None, || 5),
///
///     // You can also pass functions
///     option::ok_or_else!(Some(8), thirteen),
///     option::ok_or_else!(None, thirteen),
/// ];
///
/// assert_eq!(ARR, &[Ok(3), Err(5), Ok(8), Err(13)]);
///
/// const fn thirteen() -> u32 {
///     13
/// }
/// ```
#[doc(inline)]
pub use konst_macro_rules::opt_ok_or_else as ok_or_else;

/// A const equivalent of `Option::map`
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Option<u32>] = &[
///     // You can use a closure-like syntax to pass code that maps the Some variant.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::map!(Some(3), |x| x * 3),
///     option::map!(None::<u32>, |_| loop{}),
///
///     // You can also pass functions
///     option::map!(Some(8), double),
///     option::map!(None::<u32>, double),
/// ];
///
/// assert_eq!(ARR, &[Some(9), None, Some(16), None]);
///
/// const fn double(x: u32) -> u32 {
///     x * 2
/// }
///
/// ```
#[doc(inline)]
pub use konst_macro_rules::opt_map as map;

/// A const equivalent of `Option::and_then`
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Option<u32>] = &[
///     // You can use a closure-like syntax to pass code that uses the value in the Some variant.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::and_then!(Some(3), |x| Some(x * 3)),
///     option::and_then!(Some(3), |_| None),
///     option::and_then!(None::<u32>, |_| loop{}),
///
///     // You can also pass functions
///     option::and_then!(Some(23), checked_sub),
///     option::and_then!(Some(9), checked_sub),
///     option::and_then!(None::<u32>, checked_sub),
/// ];
///
/// assert_eq!(ARR, &[Some(9), None, None, Some(13), None, None]);
///
/// const fn checked_sub(x: u32) -> Option<u32> {
///     x.checked_sub(10)
/// }
///
/// ```
#[doc(inline)]
pub use konst_macro_rules::opt_and_then as and_then;

/// A const equivalent of `Option::or_else`
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Option<u32>] = &[
///     // You can use a closure-like syntax to pass code that runs on None.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::or_else!(Some(3), || loop{}),
///     option::or_else!(None::<u32>, || Some(5)),
///
///     // You can also pass functions
///     option::or_else!(Some(8), thirteen),
///     option::or_else!(None::<u32>, thirteen),
/// ];
///
/// assert_eq!(ARR, &[Some(3), Some(5), Some(8), Some(13)]);
///
/// const fn thirteen() -> Option<u32> {
///     Some(13)
/// }
///
/// ```
#[doc(inline)]
pub use konst_macro_rules::opt_or_else as or_else;

/// A const equivalent of `Option::flatten`
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Option<u32>] = &[
///     option::flatten!(Some(Some(8))),
///     option::flatten!(None),
/// ];
///
/// assert_eq!(ARR, &[Some(8), None]);
///
/// ```
#[doc(inline)]
pub use konst_macro_rules::opt_flatten as flatten;

/// A const equivalent of `Option::filter`
///
/// # Example
///
/// ```
/// use konst::option;
///
/// const ARR: &[Option<u32>] = &[
///     // You can use a closure-like syntax to pass code that filters the Some variant.
///     // `return` inside the "closure" returns from the function where this macro is called.
///     option::filter!(Some(0), |&x| x == 0),
///     option::filter!(Some(1), |x| *x == 0),
///     option::filter!(None, |_| loop{}),
///
///     // You can also pass functions
///     option::filter!(Some(3), is_odd),
///     option::filter!(Some(4), is_odd),
///     option::filter!(None, is_odd),
/// ];
///
/// assert_eq!(ARR, &[Some(0), None, None, Some(3), None, None]);
///
/// const fn is_odd(x: &u32) -> bool {
///     *x % 2 == 1
/// }
///
/// ```
#[doc(inline)]
pub use konst_macro_rules::opt_filter as filter;