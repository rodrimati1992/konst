

/// Const equivalent of [`Option::iter`]
/// 
/// # Example
/// 
/// ```rust
/// use konst::option;
/// 
/// let mut fwd = option::iter(&Some(5));
/// assert_eq!(fwd.next(), Some(&5));
/// assert_eq!(fwd.next(), None);
/// 
/// let mut rev = option::iter(&Some(8)).rev();
/// assert_eq!(rev.next(), Some(&8));
/// assert_eq!(rev.next(), None);
/// ```
pub use konst_kernel::into_iter::option_into_iter::iter;

/// Const equivalent of [`core::option::Iter`]
pub use konst_kernel::into_iter::option_into_iter::Iter;

/// Const equivalent of `core::iter::Rev<core::option::Iter<T>>`
pub use konst_kernel::into_iter::option_into_iter::IterRev;

/// Const equivalent of [`Option::iter_mut`]
/// 
/// # Example
/// 
/// ```rust
/// use konst::option;
/// 
/// {
///     let mut opt = Some(13);
///     let mut fwd = option::iter_mut(&mut opt);
///     assert_eq!(fwd.next(), Some(&mut 13));
///     assert_eq!(fwd.next(), None);
/// }
/// {
///     let mut opt = Some(21);
///     let mut rev = option::iter_mut(&mut opt).rev();
///     assert_eq!(rev.next(), Some(&mut 21));
///     assert_eq!(rev.next(), None);
/// }
/// ```
pub use konst_kernel::into_iter::option_into_iter::iter_mut;

/// Const equivalent of [`core::option::IterMut`]
pub use konst_kernel::into_iter::option_into_iter::IterMut;

/// Const equivalent of `core::iter::Rev<core::option::IterMut<T>>`
pub use konst_kernel::into_iter::option_into_iter::IterMutRev;
