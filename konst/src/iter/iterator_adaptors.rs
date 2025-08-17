use crate::iter::{ConstIntoIter, IsIteratorKind};

/// Const analog of [`core::iter::repeat`],
/// except that this requires the repeated value to impl `Copy`
/// (instead of `Clone`).
///
/// # Example
///
/// ```rust
/// use konst::iter::{self, collect_const};
///
/// const ARR: &[u8] = &collect_const!(u8 => iter::repeat(3),take(5));
///
/// assert_eq!(ARR, &[3, 3, 3, 3, 3]);
/// ```
pub const fn repeat<T: Copy>(val: T) -> Repeat<T> {
    Repeat(val)
}

/// Const analog of [`core::iter::Repeat`],
/// constructed by [`repeat`](crate::iter::repeat).
pub struct Repeat<T>(T);

impl<T> ConstIntoIter for Repeat<T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = T;

    // since the constructor requires T: Copy, it doesn't need dropping
    const ITEMS_NEED_DROP: bool = false;
}

impl<T: Copy> Repeat<T> {
    /// Gets the next element in the iterator
    pub const fn next(&mut self) -> Option<T> {
        Some(self.0)
    }
    /// Gets the next element in the iterator
    pub const fn next_back(&mut self) -> Option<T> {
        Some(self.0)
    }
    /// Reverses the iterator
    pub const fn rev(self) -> Self {
        self
    }
    /// Clones the iterator
    pub const fn copy(&self) -> Self {
        Self(self.0)
    }
}

/////////////////////////////////////////////////////

/// Const analog of [`core::iter::repeat_n`],
/// except that this requires the repeated value to impl `Copy`
/// (instead of `Clone`).
///
/// # Example
///
/// ```rust
/// use konst::iter::{self, collect_const};
///
/// const ARR: &[u8] = &collect_const!(u8 => iter::repeat_n(8, 3));
///
/// assert_eq!(ARR, &[8, 8, 8]);
/// ```
pub const fn repeat_n<T: Copy>(val: T, count: usize) -> RepeatN<T> {
    RepeatN { val, count }
}

/// Const analog of [`core::iter::RepeatN`],
/// constructed by [`repeat_n`](crate::iter::repeat_n).
pub struct RepeatN<T> {
    val: T,
    count: usize,
}

impl<T> ConstIntoIter for RepeatN<T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = T;

    // since the constructor requires T: Copy, it doesn't need dropping
    const ITEMS_NEED_DROP: bool = false;
}

impl<T: Copy> RepeatN<T> {
    /// Gets the next element in the iterator
    pub const fn next(&mut self) -> Option<T> {
        if let Some(ncount) = self.count.checked_sub(1) {
            self.count = ncount;
            Some(self.val)
        } else {
            None
        }
    }
    /// Gets the next element in the iterator
    pub const fn next_back(&mut self) -> Option<T> {
        self.next()
    }
    /// Reverses the iterator
    pub const fn rev(self) -> Self {
        self
    }
    /// Clones the iterator
    pub const fn copy(&self) -> Self {
        Self { ..*self }
    }
}
