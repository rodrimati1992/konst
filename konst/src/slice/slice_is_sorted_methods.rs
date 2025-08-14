/// Const equivalent of
/// [`<[T]>::is_sorted`](https://doc.rust-lang.org/std/primitive.slice.html#method.is_sorted)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const ORDERED: &[bool] = &[
///     slice::is_sorted!([0u8; 0]),
///     slice::is_sorted!([3u8, 5, 8]),
///     slice::is_sorted!([3u8, 5, 4]),
/// ];
///
/// assert_eq!(ORDERED, [true, true, false]);
///
/// ```
///
#[doc(inline)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__slice_is_sorted as is_sorted;

#[doc(hidden)]
#[macro_export]
macro_rules! __slice_is_sorted {
    ($slice:expr $(,)?) => {
        $crate::__slice_is_sorted_impl! {
            ($slice, l_item, r_item) {};
            $crate::__::matches!(
                $crate::cmp::const_cmp!(l_item, r_item),
                $crate::__::Less | $crate::__::Equal
            )
        }
    };
}

/// Const equivalent of
/// [`<[T]>::is_sorted_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.is_sorted_by)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const IS_MONOTONIC: &[bool] = &[
///     slice::is_sorted_by!([], |l: &u8, r| *l < *r),
///     slice::is_sorted_by!([3, 5, 8], |l, r| *l < *r),
///     slice::is_sorted_by!([3, 5, 5], |l, r| *l < *r),
/// ];
///
/// assert_eq!(IS_MONOTONIC, [true, true, false]);
///
/// ```
///
#[doc(inline)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__slice_is_sorted_by as is_sorted_by;

#[doc(hidden)]
#[macro_export]
macro_rules! __slice_is_sorted_by {
    ($slice:expr, $($closure:tt)*) => {
        $crate::__slice_is_sorted_impl!{
            ($slice, l_item, r_item) {};
            $crate::__parse_closure_2!(
                ($crate::__eval_closure) ((l_item, r_item),) (is_sorted_by),
                $($closure)*
            )
        }
    };
}

/// Const equivalent of
/// [`<[T]>::is_sorted_by_key`](
/// https://doc.rust-lang.org/std/primitive.slice.html#method.is_sorted_by_key)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const ORDERED_LENGTHS: &[bool] = &[
///     slice::is_sorted_by_key!([], |x: &&str| x.len()),
///     slice::is_sorted_by_key!(["foo", "hello", "world!"], |x| x.len()),
///     slice::is_sorted_by_key!(["foo", "hello", "bob"], |x| x.len()),
/// ];
///
/// assert_eq!(ORDERED_LENGTHS, [true, true, false]);
///
/// ```
///
#[doc(inline)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub use crate::__slice_is_sorted_by_key as is_sorted_by_key;

#[doc(hidden)]
#[macro_export]
macro_rules! __slice_is_sorted_by_key {
    ($slice:expr, $($closure:tt)*) => {
        $crate::__slice_is_sorted_impl!{
            ($slice, l_item, r_item)
            {
                let r_item = $crate::__parse_closure_1!{
                    ($crate::__eval_closure) (r_item,) (@default(0u8) is_sorted_by_key),
                    $($closure)*
                };
            };
            $crate::__::matches!(
                $crate::cmp::const_cmp!(l_item, r_item),
                $crate::__::Less | $crate::__::Equal
            )
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __slice_is_sorted_impl {
    (
        ($slice:expr, $l_item:ident, $r_item:ident)
        {$($convert_item_into_key:tt)*};
        $comparator:expr
    ) => (
        match ($crate::slice::__AssertSlice { x: &$slice }.x) {mut slice => {
            let mut prev = $crate::__::None;

            loop {
                match slice {
                    [] => break true,
                    [$r_item, rem @ ..] => {
                        $($convert_item_into_key)*

                        // `konst::cmp::const_cmp` (used in $comparator) needs
                        // the type of `prev` to be inferred in statements before
                        // `$comparator` to be able to do inherent method dispatch.
                        if false {
                            $crate::iter::__infer_option_of(&$r_item, &prev)
                        }

                        if let $crate::__::Some($l_item) = prev
                        && !$comparator
                        {
                            break false;
                        }

                        prev = $crate::__::Some($r_item);
                        slice = rem;
                    }
                }
            }
        }}
    )
}
