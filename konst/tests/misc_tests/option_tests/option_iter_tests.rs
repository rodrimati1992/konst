use konst::option;

#[test]
fn iter_test() {
    {
        let mut iter: option::Iter<_> = option::iter(&None::<()>);
        assert_eq!(iter.next(), None);
    }

    {
        let mut iter: option::Iter<_> = option::iter(&Some(3));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
    {
        let mut iter: option::Iter<_> = option::iter(&Some(3));
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next_back(), None);
    }
}

#[test]
fn iter_rev_test() {
    {
        let mut iter: option::IterRev<_> = option::iter(&None::<()>).rev();
        assert_eq!(iter.next(), None);
    }

    {
        let mut iter: option::IterRev<_> = option::iter(&Some(3)).rev();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
    {
        let mut iter: option::IterRev<_> = option::iter(&Some(5)).rev();
        assert_eq!(iter.next_back(), Some(&5));
        assert_eq!(iter.next_back(), None);
    }
}

#[test]
fn iter_mut_test() {
    {
        let mut opt = None::<()>;
        let mut iter: option::IterMut<_> = option::iter_mut(&mut opt);
        assert_eq!(iter.next(), None);
    }

    {
        let mut opt = Some(3);
        let mut iter: option::IterMut<_> = option::iter_mut(&mut opt);
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
    {
        let mut opt = Some(5);
        let mut iter: option::IterMut<_> = option::iter_mut(&mut opt);
        assert_eq!(iter.next_back(), Some(&mut 5));
        assert_eq!(iter.next_back(), None);
    }
}

#[test]
fn iter_mut_rev_test() {
    {
        let mut opt = None::<()>;
        let mut iter: option::IterMutRev<_> = option::iter_mut(&mut opt).rev();
        assert_eq!(iter.next(), None);
    }

    {
        let mut opt = Some(3);
        let mut iter: option::IterMutRev<_> = option::iter_mut(&mut opt).rev();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
    {
        let mut opt = Some(5);
        let mut iter: option::IterMutRev<_> = option::iter_mut(&mut opt).rev();
        assert_eq!(iter.next_back(), Some(&mut 5));
        assert_eq!(iter.next_back(), None);
    }
}

#[test]
fn into_iter_test() {
    macro_rules! test_case {($($ctor:tt)*) => {
        {
            let opt = None::<()>;
            let mut iter: option::IntoIter<_> = $($ctor)* (opt);
            assert_eq!(iter.next(), None);
        }

        {
            let opt = Some(3);
            let mut iter: option::IntoIter<_> = $($ctor)* (opt);
            assert_eq!(iter.next(), Some(3));
            assert_eq!(iter.next(), None);
        }
        {
            let opt = Some(5);
            let mut iter: option::IntoIter<_> = $($ctor)* (opt);
            assert_eq!(iter.next_back(), Some(5));
            assert_eq!(iter.next_back(), None);
        }
    }}

    test_case! {option::into_iter}
    test_case! {konst::iter::into_iter!}
}

#[test]
fn into_iter_rev_test() {
    {
        let opt = None::<()>;
        let mut iter: option::IntoIterRev<_> = option::into_iter(opt).rev();
        assert_eq!(iter.next(), None);
    }

    {
        let opt = Some(3);
        let mut iter: option::IntoIterRev<_> = option::into_iter(opt).rev();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
    {
        let opt = Some(5);
        let mut iter: option::IntoIterRev<_> = option::into_iter(opt).rev();
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next_back(), None);
    }
}
