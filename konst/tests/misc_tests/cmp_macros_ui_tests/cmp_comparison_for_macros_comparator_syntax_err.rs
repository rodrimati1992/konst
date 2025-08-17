use konst::cmp;

use std::cmp::Ordering as Orde;

const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8] asdasodijasd);
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8] asdasodijasd);
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8] asdasodijasd);
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8] asdasodijasd);
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8] asdasodijasd);
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8] asdasodijasd);
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8] asdasodijasd);

// type annotations aren't supported
const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], |x: &u32| 0u8);
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], |x: &u32| 0u8);
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], |x: &u32| 0u8);
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], |x: &u32| 0u8);
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], |x: &u32| 0u8);
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], |x: &u32| 0u8);
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], |x: &u32| 0u8);


const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], |x| );
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], |x| );
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], |x| );
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], |x| );
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], |x| );
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], |x| );
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], |x| );

// type annotations aren't supported
const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], |x: &u32, y: &u32| 0u8);
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], |x: &u32, y: &u32| 0u8);
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], |x: &u32, y: &u32| 0u8);
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], |x: &u32, y: &u32| 0u8);
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], |x: &u32, y: &u32| 0u8);
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], |x: &u32, y: &u32| 0u8);
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], |x: &u32, y: &u32| 0u8);


const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], |x, y| );
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], |x, y| );
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], |x, y| );
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], |x, y| );
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], |x, y| );
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], |x, y| );
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], |x, y| );

const _: bool = cmp::const_eq_for!(slice; [0u8], [3u8], -);
const _: bool = cmp::const_ne_for!(slice; [0u8], [3u8], -);
const _: Orde = cmp::const_cmp_for!(slice; [0u8], [3u8], -);
const _: bool = cmp::const_lt_for!(slice; [0u8], [3u8], -);
const _: bool = cmp::const_le_for!(slice; [0u8], [3u8], -);
const _: bool = cmp::const_gt_for!(slice; [0u8], [3u8], -);
const _: bool = cmp::const_ge_for!(slice; [0u8], [3u8], -);

fn main() {}