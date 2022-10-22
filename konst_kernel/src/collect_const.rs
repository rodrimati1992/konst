use crate::type_eq::{MakeTypeWitness, TypeEq, TypeWitnessTypeArg};

pub enum CollectorCmd<Ret, T, const CAP: usize> {
    ComputeLength(TypeEq<usize, Ret>),
    BuildArray(TypeEq<[T; CAP], Ret>),
}

impl<Ret, T, const CAP: usize> TypeWitnessTypeArg for CollectorCmd<Ret, T, CAP> {
    type Arg = Ret;
}

impl<T> MakeTypeWitness for CollectorCmd<usize, T, 0> {
    const MAKE: Self = Self::ComputeLength(TypeEq::NEW);
}

impl<T, const CAP: usize> MakeTypeWitness for CollectorCmd<[T; CAP], T, CAP> {
    const MAKE: Self = Self::BuildArray(TypeEq::NEW);
}

#[macro_export]
macro_rules! iter_collect_const {
    ($Item:ty => $($rem:tt)*) => {{
        const fn __func_zxe7hgbnjs<Ret_KO9Y329U2U, const CAP_KO9Y329U2U: usize>(
            cmd: $crate::__::CollectorCmd<Ret_KO9Y329U2U, $Item, CAP_KO9Y329U2U>
        ) -> Ret_KO9Y329U2U {
            let mut array = $crate::maybe_uninit::uninit_array::<_, CAP_KO9Y329U2U>();
            let mut length = 0usize;

            $crate::__process_iter_args!{
                ($crate::__iter_collect_const)
                ($Item, (cmd array length),)
                (
                    item,
                    'zxe7hgbnjs,
                    adapter,
                )
                $($rem)*
            }

            match cmd {
                $crate::__::CollectorCmd::ComputeLength(teq) => {
                    teq.to_right(length)
                }
                $crate::__::CollectorCmd::BuildArray(teq) => {
                    $crate::__::assert!{
                        length == CAP_KO9Y329U2U,
                        "initialization was skipped somehow",
                    }

                    // SAFETY: The above assert ensures that
                    // all of the array is initialized
                    let array = unsafe{ $crate::maybe_uninit::array_assume_init(array) };
                    teq.to_right(array)
                }
            }
        }

        const __COUNT81608BFNA5: $crate::__::usize =
            __func_zxe7hgbnjs($crate::__::MakeTypeWitness::MAKE);

        const __ARR81608BFNA5: [$Item; __COUNT81608BFNA5] =
            __func_zxe7hgbnjs($crate::__::MakeTypeWitness::MAKE);

        __ARR81608BFNA5
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __iter_collect_const {
    (
        @each
        $Item:ty,
        ($cmd:ident $array:ident $length:ident),
        ($item:ident adapter),
        $(,)*
    ) => {
        if let $crate::__::CollectorCmd::BuildArray(teq) = $cmd {
            teq.reachability_hint(());

            $array[$length] = $crate::__::MaybeUninit::new($item);
        }

        $length += 1;
    };
    (@end $($tt:tt)*) => {};
}
