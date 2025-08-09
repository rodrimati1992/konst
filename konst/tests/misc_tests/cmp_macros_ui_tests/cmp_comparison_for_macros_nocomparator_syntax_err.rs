use konst::cmp;

use std::cmp::Ordering as Orde;


const _: bool = cmp::const_eq_for!();
const _: bool = cmp::const_ne_for!();
const _: Orde = cmp::const_cmp_for!();
const _: bool = cmp::const_lt_for!();
const _: bool = cmp::const_le_for!();
const _: bool = cmp::const_gt_for!();
const _: bool = cmp::const_ge_for!();

const _: bool = cmp::const_eq_for!(;);
const _: bool = cmp::const_ne_for!(;);
const _: Orde = cmp::const_cmp_for!(;);
const _: bool = cmp::const_lt_for!(;);
const _: bool = cmp::const_le_for!(;);
const _: bool = cmp::const_gt_for!(;);
const _: bool = cmp::const_ge_for!(;);

const _: bool = cmp::const_eq_for!(abcd;);
const _: bool = cmp::const_ne_for!(abcd;);
const _: Orde = cmp::const_cmp_for!(abcd;);
const _: bool = cmp::const_lt_for!(abcd;);
const _: bool = cmp::const_le_for!(abcd;);
const _: bool = cmp::const_gt_for!(abcd;);
const _: bool = cmp::const_ge_for!(abcd;);

const _: bool = cmp::const_eq_for!(slice; [0u8]);
const _: bool = cmp::const_ne_for!(slice; [0u8]);
const _: Orde = cmp::const_cmp_for!(slice; [0u8]);
const _: bool = cmp::const_lt_for!(slice; [0u8]);
const _: bool = cmp::const_le_for!(slice; [0u8]);
const _: bool = cmp::const_gt_for!(slice; [0u8]);
const _: bool = cmp::const_ge_for!(slice; [0u8]);

const _: bool = cmp::const_eq_for!(slice; [0u8],);
const _: bool = cmp::const_ne_for!(slice; [0u8],);
const _: Orde = cmp::const_cmp_for!(slice; [0u8],);
const _: bool = cmp::const_lt_for!(slice; [0u8],);
const _: bool = cmp::const_le_for!(slice; [0u8],);
const _: bool = cmp::const_gt_for!(slice; [0u8],);
const _: bool = cmp::const_ge_for!(slice; [0u8],);

