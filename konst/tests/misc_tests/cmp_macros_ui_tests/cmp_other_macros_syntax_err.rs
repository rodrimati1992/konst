use konst::cmp;

const _: () = {
    let _ = cmp::coerce_to_cmp!();
    let _ = || cmp::try_equal!();
};

cmp::impl_cmp!{}

cmp::impl_cmp!{impl}


fn main() {}