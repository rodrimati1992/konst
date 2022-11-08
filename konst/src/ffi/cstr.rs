//! Const equivalents of [`CStr`] methods

#[cfg(test)]
mod err_tests;

use crate::slice::slice_up_to;

use core::{ffi::CStr, fmt};

////////////////////////////////////////////////////////////////////////////////

/// Error returned by [`from_bytes_until_nul`] when the input slice either
/// does not terminate with nul.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FromBytesUntilNulError(());

impl FromBytesUntilNulError {
    /// Const equivalent of `FromBytesUntilNulError::clone`
    pub const fn copy(&self) -> Self {
        Self(())
    }

    /// Panics with this type's error message
    #[track_caller]
    pub const fn panic(&self) -> ! {
        panic!("{}", self.err_msg())
    }
    const fn err_msg(&self) -> &str {
        "data provided does not contain a nul"
    }
}

impl fmt::Display for FromBytesUntilNulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.err_msg())
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Error returned by [`from_bytes_with_nul`] when the input slice either
/// does not terminate with nul, or contains inner nul bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FromBytesWithNulError {
    kind: HuntNulError,
}

impl FromBytesWithNulError {
    /// Const equivalent of `FromBytesWithNulError::clone`
    pub const fn copy(&self) -> Self {
        Self { kind: self.kind }
    }

    const fn err_msg(&self) -> (&str, Option<usize>) {
        match self.kind {
            HuntNulError::InternalNul(pos) => {
                ("input bytes contain an internal nul byte at: ", Some(pos))
            }
            HuntNulError::NotNulTerminated => ("input bytes don't terminate with nul", None),
        }
    }

    /// Panics with this type's error message
    #[track_caller]
    pub const fn panic(&self) -> ! {
        use const_panic::{concat_panic, FmtArg, PanicVal};

        let (msg, num) = self.err_msg();

        concat_panic(&[&[
            PanicVal::write_str(msg),
            match num {
                Some(x) => PanicVal::from_usize(x, FmtArg::DEBUG),
                None => PanicVal::EMPTY,
            },
        ]])
    }
}

impl fmt::Display for FromBytesWithNulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (msg, num) = self.err_msg();
        f.write_str(msg)?;
        if let Some(num) = num {
            write!(f, "{num}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum HuntNulError {
    InternalNul(usize),
    NotNulTerminated,
}

////////////////////////////////////////////////////////////////////////////////

struct CStrAndLen<'a> {
    cstr: &'a CStr,
    length_with_nul: usize,
}

const fn from_bytes_until_nul_inner(
    bytes: &[u8],
) -> Result<CStrAndLen<'_>, FromBytesUntilNulError> {
    crate::for_range! {i in 0..bytes.len() =>
        if bytes[i] == 0 {
            let sub = slice_up_to(bytes, i + 1);
            unsafe {
                return Ok(CStrAndLen{
                    cstr: CStr::from_bytes_with_nul_unchecked(sub),
                    length_with_nul: i + 1,
                });
            }
        }
    }

    Err(FromBytesUntilNulError(()))
}

/// Converts a byte slice which contains any amount of nul bytes into a `&CStr`.
/// Const equivalent of [`CStr::from_bytes_until_nul`]
///
/// # Example
///
/// ```rust
/// use konst::{ffi::cstr, unwrap_ctx};
///
/// use std::ffi::CStr;
///
///
/// const CS: &CStr = unwrap_ctx!(cstr::from_bytes_until_nul(b"hello\0world"));
///
/// assert_eq!(CS.to_str().unwrap(), "hello");
///
/// ```
///
pub const fn from_bytes_until_nul(bytes: &[u8]) -> Result<&CStr, FromBytesUntilNulError> {
    match from_bytes_until_nul_inner(bytes) {
        Ok(CStrAndLen { cstr, .. }) => Ok(cstr),
        Err(e) => Err(e),
    }
}

