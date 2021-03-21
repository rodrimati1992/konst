use konst::slice::{bytes_find, bytes_rfind, slice_from, slice_up_to, split_at};

#[test]
fn slice_up_to_from_test() {
    let list = (0..=258).collect::<Vec<u32>>();

    for &pow in [1usize, 8, 64, 256].iter() {
        let lengths = [pow.saturating_sub(2), pow - 1, pow, pow + 1, pow + 2];
        for &length in lengths.iter() {
            let sub = &list[..length];
            for i in 0..=sub.len() {
                assert_eq!(slice_up_to(&sub, i), &sub[..i]);

                assert_eq!(slice_from(&sub, i), &sub[i..]);

                assert_eq!(split_at(&sub, i), (&sub[..i], &sub[i..]));
            }
            assert_eq!(slice_up_to(&sub, sub.len() + 1), sub);
            assert_eq!(slice_up_to(&sub, sub.len() + 2), sub);
            assert_eq!(slice_up_to(&sub, sub.len() + 3), sub);

            assert_eq!(split_at(&sub, sub.len() + 1), (sub, &[][..]));
            assert_eq!(split_at(&sub, sub.len() + 2), (sub, &[][..]));
            assert_eq!(split_at(&sub, sub.len() + 3), (sub, &[][..]));

            assert_eq!(slice_from(&sub, sub.len() + 1), &[]);
            assert_eq!(slice_from(&sub, sub.len() + 2), &[]);
            assert_eq!(slice_from(&sub, sub.len() + 3), &[]);
        }
    }
}

#[test]
fn find_test() {
    #[track_caller]
    fn ass(
        left: &[u8],
        right: &[u8],
        finds: &[(usize, Option<usize>)],
        rfinds: &[(usize, Option<usize>)],
    ) {
        for (offset, opt) in finds.iter().copied() {
            assert_eq!(
                bytes_find(left, right, offset),
                opt,
                "in find, offset: {}",
                offset
            );

            if !right.is_empty() {
                assert_eq!(
                    bytes_find(b"", right, offset),
                    None,
                    "in find empty left, offset: {}",
                    offset
                );
            }
            if offset < left.len() {
                assert_eq!(
                    bytes_find(left, b"", offset),
                    Some(offset),
                    "in find empty right, offset: {}",
                    offset
                );
            }
        }
        for (offset, opt) in rfinds.iter().copied() {
            assert_eq!(
                bytes_rfind(left, right, offset),
                opt,
                "in rfind, offset: {}",
                offset
            );

            if !right.is_empty() {
                assert_eq!(
                    bytes_rfind(b"", right, offset),
                    None,
                    "in rfind empty left, offset: {}",
                    offset
                );
            }
            if offset < left.len() {
                assert_eq!(
                    bytes_rfind(left, b"", offset),
                    Some(offset),
                    "in rfind empty right, offset: {}",
                    offset
                );
            }
        }
    }

    ass(
        b"foo-bar-baz-foo---",
        b"foo",
        &[
            (0, Some(0)),
            (1, Some(12)),
            (4, Some(12)),
            (12, Some(12)),
            (13, None),
            (17, None),
            (18, None),
            (!0, None),
        ],
        &[
            (!0, Some(12)),
            (17, Some(12)),
            (15, Some(12)),
            (14, Some(12)),
            (13, Some(0)),
            (12, Some(0)),
            (3, Some(0)),
            (3, Some(0)),
            (2, Some(0)),
            (1, None),
            (0, None),
        ],
    );

    ass(
        b"foo-bar-baz-foo---",
        b"f",
        &[
            (0, Some(0)),
            (1, Some(12)),
            (4, Some(12)),
            (12, Some(12)),
            (13, None),
            (17, None),
            (18, None),
            (!0, None),
        ],
        &[
            (!0, Some(12)),
            (18, Some(12)),
            (17, Some(12)),
            (14, Some(12)),
            (13, Some(12)),
            (12, Some(12)),
            (11, Some(0)),
            (3, Some(0)),
            (3, Some(0)),
            (2, Some(0)),
            (0, Some(0)),
        ],
    );

    // Tests overlapping patterns
    ass(
        b"lawlawnawn--awn-lawlawn",
        b"lawn",
        &[
            (0, Some(3)),
            (1, Some(3)),
            (2, Some(3)),
            (3, Some(3)),
            (4, Some(19)),
            (16, Some(19)),
            (17, Some(19)),
            (18, Some(19)),
            (20, None),
            (22, None),
            (23, None),
            (!0, None),
        ],
        &[
            (!0, Some(19)),
            (23, Some(19)),
            (22, Some(19)),
            (20, Some(3)),
            (18, Some(3)),
            (17, Some(3)),
            (16, Some(3)),
            (8, Some(3)),
            (7, Some(3)),
            (6, Some(3)),
            (5, None),
            (4, None),
            (3, None),
            (2, None),
            (1, None),
            (0, None),
        ],
    );
}
