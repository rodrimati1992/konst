#![allow(clippy::len_without_is_empty)]

use crate::chr::{encode_utf8, Utf8Encoded};

#[doc(hidden)]
pub struct __NormalizeConcatArg<T: 'static>(pub &'static [T]);

impl __NormalizeConcatArg<char> {
    pub const fn conv(self) -> __StrConcatArg {
        __StrConcatArg::Char(self.0)
    }
}

impl __NormalizeConcatArg<&'static str> {
    pub const fn conv(self) -> __StrConcatArg {
        __StrConcatArg::Str(self.0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum __StrConcatArg {
    Char(&'static [char]),
    Str(&'static [&'static str]),
}

#[doc(hidden)]
pub struct __MakeSepArg<T>(pub T);

#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum __SepArg {
    Char(char),
    Str(&'static str),
}

impl __SepArg {
    const fn len(self) -> usize {
        match self {
            Self::Char(x) => x.len_utf8(),
            Self::Str(x) => x.len(),
        }
    }
}

#[doc(hidden)]
pub struct __ElemDispatch<T>(pub T);

macro_rules! __ref_unref_impls {
    ($($ref_token:tt $deref_token:tt)?) => {
        impl __MakeSepArg<$($ref_token)? char> {
            pub const fn conv(self) -> __SepArg {
                __SepArg::Char($($deref_token)? self.0)
            }
        }

        impl __MakeSepArg<$($ref_token)? &'static str> {
            pub const fn conv(self) -> __SepArg {
                __SepArg::Str(self.0)
            }
        }

        impl __ElemDispatch<$($ref_token)? char> {
            pub const fn as_bytesable(self) -> Utf8Encoded {
                encode_utf8($($deref_token)? self.0)
            }
            pub const fn len(self) -> usize {
                self.0.len_utf8()
            }
        }

        impl __ElemDispatch<$($ref_token)? &'static str> {
            pub const fn as_bytesable(self) -> &'static str {
                self.0
            }
            pub const fn len(self) -> usize {
                self.0.len()
            }
        }
    };
}

__ref_unref_impls! {}
__ref_unref_impls! {& *}

#[doc(hidden)]
#[macro_export]
macro_rules! __with_str_concat_slices {
    ($arg:expr, |$slices:ident| $with_slices:expr) => {
        match $arg {
            $crate::string::__StrConcatArg::Char($slices) => $with_slices,
            $crate::string::__StrConcatArg::Str($slices) => $with_slices,
        }
    };
}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! string_concat {
    ($(&)? []) => {
        ""
    };
    ($slice:expr $(,)*) => {{
        const __ARGS_81608BFNA5: $crate::string::__StrConcatArg =
            $crate::string::__NormalizeConcatArg($slice).conv();
        {
            const LEN: $crate::__::usize = $crate::string::concat_sum_lengths(__ARGS_81608BFNA5);

            const CONC: &$crate::string::ArrayStr<LEN> =
                &$crate::string::concat_strs(__ARGS_81608BFNA5);

            const STR: &$crate::__::str = CONC.as_str();

            STR
        }
    }};
}

pub const fn concat_sum_lengths(arg: __StrConcatArg) -> usize {
    let mut sum = 0usize;

    __with_str_concat_slices! {arg, |slices| {
        crate::for_range! {i in 0..slices.len() =>
            sum += __ElemDispatch(slices[i]).len();
        }
    }}

    sum
}

pub const fn concat_strs<const N: usize>(arg: __StrConcatArg) -> ArrayStr<N> {
    let mut out = [0u8; N];
    let mut out_i = 0usize;

    __with_str_concat_slices! {arg, |slices| {
        crate::for_range! {si in 0..slices.len() =>
            let byteser = __ElemDispatch(slices[si]).as_bytesable();
            let slice = byteser.as_bytes();
            crate::for_range! {i in 0..slice.len() =>
                out[out_i] = slice[i];
                out_i += 1;
            }
        }
    }}

    ArrayStr(out)
}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! string_join {
    ($sep:expr, $(&)? []) => {
        ""
    };
    ($sep:expr, $slice:expr $(,)*) => {{
        const __ARGS_81608BFNA5: $crate::string::StrJoinArgs = $crate::string::StrJoinArgs {
            sep: $crate::string::__MakeSepArg($sep).conv(),
            slice: $slice,
        };

        {
            const LEN: $crate::__::usize = $crate::string::join_sum_lengths(__ARGS_81608BFNA5);

            const CONC: &$crate::string::ArrayStr<LEN> =
                &$crate::string::join_strs(__ARGS_81608BFNA5);

            const STR: &$crate::__::str = CONC.as_str();

            STR
        }
    }};
}

#[derive(Copy, Clone)]
pub struct StrJoinArgs {
    pub sep: __SepArg,
    pub slice: &'static [&'static str],
}

pub const fn join_sum_lengths(StrJoinArgs { sep, slice }: StrJoinArgs) -> usize {
    if slice.is_empty() {
        0
    } else {
        concat_sum_lengths(__StrConcatArg::Str(slice)) + sep.len() * (slice.len() - 1)
    }
}

pub const fn join_strs<const N: usize>(
    StrJoinArgs { sep, slice: slices }: StrJoinArgs,
) -> ArrayStr<N> {
    let mut out = [0u8; N];
    let mut out_i = 0usize;

    let utf8e: Utf8Encoded;
    let sep = match sep {
        __SepArg::Char(c) => {
            utf8e = encode_utf8(c);
            utf8e.as_str()
        }
        __SepArg::Str(s) => s,
    };

    macro_rules! write_str {
        ($str:expr) => {{
            let slice = $str.as_bytes();
            crate::for_range! {i in 0..slice.len() =>
                out[out_i] = slice[i];
                out_i += 1;
            }
        }};
    }

    if let [first, rem_slices @ ..] = slices {
        write_str! {first}

        crate::for_range! {si in 0..rem_slices.len() =>
            write_str!{sep}
            write_str!{rem_slices[si]}
        }
    }

    ArrayStr(out)
}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! str_from_iter {
    ($($rem:tt)*) => {{
        $crate::__collect_const_iter_with!{
            $crate::__::u8,
            {},
            |array, written_length, item| {
                let byteser = $crate::string::__ElemDispatch(item).as_bytesable();
                let bytes = byteser.as_bytes();
                let item_len = bytes.len();
                let mut i = written_length;
                let mut j = 0;
                while j < item_len {
                    array[i] = $crate::__::MaybeUninit::new(bytes[j]);
                    i += 1;
                    j += 1;
                }
            },
            elem_length = {
                $crate::string::__ElemDispatch(item).len()
            },
            =>
            $($rem)*
        }

        const __STR81608BFNA5: &$crate::__::str =
            match core::str::from_utf8(&__ARR81608BFNA5) {
                $crate::__::Ok(x) => x,
                $crate::__::Err(_) => $crate::__::panic!("created string isn't UTF8"),
            };

        __STR81608BFNA5
    }}
}

////////////////////////////////////////////////////////////////////////////////

pub struct ArrayStr<const N: usize>([u8; N]);

impl<const N: usize> ArrayStr<N> {
    pub const fn as_str(&self) -> &str {
        match core::str::from_utf8(&self.0) {
            Ok(s) => s,
            Err(_) => panic!("bug: konst made an invalid string"),
        }
    }
}
