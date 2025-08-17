use konst::cmp;

use std::cmp::Ordering as Orde;

struct NotCmp;

struct BadCmp;

cmp::impl_cmp! {
    impl BadCmp;

    pub const fn const_eq(&self, _other: &Self) -> u8 {
        0u8
    }
    pub const fn const_cmp(&self, _other: &Self) -> u8 {
        0u8
    }
}



const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], |0u16..| 0u8);
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], |0u16..| 0u8);
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], |0u16..| 0u8);
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], |0u16..| 0u8);
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], |0u16..| 0u8);
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], |0u16..| 0u8);
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], |0u16..| 0u8);

const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], |x| NotCmp);
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], |x| NotCmp);
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], |x| NotCmp);
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], |x| NotCmp);
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], |x| NotCmp);
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], |x| NotCmp);
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], |x| NotCmp);

const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], |x| BadCmp);
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], |x| BadCmp);
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], |x| BadCmp);
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], |x| BadCmp);
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], |x| BadCmp);
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], |x| BadCmp);
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], |x| BadCmp);

fn main() {}