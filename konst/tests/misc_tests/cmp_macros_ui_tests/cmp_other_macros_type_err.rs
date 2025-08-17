use konst::cmp;

use std::cmp::Ordering::Equal;

struct NotCmp;

const _: () = {
    let _ = cmp::coerce_to_cmp!(NotCmp);

    let _ = || cmp::try_equal!(0u8);
    let _ = || -> () { cmp::try_equal!(Equal) };
    let _ = || -> () { cmp::try_equal!(Equal); };
};

fn main() {}