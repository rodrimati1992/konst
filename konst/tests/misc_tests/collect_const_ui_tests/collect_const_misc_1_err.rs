const _: [u8; 10] = {
    konst::iter::collect_const!{usize => 0usize..10}
};

const _: [usize; 9] = {
    konst::iter::collect_const!{usize => 0usize..10}
};

fn main(){}