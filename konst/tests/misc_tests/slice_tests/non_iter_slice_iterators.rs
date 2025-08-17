use crate::misc_tests::test_utils::must_panic;

use konst::slice;

// For comparing windows/chunks_chunks_exact with std equivalents
macro_rules! compare_with_std {
    ($iter_fn:ident) => {{
        use rand::rngs::SmallRng;
        use rand::{Rng, SeedableRng};

        let mut rng = SmallRng::seed_from_u64(6249204433781597762);

        let slice: &mut [u8] = &mut [1, 2, 3, 5, 8, 13, 21, 34, 55];
        let sliceb: &mut [u8] = &mut [1, 2, 3, 5, 8, 13, 21, 34, 55];
        for len in 0..=slice.len() {
            let slice = &mut slice[..len];
            let sliceb = &mut sliceb[..len];

            for window_len in (0..20).flat_map(|_| 1..=len + 2) {
                let mut iter = konst::slice::$iter_fn(slice, window_len);
                let mut std_iter = sliceb.$iter_fn(window_len);

                let mut history = Vec::new();
                for _ in 0..10 {
                    let pair = if rng.r#gen() {
                        history.push("next");
                        (iter.next(), std_iter.next())
                    } else {
                        history.push("next_back");
                        (iter.next_back(), std_iter.next_back())
                    };

                    let extra_info = || {
                        format!(
                            "len: {} window_len: {} history: {:?}",
                            len, window_len, history,
                        )
                    };

                    match pair {
                        (Some(elem), Some(elem_std)) => {
                            assert_eq!(elem, elem_std, "{}", extra_info());
                        }
                        (Some(elem), None) => {
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

// expensive, and doesn't use unsafe, so no need for miri checking
#[cfg(not(miri))]
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

// expensive, and doesn't use unsafe, so no need for miri checking
#[cfg(not(miri))]
#[test]
fn chunks_mixed_direction() {
    compare_with_std!(chunks)
}

////////////////////////////////////////////////////////////////////////////////
//                  chunks_mut iterator

#[test]
fn slice_chunks_mut_const_callable() {
    type _Pair<T> = (T, T);

    const fn __<'a>((slicea, sliceb): _Pair<&'a mut [u8]>) {
        konst::slice::chunks_mut(slicea, 3).next();
        konst::slice::chunks_mut(slicea, 3).next_back();

        let mut rev: konst::slice::ChunksMutRev<u8> = konst::slice::chunks_mut(slicea, 3).rev();

        rev.next();
        rev.next_back();

        let _: konst::slice::ChunksMut<'a, u8> = konst::slice::chunks_mut(slicea, 3);
        let _: konst::slice::ChunksMut<'a, u8> = konst::slice::chunks_mut(sliceb, 3).rev().rev();
    }
}

#[test]
fn chunks_mut_basic() {
    let slice: &mut [u8] = &mut [3, 5, 8, 13, 21];
    let sliceb: &mut [u8] = &mut [3, 5, 8, 13, 21];

    must_panic(file_span!(), || konst::slice::chunks_mut(&mut [0; 0], 0)).unwrap();
    must_panic(file_span!(), || konst::slice::chunks_mut(slice, 0)).unwrap();

    for size in 1..10 {
        assert_eq!(
            collect_const_iter!(slice::chunks_mut(slice, size)),
            sliceb.chunks_mut(size).collect::<Vec<_>>(),
            "size: {}",
            size,
        );
        assert_eq!(
            collect_const_iter!(slice::chunks_mut(slice, size).rev()),
            sliceb.chunks_mut(size).rev().collect::<Vec<_>>(),
            "size: {}",
            size,
        );
    }
}

// expensive, and doesn't use unsafe, so no need for miri checking
#[cfg(not(miri))]
#[test]
fn chunks_mut_mixed_direction() {
    compare_with_std!(chunks_mut)
}

////////////////////////////////////////////////////////////////////////////////
//                  rchunks iterator

#[test]
fn slice_rchunks_const_callable() {
    const fn __<'a>(slice: &'a [u8]) {
        let _: konst::slice::RChunks<'a, u8> = konst::slice::rchunks(slice, 3);
        konst::slice::rchunks(slice, 3).next();
        konst::slice::rchunks(slice, 3).next_back();
        konst::slice::rchunks(slice, 3).copy();

        let rev: konst::slice::RChunksRev<'a, u8> = konst::slice::rchunks(slice, 3).rev();

        rev.copy();
        let _: konst::slice::RChunks<'a, u8> = rev.copy().rev();
        rev.copy().next();
        rev.copy().next_back();
    }
}

#[test]
fn rchunks_basic() {
    let slice: &[u8] = &[3, 5, 8, 13, 21];

    must_panic(file_span!(), || konst::slice::rchunks(&[0; 0], 0)).unwrap();
    must_panic(file_span!(), || konst::slice::rchunks(slice, 0)).unwrap();

    for size in 1..10 {
        assert_eq!(
            collect_const_iter!(slice::rchunks(slice, size)),
            slice.rchunks(size).collect::<Vec<_>>(),
            "size: {}",
            size,
        );
        assert_eq!(
            collect_const_iter!(slice::rchunks(slice, size).rev()),
            slice.rchunks(size).rev().collect::<Vec<_>>(),
            "size: {}",
            size,
        );
    }
}

// expensive, and doesn't use unsafe, so no need for miri checking
#[cfg(not(miri))]
#[test]
fn rchunks_mixed_direction() {
    compare_with_std!(rchunks)
}

////////////////////////////////////////////////////////////////////////////////
//                  rchunks_mut iterator

#[test]
fn slice_rchunks_mut_const_callable() {
    type _Tup<T> = (T, T, T);

    const fn __<'a>((slicea, sliceb, slicec): _Tup<&'a mut [u8]>) {
        let _: konst::slice::RChunksMut<'a, u8> = konst::slice::rchunks_mut(slicea, 3);

        konst::slice::rchunks_mut(slicec, 3).next();
        konst::slice::rchunks_mut(slicec, 3).next_back();

        let mut rev: konst::slice::RChunksMutRev<'a, u8> =
            konst::slice::rchunks_mut(sliceb, 3).rev();

        rev.next();
        rev.next_back();
        let _: konst::slice::RChunksMut<'a, u8> = rev.rev();
    }
}

#[test]
fn rchunks_mut_basic() {
    let slice: &mut [u8] = &mut [3, 5, 8, 13, 21];
    let sliceb: &mut [u8] = &mut [3, 5, 8, 13, 21];

    must_panic(file_span!(), || konst::slice::rchunks_mut(&mut [0; 0], 0)).unwrap();
    must_panic(file_span!(), || konst::slice::rchunks_mut(slice, 0)).unwrap();

    for size in 1..10 {
        assert_eq!(
            collect_const_iter!(slice::rchunks_mut(slice, size)),
            sliceb.rchunks_mut(size).collect::<Vec<_>>(),
            "size: {}",
            size,
        );
        assert_eq!(
            collect_const_iter!(slice::rchunks_mut(slice, size).rev()),
            sliceb.rchunks_mut(size).rev().collect::<Vec<_>>(),
            "size: {}",
            size,
        );
    }
}

// expensive, and doesn't use unsafe, so no need for miri checking
#[cfg(not(miri))]
#[test]
fn rchunks_mut_mixed_direction() {
    compare_with_std!(rchunks_mut)
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
    let slice: &[u8] = &[3, 5, 8, 13, 21, 34, 55];

    must_panic(file_span!(), || konst::slice::chunks_exact(&[0; 0], 0)).unwrap();
    must_panic(file_span!(), || konst::slice::chunks_exact(slice, 0)).unwrap();

    for size in 1..10 {
        {
            let mut citer = slice::chunks_exact(slice, size);
            let mut iter = slice.chunks_exact(size);

            assert_eq!(citer.remainder(), iter.remainder());

            for _ in &mut iter {
                _ = citer.next().unwrap();
            }

            assert_eq!(citer.remainder(), iter.remainder());
        }
        {
            let mut citer = slice::chunks_exact(slice, size).rev();
            let mut iter = slice.chunks_exact(size);

            assert_eq!(citer.remainder(), iter.remainder());

            for _ in iter.by_ref().rev() {
                _ = citer.next().unwrap();
            }

            assert_eq!(citer.remainder(), iter.remainder());
        }

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

// expensive, and doesn't use unsafe, so no need for miri checking
#[cfg(not(miri))]
#[test]
fn chunks_exact_mixed_direction() {
    compare_with_std!(chunks_exact)
}

////////////////////////////////////////////////////////////////////////////////
//                  rchunks_exact iterator

#[test]
fn slice_rchunks_exact_const_callable() {
    const fn __<'a>(slice: &'a [u8]) {
        let _: konst::slice::RChunksExact<'a, u8> = konst::slice::rchunks_exact(slice, 3);
        konst::slice::rchunks_exact(slice, 3).next();
        konst::slice::rchunks_exact(slice, 3).next_back();
        konst::slice::rchunks_exact(slice, 3).copy();

        let rev: konst::slice::RChunksExactRev<'a, u8> =
            konst::slice::rchunks_exact(slice, 3).rev();

        rev.copy();
        let _: konst::slice::RChunksExact<'a, u8> = rev.copy().rev();
        rev.copy().next();
        rev.copy().next_back();
    }
}

#[test]
fn rchunks_exact_basic() {
    let slice: &[u8] = &[3, 5, 8, 13, 21, 34, 55];

    must_panic(file_span!(), || konst::slice::rchunks_exact(&[0; 0], 0)).unwrap();
    must_panic(file_span!(), || konst::slice::rchunks_exact(slice, 0)).unwrap();

    for size in 1..10 {
        {
            let mut citer = slice::rchunks_exact(slice, size);
            let mut iter = slice.rchunks_exact(size);

            assert_eq!(citer.remainder(), iter.remainder());

            for _ in &mut iter {
                _ = citer.next().unwrap();
            }

            assert_eq!(citer.remainder(), iter.remainder());
        }
        {
            let mut citer = slice::rchunks_exact(slice, size).rev();
            let mut iter = slice.rchunks_exact(size);

            assert_eq!(citer.remainder(), iter.remainder());

            for _ in iter.by_ref().rev() {
                _ = citer.next().unwrap();
            }

            assert_eq!(citer.remainder(), iter.remainder());
        }

        assert_eq!(
            collect_const_iter!(slice::rchunks_exact(slice, size)),
            slice.rchunks_exact(size).collect::<Vec<_>>(),
            "size: {}",
            size,
        );
        assert_eq!(
            collect_const_iter!(slice::rchunks_exact(slice, size).rev()),
            slice.rchunks_exact(size).rev().collect::<Vec<_>>(),
            "size: {}",
            size,
        );
    }
}

// expensive, and doesn't use unsafe, so no need for miri checking
#[cfg(not(miri))]
#[test]
fn rchunks_exact_mixed_direction() {
    compare_with_std!(rchunks_exact)
}

////////////////////////////////////////////////////////////////////////////////
//                  chunks_exact_mut iterator

#[test]
fn slice_chunks_exact_mut_const_callable() {
    type __Tup<T> = (T, T, T);

    const fn __<'a>((slicea, sliceb, slicec): __Tup<&'a mut [u8]>) {
        let _: konst::slice::ChunksExactMut<'a, u8> = konst::slice::chunks_exact_mut(slicea, 3);
        let _: konst::slice::ChunksExactMut<'a, u8> =
            konst::slice::chunks_exact_mut(sliceb, 3).rev().rev();

        konst::slice::chunks_exact_mut(slicec, 3).next();
        konst::slice::chunks_exact_mut(slicec, 3).next_back();

        let mut rev: konst::slice::ChunksExactMutRev<'a, u8> =
            konst::slice::chunks_exact_mut(slicec, 3).rev();
        rev.next();
        rev.next_back();
    }
}

#[test]
fn chunks_exact_mut_basic() {
    let slice: &mut [u8] = &mut [3, 5, 8, 13, 21, 34, 55];
    let sliceb: &mut [u8] = &mut [3, 5, 8, 13, 21, 34, 55];

    must_panic(file_span!(), || {
        konst::slice::chunks_exact_mut(&mut [0; 0], 0)
    })
    .unwrap();
    must_panic(file_span!(), || konst::slice::chunks_exact_mut(slice, 0)).unwrap();

    for size in 1..10 {
        {
            let mut citer = slice::chunks_exact_mut(slice, size);
            let mut iter = sliceb.chunks_exact_mut(size);

            for _ in &mut iter {
                _ = citer.next().unwrap();
            }

            assert_eq!(citer.into_remainder(), iter.into_remainder());
        }
        {
            let mut citer = slice::chunks_exact_mut(slice, size).rev();
            let mut iter = sliceb.chunks_exact_mut(size);

            for _ in iter.by_ref().rev() {
                _ = citer.next().unwrap();
            }

            assert_eq!(citer.into_remainder(), iter.into_remainder());
        }

        assert_eq!(
            collect_const_iter!(slice::chunks_exact_mut(slice, size)),
            sliceb.chunks_exact_mut(size).collect::<Vec<_>>(),
            "size: {}",
            size,
        );
        assert_eq!(
            collect_const_iter!(slice::chunks_exact_mut(slice, size).rev()),
            sliceb.chunks_exact_mut(size).rev().collect::<Vec<_>>(),
            "size: {}",
            size,
        );
    }
}

// expensive, and doesn't use unsafe, so no need for miri checking
#[cfg(not(miri))]
#[test]
fn chunks_exact_mut_mixed_direction() {
    compare_with_std!(chunks_exact_mut)
}

////////////////////////////////////////////////////////////////////////////////
//                  rchunks_exact_mut iterator

#[test]
fn slice_rchunks_exact_mut_const_callable() {
    type __Tup<T> = (T, T, T);

    const fn __<'a>((slicea, sliceb, slicec): __Tup<&'a mut [u8]>) {
        let _: konst::slice::RChunksExactMut<'a, u8> = konst::slice::rchunks_exact_mut(slicea, 3);
        let _: konst::slice::RChunksExactMut<'a, u8> =
            konst::slice::rchunks_exact_mut(sliceb, 3).rev().rev();

        konst::slice::rchunks_exact_mut(slicec, 3).next();
        konst::slice::rchunks_exact_mut(slicec, 3).next_back();

        let mut rev: konst::slice::RChunksExactMutRev<'a, u8> =
            konst::slice::rchunks_exact_mut(slicec, 3).rev();
        rev.next();
        rev.next_back();
    }
}

#[test]
fn rchunks_exact_mut_basic() {
    let slice: &mut [u8] = &mut [3, 5, 8, 13, 21, 34, 55];
    let sliceb: &mut [u8] = &mut [3, 5, 8, 13, 21, 34, 55];

    must_panic(file_span!(), || {
        konst::slice::rchunks_exact_mut(&mut [0; 0], 0)
    })
    .unwrap();
    must_panic(file_span!(), || konst::slice::rchunks_exact_mut(slice, 0)).unwrap();

    for size in 1..10 {
        {
            let mut citer = slice::rchunks_exact_mut(slice, size);
            let mut iter = sliceb.rchunks_exact_mut(size);

            for _ in &mut iter {
                _ = citer.next().unwrap();
            }

            assert_eq!(citer.into_remainder(), iter.into_remainder());
        }
        {
            let mut citer = slice::rchunks_exact_mut(slice, size).rev();
            let mut iter = sliceb.rchunks_exact_mut(size);

            for _ in iter.by_ref().rev() {
                _ = citer.next().unwrap();
            }

            assert_eq!(citer.into_remainder(), iter.into_remainder());
        }

        assert_eq!(
            collect_const_iter!(slice::rchunks_exact_mut(slice, size)),
            sliceb.rchunks_exact_mut(size).collect::<Vec<_>>(),
            "size: {}",
            size,
        );
        assert_eq!(
            collect_const_iter!(slice::rchunks_exact_mut(slice, size).rev()),
            sliceb.rchunks_exact_mut(size).rev().collect::<Vec<_>>(),
            "size: {}",
            size,
        );
    }
}

// expensive, and doesn't use unsafe, so no need for miri checking
#[cfg(not(miri))]
#[test]
fn rchunks_exact_mut_mixed_direction() {
    compare_with_std!(rchunks_exact_mut)
}
