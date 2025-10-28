type Array = [String; 4];

const fn to_bar(foo: [u32; 1]) -> u32 {
    konst::destructure_rec!{ [bar, baz] = foo }
    bar
}

const fn to_bar_rem(foo: [u32; 1]) -> u32 {
    konst::destructure_rec!{ [bar, baz, qux @ ..] = foo }
    bar
}

fn main(){}