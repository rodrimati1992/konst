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

#[doc(hidden)]
#[repr(C)]
pub union Dereference<'a, T: ?Sized> {
    pub ptr: *const T,
    pub reff: &'a T,
}

macro_rules! make_parse_closure_macro {
    ($_:tt $macro_name:ident $arg_count:tt ($($pat_var:ident)*)) => {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! $macro_name {
            (
                $macro:tt $args:tt $usage_site:tt,
                |$_($pat_var_:pat_param),* $_(,)?|  $_(,)?
            ) => {
                $crate::__parse_closure_no_expr_error!{$usage_site}
            };
            (
                ($_($macro:tt)*) ($_($args:tt)*)
                $usage_site:tt,
                |$($_$pat_var:pat_param),* $_(,)?| $v:expr $_(,)*
            ) => {
                $_($macro)* ! {
                    $_($args)*
                    |$($_$pat_var),*| $v
                }
            };
            (
                $macro:tt $args:tt $usage_site:tt,
                |$_($pat_var_:pat_param),* $_(,)?| $v:expr,
                $trailing_expr:expr $_(, $_($rem:tt)*)?
            ) => {
                $crate::__parse_closure_trailing_expr_error!{$usage_site}
            };
            ($macro:tt $args:tt $usage_site:tt, | $_($anything:tt)* ) => {
                $crate::__parse_closure_emit_error!{
                    $arg_count
                    $usage_site
                }
            };
            ($macro:tt $args:tt $usage_site:tt, || $_($anything:tt)* ) => {
                $crate::__parse_closure_emit_error!{
                    $arg_count
                    $usage_site
                }
            };
            (
                ($_($macro:tt)*) ($_($args:tt)*)
                $usage_site:tt, $v:expr $_(,)?
            ) => {
                match $v {func => {
                    $_($macro)* ! {
                        $_($args)*
                        |$($pat_var),*| func($($pat_var),*)
                    }
                }}
            };
            ($macro:tt $args:tt $usage_site:tt $_($rem:tt)*) => {
                $crate::__parse_closure_emit_error!{
                    $arg_count
                    $usage_site
                }
            };

            ( $_($anything:tt)* ) => {
                $crate::__::compile_error!("expected a closure argument")
            };
        }

        pub use $macro_name;
    }
}

make_parse_closure_macro! { $ __parse_closure_1 1 (aaa) }
make_parse_closure_macro! { $ __parse_closure_2 2 (aaa bbb) }

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
    (($($usage_site:tt)*)) => {
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
