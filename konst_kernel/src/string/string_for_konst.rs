#[macro_export]
macro_rules! string_concat {
    ($slice:expr $(,)*) => {{
        const __ARGS_81608BFNA5: &[&$crate::__::str] = $slice;
        {
            const LEN: $crate::__::usize = $crate::string::concat_sum_lengths(__ARGS_81608BFNA5);

            const CONC: &$crate::string::ArrayStr<LEN> =
                &$crate::string::concat_strs(__ARGS_81608BFNA5);

            const STR: &$crate::__::str = CONC.as_str();

            STR
        }
    }};
}

pub const fn concat_sum_lengths(slice: &[&str]) -> usize {
    let mut sum = 0usize;
    crate::for_range! {i in 0..slice.len() =>
        sum += slice[i].len();
    }
    sum
}

pub const fn concat_strs<const N: usize>(slices: &[&str]) -> ArrayStr<N> {
    let mut out = [0u8; N];
    let mut out_i = 0usize;

    crate::for_range! {si in 0..slices.len() =>
        let slice = slices[si].as_bytes();
        crate::for_range! {i in 0..slice.len() =>
            out[out_i] = slice[i];
            out_i += 1;
        }
    }

    ArrayStr(out)
}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! string_join {
    ($sep:expr, $slice:expr $(,)*) => {{
        const __ARGS_81608BFNA5: $crate::string::StrJoinArgs = $crate::string::StrJoinArgs {
            sep: $sep,
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
    pub sep: &'static str,
    pub slice: &'static [&'static str],
}

pub const fn join_sum_lengths(StrJoinArgs { sep, slice }: StrJoinArgs) -> usize {
    if slice.is_empty() {
        0
    } else {
        concat_sum_lengths(slice) + sep.len() * (slice.len() - 1)
    }
}

pub const fn join_strs<const N: usize>(
    StrJoinArgs { sep, slice: slices }: StrJoinArgs,
) -> ArrayStr<N> {
    let mut out = [0u8; N];
    let mut out_i = 0usize;

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
            |array, written_length, item| {
                let item: &$crate::__::str = item;
                let bytes = item.as_bytes();
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
                let item: &$crate::__::str = item;
                item.len()
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
