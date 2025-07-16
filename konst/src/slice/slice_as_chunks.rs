use crate::slice;

/// Const equivalent of
/// [`<[T]>::as_chunks`](https://doc.rust-lang.org/std/primitive.slice.html#method.as_chunks)
///
/// The equivalent std function is unstable as of Rust 1.83.0.
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
/// The equivalent std function is unstable as of Rust 1.83.0.
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

/// Const equivalent of
/// [`<[T]>::as_chunks_mut`
/// ](https://doc.rust-lang.org/std/primitive.slice.html#method.as_chunks_mut)
///
/// The equivalent std function is unstable as of Rust 1.83.0.
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
/// let slice = &mut [2u32, 3, 5, 8, 13, 21, 34, 55];
/// let (arrs, rem) = slice::as_chunks_mut::<_, 3>(slice);
///
/// assert_eq!(arrs, &mut [[2, 3, 5], [8, 13, 21]][..]);
/// assert_eq!(rem, &mut [34, 55][..])
///
/// ```
#[track_caller]
pub const fn as_chunks_mut<'a, T, const N: usize>(this: &mut [T]) -> (&mut [[T; N]], &mut [T]) {
    assert!(N != 0, "chunk size must be non-zero");

    let arrs_len = this.len() / N;
    let (arrs_in, rem) = slice::split_at_mut(this, arrs_len * N);

    // SAFETY: `arrs_in` is a `&mut [T]` that is `arrs_len * N` long,
    // its layout is compatible with the `&mut [[T; N]]` that this produces.
    let arrs: &mut [[T; N]] =
        unsafe { core::slice::from_raw_parts_mut(arrs_in.as_mut_ptr() as *mut [T; N], arrs_len) };

    (arrs, rem)
}

/// Const equivalent of
/// [`<[T]>::as_rchunks_mut`
/// ](https://doc.rust-lang.org/std/primitive.slice.html#method.as_rchunks_mut)
///
/// The equivalent std function is unstable as of Rust 1.83.0.
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
/// let mut slice = &mut [2u32, 3, 5, 8, 13, 21, 34];
/// let (rem, arrs) = slice::as_rchunks_mut::<_, 2>(slice);
///
/// assert_eq!(rem, &mut [2][..]);
/// assert_eq!(arrs, &mut [[3, 5], [8, 13], [21, 34]][..]);
/// ```
#[track_caller]
pub const fn as_rchunks_mut<'a, T, const N: usize>(this: &mut [T]) -> (&mut [T], &mut [[T; N]]) {
    assert!(N != 0, "chunk size must be non-zero");

    let arrs_len = this.len() / N;
    let rem_len = this.len() % N;
    let (rem, arrs_in) = slice::split_at_mut(this, rem_len);

    // SAFETY: `arrs_in` is a `&mut [T]` that is `arrs_len * N` long,
    // its layout is compatible with the `&mut [[T; N]]` that this produces.
    let arrs: &mut [[T; N]] =
        unsafe { core::slice::from_raw_parts_mut(arrs_in.as_mut_ptr() as *mut [T; N], arrs_len) };

    (rem, arrs)
}
