use crate::misc_tests::test_utils::assert_type;

use std::cmp::Ordering::{self, Equal, Greater, Less};

use konst::cmp::{CmpWrapper, coerce_to_cmp, impl_cmp, try_equal};

struct BadCmp;

impl_cmp! {
    impl BadCmp;
}

#[test]
fn test_coerce_to_cmp() {
    assert_type::<_, &CmpWrapper<u8>>(&coerce_to_cmp!(0u8));
    assert_type::<_, &CmpWrapper<u8>>(&coerce_to_cmp!(&0u8));
    assert_type::<_, &CmpWrapper<u8>>(&coerce_to_cmp!(&&0u8));

    assert_type::<_, &BadCmp>(&coerce_to_cmp!(BadCmp));
    assert_type::<_, &BadCmp>(&coerce_to_cmp!(&BadCmp));
    assert_type::<_, &BadCmp>(&coerce_to_cmp!(&&BadCmp));
}

#[test]
fn test_try_equal() {
    const fn with_three(arr: [Ordering; 3]) -> Ordering {
        assert!(matches!(try_equal!(arr[0]), Equal));
        assert!(matches!(try_equal!(arr[1]), Equal));
        try_equal!(arr[2])
    }

    assert_eq!(with_three([Equal, Equal, Equal]), Equal);

    assert_eq!(with_three([Less, Equal, Equal]), Less);
    assert_eq!(with_three([Equal, Less, Equal]), Less);
    assert_eq!(with_three([Equal, Equal, Less]), Less);

    assert_eq!(with_three([Greater, Equal, Equal]), Greater);
    assert_eq!(with_three([Equal, Greater, Equal]), Greater);
    assert_eq!(with_three([Equal, Equal, Greater]), Greater);
}
