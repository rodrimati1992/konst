use konst::slice::slice_concat;

#[test]
fn slice_concat_basic_test() {
    assert_eq!(slice_concat!(u8, &[]), []);
    assert_eq!(slice_concat!(u8, &[&[]]), []);
    assert_eq!(slice_concat!(u8, &[&[], &[3]]), [3]);
    assert_eq!(slice_concat!(u8, &[&[5], &[]]), [5]);
    assert_eq!(slice_concat!(u8, &[&[5], &[8, 13]]), [5, 8, 13]);
    assert_eq!(
        slice_concat!(u8, &[&[], &[8, 13], &[21, 34, 55]]),
        [8, 13, 21, 34, 55]
    );
}

#[test]
fn slice_concat_from_ref_const_test() {
    const S: &[&[&str]] = &[&["foo", "bar"], &[], &["hello"]];
    assert_eq!(slice_concat!(&str, S), ["foo", "bar", "hello"]);
}

// this test ensures that non-promotable expressions still work
#[test]
fn slice_concat_from_func_test() {
    const fn func() -> [&'static [u32]; 4] {
        [&[], &[3, 5, 8], &[13, 21], &[]]
    }

    assert_eq!(slice_concat!(u32, &func()), [3, 5, 8, 13, 21]);
}
