// used for comparing types that compare equal, but contain different data,
// important for max and min iterator methods,
// because they guarantee which comparing-equal value is returned.
//
#[derive(Debug, Eq, PartialEq)]
struct Data {
    val: u8,
    // part of the struct that's not compared
    uncmp: u8,
}

impl Data {
    pub const fn new(val: u8, uncmp: u8) -> Self {
        Self { val, uncmp }
    }
}

konst::cmp::impl_cmp! {
    impl Data;

    #[allow(dead_code)]
    pub const fn const_eq(&self, other: &Self) -> bool {
        self.val == other.val
    }

    pub const fn const_cmp(&self, other: &Self) -> std::cmp::Ordering {
        ::konst::cmp::const_cmp!(self.val, other.val)
    }
}

#[test]
fn min() {
    const fn constness(slice: &[Data]) -> Option<&Data> {
        konst::iter::eval!(slice, min())
    }

    assert_eq!(constness(&[]), None::<&Data>);

    let m = Data::new;
    assert_eq!(constness(&[m(3, 5)]), Some(&m(3, 5)));

    assert_eq!(constness(&[m(100, 0), m(3, 5)]), Some(&m(3, 5)));
    assert_eq!(constness(&[m(3, 5), m(100, 0)]), Some(&m(3, 5)));

    assert_eq!(
        constness(&[m(100, 0), m(100, 0), m(3, 5), m(5, 0), m(3, 8)]),
        Some(&m(3, 5))
    );
}

#[test]
fn max() {
    const fn constness(slice: &[Data]) -> Option<&Data> {
        konst::iter::eval!(slice, max())
    }

    assert_eq!(constness(&[]), None::<&Data>);

    let m = Data::new;

    assert_eq!(constness(&[m(3, 5)]), Some(&m(3, 5)));

    assert_eq!(constness(&[m(0, 0), m(3, 5)]), Some(&m(3, 5)));
    assert_eq!(constness(&[m(3, 5), m(0, 0)]), Some(&m(3, 5)));

    assert_eq!(
        constness(&[m(0, 0), m(0, 0), m(3, 5), m(3, 8), m(2, 0)]),
        Some(&m(3, 8))
    );
}

const MIN_BY_CASES: &[(&[u8], Option<&u8>)] = &[
    (&[], None::<&u8>),
    (&[3], Some(&3)),
    (&[12, 3], Some(&12)),
    (&[3, 12], Some(&12)),
    (&[3, 13], Some(&3)),
    (&[13, 3], Some(&13)),
    (&[3, 12, 10, 24, 100, 106], Some(&10)),
];

#[test]
fn min_by() {
    const fn constness(slice: &[u8]) -> Option<&u8> {
        konst::iter::eval!(
            slice,
            min_by(|l: &&u8, r: &&u8| konst::cmp::const_cmp!(**l % 10, **r % 10))
        )
    }

    for &(arg, expected) in MIN_BY_CASES {
        assert_eq!(constness(arg), expected, "arg: {arg:?}");
    }
}

#[test]
fn min_by_key() {
    const fn constness(slice: &[u8]) -> Option<&u8> {
        konst::iter::eval!(slice, min_by_key(|item: &&u8| **item % 10))
    }

    for &(arg, expected) in MIN_BY_CASES {
        assert_eq!(constness(arg), expected, "arg: {arg:?}");
    }
}

const MAX_BY_CASES: &[(&[u8], Option<&u8>)] = &[
    (&[], None::<&u8>),
    (&[3], Some(&3)),
    (&[12, 3], Some(&3)),
    (&[3, 12], Some(&3)),
    (&[3, 13], Some(&13)),
    (&[13, 3], Some(&3)),
    (&[3, 12, 108, 24, 18, 106], Some(&18)),
];

#[test]
fn max_by() {
    const fn constness(slice: &[u8]) -> Option<&u8> {
        konst::iter::eval!(
            slice,
            max_by(|l: &&u8, r: &&u8| konst::cmp::const_cmp!(**l % 10, **r % 10))
        )
    }

    for &(arg, expected) in MAX_BY_CASES {
        assert_eq!(constness(arg), expected, "arg: {arg:?}");
    }
}

#[test]
fn max_by_key() {
    const fn constness(slice: &[u8]) -> Option<&u8> {
        konst::iter::eval!(slice, max_by_key(|item: &&u8| **item % 10))
    }

    for &(arg, expected) in MAX_BY_CASES {
        assert_eq!(constness(arg), expected, "arg: {arg:?}");
    }
}
