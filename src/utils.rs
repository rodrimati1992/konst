#[doc(hidden)]
#[cfg(feature = "constant_time_slice")]
pub(crate) union Dereference<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}

#[inline]
pub(crate) const fn saturating_sub(l: usize, r: usize) -> usize {
    let (sub, overflowed) = l.overflowing_sub(r);
    if overflowed {
        0
    } else {
        sub
    }
}

#[inline]
#[cfg(feature = "constant_time_slice")]
pub(crate) const fn min_usize(l: usize, r: usize) -> usize {
    if l < r {
        l
    } else {
        r
    }
}
