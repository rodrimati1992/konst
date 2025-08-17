const _: std::cmp::Ordering = konst::iter::eval!{0..10, cmp(0..10)};

const _: bool = konst::iter::eval!{0..10, eq(0..10)};

const _: bool = konst::iter::eval!{0..10, ne(0..10)};

const _: bool = konst::iter::eval!{0..10, ge(0..10)};

const _: bool = konst::iter::eval!{0..10, gt(0..10)};

const _: bool = konst::iter::eval!{0..10, le(0..10)};

const _: bool = konst::iter::eval!{0..10, lt(0..10)};

const _: bool = konst::iter::eval!{0..10, is_sorted()};

const _: bool = konst::iter::eval!{0..10, is_sorted_by(|l, r| true)};

const _: bool = konst::iter::eval!{0..10, is_sorted_by_key(|x| x)};

const _: Option<i32> = konst::iter::eval!{0..10, min_by(|l, r| std::cmp::Ordering::Equal)};

const _: Option<i32> = konst::iter::eval!{0..10, min_by_key(|x| x)};

const _: Option<i32> = konst::iter::eval!{0..10, max_by(|l, r| std::cmp::Ordering::Equal)};

const _: Option<i32> = konst::iter::eval!{0..10, max_by_key(|x| x)};


fn main(){}