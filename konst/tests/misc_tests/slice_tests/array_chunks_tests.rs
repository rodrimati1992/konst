use konst::slice;

#[test]
#[should_panic]
fn as_chunks_zero_chunk_len_panics() {
    let _ = slice::as_chunks::<_, 0>(&[1, 2, 3]);
}

#[test]
fn as_chunks_const_callable() {
    const SLICE: &[u32] = &[1, 2, 3, 5, 8, 13, 21, 34];

    const PAIR2: (&[[u32; 2]], &[u32]) = slice::as_chunks(SLICE);
    assert_eq!(PAIR2, (&[[1, 2], [3, 5], [8, 13], [21, 34]][..], &[][..]));

    const PAIR3: (&[[u32; 3]], &[u32]) = slice::as_chunks(SLICE);
    assert_eq!(PAIR3, (&[[1, 2, 3], [5, 8, 13]][..], &[21, 34][..]));

    const PAIR4: (&[[u32; 4]], &[u32]) = slice::as_chunks(SLICE);
    assert_eq!(PAIR4, (&[[1, 2, 3, 5], [8, 13, 21, 34]][..], &[][..]));
}

/////////////////////////////////////////////////

#[test]
#[should_panic]
fn as_chunks_mut_zero_chunk_len_panics() {
    let _ = slice::as_chunks_mut::<_, 0>(&mut [1, 2, 3]);
}

#[test]
fn as_chunks_mut_const_callable() {
    const fn _constable<T>(slice: &mut [T]) -> (&mut [[T; 2]], &mut [T]) {
        slice::as_chunks_mut(slice)
    }
}

#[test]
fn as_chunks_mut() {
    let slice: &mut [u32] = &mut [1, 2, 3, 5, 8, 13, 21, 34];

    let pair2: (&mut [[u32; 2]], &mut [u32]) = slice::as_chunks_mut(slice);
    assert_eq!(pair2, (&mut [[1, 2], [3, 5], [8, 13], [21, 34]][..], &mut [][..]));

    let pair3: (&mut [[u32; 3]], &mut [u32]) = slice::as_chunks_mut(slice);
    assert_eq!(pair3, (&mut [[1, 2, 3], [5, 8, 13]][..], &mut [21, 34][..]));

    let pair4: (&mut [[u32; 4]], &mut [u32]) = slice::as_chunks_mut(slice);
    assert_eq!(pair4, (&mut [[1, 2, 3, 5], [8, 13, 21, 34]][..], &mut [][..]));
}

#[test]
fn as_chunks_non_zero_chunk_len() {
    fn test_case<const CHUNK_LEN: usize>() {
        let slice = [
            0u32, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610,
        ];

        for len in 0..=slice.len() {
            let slice = &slice[..len];

            let (arrs, rem) = slice::as_chunks::<_, CHUNK_LEN>(slice);

            let mut iter = slice.chunks_exact(CHUNK_LEN);
            assert!(
                iter.by_ref().eq(arrs.iter().map(|arr| arr.as_slice())),
                "len: {len}  arrs: {arrs:?}"
            );

            assert_eq!(iter.remainder(), rem);
        }
    }

    test_case::<1>();
    test_case::<2>();
    test_case::<3>();
    test_case::<4>();
}

/////////////////////////////////////////////////

#[test]
#[should_panic]
fn as_rchunks_zero_chunk_len_panics() {
    let _ = slice::as_rchunks::<_, 0>(&[1, 2, 3]);
}

#[test]
fn as_rchunks_const_callable() {
    const SLICE: &[u32] = &[1, 2, 3, 5, 8, 13, 21, 34];

    const PAIR2: (&[u32], &[[u32; 2]]) = slice::as_rchunks(SLICE);
    assert_eq!(PAIR2, (&[][..], &[[1, 2], [3, 5], [8, 13], [21, 34]][..]));

    const PAIR3: (&[u32], &[[u32; 3]]) = slice::as_rchunks(SLICE);
    assert_eq!(PAIR3, (&[1, 2][..], &[[3, 5, 8], [13, 21, 34]][..]));

    const PAIR4: (&[u32], &[[u32; 4]]) = slice::as_rchunks(SLICE);
    assert_eq!(PAIR4, (&[][..], &[[1, 2, 3, 5], [8, 13, 21, 34]][..]));
}

#[test]
fn as_rchunks_non_zero_chunk_len() {
    fn test_case<const CHUNK_LEN: usize>() {
        let slice = [
            0u32, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610,
        ];

        for len in 0..=slice.len() {
            let slice = &slice[..len];

            let (rem, arrs) = slice::as_rchunks::<_, CHUNK_LEN>(slice);

            let mut iter = slice.rchunks_exact(CHUNK_LEN);
            assert!(
                iter.by_ref()
                    .rev()
                    .eq(arrs.iter().map(|arr| arr.as_slice())),
                "slice: {slice:?} chunk_len: {CHUNK_LEN}  arrs: {arrs:?}"
            );

            assert_eq!(iter.remainder(), rem);
        }
    }

    test_case::<1>();
    test_case::<2>();
    test_case::<3>();
    test_case::<4>();
}

