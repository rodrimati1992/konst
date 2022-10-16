#[cfg(test)]
mod utils_tests;

pub(crate) enum PanikVal<'a> {
    Str(&'a str),
    Usize(usize),
}

// an approximate value is acceptable.
// because unwritten '\0' bytes are removed when printing
const fn approx_log10(n: usize) -> u32 {
    (usize::BITS - n.leading_zeros()) / 3 + 1
}

const CAP: usize = 256;
pub(crate) const fn fmt_conc(slice: &[PanikVal<'_>]) -> [u8; CAP] {
    let mut out = [0u8; CAP];
    let mut i = 0usize;

    macro_rules! write_byte {
        ($byte:expr) => {
            out[i] = $byte;
            i += 1;
        };
    }

    crate::for_range! {slice_i in 0..slice.len() =>
        match slice[slice_i] {
            PanikVal::Str(str) => {
                let mut j = 0;
                let str = str.as_bytes();
                while j < str.len() {
                    write_byte!(str[j]);
                    j += 1;
                }
            }
            PanikVal::Usize(mut number) => {
                if number == 0 {
                    write_byte!(b'0');
                } else {
                    let digit_len = approx_log10(number);
                    i += digit_len as usize;
                    let mut j = i;
                    while number != 0 {
                        j -= 1;
                        out[j] = (number % 10) as u8 + b'0';
                        number /= 10;
                    }
                }
            }
        }
    }

    out
}

#[cold]
#[track_caller]
pub(crate) const fn basic_panic(slice: &[PanikVal<'_>]) {
    let arr = fmt_conc(slice);
    match core::str::from_utf8(&arr) {
        Ok(x) => panic!("{}", x),
        Err(_) => panic!("BUG: formatted string isn't UTF-8"),
    }
}
