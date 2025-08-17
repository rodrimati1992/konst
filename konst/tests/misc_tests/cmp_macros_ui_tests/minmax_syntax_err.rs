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

const _: u8 = cmp::min!();
const _: u8 = cmp::min!(0u8);
const _: u8 = cmp::min!(0u8,);

const _: u8 = cmp::max!();
const _: u8 = cmp::max!(0u8);
const _: u8 = cmp::max!(0u8, );

const _: u8 = cmp::min_by!();
const _: u8 = cmp::min_by!(0u8);
const _: u8 = cmp::min_by!(0u8,);
const _: u8 = cmp::min_by!(0u8, 3u8);
const _: u8 = cmp::min_by!(0u8, 3u8,);
const _: u8 = cmp::min_by!(0u8, 3u8, || Ordering::Equal);
const _: u8 = cmp::min_by!(0u8, 3u8, |_| Ordering::Equal);
const _: u8 = cmp::min_by!(0u8, 3u8, |_, _| );

const _: u8 = cmp::max_by!();
const _: u8 = cmp::max_by!(0u8);
const _: u8 = cmp::max_by!(0u8,);
const _: u8 = cmp::max_by!(0u8, 3u8);
const _: u8 = cmp::max_by!(0u8, 3u8,);
const _: u8 = cmp::max_by!(0u8, 3u8, || Ordering::Equal);
const _: u8 = cmp::max_by!(0u8, 3u8, |_| Ordering::Equal);
const _: u8 = cmp::max_by!(0u8, 3u8, |_, _| );

const _: u8 = cmp::min_by_key!();
const _: u8 = cmp::min_by_key!(0u8);
const _: u8 = cmp::min_by_key!(0u8,);
const _: u8 = cmp::min_by_key!(0u8, 3u8);
const _: u8 = cmp::min_by_key!(0u8, 3u8,);
const _: u8 = cmp::min_by_key!(0u8, 3u8, || );
const _: u8 = cmp::min_by_key!(0u8, 3u8, |_| );

const _: u8 = cmp::max_by_key!();
const _: u8 = cmp::max_by_key!(0u8);
const _: u8 = cmp::max_by_key!(0u8,);
const _: u8 = cmp::max_by_key!(0u8, 3u8);
const _: u8 = cmp::max_by_key!(0u8, 3u8,);
const _: u8 = cmp::max_by_key!(0u8, 3u8, || );
const _: u8 = cmp::max_by_key!(0u8, 3u8, |_| );

fn main() {}








