use crate::into_iter::{ConstIntoIter, IntoIterWrapper, IsIteratorKind, IsStdKind};

use core::mem::ManuallyDrop;

impl<'a, T, const N: usize> ConstIntoIter for &'a [T; N] {
    type Kind = IsStdKind;
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
}

impl<'a, T, const N: usize> ConstIntoIter for &&'a [T; N] {
    type Kind = IsStdKind;
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
}

impl<'a, T, const N: usize> IntoIterWrapper<&'a [T; N], IsStdKind> {
    pub const fn const_into_iter(self) -> Iter<'a, T> {
        Iter {
            slice: ManuallyDrop::into_inner(self.iter) as &[T],
        }
    }
}
impl<'a, T, const N: usize> IntoIterWrapper<&&'a [T; N], IsStdKind> {
    pub const fn const_into_iter(self) -> Iter<'a, T> {
        Iter {
            slice: (*ManuallyDrop::into_inner(self.iter)) as &[T],
        }
    }
}

impl<'a, T> ConstIntoIter for &'a [T] {
    type Kind = IsStdKind;
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
}

impl<'a, T> IntoIterWrapper<&'a [T], IsStdKind> {
    pub const fn const_into_iter(self) -> Iter<'a, T> {
        Iter {
            slice: ManuallyDrop::into_inner(self.iter),
        }
    }
}

impl<'a, T> ConstIntoIter for &&'a [T] {
    type Kind = IsStdKind;
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
}

impl<'a, T> IntoIterWrapper<&&'a [T], IsStdKind> {
    // clippy suggests a change that doesn't compile
    #[allow(clippy::explicit_auto_deref)]
    pub const fn const_into_iter(self) -> Iter<'a, T> {
        Iter {
            slice: *ManuallyDrop::into_inner(self.iter),
        }
    }
}

pub const fn iter<T>(slice: &[T]) -> Iter<'_, T> {
    Iter { slice }
}

macro_rules! iter_shared {
    (is_forward = $is_forward:ident) => {
        iterator_shared! {
            is_forward = $is_forward,
            item = &'a T,
            iter_forward = Iter<'a, T>,
            iter_reversed = IterRev<'a, T>,
            next(self) {
                if let [elem, rem @ ..] = self.slice {
                    self.slice = rem;
                    Some((elem, self))
                } else {
                    None
                }
            },
            next_back {
                if let [rem @ .., elem] = self.slice {
                    self.slice = rem;
                    Some((elem, self))
                } else {
                    None
                }
            },
            fields = {slice},
        }

        /// Accesses the remaining slice.
        pub const fn as_slice(&self) -> &'a [T] {
            self.slice
        }
    };
}

pub struct Iter<'a, T> {
    slice: &'a [T],
}
impl<'a, T> ConstIntoIter for Iter<'a, T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = &'a T;
}

pub struct IterRev<'a, T> {
    slice: &'a [T],
}
impl<'a, T> ConstIntoIter for IterRev<'a, T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = &'a T;
}

impl<'a, T> Iter<'a, T> {
    iter_shared! {is_forward = true}
}

impl<'a, T> IterRev<'a, T> {
    iter_shared! {is_forward = false}
}

pub use copied::{iter_copied, IterCopied, IterCopiedRev};

mod copied {
    use super::*;

    pub const fn iter_copied<T: Copy>(slice: &[T]) -> IterCopied<'_, T> {
        IterCopied { slice }
    }

    macro_rules! iter_copied_shared {
        (is_forward = $is_forward:ident) => {
            iterator_shared! {
                is_forward = $is_forward,
                item = T,
                iter_forward = IterCopied<'a, T>,
                iter_reversed = IterCopiedRev<'a, T>,
                next(self) {
                    if let [elem, rem @ ..] = self.slice {
                        self.slice = rem;
                        Some((*elem, self))
                    } else {
                        None
                    }
                },
                next_back {
                    if let [rem @ .., elem] = self.slice {
                        self.slice = rem;
                        Some((*elem, self))
                    } else {
                        None
                    }
                },
                fields = {slice},
            }

            /// Accesses the remaining slice.
            pub const fn as_slice(&self) -> &'a [T] {
                self.slice
            }
        };
    }

    pub struct IterCopied<'a, T> {
        slice: &'a [T],
    }
    impl<'a, T> ConstIntoIter for IterCopied<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = T;
    }

    pub struct IterCopiedRev<'a, T> {
        slice: &'a [T],
    }
    impl<'a, T> ConstIntoIter for IterCopiedRev<'a, T> {
        type Kind = IsIteratorKind;
        type IntoIter = Self;
        type Item = T;
    }

    impl<'a, T: Copy> IterCopied<'a, T> {
        iter_copied_shared! {is_forward = true}
    }

    impl<'a, T: Copy> IterCopiedRev<'a, T> {
        iter_copied_shared! {is_forward = false}
    }
}
