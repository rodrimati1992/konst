#[allow(dead_code)]
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
#[allow(dead_code)]
pub(crate) const fn min_usize(l: usize, r: usize) -> usize {
    if l < r {
        l
    } else {
        r
    }
}
