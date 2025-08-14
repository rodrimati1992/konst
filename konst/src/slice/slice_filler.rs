/// Const equivalent of
/// [`<[T]>::fill`](https://doc.rust-lang.org/std/primitive.slice.html#method.fill),
/// which requires `Copy` instead of `Clone`.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const AA: [u8; 4] = {
///     let mut array = [0; 4];
///     slice::fill(&mut array, 13);
///     array
/// };
/// assert_eq!(AA, [13, 13, 13, 13]);
///
/// ```
///
pub const fn fill<T: Copy>(slice: &mut [T], val: T) {
    crate::for_range! {i in 0..slice.len() =>
        slice[i] = val;
    }
}

/// Const equivalent of
/// [`<[T]>::fill_with`](https://doc.rust-lang.org/std/primitive.slice.html#method.fill_with)
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// const AA: [u8; 4] = {
///     let mut array = [0; 4];
///     slice::fill_with!(&mut array, || 5);
///     array
/// };
/// assert_eq!(AA, [5, 5, 5, 5]);
///
///
/// const BB: [u8; 3] = {
///     let mut array = [0; 3];
///     slice::fill_with!(&mut array, initer);
///     array
/// };
/// assert_eq!(BB, [8, 8, 8]);
///
///
/// const fn initer() -> u8 {
///     8
/// }
/// ```
///
#[doc(inline)]
pub use crate::__slice_fill_with as fill_with;

#[doc(hidden)]
#[macro_export]
macro_rules! __slice_fill_with {
    ($slice:expr, || $v:expr $(,)?) => {
        match ($crate::slice::__AssertSliceMut { x: $slice }.x) {
            slice => {
                $crate::for_range! {i in 0..slice.len()=>
                    slice[i] = $v;
                }
            }
        }
    };
    ($opt:expr, | $($anything:tt)* ) => {
        $crate::__::compile_error!("expected a closure that takes no arguments")
    };
    ($slice:expr, $v:expr $(,)?) => {
        $crate::__slice_fill_with! {$slice, || $v()}
    };
}
