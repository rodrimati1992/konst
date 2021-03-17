use crate::polymorphism::{ConstCmpMarker, IsAConstCmpMarker, IsNotStdKind, IsStdKind};

pub struct CmpWrapper<T>(pub T);

impl<'a, T> CmpWrapper<&'a [T]> {
    /// For constructing from a reference to an array.
    ///
    /// With slices you can do `CmpWrapper(slice)` as well.
    #[inline(always)]
    pub const fn slice(x: &'a [T]) -> Self {
        Self { 0: x }
    }
}

impl<P> ConstCmpMarker for CmpWrapper<P> {
    type Kind = IsNotStdKind;
    type This = Self;
}

macro_rules! std_kind_impls {
    ($($ty:ty),* $(,)* ) => (
        $(
            impl ConstCmpMarker for $ty {
                type Kind = IsStdKind;
                type This = Self;
            }

            impl<T> IsAConstCmpMarker<IsStdKind, $ty, T> {
                /// Copies the value from `reference`, and wraps it in a `CmpWrapper`
                #[inline(always)]
                pub const fn coerce(self, reference: &$ty) -> CmpWrapper<$ty> {
                    CmpWrapper(*reference)
                }
            }

            impl CmpWrapper<$ty> {
                #[inline(always)]
                pub const fn const_eq(self, other: &$ty) -> bool {
                    self.0 == *other
                }
            }
        )*
    )
}

std_kind_impls! {
    i8, u8,
    i16, u16,
    i32, u32,
    i64, u64,
    i128, u128,
    isize, usize,
    bool, char,
}
