use konst::cmp;

use std::cmp::Ordering;

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

fn units((): (), (): ()) -> () { () }
fn unit((): ()) -> () { () }


const _: NotCmp = cmp::min!(NotCmp, NotCmp);
const _: NotCmp = cmp::max!(NotCmp, NotCmp);

const _: BadCmp = cmp::min!(BadCmp, BadCmp);
const _: BadCmp = cmp::max!(BadCmp, BadCmp);

const _: u8 = cmp::min!(0u8, 0u16);
const _: u8 = cmp::max!(0u8, 0u16);

const _: () = cmp::min!(0u8, 0u8);
const _: () = cmp::max!(0u8, 0u8);

const _: u8 = cmp::min_by!(0u8, 3u8, |0u16.., _| Ordering::Equal);
const _: u8 = cmp::min_by!(0u8, 3u8, |_, 0u16..| Ordering::Equal);
const _: u8 = cmp::min_by!(0u8, 3u8, |_, _| false);
const _: () = cmp::min_by!(0u8, 3u8, |_, _| Ordering::Equal);
const _: u8 = cmp::min_by!(0u8, 3u8, units);

const _: u8 = cmp::max_by!(0u8, 3u8, |0u16.., _| Ordering::Equal);
const _: u8 = cmp::max_by!(0u8, 3u8, |_, 0u16..| Ordering::Equal);
const _: u8 = cmp::max_by!(0u8, 3u8, |_, _| false);
const _: () = cmp::max_by!(0u8, 3u8, |_, _| Ordering::Equal);
const _: u8 = cmp::max_by!(0u8, 3u8, units);

const _: u8 = cmp::min_by_key!(0u8, 3u8, |0u16..| 0u8);
const _: u8 = cmp::min_by_key!(0u8, 3u8, |_| NotCmp);
const _: u8 = cmp::min_by_key!(0u8, 3u8, |_| BadCmp);
const _: () = cmp::min_by_key!(0u8, 3u8, |x| *x);
const _: u8 = cmp::min_by_key!(0u8, 3u8, |_| -> u32 { 0u8 });
const _: u8 = cmp::min_by_key!(0u8, 3u8, unit);

const _: u8 = cmp::max_by_key!(0u8, 3u8, |0u16..| 0u8);
const _: u8 = cmp::max_by_key!(0u8, 3u8, |_| NotCmp);
const _: u8 = cmp::max_by_key!(0u8, 3u8, |_| BadCmp);
const _: () = cmp::max_by_key!(0u8, 3u8, |x| *x);
const _: u8 = cmp::max_by_key!(0u8, 3u8, |_| -> u32 { 0u8 });
const _: u8 = cmp::max_by_key!(0u8, 3u8, unit);

fn main() {}








