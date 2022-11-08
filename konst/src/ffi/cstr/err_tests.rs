// Tests errors from the cstr module, which can't be tested in the
// top-level tests folder.

use crate::ffi::cstr;

#[test]
fn from_bytes_until_nul_errs_test() {
    {
        let cs = cstr::from_bytes_until_nul(b"hello\0world").unwrap();
        assert_eq!(cs.to_str().unwrap(), "hello");
    }

    let _ = cstr::from_bytes_until_nul(b"helloworld").unwrap_err();
    let _ = cstr::from_bytes_until_nul(b"").unwrap_err();
}

#[test]
fn from_bytes_with_nul_errs_test() {
    {
        let cs = cstr::from_bytes_with_nul(b"hello\0").unwrap();
        assert_eq!(cs.to_str().unwrap(), "hello");
    }
    for string in [&b""[..], &b"hello\0aaa"[..]] {
        let err = cstr::from_bytes_with_nul(string).unwrap_err();
        assert!(
            matches!(err.kind, cstr::HuntNulError::NotNulTerminated),
            "{err:?} {string:?}",
        );
    }

    for (string, pos) in [
        (&b"\0helloaaa\0"[..], 0),
        (&b"h\0elloaaa\0"[..], 1),
        (b"hello\0aaa\0", 5),
    ] {
        let err = cstr::from_bytes_with_nul(string).unwrap_err();
        assert_eq!(
            err.kind,
            cstr::HuntNulError::InternalNul(pos),
            "{err:?} {string:?}",
        );
    }
}
