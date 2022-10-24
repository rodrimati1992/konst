use crate::into_iter::{IntoIterKind, IsIteratorKind};

pub const fn repeat<T: Copy>(val: T) -> Repeat<T> {
    Repeat(val)
}

pub struct Repeat<T>(T);

impl<T> IntoIterKind for Repeat<T> {
    type Kind = IsIteratorKind;
}

impl<T: Copy> Repeat<T> {
    /// Gets the next element in the iterator
    pub const fn next(self) -> Option<(T, Self)> {
        Some((self.0, self))
    }
    /// Gets the next element in the iterator
    pub const fn next_back(self) -> Option<(T, Self)> {
        Some((self.0, self))
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
