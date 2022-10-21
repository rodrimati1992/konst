#[macro_export]
macro_rules! string_concat {
    ($slice:expr) => {{
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

pub struct ArrayStr<const N: usize>([u8; N]);

impl<const N: usize> ArrayStr<N> {
    pub const fn as_str(&self) -> &str {
        match core::str::from_utf8(&self.0) {
            Ok(s) => s,
            Err(_) => panic!("bug: konst made an invalid string"),
        }
    }
}
