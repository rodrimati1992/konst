const _: () = {
    konst::iter::collect_const!{() => 0..10,all(|_|false)};
};

const _: () = {
    konst::iter::collect_const!{() =>};
};

const _: () = {
    konst::iter::collect_const!{};
};

fn main(){}