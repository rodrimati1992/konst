use konst::ptr::{self, nonnull};

use std::ptr::{null, null_mut, NonNull};

#[test]
fn deref_mut_test() {
    let slice_ptr: *mut [u8] = &mut [3, 5, 8];
    let array_ptr: *mut [u8; 4] = &mut [13, 21, 34, 55];

    unsafe {
        (*slice_ptr)[1] += 10;
        (*array_ptr)[1] += 10;

        assert_eq!(&mut *slice_ptr, &mut [3, 15, 8][..]);
        assert_eq!(&mut *array_ptr, &mut [13, 31, 34, 55]);
    }
}

#[test]
fn nonnull_from_ref_test() {
    let str_ptr: NonNull<str> = nonnull::from_ref("hello");
    let array_ptr: NonNull<[u8; 4]> = nonnull::from_ref(&[3, 5, 8, 13]);

    unsafe {
        assert_eq!(str_ptr.as_ref(), "hello");
        assert_eq!(array_ptr.as_ref(), &[3, 5, 8, 13]);
    }
}

#[test]
fn nonnull_from_mut_test() {
    let slice = &mut [3u8, 5, 8, 13][..];
    let mut str_ptr: NonNull<[u8]> = nonnull::from_mut(slice);

    let array = &mut [21, 34, 55, 89];
    let mut array_ptr: NonNull<[u8; 4]> = nonnull::from_mut(array);

    unsafe {
        str_ptr.as_mut()[2] += 10;
        array_ptr.as_mut()[2] += 10;

        assert_eq!(str_ptr.as_mut(), &mut [3u8, 5, 18, 13][..]);
        assert_eq!(array_ptr.as_mut(), &mut [21, 34, 65, 89]);
    }
}
