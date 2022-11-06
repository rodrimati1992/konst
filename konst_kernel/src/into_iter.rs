use core::{marker::PhantomData, mem::ManuallyDrop};

#[doc(no_inline)]
pub use crate::polymorphism::kinds::{IsIntoIterKind, IsIteratorKind, IsStdKind};

pub mod range_into_iter;
pub mod slice_into_iter;

pub trait ConstIntoIter {
    /// What kind of type this is:
    /// - [`IsIntoIterKind`]: user-defined types that are convertible to const iterators
    /// - [`IsIteratorKind`]: const iterators
    /// - [`IsStdKind`]: standard library types that are convertible to const iterators
    type Kind;

    /// The item that `Self::IntoIter` yields on `.next()`
    type Item;

    /// The iterator that this can be converted into.
    type IntoIter: ConstIntoIter<Item = Self::Item>;
}

#[repr(transparent)]
pub struct IntoIterWrapper<I, K> {
    pub iter: ManuallyDrop<I>,
    pub marker: IsConstIntoIter<I, K>,
}

mod is_into_iter_kind {
    use super::*;

    pub struct IsConstIntoIter<T, K>(PhantomData<(fn() -> PhantomData<T>, fn() -> K)>);

    impl<T> IsConstIntoIter<T, T::Kind>
    where
        T: ConstIntoIter,
    {
        pub const NEW: Self = Self(PhantomData);
    }
}
pub use is_into_iter_kind::IsConstIntoIter;

impl<T> IntoIterWrapper<T, IsStdKind> {
    #[inline(always)]
    pub const fn coerce(self) -> Self {
        self
    }
}

impl<T> IntoIterWrapper<T, IsIntoIterKind> {
    #[inline(always)]
    pub const fn coerce(self) -> T {
        ManuallyDrop::into_inner(self.iter)
    }
}

impl<T> IntoIterWrapper<T, IsIteratorKind> {
    #[inline(always)]
    pub const fn coerce(self) -> Self {
        self
    }

    #[inline(always)]
    pub const fn const_into_iter(self) -> T
    where
        T: ConstIntoIter<IntoIter = T>,
    {
        ManuallyDrop::into_inner(self.iter)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! into_iter_macro {
    ($iter:expr) => {
        $crate::__::IntoIterWrapper {
            iter: $crate::__::ManuallyDrop::new($iter),
            marker: $crate::__::IsConstIntoIter::NEW,
        }
        .coerce()
        .const_into_iter()
    };
}
