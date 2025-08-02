/// Coerces `reference` to a type that has a `const_eq` or `const_cmp` method.
///
/// # Behavior
///
/// This requires arguments to implement the [`ConstCmp`] trait.
///
/// When a type from the standard library is passed,
/// this wraps it inside a [`CmpWrapper`],
/// which declares `const_eq` and `const_cmp` methods for many standard library types.
///
/// When a user-defined type is used, this evaluates to a reference to the passed in value,
/// dereferencing it as necessary.
///
/// # Limitations
///
/// The parameter(s) must be concrete types, and have a fully inferred type.
/// eg: if you pass an integer literal it must have a suffix to indicate its type.
///
/// # Example
///
/// ```rust
/// use konst::cmp::{CmpWrapper, coerce_to_cmp, impl_cmp};
///
/// struct Unit;
///
/// impl_cmp!{
///     impl Unit;
///     
///     pub const fn const_eq(&self, other: &Self) -> bool {
///         true
///     }
/// }
///
/// let wrapper: CmpWrapper<i32> = coerce_to_cmp!(0i32);
/// assert!( wrapper.const_eq(&0));
/// assert!(!wrapper.const_eq(&1));
///
/// let unit: &Unit = coerce_to_cmp!(Unit);
/// assert!( unit.const_eq(&Unit));
///
///
///
///
///
///
/// ```
///
/// [`ConstCmp`]: crate::cmp::ConstCmp
/// [`CmpWrapper`]: crate::cmp::CmpWrapper
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__coerce_to_cmp as coerce_to_cmp;

#[doc(hidden)]
#[macro_export]
macro_rules! __coerce_to_cmp {
    ($reference:expr $(,)*) => {{
        match $reference {
            ref reference => {
                let marker = $crate::__::IsAConstCmp::NEW;
                if false {
                    _ = marker.infer_type(reference);
                }
                marker.coerce(reference)
            }
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __coerce_to_cmp2 {
    ($left:expr, $right:expr $(,)*) => {{
        match ($left, $right) {
            (left, right) => {
                let l_marker = $crate::__::IsAConstCmp::NEW;
                let r_marker = $crate::__::IsAConstCmp::NEW;
                if false {
                    _ = l_marker.infer_type(left);
                    _ = r_marker.infer_type(right);
                }
                (l_marker.coerce(left), r_marker.unreference(right))
            }
        }
    }};
}
