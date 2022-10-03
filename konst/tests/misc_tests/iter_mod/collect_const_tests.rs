use konst::{iter::collect_const, slice};

#[test]
fn collect_const_zip() {
    {
        const ARR: [(&u8, usize); 4] = collect_const!((&u8, usize) => &[3u8, 5, 8, 13],zip(100..));

        assert_eq!(ARR, [(&3, 100), (&5, 101), (&8, 102), (&13, 103)]);
    }
    {
        const ARR: [((&u8, &str), usize); 2] = collect_const!(((&u8, &str), usize) =>
            &[3u8, 5, 8, 13],
                zip(slice::iter_copied(&["hello", "world"])),
                zip(100..),
        );

        assert_eq!(ARR, [((&3, "hello"), 100), ((&5, "world"), 101)]);
    }
    {
        const ARR: [usize; 2] = collect_const!(usize =>
            slice::iter_copied(&[3usize, 5, 8, 13]),
                zip(slice::iter_copied(&["hello", "world"])),
                zip(100..),
                map(|((a, b), c)| a + b.len() + c),
        );

        assert_eq!(ARR, [100 + 5 + 3, 101 + 5 + 5]);
    }
}

#[test]
fn collect_const_enumerate() {
    {
        const ARR: [(usize, &u8); 4] = collect_const!((usize, &u8) => &[3u8, 5, 8, 13],enumerate());

        assert_eq!(ARR, [(0, &3), (1, &5), (2, &8), (3, &13)]);
    }
    {
        const ARR: [((usize, &u8), u32); 3] = collect_const!(((usize, &u8), u32) =>
            &[3u8, 5, 8, 13],
                enumerate(),
                zip(slice::iter_copied(&[10u32, 11, 12])),
        );

        assert_eq!(ARR, [((0, &3), 10), ((1, &5), 11), ((2, &8), 12)]);
    }
    {
        const ARR: [(usize, (&u8, u32)); 3] = collect_const!((usize, (&u8, u32)) =>
            &[3u8, 5, 8, 13],
                zip(slice::iter_copied(&[10u32, 11, 12])),
                enumerate(),
        );

        assert_eq!(ARR, [(0, (&3, 10)), (1, (&5, 11)), (2, (&8, 12))]);
    }
}

#[test]
fn collect_const_filter() {
    const ARR: [&u8; 4] = collect_const!(&u8 =>
        &[3u8, 5, 8, 13, 21],
            filter(|e| !e.is_power_of_two()),
    );

    assert_eq!(ARR, [&3, &5, &13, &21]);
}

#[test]
fn collect_const_map() {
    {
        const ARR: [usize; 4] = collect_const!(usize => (1..=4),map(|e| e * 3));

        assert_eq!(ARR, [3, 6, 9, 12]);
    }

    {
        const ARR: [usize; 4] = collect_const!(usize =>
            (1..=4),
                map(|x| {
                    // testing that lifetime extension works
                    &(x * 10)
                }),
                copied(),
        );

        assert_eq!(ARR, [10, 20, 30, 40]);
    }
}

#[test]
fn collect_const_filter_map() {
    {
        use std::num::NonZeroU8;

        const ARR: [NonZeroU8; 4] = collect_const!(NonZeroU8 =>
            &[3, 0, 1, 5, 6],
                filter_map(|x| NonZeroU8::new(*x)),
        );

        assert_eq!(ARR, [3, 1, 5, 6].map(|n| NonZeroU8::new(n).unwrap()));
    }
    {
        const ARR: [u8; 5] = collect_const!(u8 =>
            slice::iter_copied(&[3u8, 0, 1, 5, 6]),
                filter_map(Some),
        );

        assert_eq!(ARR, [3u8, 0, 1, 5, 6]);
    }
    {
        const ARR: [u8; 0] = collect_const!(u8 =>
            &[3u8, 0, 1, 5, 6],
                filter_map(|_| None),
        );

        assert_eq!(ARR, []);
    }
}

#[test]
fn collect_const_flat_map() {
    {
        const ARR: [usize; 9] = collect_const!(usize =>
            &[3, 5, 8],
                flat_map(|x| {
                    let x10 = *x * 10;
                    x10..x10 + 3
                }),
        );

        assert_eq!(ARR, [30, 31, 32, 50, 51, 52, 80, 81, 82]);
    }
    {
        const ARR: [usize; 9] = collect_const!(usize =>
            &[3, 5, 8],
                flat_map(|x| {
                    let x10 = *x * 10;
                    // testing that lifetime extension works
                    &[x10, x10 + 1, x10 + 2]
                }),
                copied(),
        );

        assert_eq!(ARR, [30, 31, 32, 50, 51, 52, 80, 81, 82]);
    }
}

#[test]
fn collect_const_flatten() {
    {
        const ARR: [&u8; 4] = collect_const!(&u8 => &[&[3, 5], &[8, 13]], flatten());

        assert_eq!(ARR, [&3, &5, &8, &13]);
    }
    {
        const ARR: [usize; 5] = collect_const!(usize => &[0..3, 10..12], flatten());

        assert_eq!(ARR, [0, 1, 2, 10, 11]);
    }
}

#[test]
fn collect_const_copied() {
    const ARR: [u8; 3] = collect_const!(u8 =>
        &[2, 3, 4, 5, 6],
            copied(),
            filter(|n| *n % 2 == 0)
    );

    assert_eq!(ARR, [2, 4, 6]);
}

