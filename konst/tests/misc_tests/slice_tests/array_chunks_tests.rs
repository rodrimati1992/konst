use konst::slice;

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
        slice: &'a mut [u32],
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
