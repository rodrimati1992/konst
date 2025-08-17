use konst::cmp;

use std::cmp::Ordering as Orde;

fn units((): (), (): ()) -> () { () }

struct BadCmp;

const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], |0u16.., y| 0u8);
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], |0u16.., y| 0u8);
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], |0u16.., y| 0u8);
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], |0u16.., y| 0u8);
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], |0u16.., y| 0u8);
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], |0u16.., y| 0u8);
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], |0u16.., y| 0u8);

const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], |_, 0u16..| 0u8);
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], |_, 0u16..| 0u8);
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], |_, 0u16..| 0u8);
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], |_, 0u16..| 0u8);
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], |_, 0u16..| 0u8);
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], |_, 0u16..| 0u8);
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], |_, 0u16..| 0u8);

const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], |x, y| 0u8);
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], |x, y| 0u8);
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], |x, y| 0u8);
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], |x, y| 0u8);
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], |x, y| 0u8);
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], |x, y| 0u8);
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], |x, y| 0u8);


const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], units);
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], units);
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], units);
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], units);
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], units);
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], units);
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], units);



fn main() {}