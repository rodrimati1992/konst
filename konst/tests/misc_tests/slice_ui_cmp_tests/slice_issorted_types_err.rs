use konst::slice;

struct NoCmp;

const _: bool = slice::is_sorted!([NoCmp]);
const _: () = slice::is_sorted!([0u8]);

const _: bool = slice::is_sorted_by!([0u8; 0], |x, y| -> u8 { 0u8 });
const _: bool = slice::is_sorted_by!([0u8; 0], |x, y| 0u8);
const _: bool = slice::is_sorted_by!([0u8; 0], |x: &u16, y| false);
const _: bool = slice::is_sorted_by!([0u8; 0], |x, y: &u16| false);
const _: () = slice::is_sorted_by!([0u8; 0], |x, y| -> bool { false });

const _: bool = slice::is_sorted_by_key!([0u8; 0], |x| NoCmp);
const _: bool = slice::is_sorted_by_key!([0u8; 0], |_: &u16| 0u8);
const _: () = slice::is_sorted_by_key!([0u8; 0], |x| x);




fn main() {}