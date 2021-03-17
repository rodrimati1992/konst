use konst::slice::{slice_from, slice_up_to, split_at};

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
