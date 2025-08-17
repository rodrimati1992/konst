use konst::slice;

fn bounds<T>(source: &[T]) -> impl Iterator<Item = usize> + use<T> {
    let len = source.len();

    (0..=len.saturating_add(3)).chain((0..=5).map(|x| usize::MAX - x))
}

#[test]
fn split_off_test() {
    const fn _constness() {
        let mut slice: &[u8] = &[];
        slice::split_off(&mut slice, 4..);
        slice::split_off(&mut slice, ..=4);
        slice::split_off(&mut slice, ..4);
    }

    let source: [usize; 10] = std::array::from_fn(|i| i);

    for len in 0..=source.len() {
        for range_bound in bounds(&source) {
            macro_rules! with_range_ref {
                ($source:ident, $len:ident, $range:expr) => {{
                    let mut slice_kon = &$source[..$len];
                    let mut slice_std = &$source[..$len];

                    assert_eq!(
                        slice::split_off(&mut slice_kon, $range),
                        slice_std.split_off($range)
                    );

                    assert_eq!(slice_kon, slice_std);
                }};
            }

            with_range_ref! {source, len, ..range_bound}
            with_range_ref! {source, len, ..=range_bound}
            with_range_ref! {source, len, range_bound..}
        }
    }
}

#[test]
fn split_off_zst_test() {
    let source = [(); usize::MAX];

    for len in (0..=5).chain(source.len() - 3..=source.len()) {
        for range_bound in (0..=5).chain(len.saturating_sub(3)..=len.saturating_add(3)) {
            macro_rules! with_range_ref {
                ($source:ident, $len:ident, $range:expr) => {{
                    let mut slice_kon = &$source[..$len];
                    let mut slice_std = &$source[..$len];

                    assert_eq!(
                        slice::split_off(&mut slice_kon, $range).map(|x| x.len()),
                        slice_std.split_off($range).map(|x| x.len()),
                    );

                    assert_eq!(slice_kon.len(), slice_std.len());
                }};
            }

            with_range_ref! {source, len, ..range_bound}
            with_range_ref! {source, len, ..=range_bound}
            with_range_ref! {source, len, range_bound..}
        }
    }
}

#[test]
fn split_off_mut_test() {
    const fn _constness() {
        let mut slice: &mut [u8] = &mut [];
        slice::split_off_mut(&mut slice, 4..);
        slice::split_off_mut(&mut slice, ..=4);
        slice::split_off_mut(&mut slice, ..4);
    }

    let mut source: [usize; 10] = std::array::from_fn(|i| i);

    for len in 0..=source.len() {
        for range_bound in bounds(&source) {
            macro_rules! with_range_mut {
                ($source:ident, $len:ident, $range:expr) => {{
                    let mut clone = $source;
                    let mut slice_kon = &mut $source[..$len];
                    let mut slice_std = &mut clone[..$len];

                    assert_eq!(
                        slice::split_off_mut(&mut slice_kon, $range),
                        slice_std.split_off_mut($range),
                    );

                    assert_eq!(slice_kon, slice_std);
                }};
            }

            with_range_mut! {source, len, ..range_bound}
            with_range_mut! {source, len, ..=range_bound}
            with_range_mut! {source, len, range_bound..}
        }
    }
}

#[test]
fn split_off_mut_zst_test() {
    let mut source = [(); usize::MAX];

    for len in (0..=5).chain(source.len() - 3..=source.len()) {
        for range_bound in (0..=5).chain(len.saturating_sub(3)..=len.saturating_add(3)) {
            macro_rules! with_range_mut {
                ($source:ident, $len:ident, $range:expr) => {{
                    let mut clone = $source;
                    let mut slice_kon = &mut $source[..$len];
                    let mut slice_std = &mut clone[..$len];

                    assert_eq!(
                        slice::split_off_mut(&mut slice_kon, $range).map(|x| x.len()),
                        slice_std.split_off_mut($range).map(|x| x.len()),
                    );

                    assert_eq!(slice_kon.len(), slice_std.len());
                }};
            }

            with_range_mut! {source, len, ..range_bound}
            with_range_mut! {source, len, ..=range_bound}
            with_range_mut! {source, len, range_bound..}
        }
    }
}

#[test]
fn split_off_first_last_test() {
    const fn _constness() {
        let mut slice: &[u8] = &[];
        slice::split_off_first(&mut slice);
        slice::split_off_last(&mut slice);
    }

    let source: [usize; 10] = std::array::from_fn(|i| i);

    for len in 0..=source.len() {
        macro_rules! with_method {
            ($method:ident) => {{
                let mut slice_kon = &source[..len];
                let mut slice_std = &source[..len];

                assert_eq!(slice::$method(&mut slice_kon), slice_std.$method());

                assert_eq!(slice_kon, slice_std);
            }};
        }

        with_method! {split_off_first}
        with_method! {split_off_last}
    }
}

#[test]
fn split_off_first_last_mut_test() {
    const fn _constness() {
        let mut slice: &mut [u8] = &mut [];
        slice::split_off_first_mut(&mut slice);
        slice::split_off_last_mut(&mut slice);
    }

    let mut source: [usize; 10] = std::array::from_fn(|i| i);

    for len in 0..=source.len() {
        macro_rules! with_method {
            ($method:ident) => {{
                let mut clone = source;
                let mut slice_kon = &mut clone[..len];
                let mut slice_std = &mut source[..len];

                assert_eq!(slice::$method(&mut slice_kon), slice_std.$method());

                assert_eq!(slice_kon, slice_std);
            }};
        }

        with_method! {split_off_first_mut}
        with_method! {split_off_last_mut}
    }
}
