use crate::iter::{ConstIntoIter, IntoIterWrapper, IsIteratorKind, IsStdKind};

use core::mem::ManuallyDrop;



impl<'a, T> ConstIntoIter for &'a Option<T> {
    type Kind = IsStdKind;
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
}

impl<'a, T> ConstIntoIter for &&'a Option<T> {
    type Kind = IsStdKind;
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;
}

impl<'a, T> IntoIterWrapper<&'a Option<T>, IsStdKind> {
    /// Converts `&'a Option<T>` into an iterator
    pub const fn const_into_iter(self) -> Iter<'a, T> {
        Iter {
            opt: ManuallyDrop::into_inner(self.iter).as_ref(),
        }
    }
}
impl<'a, T> IntoIterWrapper<&&'a Option<T>, IsStdKind> {
    /// Converts `&&'a Option<T>` into an iterator
    pub const fn const_into_iter(self) -> Iter<'a, T> {
        Iter {
            opt: (*ManuallyDrop::into_inner(self.iter)).as_ref(),
        }
    }
}


/// Const equivalent of [`Option::iter`]
/// 
/// # Example
/// 
/// ```rust
/// use konst::option;
/// 
/// let mut fwd = option::iter(&Some(5));
/// assert_eq!(fwd.next(), Some(&5));
/// assert_eq!(fwd.next(), None);
/// 
/// let mut rev = option::iter(&Some(8)).rev();
/// assert_eq!(rev.next(), Some(&8));
/// assert_eq!(rev.next(), None);
/// ```
pub const fn iter<T>(opt: &Option<T>) -> Iter<'_, T> {
    Iter {
        opt: opt.as_ref()
    }
}

macro_rules! iter_shared {
    (is_forward = $is_forward:ident) => {
        crate::iterator_shared! {
            is_forward = $is_forward,
            item = &'a T,
            iter_forward = Iter<'a, T>,
            iter_reversed = IterRev<'a, T>,
            next(self) {
                self.opt.take()
            },
            next_back {
                self.opt.take()
            },
            fields = {opt},
        }
    };
}

/// Const equivalent of [`core::option::Iter`]
pub struct Iter<'a, T> {
    opt: Option<&'a T>,
}
impl<'a, T> ConstIntoIter for Iter<'a, T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = &'a T;
}

/// Const equivalent of `core::iter::Rev<core::option::Iter<T>>`
pub struct IterRev<'a, T> {
    opt: Option<&'a T>,
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

////////////////////////////////////////////////////////////////////////////////

impl<'a, T> ConstIntoIter for &'a mut Option<T> {
    type Kind = IsStdKind;
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;
}

impl<'a, T> ConstIntoIter for &'a mut &mut Option<T> {
    type Kind = IsStdKind;
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;
}

impl<'a, T> IntoIterWrapper<&'a mut Option<T>, IsStdKind> {
    /// Converts `&'a mut Option<T>` into an iterator
    pub const fn const_into_iter(self) -> IterMut<'a, T> {
        IterMut {
            opt: ManuallyDrop::into_inner(self.iter).as_mut(),
        }
    }
}
impl<'a, T> IntoIterWrapper<&'a mut &mut Option<T>, IsStdKind> {
    /// Converts `&'a mut &mut Option<T>` into an iterator
    pub const fn const_into_iter(self) -> IterMut<'a, T> {
        IterMut {
            opt: ManuallyDrop::into_inner(self.iter).as_mut(),
        }
    }
}


/// Const equivalent of [`Option::iter_mut`]
/// 
/// # Example
/// 
/// ```rust
/// use konst::option;
/// 
/// {
///     let mut opt = Some(13);
///     let mut fwd = option::iter_mut(&mut opt);
///     assert_eq!(fwd.next(), Some(&mut 13));
///     assert_eq!(fwd.next(), None);
/// }
/// {
///     let mut opt = Some(21);
///     let mut rev = option::iter_mut(&mut opt).rev();
///     assert_eq!(rev.next(), Some(&mut 21));
///     assert_eq!(rev.next(), None);
/// }
/// ```
pub const fn iter_mut<T>(opt: &mut Option<T>) -> IterMut<'_, T> {
    IterMut {
        opt: opt.as_mut()
    }
}

macro_rules! iter_mut_shared {
    (is_forward = $is_forward:ident) => {
        iterator_shared! {
            is_forward = $is_forward,
            is_copy = false,
            item = &'a mut T,
            iter_forward = IterMut<'a, T>,
            iter_reversed = IterMutRev<'a, T>,
            next(self) {
                self.opt.take()
            },
            next_back {
                self.opt.take()
            },
            fields = {opt},
        }
    };
}

/// Const equivalent of [`core::option::IterMut`]
pub struct IterMut<'a, T> {
    opt: Option<&'a mut T>,
}
impl<'a, T> ConstIntoIter for IterMut<'a, T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = &'a mut T;
}

/// Const equivalent of `core::iter::Rev<core::option::IterMut<T>>`
pub struct IterMutRev<'a, T> {
    opt: Option<&'a mut T>,
}
impl<'a, T> ConstIntoIter for IterMutRev<'a, T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = &'a mut T;
}

impl<'a, T> IterMut<'a, T> {
    iter_mut_shared! {is_forward = true}
}

impl<'a, T> IterMutRev<'a, T> {
    iter_mut_shared! {is_forward = false}
}

////////////////////////////////////////////////////////////////////////////////

impl<T> ConstIntoIter for Option<T> {
    type Kind = IsStdKind;
    type IntoIter = IntoIter<T>;
    type Item = T;
}

impl<T> IntoIterWrapper<Option<T>, IsStdKind> {
    /// Converts `Option<T>` into an iterator
    pub const fn const_into_iter(self) -> IntoIter<T> {
        IntoIter {
            opt: ManuallyDrop::into_inner(self.iter),
        }
    }
}

/// Const equivalent of [`Option::into_iter`]
/// 
/// # Example
/// 
/// ```rust
/// use konst::option;
/// 
/// {
///     let mut opt = Some(13);
///     let mut fwd = option::into_iter(opt);
///     assert_eq!(fwd.next(), Some(13));
///     assert_eq!(fwd.next(), None);
/// }
/// {
///     let mut opt = Some(21);
///     let mut rev = option::into_iter(opt).rev();
///     assert_eq!(rev.next(), Some(21));
///     assert_eq!(rev.next(), None);
/// }
/// ```
pub const fn into_iter<T>(opt: Option<T>) -> IntoIter<T> {
    IntoIter { opt }
}

macro_rules! into_iter_shared {
    (is_forward = $is_forward:ident) => {
        iterator_shared! {
            is_forward = $is_forward,
            is_copy = false,
            is_drop = true,
            item = T,
            iter_forward = IntoIter<T>,
            iter_reversed = IntoIterRev<T>,
            next(self) {
                self.opt.take()
            },
            next_back {
                self.opt.take()
            },
            fields = {opt},
        }
    };
}

/// Const equivalent of [`core::option::IntoIter`]
pub struct IntoIter<T> {
    opt: Option<T>,
}
impl<T> ConstIntoIter for IntoIter<T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = T;
}

/// Const equivalent of `core::iter::Rev<core::option::IntoIter<T>>`
pub struct IntoIterRev<T> {
    opt: Option<T>,
}
impl<T> ConstIntoIter for IntoIterRev<T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = T;
}

impl<T> IntoIter<T> {
    into_iter_shared! {is_forward = true}
}

impl<T> IntoIterRev<T> {
    into_iter_shared! {is_forward = false}
}

