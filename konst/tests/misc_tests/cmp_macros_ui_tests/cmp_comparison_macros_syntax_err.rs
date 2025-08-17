use konst::cmp;

use core::cmp::Ordering;


const _: bool = cmp::const_eq!();
const _: bool = cmp::const_eq!(0u8);
const _: bool = cmp::const_eq!(0u8,);

const _: bool = cmp::const_ne!();
const _: bool = cmp::const_ne!(0u8);
const _: bool = cmp::const_ne!(0u8,);

const _: Ordering = cmp::const_cmp!();
const _: Ordering = cmp::const_cmp!(0u8);
const _: Ordering = cmp::const_cmp!(0u8, );

const _: bool = cmp::const_lt!();
const _: bool = cmp::const_lt!(0u8);
const _: bool = cmp::const_lt!(0u8,);

const _: bool = cmp::const_le!();
const _: bool = cmp::const_le!(0u8);
const _: bool = cmp::const_le!(0u8,);

const _: bool = cmp::const_gt!();
const _: bool = cmp::const_gt!(0u8);
const _: bool = cmp::const_gt!(0u8,);

const _: bool = cmp::const_ge!();
const _: bool = cmp::const_ge!(0u8);
const _: bool = cmp::const_ge!(0u8,);



fn main() {}