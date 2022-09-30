use core::mem::{ManuallyDrop, MaybeUninit};

use crate::type_eq::TypeEq;

pub enum CollectorCmd<T, Ret, const CAP: usize> {
    ComputeLength(TypeEq<ComputedLength, Ret>),
    BuildArray(TypeEq<ArrayVec<T, CAP>, Ret>),
}

impl<T> CollectorCmd<T, ComputedLength, 0> {
    pub const COMPUTE_LENGTH: Self = Self::ComputeLength(TypeEq::NEW);
}

impl<T, const CAP: usize> CollectorCmd<T, ArrayVec<T, CAP>, CAP> {
    pub const BUILD_ARRAY: Self = Self::BuildArray(TypeEq::NEW);
}

pub struct ComputedLength {
    pub length: usize,
}

#[repr(C)]
pub struct ArrayVec<T, const CAP: usize> {
    pub length: usize,
    pub array: ManuallyDrop<[MaybeUninit<T>; CAP]>,
}

impl<T, const CAP: usize> ArrayVec<T, CAP> {
    /// # Panic
    ///
    /// Panics if `LEN > CAP`
    ///
    /// # Safety
    ///
    /// The `T` elements in `self.array[..LEN]` must be initialized
    #[inline]
    pub const unsafe fn assume_init_array<const LEN: usize>(self) -> [T; LEN] {
        if LEN > CAP {
            [(); CAP][LEN];
        }

        ManuallyDrop::into_inner(ArrayToPartialInit { full: self.array }.truncated)
    }
}

#[repr(C)]
union ArrayToPartialInit<T, const CAP: usize, const LEN: usize> {
    pub full: ManuallyDrop<[MaybeUninit<T>; CAP]>,
    pub truncated: ManuallyDrop<[T; LEN]>,
}

#[macro_export]
macro_rules! iter_collect_const {
    ($Item:ty = $iter:expr) => {{
        const fn __func_zxe7hgbnjs<Ret_KO9Y329U2U, const CAP_KO9Y329U2U: usize>(
            cmd: $crate::__::CollectorCmd<$Item, Ret_KO9Y329U2U, CAP_KO9Y329U2U>,
        ) -> Ret_KO9Y329U2U {
            match $crate::into_iter_macro!($iter) {
                mut iter => match cmd {
                    $crate::__::CollectorCmd::ComputeLength(teq) => {
                        let mut length = 0;
                        while let $crate::__::Some((_, next)) = iter.next() {
                            iter = next;
                            length += 1;
                        }
                        teq.to_right($crate::__::ComputedLength { length })
                    }
                    $crate::__::CollectorCmd::BuildArray(teq) => {
                        let mut length = 0usize;
                        let mut array = $crate::utils_1_56::uninit_array::<$Item, CAP_KO9Y329U2U>();
                        while let $crate::__::Some((elem, next)) = iter.next() {
                            array[length] = $crate::__::MaybeUninit::new(elem);
                            length += 1;
                            iter = next;
                        }
                        teq.to_right($crate::__::ArrayVec {
                            length,
                            array: $crate::__::ManuallyDrop::new(array),
                        })
                    }
                },
            }
        }

        const __COUNT81608BFNA5: $crate::__::usize =
            __func_zxe7hgbnjs($crate::__::CollectorCmd::COMPUTE_LENGTH).length;

        const __VEC81608BFNA5: $crate::__::ArrayVec<$Item, __COUNT81608BFNA5> =
            __func_zxe7hgbnjs($crate::__::CollectorCmd::BUILD_ARRAY);

        const __ARR81608BFNA5: [$Item; __VEC81608BFNA5.length] =
            // SAFETY: `__func_zxe7hgbnjs` ensures that the first `length` elements
            // of the ArrayVec are initialized.
            unsafe { __VEC81608BFNA5.assume_init_array() };

        __ARR81608BFNA5
    }};
}