/////////////////////////////////////////////////

#[test]
#[should_panic]
fn as_rchunks_mut_zero_chunk_len_panics() {
    let _ = slice::as_rchunks_mut::<_, 0>(&mut [1, 2, 3]);
}

#[test]
fn as_rchunks_mut_const_callable() {
    const fn _constable<T>(slice: &mut [T]) -> (&mut [T], &mut [[T; 2]]) {
        slice::as_rchunks_mut(slice)
    }
}

#[test]
fn as_rchunks_mut_non_zero_chunk_len() {
    fn test_case<const CHUNK_LEN: usize>() {
        let mut slice = [
            0u32, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610,
        ];
        let mut sliceb = slice;
        let slicec = slice;

        for len in 0..=slice.len() {
            let slice = &mut slice[..len];
            let sliceb = &mut sliceb[..len];

            let (rem, arrs) = slice::as_rchunks_mut::<_, CHUNK_LEN>(slice);

            let mut iter = sliceb.rchunks_exact_mut(CHUNK_LEN);
            assert!(
                iter.by_ref()
                    .rev()
                    .eq(arrs.iter().map(|arr| arr.as_slice())),
                "slice: {slicec:?} chunk_len: {CHUNK_LEN}  arrs: {arrs:?}"
            );

            assert_eq!(iter.into_remainder(), rem);
        }
    }

    test_case::<1>();
    test_case::<2>();
    test_case::<3>();
    test_case::<4>();
}

/////////////////////////////////////////////////

#[test]
#[should_panic]
fn array_chunks_zero_chunk_len_panics() {
    let _ = slice::array_chunks::<_, 0>(&[1, 2, 3]);
}

#[test]
fn array_chunks_const_callable() {
    const _SLICE: &[u32] = &[1, 2, 3, 5, 8, 13, 21, 34];

    const _: slice::ArrayChunks<'static, u32, 2> = slice::array_chunks(_SLICE);
    const _: slice::ArrayChunks<'static, u32, 3> = slice::array_chunks(_SLICE);
    const _: slice::ArrayChunks<'static, u32, 4> = slice::array_chunks(_SLICE);
}

#[test]
fn array_chunks_non_zero_chunk_len() {
    fn test_case<const CHUNK_LEN: usize>() {
        let slice = [1u32, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610];

        for len in 0..=slice.len() {
            let slice = &slice[..len];

            macro_rules! maybe_rev {($($remainder:ident)?, $($rev:tt)*) => ({
                let mut citer = slice::array_chunks::<_, CHUNK_LEN>(slice) $($rev)*;

                let mut iter = slice.chunks_exact(CHUNK_LEN) $($rev)*;

                for elem in iter.by_ref() {
                    let celem = citer.next().unwrap();
                    assert_eq!(elem, celem);
                }

                $( assert_eq!(iter.$remainder(), citer.$remainder()); )?
            })}

            maybe_rev! {remainder,}
            maybe_rev! {,.rev()}
        }
    }

    test_case::<1>();
    test_case::<2>();
    test_case::<3>();
    test_case::<4>();
}

/////////////////////////////////////////////////

#[test]
#[should_panic]
fn array_chunks_mut_zero_chunk_len_panics() {
    let _ = slice::array_chunks_mut::<_, 0>(&mut [1, 2, 3]);
}

#[test]
fn array_chunks_mut_const_callable() {
    const fn _constable<'a, const M: usize>(
        slice: &'a mut [u32]
    ) -> slice::ArrayChunksMut<'a, u32, M> {
        slice::array_chunks_mut(slice)
    }
}

#[test]
fn array_chunks_mut_non_zero_chunk_len() {
    fn test_case<const CHUNK_LEN: usize>() {
        let mut slice = [1u32, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610];
        let mut sliceb = slice;

        for len in 0..=slice.len() {
            let slice = &mut slice[..len];
            let sliceb = &mut sliceb[..len];

            macro_rules! maybe_rev {($($remainder:ident)?, $($rev:tt)*) => ({
                let mut citer = slice::array_chunks_mut::<_, CHUNK_LEN>(slice) $($rev)*;

                let mut iter = sliceb.chunks_exact_mut(CHUNK_LEN) $($rev)*;

                for elem in iter.by_ref() {
                    let celem = citer.next().unwrap();
                    assert_eq!(elem, celem);
                }

                $( assert_eq!(iter.$remainder(), citer.$remainder()); )?
            })}

            maybe_rev! {into_remainder,}
            maybe_rev! {,.rev()}
        }
    }

    test_case::<1>();
    test_case::<2>();
    test_case::<3>();
    test_case::<4>();
}
