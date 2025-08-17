const _: std::cmp::Ordering = konst::iter::eval!{0..10, cmp()};

const _: bool = konst::iter::eval!{0..10, eq()};

const _: bool = konst::iter::eval!{0..10, ne()};

const _: bool = konst::iter::eval!{0..10, ge()};

const _: bool = konst::iter::eval!{0..10, gt()};

const _: bool = konst::iter::eval!{0..10, le()};

const _: bool = konst::iter::eval!{0..10, lt()};

const _: bool = konst::iter::eval!{0..10, is_sorted_by()};
const _: bool = konst::iter::eval!{0..10, is_sorted_by([0u8; 0], |)};
const _: bool = konst::iter::eval!{0..10, is_sorted_by([0u8; 0], ||)};
const _: bool = konst::iter::eval!{0..10, is_sorted_by([0u8; 0], || false)};
const _: bool = konst::iter::eval!{0..10, is_sorted_by([0u8; 0], |x| false)};
const _: bool = konst::iter::eval!{0..10, is_sorted_by([0u8; 0], |x, y|)};

const _: bool = konst::iter::eval!{0..10, is_sorted_by_key()};
const _: bool = konst::iter::eval!{0..10, is_sorted_by_key(|)};
const _: bool = konst::iter::eval!{0..10, is_sorted_by_key(||)};
const _: bool = konst::iter::eval!{0..10, is_sorted_by_key(|| false)};
const _: bool = konst::iter::eval!{0..10, is_sorted_by_key(|x|)};

const _: Option<i32> = konst::iter::eval!{0..10, min_by()};

const _: Option<i32> = konst::iter::eval!{0..10, min_by_key()};

const _: Option<i32> = konst::iter::eval!{0..10, max_by()};

const _: Option<i32> = konst::iter::eval!{0..10, max_by_key()};


fn main(){}