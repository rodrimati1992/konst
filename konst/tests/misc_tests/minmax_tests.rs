use konst::{
    cmp::{ConstCmp, IsNotStdKind},
    const_cmp, const_eq,
};

use std::cmp::Ordering;

use crate::misc_tests::test_utils::assert_type;

#[derive(Debug, PartialEq, Eq)]
struct Mod10(u32);

impl ConstCmp for Mod10 {
    type Kind = IsNotStdKind;
}

impl Mod10 {
    #[allow(dead_code)]
    const fn const_eq(&self, other: &Self) -> bool {
        const_eq!(self.0 % 10, other.0 % 10)
    }

    const fn const_cmp(&self, other: &Self) -> Ordering {
        const_cmp!(self.0 % 10, other.0 % 10)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct NonCopy(u32);

impl ConstCmp for NonCopy {
    type Kind = IsNotStdKind;
}

impl NonCopy {
    const fn copy(&self) -> Self {
        Self(self.0)
    }
}

#[test]
fn min_std_test() {
    const fn minn(l: u32, r: u32) -> u32 {
        konst::min!(l, r)
    }

    assert_eq!(minn(3, 5), 3);
    assert_eq!(minn(5, 5), 5);
    assert_eq!(minn(5, 3), 3);
}

#[test]
fn min_custom_test() {
    const fn minn(l: Mod10, r: Mod10) -> Mod10 {
        konst::min!(l, r)
    }

    assert_eq!(minn(Mod10(3), Mod10(5)), Mod10(3));
    assert_eq!(minn(Mod10(3), Mod10(15)), Mod10(3));
    assert_eq!(minn(Mod10(13), Mod10(5)), Mod10(13));
    assert_eq!(minn(Mod10(13), Mod10(15)), Mod10(13));

    // min returns the first argument when both arguments compare equal
    assert_eq!(minn(Mod10(5), Mod10(15)), Mod10(5));
    assert_eq!(minn(Mod10(15), Mod10(5)), Mod10(15));

    assert_eq!(minn(Mod10(5), Mod10(3)), Mod10(3));
    assert_eq!(minn(Mod10(5), Mod10(13)), Mod10(13));
    assert_eq!(minn(Mod10(15), Mod10(3)), Mod10(3));
    assert_eq!(minn(Mod10(15), Mod10(13)), Mod10(13));
}

#[test]
fn max_std_test() {
    const fn maxx(l: u32, r: u32) -> u32 {
        konst::max!(l, r)
    }

    assert_eq!(maxx(3, 3), 3);
    assert_eq!(maxx(3, 5), 5);
    assert_eq!(maxx(5, 3), 5);
}

#[test]
fn max_custom_test() {
    const fn maxx(l: Mod10, r: Mod10) -> Mod10 {
        konst::max!(l, r)
    }

    assert_eq!(maxx(Mod10(3), Mod10(5)), Mod10(5));
    assert_eq!(maxx(Mod10(3), Mod10(15)), Mod10(15));
    assert_eq!(maxx(Mod10(13), Mod10(5)), Mod10(5));
    assert_eq!(maxx(Mod10(13), Mod10(15)), Mod10(15));

    // max returns the first argument when both arguments compare equal
    assert_eq!(maxx(Mod10(5), Mod10(15)), Mod10(15));
    assert_eq!(maxx(Mod10(15), Mod10(5)), Mod10(5));

    assert_eq!(maxx(Mod10(5), Mod10(3)), Mod10(5));
    assert_eq!(maxx(Mod10(5), Mod10(13)), Mod10(5));
    assert_eq!(maxx(Mod10(15), Mod10(3)), Mod10(15));
    assert_eq!(maxx(Mod10(15), Mod10(13)), Mod10(15));
}

const fn cmp_mod10(l: &u32, r: &u32) -> std::cmp::Ordering {
    konst::const_cmp!(*l % 10, *r % 10)
}
const fn cmp_nc_mod10(l: &NonCopy, r: &NonCopy) -> std::cmp::Ordering {
    cmp_mod10(&l.0, &r.0)
}

const fn mod10(n: &NonCopy) -> u32 {
    n.0 % 10
}

#[test]
fn min_by_closure_arg_order() {
    let _ = konst::min_by!(3u32, 10, |&l, &r| {
        assert_eq!((l, r), (3, 10));
        std::cmp::Ordering::Greater
    });
}

#[test]
fn min_by_test() {
    assert_eq!(
        konst::min_by!(5, 8, |l: &u128, r: &u128| {
            assert_type::<_, &u128>(&l);
            assert_type::<_, &u128>(&r);
            konst::const_cmp!(l, r)
        }),
        5,
    );

    const fn minn(l: NonCopy, r: NonCopy) -> NonCopy {
        use NonCopy as NC;

        let ret = konst::min_by!(l.copy(), r.copy(), cmp_nc_mod10);

        assert!(konst::min_by!(l.0, r.0, |l, r| cmp_mod10(l, r)) == ret.0);
        assert!(konst::min_by!(l.0, r.0, |l: &u32, r: &u32| cmp_mod10(l, r)) == ret.0);

        // mix of patterns and type annotations
        assert!(konst::min_by!(&l, &r, |l: &NC, r| cmp_mod10(&l.0, &r.0)).0 == ret.0);
        assert!(konst::min_by!(&l, &r, |l: &NC, r: &NC| cmp_mod10(&l.0, &r.0)).0 == ret.0);
        assert!(konst::min_by!(&l, &r, |l: &NC, (NC(r))| cmp_mod10(&l.0, r)).0 == ret.0);
        assert!(konst::min_by!(&l, &r, |(NC(l)), r: &NC| cmp_mod10(l, &r.0)).0 == ret.0);
        assert!(konst::min_by!(&l, &r, |(NC(l)), (NC(r))| cmp_mod10(l, r)).0 == ret.0);

        // only patterns are used
        assert!(konst::min_by!(&l, &r, |NC(l), NC(r)| cmp_mod10(l, r)).0 == ret.0);

        ret
    }

    assert_eq!(minn(NonCopy(3), NonCopy(5)), NonCopy(3));
    assert_eq!(minn(NonCopy(3), NonCopy(15)), NonCopy(3));
    assert_eq!(minn(NonCopy(13), NonCopy(5)), NonCopy(13));
    assert_eq!(minn(NonCopy(13), NonCopy(15)), NonCopy(13));

    // min returns the first argument when both arguments compare equal
    assert_eq!(minn(NonCopy(5), NonCopy(15)), NonCopy(5));
    assert_eq!(minn(NonCopy(15), NonCopy(5)), NonCopy(15));

    assert_eq!(minn(NonCopy(5), NonCopy(3)), NonCopy(3));
    assert_eq!(minn(NonCopy(5), NonCopy(13)), NonCopy(13));
    assert_eq!(minn(NonCopy(15), NonCopy(3)), NonCopy(3));
    assert_eq!(minn(NonCopy(15), NonCopy(13)), NonCopy(13));
}

#[test]
fn max_by_closure_arg_order() {
    let _ = konst::max_by!(3u32, 10, |&l, &r| {
        assert_eq!((l, r), (3, 10));
        std::cmp::Ordering::Greater
    });
}

#[test]
fn max_by_test() {
    assert_eq!(
        konst::max_by!(5, 8, |l: &u128, r: &u128| {
            assert_type::<_, &u128>(&l);
            assert_type::<_, &u128>(&r);
            konst::const_cmp!(l, r)
        }),
        8,
    );

    const fn maxx(l: NonCopy, r: NonCopy) -> NonCopy {
        use NonCopy as NC;

        let ret = konst::max_by!(l.copy(), r.copy(), cmp_nc_mod10);
        assert!(konst::max_by!(l.0, r.0, |l, r| cmp_mod10(l, r)) == ret.0);
        assert!(konst::max_by!(l.0, r.0, |l: &u32, r: &u32| cmp_mod10(l, r)) == ret.0);

        // mix of patterns and type annotations
        assert!(konst::max_by!(&l, &r, |l: &NC, r| cmp_mod10(&l.0, &r.0)).0 == ret.0);
        assert!(konst::max_by!(&l, &r, |l: &NC, r: &NC| cmp_mod10(&l.0, &r.0)).0 == ret.0);
        assert!(konst::max_by!(&l, &r, |l: &NC, (NC(r))| cmp_mod10(&l.0, r)).0 == ret.0);
        assert!(konst::max_by!(&l, &r, |(NC(l)), r: &NC| cmp_mod10(l, &r.0)).0 == ret.0);
        assert!(konst::max_by!(&l, &r, |(NC(l)), (NC(r))| cmp_mod10(l, r)).0 == ret.0);

        // only patterns are used
        assert!(konst::max_by!(&l, &r, |NC(l), NC(r)| cmp_mod10(l, r)).0 == ret.0);

        ret
    }

    assert_eq!(maxx(NonCopy(3), NonCopy(5)), NonCopy(5));
    assert_eq!(maxx(NonCopy(3), NonCopy(15)), NonCopy(15));
    assert_eq!(maxx(NonCopy(13), NonCopy(5)), NonCopy(5));
    assert_eq!(maxx(NonCopy(13), NonCopy(15)), NonCopy(15));

    // max returns the first argument when both arguments compare equal
    assert_eq!(maxx(NonCopy(5), NonCopy(15)), NonCopy(15));
    assert_eq!(maxx(NonCopy(15), NonCopy(5)), NonCopy(5));

    assert_eq!(maxx(NonCopy(5), NonCopy(3)), NonCopy(5));
    assert_eq!(maxx(NonCopy(5), NonCopy(13)), NonCopy(5));
    assert_eq!(maxx(NonCopy(15), NonCopy(3)), NonCopy(15));
    assert_eq!(maxx(NonCopy(15), NonCopy(13)), NonCopy(15));
}

#[test]
fn min_by_key_test() {
    assert!(
        konst::min_by_key!(5, 8, |n: &u128| {
            assert_type::<_, &u128>(&n);
            n
        }) == 5
    );

    const fn minn(l: NonCopy, r: NonCopy) -> NonCopy {
        let ret = konst::min_by_key!(l.copy(), r.copy(), mod10);

        assert!(konst::min_by_key!(l.0, r.0, |n| *n % 10) == ret.0);
        assert!(konst::min_by_key!(l.0, r.0, |n: &u32| *n % 10) == ret.0);
        assert!(konst::min_by_key!(&l, &r, |NonCopy(n)| *n % 10).0 == ret.0);

        ret
    }

    assert_eq!(minn(NonCopy(3), NonCopy(5)), NonCopy(3));
    assert_eq!(minn(NonCopy(3), NonCopy(15)), NonCopy(3));
    assert_eq!(minn(NonCopy(13), NonCopy(5)), NonCopy(13));
    assert_eq!(minn(NonCopy(13), NonCopy(15)), NonCopy(13));

    // min returns the first argument when both arguments compare equal
    assert_eq!(minn(NonCopy(5), NonCopy(15)), NonCopy(5));
    assert_eq!(minn(NonCopy(15), NonCopy(5)), NonCopy(15));

    assert_eq!(minn(NonCopy(5), NonCopy(3)), NonCopy(3));
    assert_eq!(minn(NonCopy(5), NonCopy(13)), NonCopy(13));
    assert_eq!(minn(NonCopy(15), NonCopy(3)), NonCopy(3));
    assert_eq!(minn(NonCopy(15), NonCopy(13)), NonCopy(13));
}

#[test]
fn max_by_key_test() {
    assert!(
        konst::max_by_key!(5, 8, |n: &u128| {
            assert_type::<_, &u128>(&n);
            n
        }) == 8
    );

    const fn maxx(l: NonCopy, r: NonCopy) -> NonCopy {
        let ret = konst::max_by_key!(l.copy(), r.copy(), mod10);

        assert!(konst::max_by_key!(l.0, r.0, |n| *n % 10) == ret.0);
        assert!(konst::max_by_key!(l.0, r.0, |n: &u32| *n % 10) == ret.0);
        assert!(konst::max_by_key!(&l, &r, |NonCopy(n)| *n % 10).0 == ret.0);

        ret
    }

    assert_eq!(maxx(NonCopy(3), NonCopy(5)), NonCopy(5));
    assert_eq!(maxx(NonCopy(3), NonCopy(15)), NonCopy(15));
    assert_eq!(maxx(NonCopy(13), NonCopy(5)), NonCopy(5));
    assert_eq!(maxx(NonCopy(13), NonCopy(15)), NonCopy(15));

    // max returns the first argument when both arguments compare equal
    assert_eq!(maxx(NonCopy(5), NonCopy(15)), NonCopy(15));
    assert_eq!(maxx(NonCopy(15), NonCopy(5)), NonCopy(5));

    assert_eq!(maxx(NonCopy(5), NonCopy(3)), NonCopy(5));
    assert_eq!(maxx(NonCopy(5), NonCopy(13)), NonCopy(5));
    assert_eq!(maxx(NonCopy(15), NonCopy(3)), NonCopy(15));
    assert_eq!(maxx(NonCopy(15), NonCopy(13)), NonCopy(15));
}
