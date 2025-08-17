use core::ops::{Range, RangeInclusive};

macro_rules! declare_range_cmp_fns {
    (
        ($type:ty, ($eq_fn_name:ident))

        docs( $docs_eq:expr, $docs_cmp:expr,)
    ) => {
        __delegate_const_eq! {
            skip_coerce;
            #[doc = $docs_eq]
            pub const fn $eq_fn_name(ref left: $type, right: &$type) -> bool {
                left.start == right.start && left.end == right.end
            }
        }
    };
}

impl<T> crate::cmp::ConstCmp for Range<T> {
    type Kind = crate::cmp::IsStdKind;
    type This = Self;
}

impl<T> crate::cmp::ConstCmp for RangeInclusive<T> {
    type Kind = crate::cmp::IsStdKind;
    type This = Self;
}

__declare_fns_with_docs! {
    (Range<u8>,    (eq_range_u8))
    (Range<u16>,   (eq_range_u16))
    (Range<u32>,   (eq_range_u32))
    (Range<u64>,   (eq_range_u64))
    (Range<u128>,  (eq_range_u128))
    (Range<usize>,  (eq_range_usize))
    (Range<i8>,    (eq_range_i8))
    (Range<i16>,   (eq_range_i16))
    (Range<i32>,   (eq_range_i32))
    (Range<i64>,   (eq_range_i64))
    (Range<i128>,  (eq_range_i128))
    (Range<isize>, (eq_range_isize))

    (Range<char>, (eq_range_char))

    docs(default)

    macro = declare_range_cmp_fns!(),
}

macro_rules! declare_rangeinclusive_cmp_fns {
    (
        ($type:ty, ($eq_fn_name:ident))

        docs( $docs_eq:expr, $docs_cmp:expr,)
    ) => {
        __delegate_const_eq! {
            skip_coerce;
            #[doc = $docs_eq]
            pub const fn $eq_fn_name(ref left: $type, right: &$type) -> bool {
                *left.start() == *right.start() && *left.end() == *right.end()
            }
        }
    };
}

__declare_fns_with_docs! {
    (RangeInclusive<u8>,    (eq_rangeinc_u8,))
    (RangeInclusive<u16>,   (eq_rangeinc_u16))
    (RangeInclusive<u32>,   (eq_rangeinc_u32))
    (RangeInclusive<u64>,   (eq_rangeinc_u64))
    (RangeInclusive<u128>,  (eq_rangeinc_u128))
    (RangeInclusive<usize>, (eq_rangeinc_usize))
    (RangeInclusive<i8>,    (eq_rangeinc_i8,))
    (RangeInclusive<i16>,   (eq_rangeinc_i16))
    (RangeInclusive<i32>,   (eq_rangeinc_i32))
    (RangeInclusive<i64>,   (eq_rangeinc_i64))
    (RangeInclusive<i128>,  (eq_rangeinc_i128))
    (RangeInclusive<isize>, (eq_rangeinc_isize))

    (RangeInclusive<char>, (eq_rangeinc_char))

    docs(default)

    macro = declare_rangeinclusive_cmp_fns!(),
}
