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
pub(crate) const fn basic_panic(slice: &[PanikVal<'_>]) -> ! {
    let arr = fmt_conc(slice);
    match core::str::from_utf8(&arr) {
        Ok(x) => panic!("{}", x),
        Err(_) => panic!("BUG: formatted string isn't UTF-8"),
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct TypeAnnot<T> {
    pub val: T,
}

////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[repr(C)]
pub union Dereference<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_2 {
    (
        $macro:tt $args:tt $usage_site:tt,
        ||  $(,)?
    ) => {
        $crate::__parse_closure_no_expr_error!{$usage_site}
    };
    (
        $macro:tt $args:tt $usage_site:tt,
        |$pat1:tt $(: $pat1_ty:ty)?, $pat2:tt $(: $pat2_ty:ty)? $(,)?| $($rem:tt)*
    ) => (
        $crate::__parse_closure_expr! {
            $usage_site $macro $args
            (
                ($crate::__unparen_pat!($pat1), $crate::__unparen_pat!($pat2)):
                ($crate::__ty_or_und!($($pat1_ty)?), $crate::__ty_or_und!($($pat2_ty)?))
            ),
            $($rem)*
        }
    );
    (
        $macro:tt $args:tt $usage_site:tt,
        |$pat1:pat_param, $pat2:pat_param $(,)?| $($rem:tt)*
    ) => (
        $crate::__parse_closure_expr! {
            $usage_site $macro $args (($pat1, $pat2)),
            $($rem)*
        }
    );
    ($macro:tt $args:tt $usage_site:tt, | $($anything:tt)* ) => {
        $crate::__parse_closure_emit_error!{2 $usage_site}
    };
    ($macro:tt $args:tt $usage_site:tt, || $($anything:tt)* ) => {
        $crate::__parse_closure_emit_error!{2 $usage_site}
    };
    (
        ($($macro:tt)*) ($($args:tt)*) $usage_site:tt,
        $v:expr $(,)?
    ) => {
        match $v {func => {
            $($macro)* ! {
                $($args)*
                ((__x, __y)) -> _ {func(__x, __y)}
            }
        }}
    };
    ($macro:tt $args:tt $usage_site:tt $($rem:tt)*) => {
        $crate::__parse_closure_emit_error!{2 $usage_site}
    };

    ( $($anything:tt)* ) => {
        $crate::__::compile_error!("expected a closure argument")
    };
}

pub use __parse_closure_2;

////////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_1 {
    (
        $macro:tt $args:tt $usage_site:tt,
        ||  $(,)?
    ) => {
        $crate::__parse_closure_no_expr_error!{$usage_site}
    };
    (
        $macro:tt $args:tt $usage_site:tt,
        |$pat:tt $(: $pat_ty:ty)? $(,)?| $($rem:tt)*
    ) => (
        $crate::__parse_closure_expr! {
            $usage_site $macro $args ($pat $(: $pat_ty)?),
            $($rem)*
        }
    );
    (
        $macro:tt $args:tt $usage_site:tt,
        |$pat:pat_param $(,)?| $($rem:tt)*
    ) => (
        $crate::__parse_closure_expr! {
            $usage_site $macro $args ($pat),
            $($rem)*
        }
    );
    ($macro:tt $args:tt $usage_site:tt, | $($anything:tt)* ) => {
        $crate::__parse_closure_emit_error!{1 $usage_site}
    };
    ($macro:tt $args:tt $usage_site:tt, || $($anything:tt)* ) => {
        $crate::__parse_closure_emit_error!{1 $usage_site}
    };
    (
        ($($macro:tt)*) ($($args:tt)*) $usage_site:tt,
        $v:expr $(,)?
    ) => {
        match $v {func => {
            $($macro)* ! {
                $($args)*
                (__x) -> _ {func(__x)}
            }
        }}
    };
    ($macro:tt $args:tt $usage_site:tt $($rem:tt)*) => {
        $crate::__parse_closure_emit_error!{1 $usage_site}
    };

    ( $($anything:tt)* ) => {
        $crate::__::compile_error!("expected a closure argument")
    };
}

pub use __parse_closure_1;

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_expr {
    (
        $usage_site:tt ($($macro:tt)*) ($($args:tt)*) ($($pattern:tt)*),
        $(,)?
    ) => {
        $crate::__parse_closure_no_expr_error!{$usage_site}
    };

    (
        $usage_site:tt ($($macro:tt)*) ($($args:tt)*) $pattern:tt,
        $v:expr $(, $($rem:expr $(, $($rem_tt:tt)*)? )? )?
    ) => ({
        $($(
            $crate::__parse_closure_trailing_expr_error!{$usage_site $rem}
        )?)?

        $($macro)* ! {
            $($args)*
            $pattern { $v }
        }
    });

    (
        $usage_site:tt
        ($($macro:tt)*) ($($args:tt)*) $pattern:tt,
        -> $ret_ty:ty $v:block $(, $($rem:expr $(, $($rem_tt:tt)*)? )? )?
    ) => ({
        $($(
            $crate::__parse_closure_trailing_expr_error!{$usage_site $rem}
        )?)?

        $($macro)* ! {
            $($args)*
            $pattern -> $ret_ty { $v }
        }
    });
}

////////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_no_expr_error {
    (($($usage_site:tt)*)) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "`",
            $crate::__::stringify!($($usage_site)*),
            "` expects valid closure syntax, passed closure without return value",
        )}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_trailing_expr_error {
    (($($usage_site:tt)*) $($rem:tt)*) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "`",
            $crate::__::stringify!($($usage_site)*),
            "` expects no arguments after closure argument",
        )}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_closure_emit_error {
    ($count:tt ($($usage_site:tt)*)) => {
        $crate::__::compile_error!{$crate::__::concat!(
            "`",
            $crate::__::stringify!($($usage_site)*),
            "` expects to be passed a ",
            $count,
            "-parameter closure",
        )}
    };
}
