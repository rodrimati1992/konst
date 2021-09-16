use konst::ptr::{self, nonnull};

use std::ptr::{null, null_mut, NonNull};

#[test]
#[cfg(feature = "rust_1_56")]
fn deref_test() {
    let str_ptr: *const str = "hello";
    let array_ptr: *const [u8; 4] = &[3, 5, 8, 13];

    unsafe {
        assert_eq!(ptr::deref(str_ptr), "hello");
        assert_eq!(ptr::deref(array_ptr), &[3, 5, 8, 13]);
    }
}

#[test]
#[cfg(feature = "rust_1_56")]
fn ptr_as_ref_test() {
    let str_ptr: *const str = "hello";
    let array_ptr: *const [u8; 4] = &[3, 5, 8, 13];

    unsafe {
        assert_eq!(ptr::as_ref(str_ptr), Some("hello"));
        assert_eq!(ptr::as_ref(array_ptr), Some(&[3, 5, 8, 13]));
        assert_eq!(ptr::as_ref(null::<u8>()), None);
    }
}

#[test]
#[cfg(feature = "rust_1_56")]
fn ptr_is_null() {
    let str_ptr: *const str = "hello";
    let array_ptr: *const [u8; 4] = &[3, 5, 8, 13];

    unsafe {
        assert!(!ptr::is_null(str_ptr));
        assert!(!ptr::is_null(array_ptr));
        assert!(ptr::is_null(null::<u8>()));
    }
}

#[test]
#[cfg(feature = "rust_1_56")]
fn nonnull_new_test() {
    let str_ptr: Option<NonNull<str>> = nonnull::new("hello" as *const _ as *mut _);

    let array_ptr: Option<NonNull<[u8; 4]>> = nonnull::new(&[3u8, 5, 8, 13] as *const _ as *mut _);

    let null_ptr: Option<NonNull<u8>> = nonnull::new(null_mut());

    unsafe {
        assert_eq!(str_ptr.unwrap().as_ref(), "hello");
        assert_eq!(array_ptr.unwrap().as_ref(), &[3, 5, 8, 13]);
        assert_eq!(null_ptr, None);
    }
}

#[test]
#[cfg(feature = "rust_1_56")]
fn nonnull_as_ref_test() {
    let str_ptr: Option<NonNull<str>> = nonnull::new("hello" as *const _ as *mut _);

    let array_ptr: Option<NonNull<[u8; 4]>> = nonnull::new(&[3u8, 5, 8, 13] as *const _ as *mut _);

    let null_ptr: Option<NonNull<u8>> = nonnull::new(null_mut());

    unsafe {
        assert_eq!(nonnull::as_ref(str_ptr.unwrap()), "hello");
        assert_eq!(nonnull::as_ref(array_ptr.unwrap()), &[3, 5, 8, 13]);
        assert_eq!(null_ptr, None);
    }
}

#[test]
#[cfg(feature = "rust_1_56")]
fn nonnull_from_ref_test() {
    let str_ptr: NonNull<str> = nonnull::from_ref("hello");
    let array_ptr: NonNull<[u8; 4]> = nonnull::from_ref(&[3, 5, 8, 13]);

    unsafe {
        assert_eq!(nonnull::as_ref(str_ptr), "hello");
        assert_eq!(nonnull::as_ref(array_ptr), &[3, 5, 8, 13]);
    }
}
