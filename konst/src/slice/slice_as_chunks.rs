use crate::slice;

/// Const equivalent of
/// [`<[T]>::as_chunks`](https://doc.rust-lang.org/std/primitive.slice.html#method.as_chunks)
///
/// # Panics
///
/// Panics if `N == 0`.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// let (arrs, rem) = slice::as_chunks::<_, 3>(&[2u32, 3, 5, 8, 13, 21, 34, 55]);
///
/// assert_eq!(arrs, &[[2, 3, 5], [8, 13, 21]][..]);
/// assert_eq!(rem, &[34, 55][..])
///
/// ```
#[track_caller]
pub const fn as_chunks<'a, T, const N: usize>(this: &[T]) -> (&[[T; N]], &[T]) {
    assert!(N != 0, "chunk size must be non-zero");

    let arrs_len = this.len() / N;
    let (arrs_in, rem) = slice::split_at(this, arrs_len * N);

    // SAFETY: `arrs_in` is a `&[T]` that is `arrs_len * N` long,
    // its layout is compatible with the `&[[T; N]]` that this produces.
    let arrs: &[[T; N]] =
        unsafe { core::slice::from_raw_parts(arrs_in.as_ptr() as *const [T; N], arrs_len) };

    (arrs, rem)
}

/// Const equivalent of
/// [`<[T]>::as_rchunks`](https://doc.rust-lang.org/std/primitive.slice.html#method.as_rchunks)
///
/// # Panics
///
/// Panics if `N == 0`.
///
/// # Example
///
/// ```rust
/// use konst::slice;
///
/// let (rem, arrs) = slice::as_rchunks::<_, 2>(&[2u32, 3, 5, 8, 13, 21, 34]);
///
/// assert_eq!(rem, &[2][..]);
/// assert_eq!(arrs, &[[3, 5], [8, 13], [21, 34]][..]);
/// ```
#[track_caller]
pub const fn as_rchunks<'a, T, const N: usize>(this: &[T]) -> (&[T], &[[T; N]]) {
    assert!(N != 0, "chunk size must be non-zero");

    let arrs_len = this.len() / N;
    let rem_len = this.len() % N;
    let (rem, arrs_in) = slice::split_at(this, rem_len);

    // SAFETY: `arrs_in` is a `&[T]` that is `arrs_len * N` long,
    // its layout is compatible with the `&[[T; N]]` that this produces.
    let arrs: &[[T; N]] =
        unsafe { core::slice::from_raw_parts(arrs_in.as_ptr() as *const [T; N], arrs_len) };

    (rem, arrs)
}
