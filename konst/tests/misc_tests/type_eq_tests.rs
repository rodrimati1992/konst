use konst::polymorphism::TypeEq;

type TypeEqA<T, const N: usize, const M: usize> = TypeEq<[T; N], [T; M]>;

#[test]
fn maybe_same_array_len_test() {
    assert!(TypeEqA::<&str, 0, 0>::MAYBE_SAME_ARRAY_LEN.is_some());
    assert!(TypeEqA::<&str, 1, 1>::MAYBE_SAME_ARRAY_LEN.is_some());
    assert!(TypeEqA::<&str, 2, 2>::MAYBE_SAME_ARRAY_LEN.is_some());

    assert!(TypeEqA::<&str, 0, 1>::MAYBE_SAME_ARRAY_LEN.is_none());
    assert!(TypeEqA::<&str, 0, 2>::MAYBE_SAME_ARRAY_LEN.is_none());

    assert!(TypeEqA::<&str, 1, 0>::MAYBE_SAME_ARRAY_LEN.is_none());
    assert!(TypeEqA::<&str, 1, 2>::MAYBE_SAME_ARRAY_LEN.is_none());

    assert!(TypeEqA::<&str, 2, 0>::MAYBE_SAME_ARRAY_LEN.is_none());
    assert!(TypeEqA::<&str, 2, 1>::MAYBE_SAME_ARRAY_LEN.is_none());
}
