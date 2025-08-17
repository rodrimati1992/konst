struct NoCmp;

const _: bool  = konst::iter::eval!{0..10u8, is_sorted()};
const _: ()  = konst::iter::eval!{0..10u8, is_sorted()};

const _: bool = konst::iter::eval!{0..10u8, is_sorted_by(|x, y| 0u8)};
const _: bool = konst::iter::eval!{0..10u8, is_sorted_by(|x, y| -> u8 { 0u8 })};
const _: bool = konst::iter::eval!{0..10u8, is_sorted_by(|x: &u16, y| false)};
const _: bool = konst::iter::eval!{0..10u8, is_sorted_by(|x, y: &u16| false)};
const _: () = konst::iter::eval!{0..10u8, is_sorted_by(|x, y| false)};
const _: () = konst::iter::eval!{0..10u8, is_sorted_by(|x, y| -> bool { false })};

const _: bool = konst::iter::eval!{0..10u8, is_sorted_by_key(|x| NoCmp)};
const _: bool = konst::iter::eval!{0..10u8, is_sorted_by_key(|_: &u16| 0u8)};
const _: () = konst::iter::eval!{0..10u8, is_sorted_by_key(|x| x)};




fn main() {}