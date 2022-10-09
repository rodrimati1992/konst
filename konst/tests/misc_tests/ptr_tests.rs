use konst::ptr::{self, nonnull};

use std::ptr::{null, null_mut, NonNull};

#[test]
fn deref_test() {
    let str_ptr: *const str = "hello";
    let array_ptr: *const [u8; 4] = &[3, 5, 8, 13];

    unsafe {
        assert_eq!(ptr::deref(str_ptr), "hello");
        assert_eq!(ptr::deref(array_ptr), &[3, 5, 8, 13]);
    }
}

#[test]
#[cfg(feature = "mut_refs")]
fn deref_mut_test() {
    let slice_ptr: *mut [u8] = &mut [3, 5, 8];
    let array_ptr: *mut [u8; 4] = &mut [13, 21, 34, 55];

    unsafe {
        ptr::deref_mut(slice_ptr)[1] += 10;
        ptr::deref_mut(array_ptr)[1] += 10;

        assert_eq!(ptr::deref_mut(slice_ptr), &mut [3, 15, 8][..]);
        assert_eq!(ptr::deref_mut(array_ptr), &mut [13, 31, 34, 55]);
    }
}

#[test]
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
#[cfg(feature = "mut_refs")]
fn ptr_as_mut_test() {
    let slice_ptr: *mut [u8] = &mut [3, 5, 8];
    let array_ptr: *mut [u8; 4] = &mut [13, 21, 34, 55];

    unsafe {
        ptr::as_mut(slice_ptr).unwrap()[1] += 10;
        ptr::as_mut(array_ptr).unwrap()[1] += 10;

        assert_eq!(ptr::as_mut(slice_ptr), Some(&mut [3, 15, 8][..]));
        assert_eq!(ptr::as_mut(array_ptr), Some(&mut [13, 31, 34, 55]));
        assert_eq!(ptr::as_mut(null_mut::<u8>()), None);
    }
}

#[test]
fn ptr_is_null() {
    let str_ptr: *const str = "hello";
    let array_ptr: *const [u8; 4] = &[3, 5, 8, 13];

    assert!(!ptr::is_null(str_ptr));
    assert!(!ptr::is_null(array_ptr));
    assert!(ptr::is_null(null::<u8>()));
}

#[test]
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
#[cfg(feature = "mut_refs")]
fn nonnull_new_mut_test() {
    let slice = &mut [3u8, 5, 8, 13][..];
    let mut str_ptr: Option<NonNull<[u8]>> = nonnull::new(slice);

    let array = &mut [21, 34, 55, 89];
    let mut array_ptr: Option<NonNull<[u8; 4]>> = nonnull::new(array);

    let null_ptr: Option<NonNull<u8>> = nonnull::new(null_mut());

    unsafe {
        str_ptr.unwrap().as_mut()[2] += 10;
        array_ptr.unwrap().as_mut()[2] += 10;

        assert_eq!(str_ptr.unwrap().as_mut(), &mut [3u8, 5, 18, 13][..]);
        assert_eq!(array_ptr.unwrap().as_mut(), &mut [21, 34, 65, 89]);
        assert_eq!(null_ptr, None);
    }
}

#[test]
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
#[cfg(feature = "mut_refs")]
fn nonnull_as_mut_test() {
    let slice = &mut [3u8, 5, 8, 13][..];
    let mut str_ptr: Option<NonNull<[u8]>> = nonnull::new(slice);

    let array = &mut [21, 34, 55, 89];
    let mut array_ptr: Option<NonNull<[u8; 4]>> = nonnull::new(array);

    let null_ptr: Option<NonNull<u8>> = nonnull::new(null_mut());

    unsafe {
        nonnull::as_mut(str_ptr.unwrap())[2] += 10;
        nonnull::as_mut(array_ptr.unwrap())[2] += 10;

        assert_eq!(nonnull::as_mut(str_ptr.unwrap()), &mut [3u8, 5, 18, 13][..]);
        assert_eq!(nonnull::as_mut(array_ptr.unwrap()), &mut [21, 34, 65, 89]);
        assert_eq!(null_ptr, None);
    }
}

#[test]
fn nonnull_from_ref_test() {
    let str_ptr: NonNull<str> = nonnull::from_ref("hello");
    let array_ptr: NonNull<[u8; 4]> = nonnull::from_ref(&[3, 5, 8, 13]);

    unsafe {
        assert_eq!(nonnull::as_ref(str_ptr), "hello");
        assert_eq!(nonnull::as_ref(array_ptr), &[3, 5, 8, 13]);
    }
}

#[test]
#[cfg(feature = "mut_refs")]
fn nonnull_from_mut_test() {
    let slice = &mut [3u8, 5, 8, 13][..];
    let mut str_ptr: NonNull<[u8]> = nonnull::from_mut(slice);

    let array = &mut [21, 34, 55, 89];
    let mut array_ptr: NonNull<[u8; 4]> = nonnull::from_mut(array);

    unsafe {
        nonnull::as_mut(str_ptr)[2] += 10;
        nonnull::as_mut(array_ptr)[2] += 10;

        assert_eq!(nonnull::as_mut(str_ptr), &mut [3u8, 5, 18, 13][..]);
        assert_eq!(nonnull::as_mut(array_ptr), &mut [21, 34, 65, 89]);
    }
}
