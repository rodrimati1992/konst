use crate::misc_tests::test_utils::must_panic;

use konst::slice;

// For comparing windows/chunks_chunks_exact with std equivalents
macro_rules! compare_with_std {
    ($iter_fn:ident) => {{
        use rand::rngs::SmallRng;
        use rand::{Rng, SeedableRng};

        let mut rng = SmallRng::seed_from_u64(6249204433781597762);

        let slice: &[u8] = &[1, 2, 3, 5, 8, 13, 21, 34, 55];
        for len in 0..=slice.len() {
            let slice = &slice[..len];

            for window_len in (0..20).flat_map(|_| 1..=len + 2) {
                let mut iter = konst::slice::$iter_fn(slice, window_len);
                let mut std_iter = slice.$iter_fn(window_len);

                let mut history = Vec::new();
                for _ in 0..10 {
                    let pair = if rng.gen() {
                        history.push("next");
                        (iter.copy().next(), std_iter.next())
                    } else {
                        history.push("next_back");
                        (iter.copy().next_back(), std_iter.next_back())
                    };

                    let extra_info = || {
                        format!(
                            "len: {} window_len: {} history: {:?}",
                            len, window_len, history,
                        )
                    };

                    match pair {
                        (Some((elem, next_iter)), Some(elem_std)) => {
                            iter = next_iter;

                            assert_eq!(elem, elem_std, "{}", extra_info());
                        }
                        (Some((elem, next_iter)), None) => {
                            iter = next_iter;
                            panic!(
                                "konst {} had {:?} when std iter was exhausted {}",
                                stringify!($iter_fn),
                                elem,
                                extra_info(),
                            )
                        }
                        (None, Some(elem)) => {
                            panic!(
                                "std {} had {:?} when konst iter was exhausted {}",
                                stringify!($iter_fn),
                                elem,
                                extra_info(),
                            )
                        }
                        (None, None) => {}
                    }
                }
            }
        }
    }};
}

#[test]
fn slice_windows_const_callable() {
    const fn __<'a>(slice: &'a [u8]) {
        let _: konst::slice::Windows<'a, u8> = konst::slice::windows(slice, 3);
        konst::slice::windows(slice, 3).next();
        konst::slice::windows(slice, 3).next_back();
        konst::slice::windows(slice, 3).copy();

        let rev: konst::slice::WindowsRev<'a, u8> = konst::slice::windows(slice, 3).rev();

        rev.copy();
        let _: konst::slice::Windows<'a, u8> = rev.copy().rev();
        rev.copy().next();
        rev.copy().next_back();
    }
}

#[test]
fn windows_basic() {
    let slice: &[u8] = &[3, 5, 8, 13, 21];

    must_panic(file_span!(), || konst::slice::windows(&[0; 0], 0)).unwrap();
    must_panic(file_span!(), || konst::slice::windows(slice, 0)).unwrap();

    for size in 1..10 {
        assert_eq!(
            collect_const_iter!(slice::windows(slice, size)),
            slice.windows(size).collect::<Vec<_>>(),
        );
        assert_eq!(
            collect_const_iter!(slice::windows(slice, size).rev()),
            slice.windows(size).rev().collect::<Vec<_>>(),
        );
    }
}

#[test]
fn windows_mixed_direction() {
    compare_with_std!(windows)
}

////////////////////////////////////////////////////////////////////////////////
//                  chunks iterator

#[test]
fn slice_chunks_const_callable() {
    const fn __<'a>(slice: &'a [u8]) {
        let _: konst::slice::Chunks<'a, u8> = konst::slice::chunks(slice, 3);
        konst::slice::chunks(slice, 3).next();
        konst::slice::chunks(slice, 3).next_back();
        konst::slice::chunks(slice, 3).copy();

        let rev: konst::slice::ChunksRev<'a, u8> = konst::slice::chunks(slice, 3).rev();

        rev.copy();
        let _: konst::slice::Chunks<'a, u8> = rev.copy().rev();
        rev.copy().next();
        rev.copy().next_back();
    }
}

#[test]
fn chunks_basic() {
    let slice: &[u8] = &[3, 5, 8, 13, 21];

    must_panic(file_span!(), || konst::slice::chunks(&[0; 0], 0)).unwrap();
    must_panic(file_span!(), || konst::slice::chunks(slice, 0)).unwrap();

    for size in 1..10 {
        assert_eq!(
            collect_const_iter!(slice::chunks(slice, size)),
            slice.chunks(size).collect::<Vec<_>>(),
            "size: {}",
            size,
        );
        assert_eq!(
            collect_const_iter!(slice::chunks(slice, size).rev()),
            slice.chunks(size).rev().collect::<Vec<_>>(),
            "size: {}",
            size,
        );
    }
}

#[test]
fn chunks_mixed_direction() {
    compare_with_std!(chunks)
}

////////////////////////////////////////////////////////////////////////////////
//                  chunks_exact iterator

#[test]
fn slice_chunks_exact_const_callable() {
    const fn __<'a>(slice: &'a [u8]) {
        let _: konst::slice::ChunksExact<'a, u8> = konst::slice::chunks_exact(slice, 3);
        konst::slice::chunks_exact(slice, 3).next();
        konst::slice::chunks_exact(slice, 3).next_back();
        konst::slice::chunks_exact(slice, 3).copy();

        let rev: konst::slice::ChunksExactRev<'a, u8> = konst::slice::chunks_exact(slice, 3).rev();

        rev.copy();
        let _: konst::slice::ChunksExact<'a, u8> = rev.copy().rev();
        rev.copy().next();
        rev.copy().next_back();
    }
}

#[test]
fn chunks_exact_basic() {
    let slice: &[u8] = &[3, 5, 8, 13, 21];

    must_panic(file_span!(), || konst::slice::chunks_exact(&[0; 0], 0)).unwrap();
    must_panic(file_span!(), || konst::slice::chunks_exact(slice, 0)).unwrap();

    for size in 1..10 {
        assert_eq!(
            collect_const_iter!(slice::chunks_exact(slice, size)),
            slice.chunks_exact(size).collect::<Vec<_>>(),
            "size: {}",
            size,
        );
        assert_eq!(
            collect_const_iter!(slice::chunks_exact(slice, size).rev()),
            slice.chunks_exact(size).rev().collect::<Vec<_>>(),
            "size: {}",
            size,
        );
    }
}

#[test]
fn chunks_exact_mixed_direction() {
    compare_with_std!(chunks_exact)
}
