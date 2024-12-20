use crate::into_iter::{ConstIntoIter, IntoIterWrapper, IsIteratorKind, IsStdKind};

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
    pub const fn const_into_iter(self) -> Iter<'a, T> {
        Iter {
            opt: ManuallyDrop::into_inner(self.iter).as_ref(),
        }
    }
}
impl<'a, T> IntoIterWrapper<&&'a Option<T>, IsStdKind> {
    pub const fn const_into_iter(self) -> Iter<'a, T> {
        Iter {
            opt: (*ManuallyDrop::into_inner(self.iter)).as_ref(),
        }
    }
}


pub const fn iter<T>(opt: &Option<T>) -> Iter<'_, T> {
    Iter {
        opt: opt.as_ref()
    }
}

macro_rules! iter_shared {
    (is_forward = $is_forward:ident) => {
        iterator_shared! {
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

pub struct Iter<'a, T> {
    opt: Option<&'a T>,
}
impl<'a, T> ConstIntoIter for Iter<'a, T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = &'a T;
}

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
    pub const fn const_into_iter(self) -> IterMut<'a, T> {
        IterMut {
            opt: ManuallyDrop::into_inner(self.iter).as_mut(),
        }
    }
}
impl<'a, T> IntoIterWrapper<&'a mut &mut Option<T>, IsStdKind> {
    pub const fn const_into_iter(self) -> IterMut<'a, T> {
        IterMut {
            opt: ManuallyDrop::into_inner(self.iter).as_mut(),
        }
    }
}


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

pub struct IterMut<'a, T> {
    opt: Option<&'a mut T>,
}
impl<'a, T> ConstIntoIter for IterMut<'a, T> {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = &'a mut T;
}

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