#[test]
fn collect_const_take() {
    {
        const ARR: [usize; 3] = collect_const!(usize => 10..,take(3));

        assert_eq!(ARR, [10, 11, 12]);
    }
    // `take` more items than are available
    {
        const ARR: [usize; 4] = collect_const!(usize => 10..14,take(10));

        assert_eq!(ARR, [10, 11, 12, 13]);
    }
}

#[test]
fn collect_const_mixing_take_skip() {
    {
        const ARR: [usize; 6] = collect_const!(usize =>10..,take(10), skip(4));

        assert_eq!(ARR, [14, 15, 16, 17, 18, 19]);
    }
    {
        const ARR: [usize; 2] = collect_const!(usize =>10..18,take(10), skip(6), take(3));

        assert_eq!(ARR, [16, 17]);
    }
}

#[test]
fn collect_const_take_while() {
    {
        const ARR: [&u8; 4] = collect_const!(&u8 =>
            &[3, 5, 8, 13, 21, 34, 55],take_while(|elem| **elem < 20 )
        );

        assert_eq!(ARR, [&3, &5, &8, &13]);
    }
    {
        const ARR: [&u8; 2] = collect_const!(&u8 =>
            &[0, 2, 3, 4, 5, 6],take_while(|elem| **elem % 2 == 0)
        );

        assert_eq!(ARR, [&0, &2]);
    }
    {
        const ARR: [&u8; 2] = collect_const!(&u8 =>
            &[0, 2, 3, 4, 5, 6]
                ,take_while(|elem| **elem < 5)
                ,take_while(|elem| **elem < 3)
        );

        assert_eq!(ARR, [&0, &2]);
    }
    {
        const ARR: [&u8; 2] = collect_const!(&u8 =>
            &[0, 2, 3, 4, 5, 6]
                ,take_while(|elem| **elem < 3)
                ,take_while(|elem| **elem < 5)
        );

        assert_eq!(ARR, [&0, &2]);
    }
    {
        const ARR: [&u8; 4] = collect_const!(&u8 =>
            &[3, 5, 8, 13],take_while(|_| true)
        );

        assert_eq!(ARR, [&3, &5, &8, &13]);
    }
    {
        const ARR: [&u8; 0] = collect_const!(&u8 =>
            &[3, 5, 8, 13, 21, 34, 55],take_while(|_| false)
        );

        assert_eq!(ARR, [&0u8; 0]);
    }
}

#[test]
fn collect_const_skip() {
    {
        const ARR: [usize; 3] = collect_const!(usize => 10..=12,skip(0));

        assert_eq!(ARR, [10, 11, 12]);
    }
    {
        const ARR: [usize; 3] = collect_const!(usize => 10..=15,skip(3));

        assert_eq!(ARR, [13, 14, 15]);
    }
    {
        const ARR: [usize; 4] = collect_const!(usize => 10..=18,skip(3),skip(2));

        assert_eq!(ARR, [15, 16, 17, 18]);
    }
}

#[test]
fn collect_const_skip_while() {
    {
        const ARR: [&u8; 3] = collect_const!(&u8 =>
            &[3, 5, 8, 13, 21, 34, 55],skip_while(|elem| **elem < 20 )
        );

        assert_eq!(ARR, [&21, &34, &55]);
    }
    {
        const ARR: [&u8; 3] = collect_const!(&u8 =>
            &[3, 5, 8, 13, 21, 34, 55],
                skip_while(|elem| **elem < 21 ),
                skip_while(|elem| **elem > 34 ),
        );

        assert_eq!(ARR, [&21, &34, &55]);
    }
    {
        const ARR: [&u8; 4] = collect_const!(&u8 =>
            &[3, 5, 8, 13],skip_while(|_| false)
        );

        assert_eq!(ARR, [&3, &5, &8, &13]);
    }
    {
        const ARR: [&u8; 0] = collect_const!(&u8 =>
            &[3, 5, 8, 13],skip_while(|_| true)
        );

        assert_eq!(ARR, [&0u8; 0]);
    }
}

#[test]
fn collect_const_both_skip_and_take_while() {
    {
        const ARR: [&u8; 0] = collect_const!(&u8 =>
            &[3, 5, 8, 13, 21, 34, 55],
                skip_while(|elem| **elem < 21 ),
                take_while(|elem| **elem > 34 ),
        );

        assert_eq!(ARR, [&0; 0]);
    }
    {
        const ARR: [&u8; 0] = collect_const!(&u8 =>
            &[3, 5, 8, 13, 21, 34, 55],
                take_while(|elem| **elem > 34 ),
                skip_while(|elem| **elem < 21 ),
        );

        assert_eq!(ARR, [&0; 0]);
    }
    {
        const ARR: [&u8; 2] = collect_const!(&u8 =>
            &[3, 5, 8, 13, 21, 34, 55],
                take_while(|elem| **elem < 20 ),
                skip_while(|elem| **elem % 10 <= 5),
        );

        assert_eq!(ARR, [&8, &13]);
    }
    {
        const ARR: [&u8; 2] = collect_const!(&u8 =>
            &[3, 5, 8, 13, 21, 34, 55],
                skip_while(|elem| **elem % 10 <= 5),
                take_while(|elem| **elem < 20 ),
        );

        assert_eq!(ARR, [&8, &13]);
    }
}
