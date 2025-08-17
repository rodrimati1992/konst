use konst::cmp;

use core::cmp::Ordering;


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


const _: bool = cmp::const_eq!(0u8, ());
const _: () = cmp::const_eq!(0u8, 0u8);
const _: () = { cmp::const_eq!(BadCmp, BadCmp); };
const _: () = cmp::const_eq!(BadCmp, BadCmp);

const _: bool = cmp::const_ne!(0u8, ());
const _: () = cmp::const_ne!(0u8, 0u8);
const _: () = { cmp::const_ne!(BadCmp, BadCmp); };
const _: () = cmp::const_ne!(BadCmp, BadCmp);

const _: Ordering = cmp::const_cmp!(0u8, ());
const _: () = cmp::const_cmp!(0u8, 0u8);
const _: () = { cmp::const_cmp!(BadCmp, BadCmp); };
const _: () = cmp::const_cmp!(BadCmp, BadCmp);

const _: bool = cmp::const_lt!(0u8, ());
const _: () = cmp::const_lt!(0u8, 0u8);
const _: () = { cmp::const_lt!(BadCmp, BadCmp); };
const _: () = cmp::const_lt!(BadCmp, BadCmp);

const _: bool = cmp::const_le!(0u8, ());
const _: () = cmp::const_le!(0u8, 0u8);
const _: () = { cmp::const_le!(BadCmp, BadCmp); };
const _: () = cmp::const_le!(BadCmp, BadCmp);

const _: bool = cmp::const_gt!(0u8, ());
const _: () = cmp::const_gt!(0u8, 0u8);
const _: () = { cmp::const_gt!(BadCmp, BadCmp); };
const _: () = cmp::const_gt!(BadCmp, BadCmp);

const _: bool = cmp::const_ge!(0u8, ());
const _: () = cmp::const_ge!(0u8, 0u8);
const _: () = { cmp::const_ge!(BadCmp, BadCmp); };
const _: () = cmp::const_ge!(BadCmp, BadCmp);


fn main() {}