/// Converts a nul-terminated byte slice into a `&CStr`.
/// Const equivalent of [`CStr::from_bytes_with_nul`]
///
/// # Example
///
/// ```rust
/// use konst::{ffi::cstr, unwrap_ctx};
///
/// use std::ffi::CStr;
///
///
/// const CS: &CStr = unwrap_ctx!(cstr::from_bytes_with_nul(b"foo bar\0"));
///
/// assert_eq!(CS.to_str().unwrap(), "foo bar");
///
/// ```
///
pub const fn from_bytes_with_nul(bytes: &[u8]) -> Result<&CStr, FromBytesWithNulError> {
    const fn make_not_null_term_err<T>() -> Result<T, FromBytesWithNulError> {
        Err(FromBytesWithNulError {
            kind: HuntNulError::NotNulTerminated,
        })
    }

    match from_bytes_until_nul_inner(bytes) {
        Ok(CStrAndLen {
            cstr,
            length_with_nul,
        }) if length_with_nul == bytes.len() => Ok(cstr),
        Ok(_) if bytes[bytes.len() - 1] != 0 => make_not_null_term_err(),
        Err(_) => make_not_null_term_err(),
        Ok(CStrAndLen {
            length_with_nul, ..
        }) => Err(FromBytesWithNulError {
            kind: HuntNulError::InternalNul(length_with_nul - 1),
        }),
    }
}

/// Converts this CStr to a byte slice, including the nul terminator.
/// Const equivalent of [`CStr::to_bytes_with_nul`]
///
/// # Performance
///
/// This function takes linear time to run, proportional to the length of `this`.
///
/// # Example
///
/// ```rust
/// use konst::{ffi::cstr, unwrap_ctx};
///
/// use std::ffi::CStr;
///
///
/// const CS: &CStr = unwrap_ctx!(cstr::from_bytes_with_nul(b"example\0"));
///
/// const BYTES: &[u8] = cstr::to_bytes_with_nul(CS);
///
/// assert_eq!(BYTES, b"example\0");
///
/// ```
pub const fn to_bytes_with_nul(this: &CStr) -> &[u8] {
    let start = this.as_ptr().cast::<u8>();
    let mut i = 0;

    unsafe {
        while *start.add(i) != 0 {
            i += 1;
        }

        core::slice::from_raw_parts(start, i + 1)
    }
}

/// Converts this CStr to a byte slice, excluding the nul terminator.
/// Const equivalent of [`CStr::to_bytes`]
///
/// # Performance
///
/// This function takes linear time to run, proportional to the length of `this`.
///
/// # Example
///
/// ```rust
/// use konst::{ffi::cstr, unwrap_ctx};
///
/// use std::ffi::CStr;
///
///
/// const CS: &CStr = unwrap_ctx!(cstr::from_bytes_with_nul(b"hmm...\0"));
///
/// const BYTES: &[u8] = cstr::to_bytes(CS);
///
/// assert_eq!(BYTES, b"hmm...");
///
/// ```
pub const fn to_bytes(this: &CStr) -> &[u8] {
    match to_bytes_with_nul(this) {
        [rem @ .., 0] => rem,
        _ => unreachable!(),
    }
}

/// Converts this CStr to a string slice, excluding the nul terminator.
/// Const equivalent of [`CStr::to_str`]
///
/// # Performance
///
/// This function takes linear time to run, proportional to the length of `this`.
///
/// # Example
///
/// ```rust
/// use konst::{ffi::cstr, unwrap_ctx};
///
/// use std::ffi::CStr;
///
///
/// const CS: &CStr = unwrap_ctx!(cstr::from_bytes_with_nul(b"of beads\0"));
///
/// const STRING: &str = unwrap_ctx!(cstr::to_str(CS));
///
/// assert_eq!(STRING, "of beads");
///
/// ```
pub const fn to_str(this: &CStr) -> Result<&str, crate::string::Utf8Error> {
    crate::string::from_utf8(to_bytes(this))
}
