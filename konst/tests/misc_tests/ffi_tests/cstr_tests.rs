// There are additional tests in cstr::err_tests

use konst::ffi::cstr;

use core::ffi::CStr;

#[test]
fn to_bytes_with_nul_test() {
    const fn func(cstr: &CStr) -> &[u8] {
        cstr::to_bytes_with_nul(cstr)
    }

    assert_eq!(func(cstr::from_bytes_with_nul(b"foo\0").unwrap()), b"foo\0");
    assert_eq!(
        func(cstr::from_bytes_until_nul(b"bar\0qux\0").unwrap()),
        b"bar\0"
    );
}

#[test]
fn to_bytes_test() {
    const fn func(cstr: &CStr) -> &[u8] {
        cstr::to_bytes(cstr)
    }

    assert_eq!(func(cstr::from_bytes_with_nul(b"foo\0").unwrap()), b"foo");
    assert_eq!(
        func(cstr::from_bytes_until_nul(b"bar\0qux\0").unwrap()),
        b"bar"
    );
}

#[test]
fn to_str_test() {
    const fn func(cstr: &CStr) -> Result<&str, konst::string::Utf8Error> {
        cstr::to_str(cstr)
    }

    {
        let input = cstr::from_bytes_with_nul(b"foo\0").unwrap();
        assert_eq!(func(input).unwrap(), "foo");
    }
    {
        let input = cstr::from_bytes_with_nul(b"foo\xFF\xFF\0").unwrap();
        let err = func(input).unwrap_err().0;
        assert_eq!(err.valid_up_to(), 3);
        assert_eq!(err.error_len(), Some(1));
    }
}
