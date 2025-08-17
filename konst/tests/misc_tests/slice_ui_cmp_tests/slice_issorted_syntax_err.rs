use konst::slice;

const _: bool = slice::is_sorted!();

const _: bool = slice::is_sorted_by!();
const _: bool = slice::is_sorted_by!([0u8; 0], |);
const _: bool = slice::is_sorted_by!([0u8; 0], ||);
const _: bool = slice::is_sorted_by!([0u8; 0], || false);
const _: bool = slice::is_sorted_by!([0u8; 0], |x| false);
const _: bool = slice::is_sorted_by!([0u8; 0], |x, y|);

const _: bool = slice::is_sorted_by_key!();
const _: bool = slice::is_sorted_by_key!([0u8; 0], |);
const _: bool = slice::is_sorted_by_key!([0u8; 0], ||);
const _: bool = slice::is_sorted_by_key!([0u8; 0], || false);
const _: bool = slice::is_sorted_by_key!([0u8; 0], |x|);




fn main() {}