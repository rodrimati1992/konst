#[doc(hidden)]
#[macro_export]
macro_rules! __for_each_hidden {
    (
        $pattern:pat in $iterator:expr
            $($(,)+ $iterator_method:ident ($($method_args:tt)*) )*
            $(,)?
        =>
        $($code:tt)*
    ) => ({
        $crate::iter::eval!{
            $iterator
                $(, $iterator_method ($($method_args)*) )*
                ,for_each(|$pattern| { $($code)* })
        }
    });
}

/// Iterates over all elements of an [iterator](crate::iter::ConstIntoIter),
/// const equivalent of [`Iterator::for_each`]
///
/// This macro supports emulating iterator methods by expanding to equivalent code.
/// They are documented in the [`iterator_dsl`] module,
/// because they are also supported by other `konst::iter` macros.
///
/// # Examples
///
/// ### Custom iterator
///
/// ```rust
/// use konst::iter::{ConstIntoIter, IsIteratorKind};
///
/// struct Upto10(u8);
///
/// impl ConstIntoIter for Upto10 {
///     type Kind = IsIteratorKind;
///     type IntoIter = Self;
///     type Item = u8;
///     const ITEMS_NEED_DROP: bool = false;
/// }
///
/// impl Upto10 {
///     const fn next(&mut self) -> Option<u8> {
///         if self.0 < 10 {
///             let ret = self.0;
///             self.0 += 1;
///             Some(ret)
///         } else {
///             None
///         }
///     }
/// }
///
/// const N: u32 = {
///     let mut n = 0u32;
///     konst::iter::for_each!{elem in Upto10(7) =>
///         n = n * 10 + elem as u32;
///     }
///     n
/// };
///
/// assert_eq!(N, 789);
///
/// ```
///
/// ### Summing pairs
///
// TODO: remove the ignore once array has by-value iteration
/// ```rust, ignore
/// use konst::iter::for_each;
///     
/// const fn add_pairs<const N: usize>(l: [u32; N], r: [u32; N]) -> [u32; N] {
///     let mut out = [0u32; N];
///
///     for_each!{(i, val) in l,zip(r),map(|(l, r)| l + r),enumerate() =>
///         out[i] = val;
///     }
///
///     out
/// }
///
/// assert_eq!(add_pairs([], []), []);
/// assert_eq!(add_pairs([3], [5]), [8]);
/// assert_eq!(add_pairs([3, 5], [8, 13]), [11, 18]);
///
/// ```
///
/// [`iterator_dsl`]: crate::iter::iterator_dsl
#[doc(inline)]
pub use __for_each_hidden as for_each;
